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

pub trait GraphicsLibrary {
    fn get_screen_dimensions(&self) -> Size;
    fn fill_rect(&self, position: Position, size: Size, color: Color);
    fn fill(&self, color: Color);
}
