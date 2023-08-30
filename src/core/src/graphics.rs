#[derive(Debug, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Position {
    pub fn origin() -> Self {
        return Self { x: 0., y: 0. };
    }
}

impl std::ops::Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

impl Size {
    pub fn zero() -> Self {
        return Self {
            width: 0.,
            height: 0.,
        };
    }
}

pub trait GraphicsLibrary {
    fn get_screen_dimensions(&self) -> Size;
    fn fill_rect(&self, position: &Position, size: &Size, color: &Color);
    fn fill(&self, color: &Color);
    fn update(&self);
}
