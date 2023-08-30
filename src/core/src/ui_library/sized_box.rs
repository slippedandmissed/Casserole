use std::{cell::RefCell, rc::Weak};

use key_segment::KeySegment;
use key_segment_derive::KeySegment;

use crate::{
    graphics::{Position, Size},
    state::StateManager,
    widget_default_methods,
};

use super::{Key, Widget, WidgetData};

#[derive(Debug, KeySegment)]
pub struct SizedBox {
    widget_data: WidgetData,
    size: Size,
    child: Box<dyn Widget>,
}

impl SizedBox {
    pub fn new(size: Size, child: Box<dyn Widget>) -> Box<Self> {
        return Box::new(Self {
            widget_data: WidgetData::new(),
            size,
            child,
        });
    }
}

impl Widget for SizedBox {
    widget_default_methods!();

    fn does_expand(&self) -> bool {
        return false;
    }

    fn set_layout(&mut self, position: Position, available_space: Size) {
        self.widget_data.position = position;
        self.widget_data.available_space = available_space;
        self.child.set_layout(Position::origin(), self.size.clone());
    }

    fn draw(&self, parent_position: Position, platform: &dyn crate::platform::Platform) -> () {
        self.child.draw(
            parent_position + self.widget_data.position.clone(),
            platform,
        );
    }

    fn get_width(&self, _available_space: &Size) -> f64 {
        return self.child.get_width(&self.size);
    }

    fn get_height(&self, _available_space: &Size) -> f64 {
        return self.child.get_height(&self.size);
    }

    fn get_children_mut(&mut self) -> Vec<&mut dyn Widget> {
        return vec![self.child.as_mut()];
    }
    fn get_children(&self) -> Vec<&dyn Widget> {
        return vec![self.child.as_ref()];
    }
}
