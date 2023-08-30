use std::{rc::Weak, cell::RefCell};

use crate::{graphics::{Position, Size}, state::StateManager, widget_default_methods};

use super::{Widget, Key, KeySegment, WidgetData};


#[derive(Debug)]
pub struct Expanded {
    widget_data: WidgetData,
}

impl Expanded {
    pub fn new() -> Box<Self> {
        return Box::new(Self {
            widget_data: WidgetData::new(),
        });
    }
}

impl KeySegment for Expanded {
    fn key_segment(&self) -> String {
        return "Expanded".to_string();
    }
}

impl Widget for Expanded {
    widget_default_methods!();

    fn does_expand(&self) -> bool {
        return true;
    }

    fn set_layout(&mut self, position: Position, available_space: Size) {
        self.widget_data.position = position;
        self.widget_data.available_space = available_space;
    }

    fn draw(&self, _parent_position: Position, _platform: &dyn crate::platform::Platform) -> () {}

    fn get_size(&self, available_space: &Size) -> Size {
        return available_space.clone();
    }

    fn get_width(&self, available_space: &Size) -> f64 {
        return available_space.width;
    }

    fn get_height(&self, available_space: &Size) -> f64 {
        return available_space.height;
    }

    fn get_children_mut(&mut self) -> Vec<&mut dyn Widget> {
        return vec![];
    }
    fn get_children(&self) -> Vec<&dyn Widget> {
        return vec![];
    }
}
