use std::fmt::Debug;

use crate::{
    event_handlers::HandledEventInfo,
    graphics::{Color, Position, Size},
    ui_library::{
        container::Container, expanded::Expanded, sized_box::SizedBox, CompoundWidget, Widget,
    },
};

pub struct Hoverable {
    cached_build: Option<Box<dyn Widget>>,
    position: Position,
    available_space: Size,

    is_hovered: bool,

    child: Box<dyn Fn(bool) -> Box<dyn Widget>>,
}

impl Debug for Hoverable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return f.write_str("Hoverable");
    }
}

impl Hoverable {
    pub fn new(child: Box<dyn Fn(bool) -> Box<dyn Widget>>) -> Box<Self> {
        return Box::new(Self {
            cached_build: None,
            position: Position::origin(),
            available_space: Size::zero(),
            is_hovered: false,
            child,
        });
    }
}

impl CompoundWidget for Hoverable {
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
        return (self.child)(self.is_hovered);
    }

    fn on_mouse_move(
        &mut self,
        mouse_pos: &Position,
        platform: &dyn crate::platform::Platform,
        handled_event_info: &mut HandledEventInfo,
    ) -> bool {
        platform
            .logger()
            .log(&format!("Mouse moved on hoverable: {:?}", mouse_pos));
        self.is_hovered = true;
        self.rebuild();
        handled_event_info.needs_relayout = true;
        handled_event_info.needs_redraw = true;
        return true;
    }
}
