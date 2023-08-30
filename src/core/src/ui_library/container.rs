use std::{cell::RefCell, rc::Weak};

use crate::{
    graphics::{Color, Position, Size},
    state::StateManager,
    widget_default_methods,
};

use super::{Key, Widget, WidgetData};
use key_segment::KeySegment;
use key_segment_derive::KeySegment;

#[derive(Debug, KeySegment)]
pub struct Container {
    widget_data: WidgetData,
    background: Color,
    child: Box<dyn Widget>,
}

impl Container {
    pub fn new(background: Color, child: Box<dyn Widget>) -> Box<Self> {
        return Box::new(Self {
            widget_data: WidgetData::new(),
            background: background,
            child: child,
        });
    }
}

impl Widget for Container {
    widget_default_methods!();

    fn set_layout(&mut self, position: Position, available_space: Size) {
        self.widget_data.position = position;
        self.widget_data.available_space = available_space;
        self.child
            .set_layout(Position::origin(), self.widget_data.available_space.clone());
    }

    fn draw(&self, parent_position: Position, platform: &dyn crate::platform::Platform) -> () {
        platform.graphics().fill_rect(
            &(parent_position.clone() + self.widget_data.position.clone()),
            &self.child.get_size(&self.widget_data.available_space),
            &self.background,
        );
        self.child.draw(
            parent_position + self.widget_data.position.clone(),
            platform,
        );
    }

    fn get_width(&self, available_space: &Size) -> f64 {
        return self.child.get_width(available_space);
    }

    fn get_height(&self, available_space: &Size) -> f64 {
        return self.child.get_height(available_space);
    }

    fn get_children_mut(&mut self) -> Vec<&mut dyn Widget> {
        return vec![self.child.as_mut()];
    }
    fn get_children(&self) -> Vec<&dyn Widget> {
        return vec![self.child.as_ref()];
    }
}
