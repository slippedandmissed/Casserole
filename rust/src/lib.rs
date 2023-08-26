pub mod graphics;
pub mod js_interop;
pub mod event_handlers;
pub mod logging;

use crate::graphics::*;
use crate::logging::*;

pub fn draw_frame() {
    let screen_size = get_screen_dimensions();
    log(&format!("width: {}, height: {}", screen_size.width, screen_size.height));
    let rect_size = Size { width: 168, height: 100 };

    fill(Color { r: 0, g: 255, b: 255 });
    fill_rect(
        Position {
            x: i32::try_from((screen_size.width - rect_size.width) / 2).unwrap(),
            y: i32::try_from((screen_size.height - rect_size.height) / 2).unwrap(),
        },
        rect_size,
        Color { r: 255, g: 0, b: 0 }
    )
}

pub fn entry_point() {
    draw_frame();
}
