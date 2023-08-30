use std::{rc::Weak, cell::RefCell};

use crate::{graphics::{Position, Size}, state::StateManager, widget_default_methods};

use super::{Widget, Key, KeySegment, WidgetData};


#[derive(Debug)]
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

impl KeySegment for SizedBox {
    fn key_segment(&self) -> String {
        return "SizedBox".to_string();
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
        self.child
            .set_layout(Position::origin(), self.size.clone());
    }

    fn draw(&self, parent_position: Position, platform: &dyn crate::platform::Platform) -> () {
        self.child
            .draw(parent_position + self.widget_data.position.clone(), platform);
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
