#include "imgui.h"

// This file contains custom extensions for functionality that isn't
// properly supported by Dear Bindings yet. Namely:
// https://github.com/dearimgui/dear_bindings/issues/55

// Wrap this in a namespace to keep it separate from the C++ API
namespace cimgui
{
#include "dcimgui.h"
}

extern "C"
{
CIMGUI_API void ImFontConfig_ImFontConfig(cimgui::ImFontConfig* self)
{
    static_assert(sizeof(cimgui::ImFontConfig) == sizeof(::ImFontConfig), "ImFontConfig size mismatch");
    static_assert(alignof(cimgui::ImFontConfig) == alignof(::ImFontConfig), "ImFontConfig alignment mismatch");
    ::ImFontConfig defaults;
    *reinterpret_cast<::ImFontConfig*>(self) = defaults;
}

CIMGUI_API void ImGuiStyle_ImGuiStyle(cimgui::ImGuiStyle* self)
{
    static_assert(sizeof(cimgui::ImGuiStyle) == sizeof(::ImGuiStyle), "ImGuiStyle size mismatch");
    static_assert(alignof(cimgui::ImGuiStyle) == alignof(::ImGuiStyle), "ImGuiStyle alignment mismatch");
    ::ImGuiStyle defaults;
    *reinterpret_cast<::ImGuiStyle*>(self) = defaults;
}
}
