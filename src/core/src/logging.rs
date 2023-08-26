use web_sys::console as js_console;

pub fn log(msg: &str) {
    js_console::log_1(&msg.into());
}
