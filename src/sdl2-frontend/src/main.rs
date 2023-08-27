pub mod graphics;
pub mod logging;


use casserole_core::{platform::Platform, entry_point};
use graphics::GRAPHICS_LIBRARY;
use logging::LOGGER;
use once_cell::sync::Lazy;

pub struct SDL2Platform;

impl Platform for SDL2Platform {
    fn graphics(&self) -> &dyn casserole_core::graphics::GraphicsLibrary {
        let graphics = Lazy::force(&GRAPHICS_LIBRARY);
        return graphics;
    }
    fn logger(&self) -> &dyn casserole_core::logging::Logger {
        return &LOGGER;
    }
}

pub const PLATFORM: SDL2Platform = SDL2Platform {};
 
pub fn main() {
    let graphics = Lazy::force(&GRAPHICS_LIBRARY);
    graphics.init();
    entry_point(&PLATFORM);
    loop {}
}
