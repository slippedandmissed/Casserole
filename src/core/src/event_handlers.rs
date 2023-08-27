use crate::platform::Platform;

pub fn on_window_resize(p: &dyn Platform) {
  crate::draw_frame(p);
}
