use std::{cell::RefCell, rc::Weak};

use crate::{
    event_handlers::HandledEventInfo,
    graphics::{Position, Size},
    state::{State, StateManager},
    ui_library::{Key, KeySegment, StatefulWidget, StatefulWidgetEventHandlerInfo, Widget, CompoundWidgetData}, widget_default_methods, compound_widget_default_methods,
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
    widget_data: CompoundWidgetData,

    #[derivative(Debug = "ignore")]
    child: Box<dyn Fn(bool) -> Box<dyn Widget>>,
}

impl Hoverable {
    pub fn new(child: Box<dyn Fn(bool) -> Box<dyn Widget>>) -> Box<Self> {
        return Box::new(Self {
            widget_data: CompoundWidgetData::new(),
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

    compound_widget_default_methods!();

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
