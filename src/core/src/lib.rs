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
use ui_library::{CompoundWidget, Widget, Key, KeySegment};

use crate::platform::Platform;
use crate::ui_library::list::*;

use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct App {
    key: Option<Key>,
    #[derivative(Debug = "ignore")]
    state_manager: Weak<RefCell<StateManager>>,
    cached_build: Option<Box<dyn Widget>>,
    position: Position,
    available_space: Size,
}

impl App {
    pub fn new() -> Self {
        return Self {
            key: None,
            state_manager: Weak::new(),
            cached_build: None,
            position: Position::origin(),
            available_space: Size::zero(),
        };
    }
}

impl KeySegment for App {
    fn key_segment(&self) -> String {
        return "App".to_string();
    }
}

impl CompoundWidget for App {
    fn get_key(&self) -> &Key {
        return match &self.key {
            Some(x) => x,
            None => panic!()
        };
    }

    fn set_key(&mut self, key: Key) -> () {
        self.key = Some(key);
    }
    
    fn get_state_manager(&self) -> Weak<RefCell<StateManager>> {
        return self.state_manager.clone();
    }

    fn set_state_manager(&mut self, state_manager: Weak<RefCell<StateManager>>) -> () {
        self.state_manager = state_manager;
    }

    fn get_cached_build_mut(&mut self) -> Option<&mut dyn Widget> {
        return match &mut self.cached_build {
            Some(x) => Some(x.as_mut()),
            None => None,
        };
    }
    fn get_cached_build(&self) -> Option<&dyn Widget> {
        return match &self.cached_build {
            Some(x) => Some(x.as_ref()),
            None => None,
        };
    }

    fn set_cached_build(&mut self, cached_build: Box<dyn Widget>) -> () {
        self.cached_build = Some(cached_build);
    }

    fn get_position(&self) -> &Position {
        return &self.position;
    }

    fn set_position(&mut self, position: Position) -> () {
        self.position = position;
    }

    fn get_available_space(&self) -> &Size {
        return &self.available_space;
    }

    fn set_available_space(&mut self, available_space: Size) -> () {
        self.available_space = available_space;
    }

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
