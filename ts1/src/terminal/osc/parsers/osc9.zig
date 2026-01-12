const std = @import("std");

const Parser = @import("../../osc.zig").Parser;
const Command = @import("../../osc.zig").Command;

/// Parse OSC 9, which could be an iTerm2 notification or a ConEmu extension.
pub fn parse(parser: *Parser, _: ?u8) ?*Command {
    const writer = parser.writer orelse {
        parser.state = .invalid;
        return null;
    };

    // Check first to see if this is a ConEmu OSC
    // https://conemu.github.io/en/AnsiEscapeCodes.html#ConEmu_specific_OSC
    conemu: {
        var data = writer.buffered();
        if (data.len == 0) break :conemu;
        switch (data[0]) {
            // Check for OSC 9;1 9;10 9;12
            '1' => {
                if (data.len < 2) break :conemu;
                switch (data[1]) {
                    // OSC 9;1
                    ';' => {
                        parser.command = .{
                            .conemu_sleep = .{
                                .duration_ms = if (std.fmt.parseUnsigned(u16, data[2..], 10)) |num| @min(num, 10_000) else |_| 100,
                            },
                        };
                        return &parser.command;
                    },
                    // OSC 9;10
                    '0' => {
                        parser.state = .invalid;
                        return null;
                    },
                    // OSC 9;12
                    '2' => {
                        parser.command = .{
                            .prompt_start = .{},
                        };
                        return &parser.command;
                    },
                    else => break :conemu,
                }
            },
            // OSC 9;2
            '2' => {
                if (data.len < 2) break :conemu;
                if (data[1] != ';') break :conemu;
                writer.writeByte(0) catch {
                    parser.state = .invalid;
                    return null;
                };
                data = writer.buffered();
                parser.command = .{
                    .conemu_show_message_box = data[2 .. data.len - 1 :0],
                };
                return &parser.command;
            },
            // OSC 9;3
            '3' => {
                if (data.len < 2) break :conemu;
                if (data[1] != ';') break :conemu;
                if (data.len == 2) {
                    parser.command = .{
                        .conemu_change_tab_title = .reset,
                    };
                    return &parser.command;
                }
                writer.writeByte(0) catch {
                    parser.state = .invalid;
                    return null;
                };
                data = writer.buffered();
                parser.command = .{
                    .conemu_change_tab_title = .{
                        .value = data[2 .. data.len - 1 :0],
                    },
                };
                return &parser.command;
            },
            // OSC 9;4
            '4' => {
                if (data.len < 2) break :conemu;
                if (data[1] != ';') break :conemu;
                if (data.len < 3) break :conemu;
                switch (data[2]) {
                    '0' => {
                        parser.command = .{
                            .conemu_progress_report = .{
                                .state = .remove,
                            },
                        };
                    },
                    '1' => {
                        parser.command = .{
                            .conemu_progress_report = .{
                                .state = .set,
                                .progress = 0,
                            },
                        };
                    },
                    '2' => {
                        parser.command = .{
                            .conemu_progress_report = .{
                                .state = .@"error",
                            },
                        };
                    },
                    '3' => {
                        parser.command = .{
                            .conemu_progress_report = .{
                                .state = .indeterminate,
                            },
                        };
                    },
                    '4' => {
                        parser.command = .{
                            .conemu_progress_report = .{
                                .state = .pause,
                            },
                        };
                    },
                    else => break :conemu,
                }
                switch (parser.command.conemu_progress_report.state) {
                    .remove, .indeterminate => {},
                    .set, .@"error", .pause => progress: {
                        if (data.len < 4) break :progress;
                        if (data[3] != ';') break :progress;
                        // parse the progress value
                        parser.command.conemu_progress_report.progress = value: {
                            break :value @intCast(std.math.clamp(
                                std.fmt.parseUnsigned(usize, data[4..], 10) catch break :value null,
                                0,
                                100,
                            ));
                        };
                    },
                }
                return &parser.command;
            },
            // OSC 9;5
            '5' => {
                parser.command = .conemu_wait_input;
                return &parser.command;
            },
            // OSC 9;6
            '6' => {
                if (data.len < 2) break :conemu;
                if (data[1] != ';') break :conemu;
                writer.writeByte(0) catch {
                    parser.state = .invalid;
                    return null;
                };
                data = writer.buffered();
                parser.command = .{
                    .conemu_guimacro = data[2 .. data.len - 1 :0],
                };
                return &parser.command;
            },
            // OSC 9;7
            '7' => {
                if (data.len < 2) break :conemu;
                if (data[1] != ';') break :conemu;
                parser.state = .invalid;
                return null;
            },
            // OSC 9;8
            '8' => {
                if (data.len < 2) break :conemu;
                if (data[1] != ';') break :conemu;
                parser.state = .invalid;
                return null;
            },
            // OSC 9;9
            '9' => {
                if (data.len < 2) break :conemu;
                if (data[1] != ';') break :conemu;
                parser.state = .invalid;
                return null;
            },
            else => break :conemu,
        }
    }

    // If it's not a ConEmu OSC, it's an iTerm2 notification

    writer.writeByte(0) catch {
        parser.state = .invalid;
        return null;
    };
    const data = writer.buffered();
    parser.command = .{
        .show_desktop_notification = .{
            .title = "",
            .body = data[0 .. data.len - 1 :0],
        },
    };
    return &parser.command;
}

test "OSC 9: show desktop notification" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "9;Hello world";
    for (input) |ch| p.next(ch);

    const cmd = p.end('\x1b').?.*;
    try testing.expect(cmd == .show_desktop_notification);
    try testing.expectEqualStrings("", cmd.show_desktop_notification.title);
    try testing.expectEqualStrings("Hello world", cmd.show_desktop_notification.body);
}

test "OSC 9: show single character desktop notification" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "9;H";
    for (input) |ch| p.next(ch);

    const cmd = p.end('\x1b').?.*;
    try testing.expect(cmd == .show_desktop_notification);
    try testing.expectEqualStrings("", cmd.show_desktop_notification.title);
    try testing.expectEqualStrings("H", cmd.show_desktop_notification.body);
}

test "OSC 9;1: ConEmu sleep" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "9;1;420";
    for (input) |ch| p.next(ch);

    const cmd = p.end('\x1b').?.*;

    try testing.expect(cmd == .conemu_sleep);
    try testing.expectEqual(420, cmd.conemu_sleep.duration_ms);
}

test "OSC 9;1: ConEmu sleep with no value default to 100ms" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "9;1;";
    for (input) |ch| p.next(ch);

    const cmd = p.end('\x1b').?.*;

    try testing.expect(cmd == .conemu_sleep);
    try testing.expectEqual(100, cmd.conemu_sleep.duration_ms);
}

test "OSC 9;1: conemu sleep cannot exceed 10000ms" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "9;1;12345";
    for (input) |ch| p.next(ch);

    const cmd = p.end('\x1b').?.*;

    try testing.expect(cmd == .conemu_sleep);
    try testing.expectEqual(10000, cmd.conemu_sleep.duration_ms);
}

test "OSC 9;1: conemu sleep invalid input" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "9;1;foo";
    for (input) |ch| p.next(ch);

    const cmd = p.end('\x1b').?.*;

    try testing.expect(cmd == .conemu_sleep);
    try testing.expectEqual(100, cmd.conemu_sleep.duration_ms);
}

test "OSC 9;1: conemu sleep -> desktop notification 1" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "9;1";
    for (input) |ch| p.next(ch);

    const cmd = p.end('\x1b').?.*;

    try testing.expect(cmd == .show_desktop_notification);
    try testing.expectEqualStrings("1", cmd.show_desktop_notification.body);
}

test "OSC 9;1: conemu sleep -> desktop notification 2" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "9;1a";
    for (input) |ch| p.next(ch);

    const cmd = p.end('\x1b').?.*;

    try testing.expect(cmd == .show_desktop_notification);
    try testing.expectEqualStrings("1a", cmd.show_desktop_notification.body);
}

test "OSC 9;2: ConEmu message box" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "9;2;hello world";
    for (input) |ch| p.next(ch);

    const cmd = p.end('\x1b').?.*;
    try testing.expect(cmd == .conemu_show_message_box);
    try testing.expectEqualStrings("hello world", cmd.conemu_show_message_box);
}

test "OSC 9;2: ConEmu message box invalid input" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "9;2";
    for (input) |ch| p.next(ch);

    const cmd = p.end('\x1b').?.*;
    try testing.expect(cmd == .show_desktop_notification);
    try testing.expectEqualStrings("2", cmd.show_desktop_notification.body);
}

test "OSC 9;2: ConEmu message box empty message" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "9;2;";
    for (input) |ch| p.next(ch);

    const cmd = p.end('\x1b').?.*;
    try testing.expect(cmd == .conemu_show_message_box);
    try testing.expectEqualStrings("", cmd.conemu_show_message_box);
}

test "OSC 9;2: ConEmu message box spaces only message" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "9;2;   ";
    for (input) |ch| p.next(ch);

    const cmd = p.end('\x1b').?.*;
    try testing.expect(cmd == .conemu_show_message_box);
    try testing.expectEqualStrings("   ", cmd.conemu_show_message_box);
}

test "OSC 9;2: message box -> desktop notification 1" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "9;2";
    for (input) |ch| p.next(ch);

    const cmd = p.end('\x1b').?.*;

    try testing.expect(cmd == .show_desktop_notification);
    try testing.expectEqualStrings("2", cmd.show_desktop_notification.body);
}

test "OSC 9;2: message box -> desktop notification 2" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "9;2a";
    for (input) |ch| p.next(ch);

    const cmd = p.end('\x1b').?.*;

    try testing.expect(cmd == .show_desktop_notification);
    try testing.expectEqualStrings("2a", cmd.show_desktop_notification.body);
}

test "OSC 9;3: ConEmu change tab title" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "9;3;foo bar";
    for (input) |ch| p.next(ch);

    const cmd = p.end('\x1b').?.*;
    try testing.expect(cmd == .conemu_change_tab_title);
    try testing.expectEqualStrings("foo bar", cmd.conemu_change_tab_title.value);
}

test "OSC 9;3: ConEmu change tab title reset" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "9;3;";
    for (input) |ch| p.next(ch);

    const cmd = p.end('\x1b').?.*;

    const expected_command: Command = .{ .conemu_change_tab_title = .reset };
    try testing.expectEqual(expected_command, cmd);
}

test "OSC 9;3: ConEmu change tab title spaces only" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "9;3;   ";
    for (input) |ch| p.next(ch);

    const cmd = p.end('\x1b').?.*;

    try testing.expect(cmd == .conemu_change_tab_title);
    try testing.expectEqualStrings("   ", cmd.conemu_change_tab_title.value);
}

test "OSC 9;3: change tab title -> desktop notification 1" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "9;3";
    for (input) |ch| p.next(ch);

    const cmd = p.end('\x1b').?.*;

    try testing.expect(cmd == .show_desktop_notification);
    try testing.expectEqualStrings("3", cmd.show_desktop_notification.body);
}

test "OSC 9;3: message box -> desktop notification 2" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "9;3a";
    for (input) |ch| p.next(ch);

    const cmd = p.end('\x1b').?.*;

    try testing.expect(cmd == .show_desktop_notification);
    try testing.expectEqualStrings("3a", cmd.show_desktop_notification.body);
}

test "OSC 9;4: ConEmu progress set" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "9;4;1;100";
    for (input) |ch| p.next(ch);

    const cmd = p.end('\x1b').?.*;
    try testing.expect(cmd == .conemu_progress_report);
    try testing.expect(cmd.conemu_progress_report.state == .set);
    try testing.expect(cmd.conemu_progress_report.progress == 100);
}

test "OSC 9;4: ConEmu progress set overflow" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "9;4;1;900";
    for (input) |ch| p.next(ch);

    const cmd = p.end('\x1b').?.*;
    try testing.expect(cmd == .conemu_progress_report);
    try testing.expect(cmd.conemu_progress_report.state == .set);
    try testing.expectEqual(100, cmd.conemu_progress_report.progress);
}

test "OSC 9;4: ConEmu progress set single digit" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "9;4;1;9";
    for (input) |ch| p.next(ch);

    const cmd = p.end('\x1b').?.*;
    try testing.expect(cmd == .conemu_progress_report);
    try testing.expect(cmd.conemu_progress_report.state == .set);
    try testing.expect(cmd.conemu_progress_report.progress == 9);
}

test "OSC 9;4: ConEmu progress set double digit" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "9;4;1;94";
    for (input) |ch| p.next(ch);

    const cmd = p.end('\x1b').?.*;
    try testing.expect(cmd == .conemu_progress_report);
    try testing.expect(cmd.conemu_progress_report.state == .set);
    try testing.expectEqual(94, cmd.conemu_progress_report.progress);
}

test "OSC 9;4: ConEmu progress set extra semicolon ignored" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "9;4;1;100";
    for (input) |ch| p.next(ch);

    const cmd = p.end('\x1b').?.*;
    try testing.expect(cmd == .conemu_progress_report);
    try testing.expect(cmd.conemu_progress_report.state == .set);
    try testing.expectEqual(100, cmd.conemu_progress_report.progress);
}

test "OSC 9;4: ConEmu progress remove with no progress" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "9;4;0;";
    for (input) |ch| p.next(ch);

    const cmd = p.end('\x1b').?.*;
    try testing.expect(cmd == .conemu_progress_report);
    try testing.expect(cmd.conemu_progress_report.state == .remove);
    try testing.expect(cmd.conemu_progress_report.progress == null);
}

test "OSC 9;4: ConEmu progress remove with double semicolon" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "9;4;0;;";
    for (input) |ch| p.next(ch);

    const cmd = p.end('\x1b').?.*;
    try testing.expect(cmd == .conemu_progress_report);
    try testing.expect(cmd.conemu_progress_report.state == .remove);
    try testing.expect(cmd.conemu_progress_report.progress == null);
}

test "OSC 9;4: ConEmu progress remove ignores progress" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "9;4;0;100";
    for (input) |ch| p.next(ch);

    const cmd = p.end('\x1b').?.*;
    try testing.expect(cmd == .conemu_progress_report);
    try testing.expect(cmd.conemu_progress_report.state == .remove);
    try testing.expect(cmd.conemu_progress_report.progress == null);
}

test "OSC 9;4: ConEmu progress remove extra semicolon" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "9;4;0;100;";
    for (input) |ch| p.next(ch);

    const cmd = p.end('\x1b').?.*;
    try testing.expect(cmd == .conemu_progress_report);
    try testing.expect(cmd.conemu_progress_report.state == .remove);
}

test "OSC 9;4: ConEmu progress error" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "9;4;2";
    for (input) |ch| p.next(ch);

    const cmd = p.end('\x1b').?.*;
    try testing.expect(cmd == .conemu_progress_report);
    try testing.expect(cmd.conemu_progress_report.state == .@"error");
    try testing.expect(cmd.conemu_progress_report.progress == null);
}

test "OSC 9;4: ConEmu progress error with progress" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "9;4;2;100";
    for (input) |ch| p.next(ch);

    const cmd = p.end('\x1b').?.*;
    try testing.expect(cmd == .conemu_progress_report);
    try testing.expect(cmd.conemu_progress_report.state == .@"error");
    try testing.expect(cmd.conemu_progress_report.progress == 100);
}

test "OSC 9;4: progress pause" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "9;4;4";
    for (input) |ch| p.next(ch);

    const cmd = p.end('\x1b').?.*;
    try testing.expect(cmd == .conemu_progress_report);
    try testing.expect(cmd.conemu_progress_report.state == .pause);
    try testing.expect(cmd.conemu_progress_report.progress == null);
}

test "OSC 9;4: ConEmu progress pause with progress" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "9;4;4;100";
    for (input) |ch| p.next(ch);

    const cmd = p.end('\x1b').?.*;
    try testing.expect(cmd == .conemu_progress_report);
    try testing.expect(cmd.conemu_progress_report.state == .pause);
    try testing.expect(cmd.conemu_progress_report.progress == 100);
}

test "OSC 9;4: progress -> desktop notification 1" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "9;4";
    for (input) |ch| p.next(ch);

    const cmd = p.end('\x1b').?.*;

    try testing.expect(cmd == .show_desktop_notification);
    try testing.expectEqualStrings("4", cmd.show_desktop_notification.body);
}

test "OSC 9;4: progress -> desktop notification 2" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "9;4;";
    for (input) |ch| p.next(ch);

    const cmd = p.end('\x1b').?.*;

    try testing.expect(cmd == .show_desktop_notification);
    try testing.expectEqualStrings("4;", cmd.show_desktop_notification.body);
}

test "OSC 9;4: progress -> desktop notification 3" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "9;4;5";
    for (input) |ch| p.next(ch);

    const cmd = p.end('\x1b').?.*;

    try testing.expect(cmd == .show_desktop_notification);
    try testing.expectEqualStrings("4;5", cmd.show_desktop_notification.body);
}

test "OSC 9;4: progress -> desktop notification 4" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "9;4;5a";
    for (input) |ch| p.next(ch);

    const cmd = p.end('\x1b').?.*;

    try testing.expect(cmd == .show_desktop_notification);
    try testing.expectEqualStrings("4;5a", cmd.show_desktop_notification.body);
}

test "OSC 9;5: ConEmu wait input" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "9;5";
    for (input) |ch| p.next(ch);

    const cmd = p.end('\x1b').?.*;
    try testing.expect(cmd == .conemu_wait_input);
}

test "OSC 9;5: ConEmu wait ignores trailing characters" {
    const testing = std.testing;

    var p: Parser = .init(null);

    const input = "9;5;foo";
    for (input) |ch| p.next(ch);

    const cmd = p.end('\x1b').?.*;
    try testing.expect(cmd == .conemu_wait_input);
}

test "OSC 9;6: ConEmu guimacro 1" {
    const testing = std.testing;

    var p: Parser = .init(testing.allocator);
    defer p.deinit();

    const input = "9;6;a";
    for (input) |ch| p.next(ch);

    const cmd = p.end('\x1b').?.*;
    try testing.expect(cmd == .conemu_guimacro);
    try testing.expectEqualStrings("a", cmd.conemu_guimacro);
}

test "OSC: 9;6: ConEmu guimacro 2" {
    const testing = std.testing;

    var p: Parser = .init(testing.allocator);
    defer p.deinit();

    const input = "9;6;ab";
    for (input) |ch| p.next(ch);

    const cmd = p.end('\x1b').?.*;
    try testing.expect(cmd == .conemu_guimacro);
    try testing.expectEqualStrings("ab", cmd.conemu_guimacro);
}

test "OSC: 9;6: ConEmu guimacro 3 incomplete -> desktop notification" {
    const testing = std.testing;

    var p: Parser = .init(testing.allocator);
    defer p.deinit();

    const input = "9;6";
    for (input) |ch| p.next(ch);

    const cmd = p.end('\x1b').?.*;
    try testing.expect(cmd == .show_desktop_notification);
    try testing.expectEqualStrings("6", cmd.show_desktop_notification.body);
}
