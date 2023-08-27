use std::process::exit;

use crate::platform::Platform;

pub fn on_window_resize(p: &dyn Platform) {
  crate::draw_frame(p);
}

pub fn on_quit() {
  exit(0);
}