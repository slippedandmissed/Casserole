use std::process::exit;

use crate::{graphics::Position, platform::Platform, ui_library::Widget, AppRunner};

pub struct HandledEventInfo {
    pub needs_relayout: bool,
    pub needs_redraw: bool,
}

impl HandledEventInfo {
    pub fn new() -> Self {
        return Self {
            needs_relayout: false,
            needs_redraw: false,
        };
    }
}

impl<PlatformType: Platform, AppType: Widget> AppRunner<PlatformType, AppType> {
    pub fn on_window_resize(&mut self) {
        self.app.set_layout(
            Position::origin(),
            self.platform.graphics().get_screen_dimensions(),
        );
        self.draw_frame();
    }

    pub fn on_quit(&self) {
        exit(0);
    }

    pub fn on_mouse_move(&mut self, position: Position) {
        let mut handled_event_info = HandledEventInfo::new();
        self.app.on_mouse_move_anywhere(
            Position::origin(),
            &position,
            &self.platform,
            &mut handled_event_info,
        );
        if handled_event_info.needs_relayout {
            self.app.set_layout(
                Position::origin(),
                self.platform.graphics().get_screen_dimensions(),
            );
        }
        if handled_event_info.needs_redraw {
            self.draw_frame();
        }
    }
}
