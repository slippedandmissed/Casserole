pub mod event_handlers;
pub mod graphics;
pub mod logging;
pub mod platform;
pub mod ui_library;
pub mod state;

use std::cell::RefCell;
use std::rc::{Weak, Rc};

use graphics::{Color, Position, Size};
use state::StateManager;
use ui_library::compound::button::Button;
use ui_library::container::Container;
use ui_library::expanded::Expanded;
use ui_library::padding::{Inset, Padding};
use ui_library::sized_box::SizedBox;
use ui_library::{CompoundWidget, Widget, Key, KeySegment, CompoundWidgetData};

use crate::platform::Platform;
use crate::ui_library::list::*;


#[derive(Debug)]
pub struct App {
    widget_data: CompoundWidgetData,
}

impl App {
    pub fn new() -> Self {
        return Self {
            widget_data: CompoundWidgetData::new(),
        };
    }
}

impl KeySegment for App {
    fn key_segment(&self) -> String {
        return "App".to_string();
    }
}

impl CompoundWidget for App {
    compound_widget_default_methods!();

    fn build(&self) -> Box<dyn Widget> {
        return List::new(
            ListDirection::Column,
            MainAxisAlignment::Start,
            CrossAxisAlignment::Start,
            MainAxisSize::Max,
            CrossAxisSize::Max,
            vec![
                Container::new(
                    Color { r: 255, g: 0, b: 0 },
                    Padding::new(
                        Inset::symmetric(5., 20.),
                        Some(List::new(
                            ListDirection::Row,
                            MainAxisAlignment::SpaceBetween,
                            CrossAxisAlignment::Center,
                            MainAxisSize::Max,
                            CrossAxisSize::Min,
                            vec![
                                SizedBox::new(
                                    Size {
                                        width: 100.,
                                        height: 20.,
                                    },
                                    Container::new(Color { r: 0, g: 0, b: 255 }, Expanded::new()),
                                ),
                                Button::new(),
                            ],
                        )),
                    ),
                ),
                Container::new(Color { r: 0, g: 255, b: 0 }, Expanded::new()),
            ],
        );
    }
}

pub struct AppRunner<PlatformType: Platform, AppType> where AppType: Widget{
    pub platform: PlatformType,
    pub app: AppType,
    pub state_manager: Rc<RefCell<StateManager>>,
}

impl<PlatformType: Platform, AppType: Widget> AppRunner<PlatformType, AppType> {
    pub fn draw_frame(&self) {
        let g = self.platform.graphics();
        let l = self.platform.logger();
        let screen_size = g.get_screen_dimensions();
        l.log(&format!(
            "width: {}, height: {}",
            screen_size.width, screen_size.height
        ));

        self.app.draw(Position::origin(), &self.platform);

        g.update();
    }
}

pub fn entry_point<PlatformType: Platform>(platform: PlatformType) -> AppRunner<PlatformType, App> {
    let state_manager = Rc::new(RefCell::new(StateManager::new()));
    
    let mut app = App::new();
    app.rebuild_with_key("".into(), Rc::downgrade(&state_manager));
    app.set_layout(
        Position::origin(),
        platform.graphics().get_screen_dimensions(),
    );
    let app_runner = AppRunner { platform, app, state_manager };
    return app_runner;
}
