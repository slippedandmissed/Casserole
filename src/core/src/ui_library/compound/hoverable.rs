use std::{cell::RefCell, rc::Weak};

use crate::{
    event_handlers::HandledEventInfo,
    graphics::{Position, Size},
    state::{State, StateManager},
    ui_library::{Key, KeySegment, StatefulWidget, StatefulWidgetEventHandlerInfo, Widget},
};
use derivative::Derivative;

pub struct HoverableState {
    is_hovered: bool,
}

impl HoverableState {
    pub fn new() -> Box<Self> {
        return Box::new(Self { is_hovered: false });
    }
}

impl State for HoverableState {}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Hoverable {
    key: Option<Key>,
    #[derivative(Debug = "ignore")]
    state_manager: Weak<RefCell<StateManager>>,
    cached_build: Option<Box<dyn Widget>>,
    position: Position,
    available_space: Size,

    #[derivative(Debug = "ignore")]
    child: Box<dyn Fn(bool) -> Box<dyn Widget>>,
}

impl Hoverable {
    pub fn new(child: Box<dyn Fn(bool) -> Box<dyn Widget>>) -> Box<Self> {
        return Box::new(Self {
            key: None,
            state_manager: Weak::new(),
            cached_build: None,
            position: Position::origin(),
            available_space: Size::zero(),
            child,
        });
    }
}

impl KeySegment for Hoverable {
    fn key_segment(&self) -> String {
        return "Hoverable".to_string();
    }
}

impl StatefulWidget for Hoverable {
    type T = HoverableState;

    fn get_key(&self) -> &Key {
        return match &self.key {
            Some(x) => x,
            None => panic!(),
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

    fn state_factory(&self) -> Box<Self::T> {
        return HoverableState::new();
    }

    fn build(&self, state: &mut HoverableState) -> Box<dyn Widget> {
        return (self.child)(state.is_hovered);
    }

    fn on_mouse_move(
        &mut self,
        state: &mut HoverableState,
        mouse_pos: &Position,
        platform: &dyn crate::platform::Platform,
        handled_event_info: &mut HandledEventInfo,
    ) -> StatefulWidgetEventHandlerInfo {
        platform.logger().log(&format!(
            "Mouse moved on hoverable: {:?}, {:?}",
            mouse_pos,
            StatefulWidget::get_key(self)
        ));

        state.is_hovered = true;

        handled_event_info.needs_relayout = true;
        handled_event_info.needs_redraw = true;
        return StatefulWidgetEventHandlerInfo {
            was_captured: true,
            needs_rebuild: true,
        };
    }
}
