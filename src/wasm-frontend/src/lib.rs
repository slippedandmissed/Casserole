pub mod event_handlers;
pub mod graphics;
pub mod logging;

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;

use crate::graphics::WASMGraphicsLibrary;
use crate::logging::WASMLogger;

use casserole_core::platform::Platform;

pub struct WASMPlatform {
    graphics: WASMGraphicsLibrary,
    logger: WASMLogger,
}
impl WASMPlatform {
    pub fn new() -> Self {
        return Self {
            graphics: WASMGraphicsLibrary {},
            logger: WASMLogger {},
        };
    }
}
impl Platform for WASMPlatform {
    fn graphics(&self) -> &dyn casserole_core::graphics::GraphicsLibrary {
        return &self.graphics;
    }
    fn logger(&self) -> &dyn casserole_core::logging::Logger {
        return &self.logger;
    }
}

#[wasm_bindgen]
pub fn entry_point() {
    let platform = WASMPlatform::new();
    let app_runner = Rc::new(RefCell::new(casserole_core::entry_point(platform)));
    app_runner.borrow().draw_frame();
    event_handlers::register_event_listeners(app_runner.clone());
}
