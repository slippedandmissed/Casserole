use crate::js_interop::graphics as js_graphics;

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub struct Size {
    pub width: u32,
    pub height: u32,
}

pub fn get_screen_dimensions() -> Size {
    return js_graphics::getScreenDimensions().to_native();
}

pub fn fill_rect(position: Position, size: Size, color: Color) {
    js_graphics::setFillStyle(&format!("rgb({},{},{})", color.r, color.g, color.b));
    js_graphics::fillRect(position.x, position.y, size.width, size.height);
}

pub fn fill(color: Color) {
    fill_rect(Position { x: 0, y: 0 }, get_screen_dimensions(), color);
}
