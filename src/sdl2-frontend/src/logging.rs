use casserole_core::logging::Logger;

pub struct SDL2Logger;
impl Logger for SDL2Logger {
    fn log(&self, msg: &str) {
        println!("{}", msg);
    }
}

pub const LOGGER: SDL2Logger = SDL2Logger {};
