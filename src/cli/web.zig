const std = @import("std");
const Allocator = std.mem.Allocator;
const posix = std.posix;
const args_mod = @import("args.zig");
const Action = @import("ghostty.zig").Action;

pub const Options = struct {
    pub fn deinit(self: Options) void {
        _ = self;
    }

    /// Enables `-h` and `--help` to work.
    pub fn help(self: Options) !void {
        _ = self;
        return Action.help_error;
    }
};

/// The `web` command manages browser panes in TermSurf.
///
/// Subcommands:
///   open [options] [url]    Open a URL in a browser pane
///   file <filename>         Open a local file in a browser pane
///   close [webview-id]      Close a browser pane
///   ping                    Test connectivity to TermSurf
///   bookmark <subcommand>   Manage bookmarks
///   help                    Show help message
///
/// This command requires running inside TermSurf (TERMSURF_SOCKET must be set).
pub fn run(alloc: Allocator) !u8 {
    // Get all arguments
    var iter = try std.process.argsWithAllocator(alloc);
    defer iter.deinit();

    // Skip the executable name (e.g., "termsurf" or "web")
    _ = iter.next();

    // Collect remaining arguments, skipping "+web" if present
    // (When invoked via symlink "web", there's no "+web" in argv)
    var argsList: std.ArrayList([]const u8) = .empty;
    defer argsList.deinit(alloc);

    while (iter.next()) |arg| {
        // Skip the "+web" flag if present (explicit invocation)
        if (std.mem.eql(u8, arg, "+web")) {
            continue;
        }
        try argsList.append(alloc, arg);
    }

    const args = argsList.items;

    // No subcommand: open default homepage
    if (args.len < 1) {
        return cmdOpen(alloc, &.{});
    }

    const command = args[0];

    if (std.mem.eql(u8, command, "ping")) {
        return cmdPing(alloc);
    } else if (std.mem.eql(u8, command, "open")) {
        return cmdOpen(alloc, args[1..]);
    } else if (std.mem.eql(u8, command, "file")) {
        return cmdFile(alloc, args[1..]);
    } else if (std.mem.eql(u8, command, "close")) {
        return cmdClose(alloc, args[1..]);
    } else if (std.mem.eql(u8, command, "bookmark")) {
        return cmdBookmark(alloc, args[1..]);
    } else if (std.mem.eql(u8, command, "help") or std.mem.eql(u8, command, "--help") or std.mem.eql(u8, command, "-h")) {
        return Action.help_error;
    } else {
        // Unknown command: treat as URL (implicit open)
        return cmdOpen(alloc, args[0..]);
    }
}

// MARK: - Commands

fn cmdPing(allocator: Allocator) !u8 {
    const response = try sendPingRequest(allocator);
    defer allocator.free(response);

    // Parse response to check status
    const parsed = try std.json.parseFromSlice(Response, allocator, response, .{
        .ignore_unknown_fields = true,
    });
    defer parsed.deinit();

    if (std.mem.eql(u8, parsed.value.status, "ok")) {
        var buffer: [256]u8 = undefined;
        var stdout_writer = std.fs.File.stdout().writer(&buffer);
        const stdout = &stdout_writer.interface;
        try stdout.writeAll("pong\n");
        try stdout.flush();
        return 0;
    } else {
        var buffer: [1024]u8 = undefined;
        var stderr_writer = std.fs.File.stderr().writer(&buffer);
        const stderr = &stderr_writer.interface;
        if (parsed.value.@"error") |err| {
            try stderr.print("Error: {s}\n", .{err});
        } else {
            try stderr.writeAll("Error: Unknown error\n");
        }
        try stderr.flush();
        return 1;
    }
}

fn cmdOpen(allocator: Allocator, args: []const []const u8) !u8 {
    var url_or_bookmark: ?[]const u8 = null;
    var profile: ?[]const u8 = null;
    var incognito = false;
    var jsApi = false;

    var i: usize = 0;
    while (i < args.len) : (i += 1) {
        const arg = args[i];
        if (std.mem.eql(u8, arg, "--js-api")) {
            jsApi = true;
        } else if (std.mem.eql(u8, arg, "--incognito")) {
            incognito = true;
        } else if (std.mem.eql(u8, arg, "--profile") or std.mem.eql(u8, arg, "-p")) {
            i += 1;
            if (i >= args.len) {
                std.debug.print("Error: --profile requires an argument\n", .{});
                return 1;
            }
            profile = args[i];
        } else if (!std.mem.startsWith(u8, arg, "-")) {
            url_or_bookmark = arg;
        } else {
            std.debug.print("Unknown option: {s}\n", .{arg});
            return 1;
        }
    }

    // Check for mutually exclusive options
    if (incognito and profile != null) {
        std.debug.print("Error: --incognito and --profile are mutually exclusive\n", .{});
        return 1;
    }

    // Resolve URL: either direct URL or bookmark lookup
    var url: ?[]const u8 = null;
    var resolved_url_buf: ?[]u8 = null;
    defer if (resolved_url_buf) |buf| allocator.free(buf);

    if (url_or_bookmark) |arg| {
        if (looksLikeUrl(arg)) {
            // Direct URL
            url = arg;
        } else {
            // Try to resolve as bookmark
            const bookmark_url = try resolveBookmark(allocator, arg, profile);
            if (bookmark_url) |resolved| {
                resolved_url_buf = resolved;
                url = resolved;
            } else {
                // Bookmark not found
                var stderr_buffer: [1024]u8 = undefined;
                var stderr_writer = std.fs.File.stderr().writer(&stderr_buffer);
                const stderr = &stderr_writer.interface;
                try stderr.print("Error: Bookmark '{s}' not found\n", .{arg});
                try stderr.flush();
                return 1;
            }
        }
    }

    // URL can be null - app will use default homepage

    // Get pane ID from environment
    const paneId = std.posix.getenv("TERMSURF_PANE_ID");

    // Get socket path from environment
    const socketPath = std.posix.getenv("TERMSURF_SOCKET") orelse {
        std.debug.print("Error: Not running inside TermSurf (TERMSURF_SOCKET not set)\n", .{});
        return 1;
    };

    // Connect to socket - keep connection open for event streaming
    const socket = try connectToSocket(socketPath);
    defer posix.close(socket);

    // Build and send request
    const request = try buildOpenRequest(allocator, url, paneId, profile, incognito, jsApi);
    defer allocator.free(request);
    _ = try posix.write(socket, request);

    // Read response
    const response = try readResponse(allocator, socket);
    defer allocator.free(response);

    // Parse response
    const parsed = try std.json.parseFromSlice(Response, allocator, response, .{
        .ignore_unknown_fields = true,
    });
    defer parsed.deinit();

    if (std.mem.eql(u8, parsed.value.status, "ok")) {
        // Extract webview ID for logging
        var webviewIdStr: ?[]const u8 = null;
        if (parsed.value.data) |dataObj| {
            if (dataObj.object.get("webviewId")) |wvId| {
                if (wvId == .string) {
                    webviewIdStr = wvId.string;
                }
            }
        }

        // Enter event loop - block and stream console output until webview closes
        return try eventLoop(allocator, socket, webviewIdStr);
    } else {
        var stderr_buffer: [1024]u8 = undefined;
        var stderr_writer = std.fs.File.stderr().writer(&stderr_buffer);
        const stderr = &stderr_writer.interface;

        if (parsed.value.@"error") |err| {
            try stderr.print("Error: {s}\n", .{err});
        } else {
            try stderr.writeAll("Error: Unknown error\n");
        }
        try stderr.flush();
        return 1;
    }
}

fn cmdFile(allocator: Allocator, args: []const []const u8) !u8 {
    var stderr_buffer: [1024]u8 = undefined;
    var stderr_writer = std.fs.File.stderr().writer(&stderr_buffer);
    const stderr = &stderr_writer.interface;

    var filepath: ?[]const u8 = null;
    var profile: ?[]const u8 = null;
    var incognito = false;
    var jsApi = false;

    var i: usize = 0;
    while (i < args.len) : (i += 1) {
        const arg = args[i];
        if (std.mem.eql(u8, arg, "--js-api")) {
            jsApi = true;
        } else if (std.mem.eql(u8, arg, "--incognito")) {
            incognito = true;
        } else if (std.mem.eql(u8, arg, "--profile") or std.mem.eql(u8, arg, "-p")) {
            i += 1;
            if (i >= args.len) {
                try stderr.writeAll("Error: --profile requires an argument\n");
                try stderr.flush();
                return 1;
            }
            profile = args[i];
        } else if (!std.mem.startsWith(u8, arg, "-")) {
            filepath = arg;
        } else {
            try stderr.print("Error: unknown option: {s}\n", .{arg});
            try stderr.flush();
            return 1;
        }
    }

    if (filepath == null) {
        try stderr.writeAll("Error: file command requires a filename\n");
        try stderr.writeAll("Usage: web file <filename> [--profile <name>] [--incognito] [--js-api]\n");
        try stderr.flush();
        return 1;
    }

    // Resolve to absolute path
    const abs_path = try resolveToAbsolutePath(allocator, filepath.?);
    defer allocator.free(abs_path);

    // Check if file exists (need null-terminated path for fs operations)
    const abs_path_z = try allocator.dupeZ(u8, abs_path);
    defer allocator.free(abs_path_z);

    std.fs.accessAbsolute(abs_path_z, .{}) catch {
        try stderr.print("Error: file not found: {s}\n", .{abs_path});
        try stderr.flush();
        return 1;
    };

    // Build file:// URL
    const file_url = try std.fmt.allocPrint(allocator, "file://{s}", .{abs_path});
    defer allocator.free(file_url);

    // Debug: print the URL being opened
    std.debug.print("Opening: {s}\n", .{file_url});

    // Get pane ID from environment
    const paneId = std.posix.getenv("TERMSURF_PANE_ID");

    // Get socket path from environment
    const socketPath = std.posix.getenv("TERMSURF_SOCKET") orelse {
        try stderr.writeAll("Error: Not running inside TermSurf (TERMSURF_SOCKET not set)\n");
        try stderr.flush();
        return 1;
    };

    // Connect to socket
    const socket = try connectToSocket(socketPath);
    defer posix.close(socket);

    // Build and send request
    const request = try buildOpenRequest(allocator, file_url, paneId, profile, incognito, jsApi);
    defer allocator.free(request);
    _ = try posix.write(socket, request);

    // Read response
    const response = try readResponse(allocator, socket);
    defer allocator.free(response);

    // Parse response
    const parsed = try std.json.parseFromSlice(Response, allocator, response, .{
        .ignore_unknown_fields = true,
    });
    defer parsed.deinit();

    if (std.mem.eql(u8, parsed.value.status, "ok")) {
        // Extract webview ID for logging
        var webviewIdStr: ?[]const u8 = null;
        if (parsed.value.data) |dataObj| {
            if (dataObj.object.get("webviewId")) |wvId| {
                if (wvId == .string) {
                    webviewIdStr = wvId.string;
                }
            }
        }

        // Enter event loop - block and stream console output until webview closes
        return try eventLoop(allocator, socket, webviewIdStr);
    } else {
        if (parsed.value.@"error") |err| {
            try stderr.print("Error: {s}\n", .{err});
        } else {
            try stderr.writeAll("Error: Unknown error\n");
        }
        try stderr.flush();
        return 1;
    }
}

/// Resolve a path to an absolute path
/// - Absolute paths (starting with /) are returned as-is
/// - Paths starting with ~ are expanded to home directory
/// - Relative paths are resolved against cwd
fn resolveToAbsolutePath(allocator: Allocator, path: []const u8) ![]u8 {
    // Already absolute
    if (path.len > 0 and path[0] == '/') {
        return try allocator.dupe(u8, path);
    }

    // Home directory expansion
    if (path.len > 0 and path[0] == '~') {
        const home = std.posix.getenv("HOME") orelse return error.HomeNotSet;
        if (path.len == 1) {
            return try allocator.dupe(u8, home);
        }
        // ~/ or ~/something
        if (path[1] == '/') {
            return try std.fmt.allocPrint(allocator, "{s}{s}", .{ home, path[1..] });
        }
        // ~username - not supported, treat as relative
    }

    // Relative path - resolve against cwd
    const cwd = try std.process.getCwdAlloc(allocator);
    defer allocator.free(cwd);

    if (path.len == 0 or std.mem.eql(u8, path, ".")) {
        return try allocator.dupe(u8, cwd);
    }

    // Handle ./ prefix
    const clean_path = if (std.mem.startsWith(u8, path, "./")) path[2..] else path;

    return try std.fs.path.join(allocator, &.{ cwd, clean_path });
}

/// Event loop: read events from socket and handle them
/// Returns exit code when webview closes
fn eventLoop(allocator: Allocator, socket: posix.socket_t, webviewId: ?[]const u8) !u8 {
    _ = webviewId; // May be used for logging in the future

    var buffer: [8192]u8 = undefined;
    var accumulated: std.ArrayListUnmanaged(u8) = .empty;
    defer accumulated.deinit(allocator);

    while (true) {
        const bytesRead = posix.read(socket, &buffer) catch |err| {
            // Connection closed or error
            if (err == error.BrokenPipe or err == error.ConnectionResetByPeer) {
                return 0;
            }
            return err;
        };

        if (bytesRead == 0) {
            // Server closed connection
            return 0;
        }

        try accumulated.appendSlice(allocator, buffer[0..bytesRead]);

        // Process all complete lines
        while (std.mem.indexOfScalar(u8, accumulated.items, '\n')) |newlineIdx| {
            const line = accumulated.items[0..newlineIdx];

            // Parse and handle the event
            const exitCode = try handleEvent(line);
            if (exitCode) |code| {
                return code;
            }

            // Remove processed line from buffer
            const remaining = accumulated.items[newlineIdx + 1 ..];
            std.mem.copyForwards(u8, accumulated.items[0..remaining.len], remaining);
            accumulated.shrinkRetainingCapacity(remaining.len);
        }
    }
}

/// Handle a single event, returns exit code if webview closed
fn handleEvent(line: []const u8) !?u8 {
    const parsed = std.json.parseFromSlice(Event, std.heap.page_allocator, line, .{
        .ignore_unknown_fields = true,
    }) catch {
        // Not a valid event, ignore
        return null;
    };
    defer parsed.deinit();

    const event = parsed.value;

    if (std.mem.eql(u8, event.event, "console")) {
        // Console output event
        if (event.data) |dataObj| {
            const level = if (dataObj.object.get("level")) |l| l.string else "log";
            const message = if (dataObj.object.get("message")) |m| m.string else "";

            // Route to stdout or stderr based on level
            if (std.mem.eql(u8, level, "error") or std.mem.eql(u8, level, "warn")) {
                var stderr_buffer: [8192]u8 = undefined;
                var stderr_writer = std.fs.File.stderr().writer(&stderr_buffer);
                const stderr = &stderr_writer.interface;
                try stderr.print("{s}\n", .{message});
                try stderr.flush();
            } else {
                var stdout_buffer: [8192]u8 = undefined;
                var stdout_writer = std.fs.File.stdout().writer(&stdout_buffer);
                const stdout = &stdout_writer.interface;
                try stdout.print("{s}\n", .{message});
                try stdout.flush();
            }
        }
        return null;
    } else if (std.mem.eql(u8, event.event, "closed")) {
        // Webview closed event - extract exit code and return
        var exitCode: u8 = 0;
        if (event.data) |dataObj| {
            if (dataObj.object.get("exitCode")) |code| {
                if (code == .integer) {
                    exitCode = @intCast(@max(0, @min(255, code.integer)));
                }
            }
        }
        return exitCode;
    }

    return null;
}

fn cmdClose(allocator: Allocator, args: []const []const u8) !u8 {
    const webviewId: ?[]const u8 = if (args.len > 0) args[0] else null;
    const paneId = std.posix.getenv("TERMSURF_PANE_ID");

    const response = try sendCloseRequest(allocator, paneId, webviewId);
    defer allocator.free(response);

    // Parse response
    const parsed = try std.json.parseFromSlice(Response, allocator, response, .{
        .ignore_unknown_fields = true,
    });
    defer parsed.deinit();

    var stdout_buffer: [256]u8 = undefined;
    var stdout_writer = std.fs.File.stdout().writer(&stdout_buffer);
    const stdout = &stdout_writer.interface;

    var stderr_buffer: [1024]u8 = undefined;
    var stderr_writer = std.fs.File.stderr().writer(&stderr_buffer);
    const stderr = &stderr_writer.interface;

    if (std.mem.eql(u8, parsed.value.status, "ok")) {
        try stdout.writeAll("Webview closed\n");
        try stdout.flush();
        return 0;
    } else {
        if (parsed.value.@"error") |err| {
            try stderr.print("Error: {s}\n", .{err});
        } else {
            try stderr.writeAll("Error: Unknown error\n");
        }
        try stderr.flush();
        return 1;
    }
}

fn cmdBookmark(allocator: Allocator, args: []const []const u8) !u8 {
    var stderr_buffer: [1024]u8 = undefined;
    var stderr_writer = std.fs.File.stderr().writer(&stderr_buffer);
    const stderr = &stderr_writer.interface;

    if (args.len < 1) {
        try stderr.writeAll("Error: bookmark requires a subcommand (add, get, list, update, delete)\n");
        try stderr.flush();
        return 1;
    }

    const subcommand = args[0];

    if (std.mem.eql(u8, subcommand, "add")) {
        return cmdBookmarkAdd(allocator, args[1..]);
    } else if (std.mem.eql(u8, subcommand, "get")) {
        return cmdBookmarkGet(allocator, args[1..]);
    } else if (std.mem.eql(u8, subcommand, "list")) {
        return cmdBookmarkList(allocator, args[1..]);
    } else if (std.mem.eql(u8, subcommand, "update")) {
        return cmdBookmarkUpdate(allocator, args[1..]);
    } else if (std.mem.eql(u8, subcommand, "delete")) {
        return cmdBookmarkDelete(allocator, args[1..]);
    } else {
        try stderr.print("Error: unknown bookmark subcommand: {s}\n", .{subcommand});
        try stderr.flush();
        return 1;
    }
}

fn cmdBookmarkAdd(allocator: Allocator, args: []const []const u8) !u8 {
    var stderr_buffer: [1024]u8 = undefined;
    var stderr_writer = std.fs.File.stderr().writer(&stderr_buffer);
    const stderr = &stderr_writer.interface;

    var name: ?[]const u8 = null;
    var url: ?[]const u8 = null;
    var title: ?[]const u8 = null;
    var profile: ?[]const u8 = null;

    var i: usize = 0;
    while (i < args.len) : (i += 1) {
        const arg = args[i];
        if (std.mem.eql(u8, arg, "--url")) {
            i += 1;
            if (i >= args.len) {
                try stderr.writeAll("Error: --url requires an argument\n");
                try stderr.flush();
                return 1;
            }
            url = args[i];
        } else if (std.mem.eql(u8, arg, "--name")) {
            i += 1;
            if (i >= args.len) {
                try stderr.writeAll("Error: --name requires an argument\n");
                try stderr.flush();
                return 1;
            }
            name = args[i];
        } else if (std.mem.eql(u8, arg, "--title")) {
            i += 1;
            if (i >= args.len) {
                try stderr.writeAll("Error: --title requires an argument\n");
                try stderr.flush();
                return 1;
            }
            title = args[i];
        } else if (std.mem.eql(u8, arg, "--profile") or std.mem.eql(u8, arg, "-p")) {
            i += 1;
            if (i >= args.len) {
                try stderr.writeAll("Error: --profile requires an argument\n");
                try stderr.flush();
                return 1;
            }
            profile = args[i];
        } else if (!std.mem.startsWith(u8, arg, "-")) {
            if (name == null) {
                name = arg;
            }
        } else {
            try stderr.print("Error: unknown option: {s}\n", .{arg});
            try stderr.flush();
            return 1;
        }
    }

    if (name == null) {
        try stderr.writeAll("Error: bookmark add requires a name\n");
        try stderr.flush();
        return 1;
    }

    if (url == null) {
        try stderr.writeAll("Error: bookmark add requires --url\n");
        try stderr.flush();
        return 1;
    }

    // Build and send request
    const response = try sendBookmarkRequest(allocator, "add", name.?, url, title, profile);
    defer allocator.free(response);

    // Parse and handle response
    return handleBookmarkResponse(response);
}

fn cmdBookmarkGet(allocator: Allocator, args: []const []const u8) !u8 {
    var stderr_buffer: [1024]u8 = undefined;
    var stderr_writer = std.fs.File.stderr().writer(&stderr_buffer);
    const stderr = &stderr_writer.interface;

    var name: ?[]const u8 = null;
    var profile: ?[]const u8 = null;

    var i: usize = 0;
    while (i < args.len) : (i += 1) {
        const arg = args[i];
        if (std.mem.eql(u8, arg, "--name")) {
            i += 1;
            if (i >= args.len) {
                try stderr.writeAll("Error: --name requires an argument\n");
                try stderr.flush();
                return 1;
            }
            name = args[i];
        } else if (std.mem.eql(u8, arg, "--profile") or std.mem.eql(u8, arg, "-p")) {
            i += 1;
            if (i >= args.len) {
                try stderr.writeAll("Error: --profile requires an argument\n");
                try stderr.flush();
                return 1;
            }
            profile = args[i];
        } else if (!std.mem.startsWith(u8, arg, "-")) {
            if (name == null) {
                name = arg;
            }
        } else {
            try stderr.print("Error: unknown option: {s}\n", .{arg});
            try stderr.flush();
            return 1;
        }
    }

    if (name == null) {
        try stderr.writeAll("Error: bookmark get requires a name\n");
        try stderr.flush();
        return 1;
    }

    // Build and send request
    const response = try sendBookmarkRequest(allocator, "get", name.?, null, null, profile);
    defer allocator.free(response);

    // Parse response
    const parsed = try std.json.parseFromSlice(Response, allocator, response, .{
        .ignore_unknown_fields = true,
    });
    defer parsed.deinit();

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_writer = std.fs.File.stdout().writer(&stdout_buffer);
    const stdout = &stdout_writer.interface;

    if (std.mem.eql(u8, parsed.value.status, "ok")) {
        // Print the URL
        if (parsed.value.data) |dataObj| {
            if (dataObj.object.get("url")) |urlVal| {
                if (urlVal == .string) {
                    try stdout.print("{s}\n", .{urlVal.string});
                    try stdout.flush();
                }
            }
        }
        return 0;
    } else {
        if (parsed.value.@"error") |err| {
            try stderr.print("Error: {s}\n", .{err});
        } else {
            try stderr.writeAll("Error: Unknown error\n");
        }
        try stderr.flush();
        return 1;
    }
}

fn cmdBookmarkList(allocator: Allocator, args: []const []const u8) !u8 {
    var stderr_buffer: [1024]u8 = undefined;
    var stderr_writer = std.fs.File.stderr().writer(&stderr_buffer);
    const stderr = &stderr_writer.interface;

    var profile: ?[]const u8 = null;

    var i: usize = 0;
    while (i < args.len) : (i += 1) {
        const arg = args[i];
        if (std.mem.eql(u8, arg, "--profile") or std.mem.eql(u8, arg, "-p")) {
            i += 1;
            if (i >= args.len) {
                try stderr.writeAll("Error: --profile requires an argument\n");
                try stderr.flush();
                return 1;
            }
            profile = args[i];
        } else {
            try stderr.print("Error: unknown option: {s}\n", .{arg});
            try stderr.flush();
            return 1;
        }
    }

    // Build and send request
    const response = try sendBookmarkListRequest(allocator, profile);
    defer allocator.free(response);

    // Parse response
    const parsed = try std.json.parseFromSlice(Response, allocator, response, .{
        .ignore_unknown_fields = true,
    });
    defer parsed.deinit();

    var stdout_buffer: [8192]u8 = undefined;
    var stdout_writer = std.fs.File.stdout().writer(&stdout_buffer);
    const stdout = &stdout_writer.interface;

    if (std.mem.eql(u8, parsed.value.status, "ok")) {
        // Print bookmarks in name\turl format
        if (parsed.value.data) |dataObj| {
            if (dataObj.object.get("bookmarks")) |bookmarksVal| {
                if (bookmarksVal == .object) {
                    var iter = bookmarksVal.object.iterator();
                    while (iter.next()) |entry| {
                        const bookmarkName = entry.key_ptr.*;
                        const bookmark = entry.value_ptr.*;
                        if (bookmark == .object) {
                            const urlVal = bookmark.object.get("url");
                            if (urlVal) |u| {
                                if (u == .string) {
                                    try stdout.print("{s}\t{s}\n", .{ bookmarkName, u.string });
                                }
                            }
                        }
                    }
                    try stdout.flush();
                }
            }
        }
        return 0;
    } else {
        if (parsed.value.@"error") |err| {
            try stderr.print("Error: {s}\n", .{err});
        } else {
            try stderr.writeAll("Error: Unknown error\n");
        }
        try stderr.flush();
        return 1;
    }
}

fn cmdBookmarkUpdate(allocator: Allocator, args: []const []const u8) !u8 {
    var stderr_buffer: [1024]u8 = undefined;
    var stderr_writer = std.fs.File.stderr().writer(&stderr_buffer);
    const stderr = &stderr_writer.interface;

    var name: ?[]const u8 = null;
    var url: ?[]const u8 = null;
    var title: ?[]const u8 = null;
    var profile: ?[]const u8 = null;

    var i: usize = 0;
    while (i < args.len) : (i += 1) {
        const arg = args[i];
        if (std.mem.eql(u8, arg, "--url")) {
            i += 1;
            if (i >= args.len) {
                try stderr.writeAll("Error: --url requires an argument\n");
                try stderr.flush();
                return 1;
            }
            url = args[i];
        } else if (std.mem.eql(u8, arg, "--name")) {
            i += 1;
            if (i >= args.len) {
                try stderr.writeAll("Error: --name requires an argument\n");
                try stderr.flush();
                return 1;
            }
            name = args[i];
        } else if (std.mem.eql(u8, arg, "--title")) {
            i += 1;
            if (i >= args.len) {
                try stderr.writeAll("Error: --title requires an argument\n");
                try stderr.flush();
                return 1;
            }
            title = args[i];
        } else if (std.mem.eql(u8, arg, "--profile") or std.mem.eql(u8, arg, "-p")) {
            i += 1;
            if (i >= args.len) {
                try stderr.writeAll("Error: --profile requires an argument\n");
                try stderr.flush();
                return 1;
            }
            profile = args[i];
        } else if (!std.mem.startsWith(u8, arg, "-")) {
            if (name == null) {
                name = arg;
            }
        } else {
            try stderr.print("Error: unknown option: {s}\n", .{arg});
            try stderr.flush();
            return 1;
        }
    }

    if (name == null) {
        try stderr.writeAll("Error: bookmark update requires a name\n");
        try stderr.flush();
        return 1;
    }

    if (url == null and title == null) {
        try stderr.writeAll("Error: bookmark update requires --url or --title\n");
        try stderr.flush();
        return 1;
    }

    // Build and send request
    const response = try sendBookmarkRequest(allocator, "update", name.?, url, title, profile);
    defer allocator.free(response);

    // Parse and handle response
    return handleBookmarkResponse(response);
}

fn cmdBookmarkDelete(allocator: Allocator, args: []const []const u8) !u8 {
    var stderr_buffer: [1024]u8 = undefined;
    var stderr_writer = std.fs.File.stderr().writer(&stderr_buffer);
    const stderr = &stderr_writer.interface;

    var name: ?[]const u8 = null;
    var profile: ?[]const u8 = null;

    var i: usize = 0;
    while (i < args.len) : (i += 1) {
        const arg = args[i];
        if (std.mem.eql(u8, arg, "--name")) {
            i += 1;
            if (i >= args.len) {
                try stderr.writeAll("Error: --name requires an argument\n");
                try stderr.flush();
                return 1;
            }
            name = args[i];
        } else if (std.mem.eql(u8, arg, "--profile") or std.mem.eql(u8, arg, "-p")) {
            i += 1;
            if (i >= args.len) {
                try stderr.writeAll("Error: --profile requires an argument\n");
                try stderr.flush();
                return 1;
            }
            profile = args[i];
        } else if (!std.mem.startsWith(u8, arg, "-")) {
            if (name == null) {
                name = arg;
            }
        } else {
            try stderr.print("Error: unknown option: {s}\n", .{arg});
            try stderr.flush();
            return 1;
        }
    }

    if (name == null) {
        try stderr.writeAll("Error: bookmark delete requires a name\n");
        try stderr.flush();
        return 1;
    }

    // Build and send request
    const response = try sendBookmarkRequest(allocator, "delete", name.?, null, null, profile);
    defer allocator.free(response);

    // Parse and handle response
    return handleBookmarkResponse(response);
}

/// Handle a simple bookmark response (for add, update, delete)
fn handleBookmarkResponse(response: []const u8) !u8 {
    const parsed = try std.json.parseFromSlice(Response, std.heap.page_allocator, response, .{
        .ignore_unknown_fields = true,
    });
    defer parsed.deinit();

    if (!std.mem.eql(u8, parsed.value.status, "ok")) {
        var stderr_buffer: [1024]u8 = undefined;
        var stderr_writer = std.fs.File.stderr().writer(&stderr_buffer);
        const stderr = &stderr_writer.interface;

        if (parsed.value.@"error") |err| {
            try stderr.print("Error: {s}\n", .{err});
        } else {
            try stderr.writeAll("Error: Unknown error\n");
        }
        try stderr.flush();
        return 1;
    }
    // Success: silent
    return 0;
}

// MARK: - URL/Bookmark Helpers

/// Check if a string looks like a URL (has scheme or contains a dot)
fn looksLikeUrl(arg: []const u8) bool {
    // Has scheme
    if (std.mem.startsWith(u8, arg, "http://") or
        std.mem.startsWith(u8, arg, "https://") or
        std.mem.startsWith(u8, arg, "file://"))
    {
        return true;
    }

    // Contains a dot (domain-like, e.g., "google.com", "localhost:3000" would need port handling)
    if (std.mem.indexOfScalar(u8, arg, '.') != null) {
        return true;
    }

    // Contains a colon (e.g., "localhost:3000")
    if (std.mem.indexOfScalar(u8, arg, ':') != null) {
        return true;
    }

    return false;
}

/// Try to resolve a name as a bookmark. Returns the URL if found, null otherwise.
fn resolveBookmark(allocator: Allocator, name: []const u8, profile: ?[]const u8) !?[]u8 {
    // Build and send bookmark get request
    const response = sendBookmarkRequest(allocator, "get", name, null, null, profile) catch |err| {
        // Socket error means we're not in TermSurf, can't resolve bookmarks
        if (err == error.FileNotFound or err == error.ConnectionRefused) {
            return null;
        }
        return err;
    };
    defer allocator.free(response);

    // Parse response
    const parsed = std.json.parseFromSlice(Response, allocator, response, .{
        .ignore_unknown_fields = true,
    }) catch {
        return null;
    };
    defer parsed.deinit();

    if (std.mem.eql(u8, parsed.value.status, "ok")) {
        // Extract URL from response
        if (parsed.value.data) |dataObj| {
            if (dataObj.object.get("url")) |urlVal| {
                if (urlVal == .string) {
                    // Return a copy since parsed will be freed
                    return try allocator.dupe(u8, urlVal.string);
                }
            }
        }
    }

    return null;
}

// MARK: - Socket Communication

const Response = struct {
    id: []const u8,
    status: []const u8,
    data: ?std.json.Value = null,
    @"error": ?[]const u8 = null,
};

const Event = struct {
    id: []const u8,
    event: []const u8,
    data: ?std.json.Value = null,
};

fn sendPingRequest(allocator: Allocator) ![]u8 {
    return sendJsonRequest(allocator, "{\"id\":\"1\",\"action\":\"ping\"}\n");
}

/// Build an open request JSON string (does not send it)
fn buildOpenRequest(allocator: Allocator, url: ?[]const u8, paneId: ?[]const u8, profile: ?[]const u8, incognito: bool, jsApi: bool) ![]u8 {
    var jsonBuf: std.ArrayListUnmanaged(u8) = .empty;
    errdefer jsonBuf.deinit(allocator);

    const writer = jsonBuf.writer(allocator);
    try writer.writeAll("{\"id\":\"1\",\"action\":\"open\"");

    if (paneId) |pid| {
        try writer.writeAll(",\"paneId\":\"");
        try writer.writeAll(pid);
        try writer.writeAll("\"");
    }

    try writer.writeAll(",\"data\":{");

    // URL is optional - if not provided, app uses default homepage
    var hasField = false;
    if (url) |u| {
        try writer.writeAll("\"url\":\"");
        // Escape URL for JSON
        for (u) |c| {
            switch (c) {
                '"' => try writer.writeAll("\\\""),
                '\\' => try writer.writeAll("\\\\"),
                else => try writer.writeByte(c),
            }
        }
        try writer.writeAll("\"");
        hasField = true;
    }

    if (profile) |p| {
        if (hasField) try writer.writeAll(",");
        try writer.writeAll("\"profile\":\"");
        try writer.writeAll(p);
        try writer.writeAll("\"");
        hasField = true;
    }

    if (incognito) {
        if (hasField) try writer.writeAll(",");
        try writer.writeAll("\"incognito\":true");
        hasField = true;
    }

    if (jsApi) {
        if (hasField) try writer.writeAll(",");
        try writer.writeAll("\"jsApi\":true");
    }

    try writer.writeAll("}}\n");

    return try jsonBuf.toOwnedSlice(allocator);
}

/// Read a single response from socket (up to newline)
fn readResponse(allocator: Allocator, socket: posix.socket_t) ![]u8 {
    var responseBuf: std.ArrayListUnmanaged(u8) = .empty;
    errdefer responseBuf.deinit(allocator);

    var readBuf: [4096]u8 = undefined;
    while (true) {
        const bytesRead = try posix.read(socket, &readBuf);
        if (bytesRead == 0) break;

        try responseBuf.appendSlice(allocator, readBuf[0..bytesRead]);

        // Check for newline (end of response)
        if (std.mem.indexOfScalar(u8, responseBuf.items, '\n')) |_| {
            break;
        }
    }

    // Remove trailing newline if present
    if (responseBuf.items.len > 0 and responseBuf.items[responseBuf.items.len - 1] == '\n') {
        _ = responseBuf.pop();
    }

    return try responseBuf.toOwnedSlice(allocator);
}

fn sendCloseRequest(allocator: Allocator, paneId: ?[]const u8, webviewId: ?[]const u8) ![]u8 {
    var jsonBuf: std.ArrayListUnmanaged(u8) = .empty;
    defer jsonBuf.deinit(allocator);

    const writer = jsonBuf.writer(allocator);
    try writer.writeAll("{\"id\":\"1\",\"action\":\"close\"");

    if (paneId) |pid| {
        try writer.writeAll(",\"paneId\":\"");
        try writer.writeAll(pid);
        try writer.writeAll("\"");
    }

    if (webviewId) |wid| {
        try writer.writeAll(",\"data\":{\"webviewId\":\"");
        try writer.writeAll(wid);
        try writer.writeAll("\"}");
    }

    try writer.writeAll("}\n");

    return sendJsonRequest(allocator, jsonBuf.items);
}

fn sendJsonRequest(allocator: Allocator, json: []const u8) ![]u8 {
    // Get socket path from environment
    const socketPath = std.posix.getenv("TERMSURF_SOCKET") orelse {
        std.debug.print("Error: Not running inside TermSurf (TERMSURF_SOCKET not set)\n", .{});
        return error.SocketNotFound;
    };

    // Connect to Unix socket
    const socket = try connectToSocket(socketPath);
    defer posix.close(socket);

    // Send request
    _ = try posix.write(socket, json);

    // Read response
    var responseBuf: std.ArrayListUnmanaged(u8) = .empty;
    errdefer responseBuf.deinit(allocator);

    var readBuf: [4096]u8 = undefined;
    while (true) {
        const bytesRead = try posix.read(socket, &readBuf);
        if (bytesRead == 0) break;

        try responseBuf.appendSlice(allocator, readBuf[0..bytesRead]);

        // Check for newline (end of response)
        if (std.mem.indexOfScalar(u8, responseBuf.items, '\n')) |_| {
            break;
        }
    }

    // Remove trailing newline if present
    if (responseBuf.items.len > 0 and responseBuf.items[responseBuf.items.len - 1] == '\n') {
        _ = responseBuf.pop();
    }

    return try responseBuf.toOwnedSlice(allocator);
}

fn sendBookmarkRequest(
    allocator: Allocator,
    subaction: []const u8,
    name: []const u8,
    url: ?[]const u8,
    title: ?[]const u8,
    profile: ?[]const u8,
) ![]u8 {
    var jsonBuf: std.ArrayListUnmanaged(u8) = .empty;
    defer jsonBuf.deinit(allocator);

    const writer = jsonBuf.writer(allocator);
    try writer.writeAll("{\"id\":\"1\",\"action\":\"bookmark\",\"subaction\":\"");
    try writer.writeAll(subaction);
    try writer.writeAll("\",\"data\":{\"name\":\"");
    try writeJsonEscaped(writer, name);
    try writer.writeAll("\"");

    if (profile) |p| {
        try writer.writeAll(",\"profile\":\"");
        try writeJsonEscaped(writer, p);
        try writer.writeAll("\"");
    }

    if (url) |u| {
        try writer.writeAll(",\"url\":\"");
        try writeJsonEscaped(writer, u);
        try writer.writeAll("\"");
    }

    if (title) |t| {
        try writer.writeAll(",\"title\":\"");
        try writeJsonEscaped(writer, t);
        try writer.writeAll("\"");
    }

    try writer.writeAll("}}\n");

    return sendJsonRequest(allocator, jsonBuf.items);
}

fn sendBookmarkListRequest(allocator: Allocator, profile: ?[]const u8) ![]u8 {
    var jsonBuf: std.ArrayListUnmanaged(u8) = .empty;
    defer jsonBuf.deinit(allocator);

    const writer = jsonBuf.writer(allocator);
    try writer.writeAll("{\"id\":\"1\",\"action\":\"bookmark\",\"subaction\":\"list\",\"data\":{");

    if (profile) |p| {
        try writer.writeAll("\"profile\":\"");
        try writeJsonEscaped(writer, p);
        try writer.writeAll("\"");
    }

    try writer.writeAll("}}\n");

    return sendJsonRequest(allocator, jsonBuf.items);
}

/// Write a string with JSON escaping
fn writeJsonEscaped(writer: anytype, str: []const u8) !void {
    for (str) |c| {
        switch (c) {
            '"' => try writer.writeAll("\\\""),
            '\\' => try writer.writeAll("\\\\"),
            '\n' => try writer.writeAll("\\n"),
            '\r' => try writer.writeAll("\\r"),
            '\t' => try writer.writeAll("\\t"),
            else => try writer.writeByte(c),
        }
    }
}

fn connectToSocket(path: []const u8) !posix.socket_t {
    const socket = try posix.socket(posix.AF.UNIX, posix.SOCK.STREAM, 0);
    errdefer posix.close(socket);

    var addr: posix.sockaddr.un = .{
        .family = posix.AF.UNIX,
        .path = undefined,
    };

    // Copy path to addr.path
    if (path.len >= addr.path.len) {
        return error.PathTooLong;
    }
    @memcpy(addr.path[0..path.len], path);
    addr.path[path.len] = 0;

    posix.connect(socket, @ptrCast(&addr), @sizeOf(posix.sockaddr.un)) catch |err| {
        std.debug.print("Error: Could not connect to TermSurf socket at {s}\n", .{path});
        std.debug.print("       Is TermSurf running?\n", .{});
        return err;
    };

    return socket;
}
