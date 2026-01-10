const std = @import("std");

pub const change_window_icon = @import("parsers/change_window_icon.zig");
pub const change_window_title = @import("parsers/change_window_title.zig");
pub const clipboard_operation = @import("parsers/clipboard_operation.zig");
pub const color = @import("parsers/color.zig");
pub const hyperlink = @import("parsers/hyperlink.zig");
pub const kitty_color = @import("parsers/kitty_color.zig");
pub const mouse_shape = @import("parsers/mouse_shape.zig");
pub const osc9 = @import("parsers/osc9.zig");
pub const report_pwd = @import("parsers/report_pwd.zig");
pub const rxvt_extension = @import("parsers/rxvt_extension.zig");
pub const semantic_prompt = @import("parsers/semantic_prompt.zig");

test {
    _ = change_window_icon;
    _ = change_window_title;
    _ = clipboard_operation;
    _ = color;
    _ = hyperlink;
    _ = kitty_color;
    _ = mouse_shape;
    _ = osc9;
    _ = report_pwd;
    _ = rxvt_extension;
    _ = semantic_prompt;
}
