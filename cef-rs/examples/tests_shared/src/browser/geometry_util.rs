use cef::*;

pub const fn logical_value_to_device(value: i32, scale_factor: f32) -> i32 {
    (value as f32 * scale_factor) as i32
}

pub const fn logical_rect_to_device(rect: Rect, scale_factor: f32) -> Rect {
    Rect {
        x: logical_value_to_device(rect.x, scale_factor),
        y: logical_value_to_device(rect.y, scale_factor),
        width: logical_value_to_device(rect.width, scale_factor),
        height: logical_value_to_device(rect.height, scale_factor),
    }
}

pub const fn device_value_to_logical(value: i32, scale_factor: f32) -> i32 {
    (value as f32 / scale_factor) as i32
}

pub const fn device_rect_to_logical(rect: Rect, scale_factor: f32) -> Rect {
    Rect {
        x: device_value_to_logical(rect.x, scale_factor),
        y: device_value_to_logical(rect.y, scale_factor),
        width: device_value_to_logical(rect.width, scale_factor),
        height: device_value_to_logical(rect.height, scale_factor),
    }
}

pub const fn device_mouse_event_to_logical(event: MouseEvent, scale_factor: f32) -> MouseEvent {
    MouseEvent {
        x: device_value_to_logical(event.x, scale_factor),
        y: device_value_to_logical(event.y, scale_factor),
        ..event
    }
}

pub const fn device_touch_event_to_logical(event: TouchEvent, scale_factor: f32) -> TouchEvent {
    TouchEvent {
        x: event.x / scale_factor,
        y: event.y / scale_factor,
        ..event
    }
}

pub fn constrain_window_bounds(display: &Rect, window: &mut Rect) {
    window.x = window.x.max(display.x);
    window.y = window.y.max(display.y);
    window.width = window.width.clamp(100, display.width);
    window.height = window.height.clamp(100, display.height);
    if window.x + window.width > display.x + display.width {
        window.x = display.x + display.width - window.width;
    }
    if window.y + window.height > display.y + display.height {
        window.y = display.y + display.height - window.height;
    }
}
