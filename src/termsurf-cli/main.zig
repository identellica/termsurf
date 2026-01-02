const std = @import("std");
const posix = std.posix;

const version = "0.1.0";

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    if (args.len < 2) {
        try printUsage();
        std.process.exit(1);
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
        std.debug.print("Unknown command: {s}\n", .{command});
        try printUsage();
        std.process.exit(1);
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
        \\    termsurf <command> [options]
        \\
        \\COMMANDS:
        \\    ping                    Test connectivity to TermSurf
        \\    open [--wait] <url>     Open a URL in a browser pane
        \\    close [webview-id]      Close a browser pane
        \\    version                 Show version information
        \\    help                    Show this help message
        \\
        \\ENVIRONMENT:
        \\    TERMSURF_SOCKET         Path to TermSurf Unix socket
        \\    TERMSURF_PANE_ID        Current pane identifier
        \\
        \\EXAMPLES:
        \\    termsurf ping
        \\    termsurf open https://google.com
        \\    termsurf open --wait https://localhost:3000
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
    var wait = false;
    var profile: ?[]const u8 = null;

    var i: usize = 0;
    while (i < args.len) : (i += 1) {
        const arg = args[i];
        if (std.mem.eql(u8, arg, "--wait") or std.mem.eql(u8, arg, "-w")) {
            wait = true;
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

    if (url == null) {
        std.debug.print("Error: URL is required\n", .{});
        std.debug.print("Usage: termsurf open [--wait] [--profile NAME] <url>\n", .{});
        std.process.exit(1);
    }

    // Prepend https:// if no scheme
    const finalUrl = try normalizeUrl(allocator, url.?);
    defer if (finalUrl.ptr != url.?.ptr) allocator.free(finalUrl);

    // Get pane ID from environment
    const paneId = std.posix.getenv("TERMSURF_PANE_ID");

    const response = try sendOpenRequest(allocator, finalUrl, paneId, wait, profile);
    defer allocator.free(response);

    // Parse response
    const parsed = try std.json.parseFromSlice(Response, allocator, response, .{
        .ignore_unknown_fields = true,
    });
    defer parsed.deinit();

    var stdout_buffer: [2048]u8 = undefined;
    var stdout_writer = std.fs.File.stdout().writer(&stdout_buffer);
    const stdout = &stdout_writer.interface;

    var stderr_buffer: [1024]u8 = undefined;
    var stderr_writer = std.fs.File.stderr().writer(&stderr_buffer);
    const stderr = &stderr_writer.interface;

    if (std.mem.eql(u8, parsed.value.status, "ok")) {
        if (parsed.value.data) |dataObj| {
            if (dataObj.object.get("webviewId")) |wvId| {
                if (wvId == .string) {
                    try stdout.print("Opened webview: {s}\n", .{wvId.string});
                }
            }
            if (dataObj.object.get("message")) |msg| {
                if (msg == .string) {
                    try stdout.print("{s}\n", .{msg.string});
                }
            }
        }

        // If --wait was specified, we would keep the connection open
        // and wait for events. For now, just exit.
        if (wait) {
            try stdout.writeAll("Note: --wait not yet fully implemented\n");
        }
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

// MARK: - Helpers

fn normalizeUrl(allocator: std.mem.Allocator, url: []const u8) ![]const u8 {
    // If URL already has a scheme, return as-is
    if (std.mem.startsWith(u8, url, "http://") or
        std.mem.startsWith(u8, url, "https://") or
        std.mem.startsWith(u8, url, "file://"))
    {
        return url;
    }

    // Prepend https://
    return try std.fmt.allocPrint(allocator, "https://{s}", .{url});
}

// MARK: - Socket Communication

const Response = struct {
    id: []const u8,
    status: []const u8,
    data: ?std.json.Value = null,
    @"error": ?[]const u8 = null,
};

fn sendPingRequest(allocator: std.mem.Allocator) ![]u8 {
    return sendJsonRequest(allocator, "{\"id\":\"1\",\"action\":\"ping\"}\n");
}

fn sendOpenRequest(allocator: std.mem.Allocator, url: []const u8, paneId: ?[]const u8, wait: bool, profile: ?[]const u8) ![]u8 {
    var jsonBuf: std.ArrayListUnmanaged(u8) = .empty;
    defer jsonBuf.deinit(allocator);

    const writer = jsonBuf.writer(allocator);
    try writer.writeAll("{\"id\":\"1\",\"action\":\"open\"");

    if (paneId) |pid| {
        try writer.writeAll(",\"paneId\":\"");
        try writer.writeAll(pid);
        try writer.writeAll("\"");
    }

    try writer.writeAll(",\"data\":{\"url\":\"");
    // Escape URL for JSON
    for (url) |c| {
        switch (c) {
            '"' => try writer.writeAll("\\\""),
            '\\' => try writer.writeAll("\\\\"),
            else => try writer.writeByte(c),
        }
    }
    try writer.writeAll("\"");

    if (wait) {
        try writer.writeAll(",\"wait\":true");
    }

    if (profile) |p| {
        try writer.writeAll(",\"profile\":\"");
        try writer.writeAll(p);
        try writer.writeAll("\"");
    }

    try writer.writeAll("}}\n");

    return sendJsonRequest(allocator, jsonBuf.items);
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
