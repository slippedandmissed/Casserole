use wasm_bindgen::prelude::*;
use casserole_core::graphics::{self as core_graphics, GraphicsLibrary, Position};

#[wasm_bindgen]
pub struct Size {
    pub width: u32,
    pub height: u32,
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
pub fn return_size(width: u32, height: u32) -> Size {
    return Size { width, height };
}

#[wasm_bindgen]
extern {
  #[wasm_bindgen(js_namespace = ["window", "library", "graphics"])]
  pub fn getScreenDimensions() -> Size;
  #[wasm_bindgen(js_namespace = ["window", "library", "graphics"])]
  pub fn setFillStyle(fillStyle: &str);
  #[wasm_bindgen(js_namespace = ["window", "library", "graphics"])]
  pub fn fillRect(x: i32, y: i32, width: u32, height: u32);
}

pub struct WASMGraphicsLibrary;
impl GraphicsLibrary for WASMGraphicsLibrary {
    fn fill(&self, color: casserole_core::graphics::Color) {
        self.fill_rect(Position { x: 0, y: 0 }, self.get_screen_dimensions(), color);
    }
    fn fill_rect(&self, position: casserole_core::graphics::Position, size: casserole_core::graphics::Size, color: casserole_core::graphics::Color) {
        setFillStyle(&format!("rgb({},{},{})", color.r, color.g, color.b));
        fillRect(position.x, position.y, size.width, size.height);
    }
    fn get_screen_dimensions(&self) -> casserole_core::graphics::Size {
        return getScreenDimensions().to_native();
    }
}
pub const GRAPHICS_LIBRARY: WASMGraphicsLibrary = WASMGraphicsLibrary {};