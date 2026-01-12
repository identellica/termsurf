const std = @import("std");

const string_encoding = @import("../../../os/string_encoding.zig");

const Parser = @import("../../osc.zig").Parser;
const Command = @import("../../osc.zig").Command;

const log = std.log.scoped(.osc_semantic_prompt);

/// Parse OSC 133, semantic prompts
pub fn parse(parser: *Parser, _: ?u8) ?*Command {
    const writer = parser.writer orelse {
        parser.state = .invalid;
        return null;
    };
    const data = writer.buffered();
    if (data.len == 0) {
        parser.state = .invalid;
        return null;
    }
    switch (data[0]) {
        'A' => prompt_start: {
            parser.command = .{
                .prompt_start = .{},
            };
            if (data.len == 1) break :prompt_start;
            if (data[1] != ';') {
                parser.state = .invalid;
                return null;
            }
            var it = SemanticPromptKVIterator.init(writer) catch {
                parser.state = .invalid;
                return null;
            };
            while (it.next()) |kv| {
                if (std.mem.eql(u8, kv.key, "aid")) {
                    parser.command.prompt_start.aid = kv.value;
                } else if (std.mem.eql(u8, kv.key, "redraw")) redraw: {
                    // https://sw.kovidgoyal.net/kitty/shell-integration/#notes-for-shell-developers
                    // Kitty supports a "redraw" option for prompt_start. I can't find
                    // this documented anywhere but can see in the code that this is used
                    // by shell environments to tell the terminal that the shell will NOT
                    // redraw the prompt so we should attempt to resize it.
                    parser.command.prompt_start.redraw = (value: {
                        if (kv.value.len != 1) break :value null;
                        switch (kv.value[0]) {
                            '0' => break :value false,
                            '1' => break :value true,
                            else => break :value null,
                        }
                    }) orelse {
                        log.info("OSC 133 A: invalid redraw value: {s}", .{kv.value});
                        break :redraw;
                    };
                } else if (std.mem.eql(u8, kv.key, "special_key")) redraw: {
                    // https://sw.kovidgoyal.net/kitty/shell-integration/#notes-for-shell-developers
                    parser.command.prompt_start.special_key = (value: {
                        if (kv.value.len != 1) break :value null;
                        switch (kv.value[0]) {
                            '0' => break :value false,
                            '1' => break :value true,
                            else => break :value null,
                        }
                    }) orelse {
                        log.info("OSC 133 A invalid special_key value: {s}", .{kv.value});
                        break :redraw;
                    };
                } else if (std.mem.eql(u8, kv.key, "click_events")) redraw: {
                    // https://sw.kovidgoyal.net/kitty/shell-integration/#notes-for-shell-developers
                    parser.command.prompt_start.click_events = (value: {
                        if (kv.value.len != 1) break :value null;
                        switch (kv.value[0]) {
                            '0' => break :value false,
                            '1' => break :value true,
                            else => break :value null,
                        }
                    }) orelse {
                        log.info("OSC 133 A invalid click_events value: {s}", .{kv.value});
                        break :redraw;
                    };
                } else if (std.mem.eql(u8, kv.key, "k")) k: {
                    // The "k" marks the kind of prompt, or "primary" if we don't know.
                    // This can be used to distinguish between the first (initial) prompt,
                    // a continuation, etc.
                    if (kv.value.len != 1) break :k;
                    parser.command.prompt_start.kind = switch (kv.value[0]) {
                        'c' => .continuation,
                        's' => .secondary,
                        'r' => .right,
                        'i' => .primary,
                        else => .primary,
                    };
                } else log.info("OSC 133 A: unknown semantic prompt option: {s}", .{kv.key});
            }
        },
        'B' => prompt_end: {
            parser.command = .prompt_end;
            if (data.len == 1) break :prompt_end;
            if (data[1] != ';') {
                parser.state = .invalid;
                return null;
            }
            var it = SemanticPromptKVIterator.init(writer) catch {
                parser.state = .invalid;
                return null;
            };
            while (it.next()) |kv| {
                log.info("OSC 133 B: unknown semantic prompt option: {s}", .{kv.key});
            }
        },
        'C' => end_of_input: {
            parser.command = .{
                .end_of_input = .{},
            };
            if (data.len == 1) break :end_of_input;
            if (data[1] != ';') {
                parser.state = .invalid;
                return null;
            }
            var it = SemanticPromptKVIterator.init(writer) catch {
                parser.state = .invalid;
                return null;
            };
            while (it.next()) |kv| {
                if (std.mem.eql(u8, kv.key, "cmdline")) {
                    parser.command.end_of_input.cmdline = string_encoding.printfQDecode(kv.value) catch null;
                } else if (std.mem.eql(u8, kv.key, "cmdline_url")) {
                    parser.command.end_of_input.cmdline = string_encoding.urlPercentDecode(kv.value) catch null;
                } else {
                    log.info("OSC 133 C: unknown semantic prompt option: {s}", .{kv.key});
                }
            }
        },
        'D' => {
            const exit_code: ?u8 = exit_code: {
                if (data.len == 1) break :exit_code null;
                if (data[1] != ';') {
                    parser.state = .invalid;
                    return null;
                }
                break :exit_code std.fmt.parseUnsigned(u8, data[2..], 10) catch null;
            };
            parser.command = .{
                .end_of_command = .{
                    .exit_code = exit_code,
                },
            };
        },
        else => {
            parser.state = .invalid;
            return null;
        },
    }
    return &parser.command;
}

const SemanticPromptKVIterator = struct {
    index: usize,
    string: []u8,

    pub const SemanticPromptKV = struct {
        key: [:0]u8,
        value: [:0]u8,
    };

    pub fn init(writer: *std.Io.Writer) std.Io.Writer.Error!SemanticPromptKVIterator {
        // add a semicolon to make it easier to find and sentinel terminate the values
        try writer.writeByte(';');
        return .{
            .index = 0,
            .string = writer.buffered()[2..],
        };
    }

    pub fn next(self: *SemanticPromptKVIterator) ?SemanticPromptKV {
        if (self.index >= self.string.len) return null;

        const kv = kv: {
            const index = std.mem.indexOfScalarPos(u8, self.string, self.index, ';') orelse {
                self.index = self.string.len;
                return null;
            };
            self.string[index] = 0;
            const kv = self.string[self.index..index :0];
            self.index = index + 1;
            break :kv kv;
        };

        const key = key: {
            const index = std.mem.indexOfScalar(u8, kv, '=') orelse break :key kv;
            kv[index] = 0;
            const key = kv[0..index :0];
            break :key key;
        };

        const value = kv[key.len + 1 .. :0];

        return .{
            .key = key,
            .value = value,
        };
    }
};

test "OSC 133: prompt_start" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "133;A";
    for (input) |ch| p.next(ch);

    const cmd = p.end(null).?.*;
    try testing.expect(cmd == .prompt_start);
    try testing.expect(cmd.prompt_start.aid == null);
    try testing.expect(cmd.prompt_start.redraw);
}

test "OSC 133: prompt_start with single option" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "133;A;aid=14";
    for (input) |ch| p.next(ch);

    const cmd = p.end(null).?.*;
    try testing.expect(cmd == .prompt_start);
    try testing.expectEqualStrings("14", cmd.prompt_start.aid.?);
}

test "OSC 133: prompt_start with '=' in aid" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "133;A;aid=a=b;redraw=0";
    for (input) |ch| p.next(ch);

    const cmd = p.end(null).?.*;
    try testing.expect(cmd == .prompt_start);
    try testing.expectEqualStrings("a=b", cmd.prompt_start.aid.?);
    try testing.expect(!cmd.prompt_start.redraw);
}

test "OSC 133: prompt_start with redraw disabled" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "133;A;redraw=0";
    for (input) |ch| p.next(ch);

    const cmd = p.end(null).?.*;
    try testing.expect(cmd == .prompt_start);
    try testing.expect(!cmd.prompt_start.redraw);
}

test "OSC 133: prompt_start with redraw invalid value" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "133;A;redraw=42";
    for (input) |ch| p.next(ch);

    const cmd = p.end(null).?.*;
    try testing.expect(cmd == .prompt_start);
    try testing.expect(cmd.prompt_start.redraw);
    try testing.expect(cmd.prompt_start.kind == .primary);
}

test "OSC 133: prompt_start with continuation" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "133;A;k=c";
    for (input) |ch| p.next(ch);

    const cmd = p.end(null).?.*;
    try testing.expect(cmd == .prompt_start);
    try testing.expect(cmd.prompt_start.kind == .continuation);
}

test "OSC 133: prompt_start with secondary" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "133;A;k=s";
    for (input) |ch| p.next(ch);

    const cmd = p.end(null).?.*;
    try testing.expect(cmd == .prompt_start);
    try testing.expect(cmd.prompt_start.kind == .secondary);
}

test "OSC 133: prompt_start with special_key" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "133;A;special_key=1";
    for (input) |ch| p.next(ch);

    const cmd = p.end(null).?.*;
    try testing.expect(cmd == .prompt_start);
    try testing.expect(cmd.prompt_start.special_key == true);
}

test "OSC 133: prompt_start with special_key invalid" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "133;A;special_key=bobr";
    for (input) |ch| p.next(ch);

    const cmd = p.end(null).?.*;
    try testing.expect(cmd == .prompt_start);
    try testing.expect(cmd.prompt_start.special_key == false);
}

test "OSC 133: prompt_start with special_key 0" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "133;A;special_key=0";
    for (input) |ch| p.next(ch);

    const cmd = p.end(null).?.*;
    try testing.expect(cmd == .prompt_start);
    try testing.expect(cmd.prompt_start.special_key == false);
}

test "OSC 133: prompt_start with special_key empty" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "133;A;special_key=";
    for (input) |ch| p.next(ch);

    const cmd = p.end(null).?.*;
    try testing.expect(cmd == .prompt_start);
    try testing.expect(cmd.prompt_start.special_key == false);
}

test "OSC 133: prompt_start with click_events true" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "133;A;click_events=1";
    for (input) |ch| p.next(ch);

    const cmd = p.end(null).?.*;
    try testing.expect(cmd == .prompt_start);
    try testing.expect(cmd.prompt_start.click_events == true);
}

test "OSC 133: prompt_start with click_events false" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "133;A;click_events=0";
    for (input) |ch| p.next(ch);

    const cmd = p.end(null).?.*;
    try testing.expect(cmd == .prompt_start);
    try testing.expect(cmd.prompt_start.click_events == false);
}

test "OSC 133: prompt_start with click_events empty" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "133;A;click_events=";
    for (input) |ch| p.next(ch);

    const cmd = p.end(null).?.*;
    try testing.expect(cmd == .prompt_start);
    try testing.expect(cmd.prompt_start.click_events == false);
}

test "OSC 133: end_of_command no exit code" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "133;D";
    for (input) |ch| p.next(ch);

    const cmd = p.end(null).?.*;
    try testing.expect(cmd == .end_of_command);
}

test "OSC 133: end_of_command with exit code" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "133;D;25";
    for (input) |ch| p.next(ch);

    const cmd = p.end(null).?.*;
    try testing.expect(cmd == .end_of_command);
    try testing.expectEqual(@as(u8, 25), cmd.end_of_command.exit_code.?);
}

test "OSC 133: prompt_end" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "133;B";
    for (input) |ch| p.next(ch);

    const cmd = p.end(null).?.*;
    try testing.expect(cmd == .prompt_end);
}

test "OSC 133: end_of_input" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "133;C";
    for (input) |ch| p.next(ch);

    const cmd = p.end(null).?.*;
    try testing.expect(cmd == .end_of_input);
}

test "OSC 133: end_of_input with cmdline 1" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "133;C;cmdline=echo bobr kurwa";
    for (input) |ch| p.next(ch);

    const cmd = p.end(null).?.*;
    try testing.expect(cmd == .end_of_input);
    try testing.expect(cmd.end_of_input.cmdline != null);
    try testing.expectEqualStrings("echo bobr kurwa", cmd.end_of_input.cmdline.?);
}

test "OSC 133: end_of_input with cmdline 2" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "133;C;cmdline=echo bobr\\ kurwa";
    for (input) |ch| p.next(ch);

    const cmd = p.end(null).?.*;
    try testing.expect(cmd == .end_of_input);
    try testing.expect(cmd.end_of_input.cmdline != null);
    try testing.expectEqualStrings("echo bobr kurwa", cmd.end_of_input.cmdline.?);
}

test "OSC 133: end_of_input with cmdline 3" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "133;C;cmdline=echo bobr\\nkurwa";
    for (input) |ch| p.next(ch);

    const cmd = p.end(null).?.*;
    try testing.expect(cmd == .end_of_input);
    try testing.expect(cmd.end_of_input.cmdline != null);
    try testing.expectEqualStrings("echo bobr\nkurwa", cmd.end_of_input.cmdline.?);
}

test "OSC 133: end_of_input with cmdline 4" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "133;C;cmdline=$'echo bobr kurwa'";
    for (input) |ch| p.next(ch);

    const cmd = p.end(null).?.*;
    try testing.expect(cmd == .end_of_input);
    try testing.expect(cmd.end_of_input.cmdline != null);
    try testing.expectEqualStrings("echo bobr kurwa", cmd.end_of_input.cmdline.?);
}

test "OSC 133: end_of_input with cmdline 5" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "133;C;cmdline='echo bobr kurwa'";
    for (input) |ch| p.next(ch);

    const cmd = p.end(null).?.*;
    try testing.expect(cmd == .end_of_input);
    try testing.expect(cmd.end_of_input.cmdline != null);
    try testing.expectEqualStrings("echo bobr kurwa", cmd.end_of_input.cmdline.?);
}

test "OSC 133: end_of_input with cmdline 6" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "133;C;cmdline='echo bobr kurwa";
    for (input) |ch| p.next(ch);

    const cmd = p.end(null).?.*;
    try testing.expect(cmd == .end_of_input);
    try testing.expect(cmd.end_of_input.cmdline == null);
}

test "OSC 133: end_of_input with cmdline 7" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "133;C;cmdline=$'echo bobr kurwa";
    for (input) |ch| p.next(ch);

    const cmd = p.end(null).?.*;
    try testing.expect(cmd == .end_of_input);
    try testing.expect(cmd.end_of_input.cmdline == null);
}

test "OSC 133: end_of_input with cmdline 8" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "133;C;cmdline=$'";
    for (input) |ch| p.next(ch);

    const cmd = p.end(null).?.*;
    try testing.expect(cmd == .end_of_input);
    try testing.expect(cmd.end_of_input.cmdline == null);
}

test "OSC 133: end_of_input with cmdline 9" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "133;C;cmdline=$'";
    for (input) |ch| p.next(ch);

    const cmd = p.end(null).?.*;
    try testing.expect(cmd == .end_of_input);
    try testing.expect(cmd.end_of_input.cmdline == null);
}

test "OSC 133: end_of_input with cmdline 10" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "133;C;cmdline=";
    for (input) |ch| p.next(ch);

    const cmd = p.end(null).?.*;
    try testing.expect(cmd == .end_of_input);
    try testing.expect(cmd.end_of_input.cmdline != null);
    try testing.expectEqualStrings("", cmd.end_of_input.cmdline.?);
}

test "OSC 133: end_of_input with cmdline_url 1" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "133;C;cmdline_url=echo bobr kurwa";
    for (input) |ch| p.next(ch);

    const cmd = p.end(null).?.*;
    try testing.expect(cmd == .end_of_input);
    try testing.expect(cmd.end_of_input.cmdline != null);
    try testing.expectEqualStrings("echo bobr kurwa", cmd.end_of_input.cmdline.?);
}

test "OSC 133: end_of_input with cmdline_url 2" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "133;C;cmdline_url=echo bobr%20kurwa";
    for (input) |ch| p.next(ch);

    const cmd = p.end(null).?.*;
    try testing.expect(cmd == .end_of_input);
    try testing.expect(cmd.end_of_input.cmdline != null);
    try testing.expectEqualStrings("echo bobr kurwa", cmd.end_of_input.cmdline.?);
}

test "OSC 133: end_of_input with cmdline_url 3" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "133;C;cmdline_url=echo bobr%3bkurwa";
    for (input) |ch| p.next(ch);

    const cmd = p.end(null).?.*;
    try testing.expect(cmd == .end_of_input);
    try testing.expect(cmd.end_of_input.cmdline != null);
    try testing.expectEqualStrings("echo bobr;kurwa", cmd.end_of_input.cmdline.?);
}

test "OSC 133: end_of_input with cmdline_url 4" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "133;C;cmdline_url=echo bobr%3kurwa";
    for (input) |ch| p.next(ch);

    const cmd = p.end(null).?.*;
    try testing.expect(cmd == .end_of_input);
    try testing.expect(cmd.end_of_input.cmdline == null);
}

test "OSC 133: end_of_input with cmdline_url 5" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "133;C;cmdline_url=echo bobr%kurwa";
    for (input) |ch| p.next(ch);

    const cmd = p.end(null).?.*;
    try testing.expect(cmd == .end_of_input);
    try testing.expect(cmd.end_of_input.cmdline == null);
}

test "OSC 133: end_of_input with cmdline_url 6" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "133;C;cmdline_url=echo bobr%kurwa";
    for (input) |ch| p.next(ch);

    const cmd = p.end(null).?.*;
    try testing.expect(cmd == .end_of_input);
    try testing.expect(cmd.end_of_input.cmdline == null);
}

test "OSC 133: end_of_input with cmdline_url 7" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "133;C;cmdline_url=echo bobr kurwa%20";
    for (input) |ch| p.next(ch);

    const cmd = p.end(null).?.*;
    try testing.expect(cmd == .end_of_input);
    try testing.expect(cmd.end_of_input.cmdline != null);
    try testing.expectEqualStrings("echo bobr kurwa ", cmd.end_of_input.cmdline.?);
}

test "OSC 133: end_of_input with cmdline_url 8" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "133;C;cmdline_url=echo bobr kurwa%2";
    for (input) |ch| p.next(ch);

    const cmd = p.end(null).?.*;
    try testing.expect(cmd == .end_of_input);
    try testing.expect(cmd.end_of_input.cmdline == null);
}

test "OSC 133: end_of_input with cmdline_url 9" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "133;C;cmdline_url=echo bobr kurwa%2";
    for (input) |ch| p.next(ch);

    const cmd = p.end(null).?.*;
    try testing.expect(cmd == .end_of_input);
    try testing.expect(cmd.end_of_input.cmdline == null);
}
