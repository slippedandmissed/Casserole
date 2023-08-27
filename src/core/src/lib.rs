pub mod graphics;
pub mod event_handlers;
pub mod logging;
pub mod platform;

use crate::platform::Platform;
use crate::graphics::*;

pub fn draw_frame(p: &dyn Platform) {
    let g = p.graphics();
    let l = p.logger();
    let screen_size = g.get_screen_dimensions();
    l.log(&format!("width: {}, height: {}", screen_size.width, screen_size.height));
    let rect_size = Size { width: 168, height: 100 };

    g.fill(Color { r: 0, g: 255, b: 255 });
    g.fill_rect(
        Position {
            x: i32::try_from((screen_size.width - rect_size.width) / 2).unwrap(),
            y: i32::try_from((screen_size.height - rect_size.height) / 2).unwrap(),
        },
        rect_size,
        Color { r: 255, g: 0, b: 0 }
    );

    g.update();
}

pub fn entry_point(p: &dyn Platform) {
    draw_frame(p);
}
