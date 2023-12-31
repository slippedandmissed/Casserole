use std::{cell::RefCell, rc::Weak};

use crate::{
    graphics::{Position, Size},
    platform::Platform,
    state::StateManager,
    widget_default_methods,
};

use super::{Key, Widget, WidgetData};

use key_segment::KeySegment;
use key_segment_derive::KeySegment;

#[derive(Clone, Debug)]
pub struct Inset {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
}

impl Inset {
    pub fn every(size: f64) -> Self {
        return Self {
            left: size,
            top: size,
            right: size,
            bottom: size,
        };
    }

    pub fn symmetric(vertical: f64, horizontal: f64) -> Self {
        return Self {
            left: horizontal,
            top: vertical,
            right: horizontal,
            bottom: vertical,
        };
    }
}

#[derive(Debug, KeySegment)]
pub struct Padding {
    widget_data: WidgetData,
    padding: Inset,
    child: Option<Box<dyn Widget>>,
}

impl Padding {
    pub fn new(padding: Inset, child: Option<Box<dyn Widget>>) -> Box<Self> {
        return Box::new(Self {
            widget_data: WidgetData::new(),
            padding,
            child,
        });
    }

    fn get_available_space_for_child(&self, available_space: &Size) -> Size {
        return Size {
            width: available_space.width - self.padding.left - self.padding.right,
            height: available_space.height - self.padding.top - self.padding.bottom,
        };
    }
}

impl Widget for Padding {
    widget_default_methods!();

    fn set_layout(&mut self, position: Position, available_space: Size) {
        self.widget_data.position = position;
        self.widget_data.available_space = available_space;
        let available_space_for_child =
            self.get_available_space_for_child(&self.widget_data.available_space);
        match &mut self.child {
            Some(child) => child.set_layout(
                Position {
                    x: self.padding.left,
                    y: self.padding.top,
                },
                available_space_for_child,
            ),
            None => (),
        };
    }

    fn draw(&self, parent_position: Position, platform: &dyn Platform) -> () {
        match &self.child {
            Some(child) => child.draw(
                parent_position + self.widget_data.position.clone(),
                platform,
            ),
            None => (),
        };
    }

    fn get_width(&self, available_space: &Size) -> f64 {
        let child_width = match &self.child {
            Some(child) => child.get_width(&self.get_available_space_for_child(available_space)),
            None => 0.,
        };
        return child_width + self.padding.left + self.padding.right;
    }

    fn get_height(&self, available_space: &Size) -> f64 {
        let child_height = match &self.child {
            Some(child) => child.get_height(&self.get_available_space_for_child(available_space)),
            None => 0.,
        };
        return child_height + self.padding.top + self.padding.bottom;
    }

    fn get_children_mut(&mut self) -> Vec<&mut dyn Widget> {
        return match &mut self.child {
            Some(x) => vec![x.as_mut()],
            None => vec![],
        };
    }
    fn get_children(&self) -> Vec<&dyn Widget> {
        return match &self.child {
            Some(x) => vec![x.as_ref()],
            None => vec![],
        };
    }
}
