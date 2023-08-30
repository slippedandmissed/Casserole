use crate::graphics::{Position, Size};

use super::Widget;

#[derive(Debug)]
pub struct SizedBox {
    pub position: Position,
    pub available_space: Size,
    pub size: Size,
    pub child: Box<dyn Widget>,
}

impl SizedBox {
    pub fn new(size: Size, child: Box<dyn Widget>) -> Box<Self> {
        return Box::new(Self {
            position: Position::origin(),
            available_space: Size::zero(),
            size,
            child,
        });
    }
}

impl Widget for SizedBox {
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

    fn does_expand(&self) -> bool {
        return false;
    }

    fn set_layout(&mut self, position: Position, available_space: Size) {
        self.position = position;
        self.available_space = available_space;
        self.child
            .set_layout(Position::origin(), self.size.clone());
    }

    fn draw(&self, parent_position: Position, platform: &dyn crate::platform::Platform) -> () {
        self.child
            .draw(parent_position + self.position.clone(), platform);
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
