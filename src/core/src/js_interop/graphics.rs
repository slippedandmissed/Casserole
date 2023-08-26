use wasm_bindgen::prelude::*;
use crate::graphics as native_graphics;

#[wasm_bindgen]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl Size {
    pub fn from_native(native: native_graphics::Size) -> Self {
        return Self { width: native.width, height: native.height };
    }

    pub fn to_native(&self) -> native_graphics::Size {
        return native_graphics::Size { width: self.width, height: self.height };
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
