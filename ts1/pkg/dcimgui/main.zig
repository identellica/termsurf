pub const build_options = @import("build_options");

pub const c = @cImport({
    // This is set during the build so it also has to be set
    // during import time to get the right types. Without this
    // you get stack size mismatches on some structs.
    @cDefine("IMGUI_USE_WCHAR32", "1");
    @cInclude("dcimgui.h");
});

// OpenGL3 backend
pub extern fn ImGui_ImplOpenGL3_Init(glsl_version: ?[*:0]const u8) callconv(.c) bool;
pub extern fn ImGui_ImplOpenGL3_Shutdown() callconv(.c) void;
pub extern fn ImGui_ImplOpenGL3_NewFrame() callconv(.c) void;
pub extern fn ImGui_ImplOpenGL3_RenderDrawData(draw_data: *c.ImDrawData) callconv(.c) void;

// Metal backend
pub extern fn ImGui_ImplMetal_Init(device: *anyopaque) callconv(.c) bool;
pub extern fn ImGui_ImplMetal_Shutdown() callconv(.c) void;
pub extern fn ImGui_ImplMetal_NewFrame(render_pass_descriptor: *anyopaque) callconv(.c) void;
pub extern fn ImGui_ImplMetal_RenderDrawData(draw_data: *c.ImDrawData, command_buffer: *anyopaque, command_encoder: *anyopaque) callconv(.c) void;

// OSX
pub extern fn ImGui_ImplOSX_Init(*anyopaque) callconv(.c) bool;
pub extern fn ImGui_ImplOSX_Shutdown() callconv(.c) void;
pub extern fn ImGui_ImplOSX_NewFrame(*anyopaque) callconv(.c) void;

// Internal API functions from dcimgui_internal.h
// We declare these manually because the internal header contains bitfields
// that Zig's cImport cannot translate.
pub extern fn ImGui_DockBuilderDockWindow(window_name: [*:0]const u8, node_id: c.ImGuiID) callconv(.c) void;
pub extern fn ImGui_DockBuilderSplitNode(node_id: c.ImGuiID, split_dir: c.ImGuiDir, size_ratio_for_node_at_dir: f32, out_id_at_dir: *c.ImGuiID, out_id_at_opposite_dir: *c.ImGuiID) callconv(.c) c.ImGuiID;
pub extern fn ImGui_DockBuilderFinish(node_id: c.ImGuiID) callconv(.c) void;

// Extension functions from ext.cpp
pub const ext = struct {
    pub extern fn ImFontConfig_ImFontConfig(self: *c.ImFontConfig) callconv(.c) void;
    pub extern fn ImGuiStyle_ImGuiStyle(self: *c.ImGuiStyle) callconv(.c) void;
};

test {
    _ = c;
}
