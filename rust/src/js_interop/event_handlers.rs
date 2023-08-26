use wasm_bindgen::prelude::*;
use crate::event_handlers as native_handlers;

#[wasm_bindgen]
pub fn on_window_resize() {
    native_handlers::on_window_resize();
}

#[wasm_bindgen]
pub fn entry_point() {
    crate::entry_point();
}
