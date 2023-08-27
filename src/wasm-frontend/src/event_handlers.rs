use wasm_bindgen::prelude::*;
use casserole_core::event_handlers as native_handlers;

#[wasm_bindgen]
pub fn on_window_resize() {
    native_handlers::on_window_resize(&crate::PLATFORM);
}
