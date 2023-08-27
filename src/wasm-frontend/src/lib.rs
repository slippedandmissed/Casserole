use wasm_bindgen::prelude::*;

use casserole_core::platform::Platform;
use logging::LOGGER;
use graphics::GRAPHICS_LIBRARY;

pub mod event_handlers;
pub mod graphics;
pub mod logging;


pub struct WASMPlatform;
impl Platform for WASMPlatform {
    fn graphics(&self) -> &dyn casserole_core::graphics::GraphicsLibrary {
        return &GRAPHICS_LIBRARY;
    }
    fn logger(&self) -> &dyn casserole_core::logging::Logger {
        return &LOGGER;
    }
}
const PLATFORM: WASMPlatform = WASMPlatform {};

#[wasm_bindgen]
pub fn entry_point() {
    casserole_core::entry_point(&PLATFORM);
}
