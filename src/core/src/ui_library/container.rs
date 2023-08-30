use crate::graphics::{Color, Position, Size};

use super::Widget;

#[derive(Debug)]
pub struct Container {
    pub position: Position,
    pub available_space: Size,
    pub background: Color,
    pub child: Box<dyn Widget>,
}

impl Container {
    pub fn new(background: Color, child: Box<dyn Widget>) -> Box<Self> {
        return Box::new(Self {
            position: Position::origin(),
            available_space: Size::zero(),
            background: background,
            child: child,
        });
    }
}

impl Widget for Container {
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
        self.child.set_layout(Position::origin(), self.available_space.clone());
    }

    fn draw(&self, parent_position: Position, platform: &dyn crate::platform::Platform) -> () {
        platform.graphics().fill_rect(
            &(parent_position.clone() + self.position.clone()),
            &self.child.get_size(&self.available_space),
            &self.background,
        );
        self.child.draw(
            parent_position + self.position.clone(),
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
