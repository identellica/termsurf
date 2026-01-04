const std = @import("std");
const posix = std.posix;

const version = "0.1.0";

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    // No args: open default homepage
    if (args.len < 2) {
        try cmdOpen(allocator, &.{});
        return;
    }

    const command = args[1];

    if (std.mem.eql(u8, command, "ping")) {
        try cmdPing(allocator);
    } else if (std.mem.eql(u8, command, "open")) {
        try cmdOpen(allocator, args[2..]);
    } else if (std.mem.eql(u8, command, "close")) {
        try cmdClose(allocator, args[2..]);
    } else if (std.mem.eql(u8, command, "version") or std.mem.eql(u8, command, "--version") or std.mem.eql(u8, command, "-v")) {
        try printVersion();
    } else if (std.mem.eql(u8, command, "help") or std.mem.eql(u8, command, "--help") or std.mem.eql(u8, command, "-h")) {
        try printUsage();
    } else {
        // Unknown command: treat as URL (implicit open)
        try cmdOpen(allocator, args[1..]);
    }
}

fn printVersion() !void {
    var buffer: [256]u8 = undefined;
    var stdout_writer = std.fs.File.stdout().writer(&buffer);
    const stdout = &stdout_writer.interface;
    try stdout.print("termsurf {s}\n", .{version});
    try stdout.flush();
}

fn printUsage() !void {
    var buffer: [4096]u8 = undefined;
    var stdout_writer = std.fs.File.stdout().writer(&buffer);
    const stdout = &stdout_writer.interface;
    try stdout.writeAll(
        \\termsurf - CLI tool for TermSurf browser pane integration
        \\
        \\USAGE:
        \\    termsurf [url]              Open URL (or default homepage)
        \\    termsurf <command> [options]
        \\
        \\COMMANDS:
        \\    open [options] [url]    Open a URL in a browser pane
        \\    ping                    Test connectivity to TermSurf
        \\    close [webview-id]      Close a browser pane
        \\    version                 Show version information
        \\    help                    Show this help message
        \\
        \\OPTIONS (for open):
        \\    --js-api                Enable window.termsurf JavaScript API
        \\    --profile, -p NAME      Use isolated browser profile
        \\    --incognito             Use ephemeral session (no data persisted)
        \\
        \\ENVIRONMENT:
        \\    TERMSURF_SOCKET         Path to TermSurf Unix socket
        \\    TERMSURF_PANE_ID        Current pane identifier
        \\
        \\EXAMPLES:
        \\    termsurf                        Open default homepage
        \\    termsurf google.com             Open https://google.com
        \\    termsurf open localhost:3000    Open local dev server
        \\
    );
    try stdout.flush();
}

// MARK: - Commands

fn cmdPing(allocator: std.mem.Allocator) !void {
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
        std.process.exit(1);
    }
}

fn cmdOpen(allocator: std.mem.Allocator, args: []const []const u8) !void {
    var url: ?[]const u8 = null;
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
                std.process.exit(1);
            }
            profile = args[i];
        } else if (!std.mem.startsWith(u8, arg, "-")) {
            url = arg;
        } else {
            std.debug.print("Unknown option: {s}\n", .{arg});
            std.process.exit(1);
        }
    }

    // Check for mutually exclusive options
    if (incognito and profile != null) {
        std.debug.print("Error: --incognito and --profile are mutually exclusive\n", .{});
        std.process.exit(1);
    }

    // URL can be null - app will use default homepage
    // URL normalization (https:// prefix) is handled by the app

    // Get pane ID from environment
    const paneId = std.posix.getenv("TERMSURF_PANE_ID");

    // Get socket path from environment
    const socketPath = std.posix.getenv("TERMSURF_SOCKET") orelse {
        std.debug.print("Error: Not running inside TermSurf (TERMSURF_SOCKET not set)\n", .{});
        std.process.exit(1);
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
        const exitCode = try eventLoop(allocator, socket, webviewIdStr);
        std.process.exit(exitCode);
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
        std.process.exit(1);
    }
}

/// Event loop: read events from socket and handle them
/// Returns exit code when webview closes
fn eventLoop(allocator: std.mem.Allocator, socket: posix.socket_t, webviewId: ?[]const u8) !u8 {
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

fn cmdClose(allocator: std.mem.Allocator, args: []const []const u8) !void {
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
    } else {
        if (parsed.value.@"error") |err| {
            try stderr.print("Error: {s}\n", .{err});
        } else {
            try stderr.writeAll("Error: Unknown error\n");
        }
        try stderr.flush();
        std.process.exit(1);
    }
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

fn sendPingRequest(allocator: std.mem.Allocator) ![]u8 {
    return sendJsonRequest(allocator, "{\"id\":\"1\",\"action\":\"ping\"}\n");
}

/// Build an open request JSON string (does not send it)
fn buildOpenRequest(allocator: std.mem.Allocator, url: ?[]const u8, paneId: ?[]const u8, profile: ?[]const u8, incognito: bool, jsApi: bool) ![]u8 {
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
fn readResponse(allocator: std.mem.Allocator, socket: posix.socket_t) ![]u8 {
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

fn sendCloseRequest(allocator: std.mem.Allocator, paneId: ?[]const u8, webviewId: ?[]const u8) ![]u8 {
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

fn sendJsonRequest(allocator: std.mem.Allocator, json: []const u8) ![]u8 {
    // Get socket path from environment
    const socketPath = std.posix.getenv("TERMSURF_SOCKET") orelse {
        std.debug.print("Error: Not running inside TermSurf (TERMSURF_SOCKET not set)\n", .{});
        std.process.exit(1);
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
