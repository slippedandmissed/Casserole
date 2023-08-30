pub mod graphics;
pub mod logging;


use std::{rc::Rc, cell::RefCell};

use casserole_core::platform::Platform;
use graphics::SDL2GraphicsLibrary;
use logging::SDL2Logger;
pub struct SDL2Platform {
    graphics: SDL2GraphicsLibrary,
    logger: SDL2Logger,
}

impl SDL2Platform {
    pub fn new() -> Self {
        return Self {
            graphics: SDL2GraphicsLibrary::new(),
            logger: SDL2Logger { },
        };
    }
}

impl Platform for SDL2Platform {
    fn graphics(&self) -> &dyn casserole_core::graphics::GraphicsLibrary {
        return &self.graphics;
    }
    fn logger(&self) -> &dyn casserole_core::logging::Logger {
        return &self.logger;
    }
}
 
pub fn main() {
    let platform = SDL2Platform::new();
    platform.graphics.init();
    let app_runner = Rc::new(RefCell::new(casserole_core::entry_point(platform)));
    app_runner.borrow().draw_frame();
    let event_receiver = app_runner.borrow().platform.graphics.event_receiver.clone();
    loop {
        for event in event_receiver.try_iter() {
            match event {
                graphics::EventData::Quit => {
                    app_runner.borrow_mut().on_quit();
                }
                graphics::EventData::WindowResize => {
                    app_runner.borrow_mut().on_window_resize();
                }
                graphics::EventData::MouseMove(position) => {
                    app_runner.borrow_mut().on_mouse_move(position);
                }
            }
        }
    }
}
