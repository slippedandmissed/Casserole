use crate::{
    graphics::{Position, Size},
    platform::Platform,
};

use super::Widget;

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

#[derive(Debug)]
pub struct Padding {
    pub position: Position,
    pub available_space: Size,
    pub padding: Inset,
    pub child: Option<Box<dyn Widget>>,
}

impl Padding {
    pub fn new(padding: Inset, child: Option<Box<dyn Widget>>) -> Box<Self> {
        return Box::new(Self {
            position: Position::origin(),
            available_space: Size::zero(),
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

    fn set_layout(&mut self, position: Position, available_space: Size) {
        self.position = position;
        self.available_space = available_space;
        let available_space_for_child = self.get_available_space_for_child(&self.available_space);
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
            Some(child) => child.draw(parent_position + self.position.clone(), platform),
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
