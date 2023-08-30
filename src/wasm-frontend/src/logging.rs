use web_sys::console as js_console;
use casserole_core::logging::Logger;

pub struct WASMLogger;
impl Logger for WASMLogger {
    fn log(&self, msg: &str) {
        js_console::log_1(&msg.into());
    }
}
