use wasm_bindgen::prelude::*;
use casserole_core::graphics::{self as core_graphics, GraphicsLibrary, Position};

#[wasm_bindgen]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

impl Size {
    pub fn from_native(native: core_graphics::Size) -> Self {
        return Self { width: native.width, height: native.height };
    }

    pub fn to_native(&self) -> core_graphics::Size {
        return core_graphics::Size { width: self.width, height: self.height };
    }
}

#[wasm_bindgen]
pub fn return_size(width: f64, height: f64) -> Size {
    return Size { width, height };
}

#[wasm_bindgen]
extern {
  #[wasm_bindgen(js_namespace = ["window", "library", "graphics"])]
  pub fn getScreenDimensions() -> Size;
  #[wasm_bindgen(js_namespace = ["window", "library", "graphics"])]
  pub fn setFillStyle(fillStyle: &str);
  #[wasm_bindgen(js_namespace = ["window", "library", "graphics"])]
  pub fn fillRect(x: f64, y: f64, width: f64, height: f64);
}

pub struct WASMGraphicsLibrary;
impl GraphicsLibrary for WASMGraphicsLibrary {
    fn fill(&self, color: &casserole_core::graphics::Color) {
        self.fill_rect(&Position { x: 0., y: 0. }, &self.get_screen_dimensions(), &color);
    }
    fn fill_rect(&self, position: &casserole_core::graphics::Position, size: &casserole_core::graphics::Size, color: &casserole_core::graphics::Color) {
        setFillStyle(&format!("rgb({},{},{})", color.r, color.g, color.b));
        fillRect(position.x, position.y, size.width, size.height);
    }
    fn get_screen_dimensions(&self) -> casserole_core::graphics::Size {
        return getScreenDimensions().to_native();
    }
    fn update(&self) {
        // The screen automatically updates in the canvas
    }
}
