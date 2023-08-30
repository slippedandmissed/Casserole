pub mod event_handlers;
pub mod graphics;
pub mod logging;
pub mod platform;
pub mod ui_library;

use graphics::{Color, Position, Size};
use ui_library::compound::button::Button;
use ui_library::container::Container;
use ui_library::expanded::Expanded;
use ui_library::padding::{Inset, Padding};
use ui_library::sized_box::SizedBox;
use ui_library::{CompoundWidget, Widget};

use crate::platform::Platform;
use crate::ui_library::list::*;

#[derive(Debug)]
pub struct App {
    cached_build: Option<Box<dyn Widget>>,
    position: Position,
    available_space: Size,
}

impl App {
    pub fn new() -> Self {
        return Self {
            cached_build: None,
            position: Position::origin(),
            available_space: Size::zero(),
        };
    }
}

impl CompoundWidget for App {
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

pub struct AppRunner<PlatformType: Platform, AppType: Widget> {
    pub platform: PlatformType,
    pub app: AppType,
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
    let mut app = App::new();
    app.rebuild();
    app.set_layout(
        Position::origin(),
        platform.graphics().get_screen_dimensions(),
    );
    let app_runner = AppRunner { platform, app };
    return app_runner;
}
