use std::{rc::Weak, cell::RefCell};

use crate::{graphics::{Position, Size}, state::StateManager};

use super::{Widget, Key, KeySegment};

use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct SizedBox {
    key: Option<Key>,
    #[derivative(Debug = "ignore")]
    state_manager: Weak<RefCell<StateManager>>,
    position: Position,
    available_space: Size,

    size: Size,
    child: Box<dyn Widget>,
}

impl SizedBox {
    pub fn new(size: Size, child: Box<dyn Widget>) -> Box<Self> {
        return Box::new(Self {
            key: None,
            state_manager: Weak::new(),
            position: Position::origin(),
            available_space: Size::zero(),
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
    fn get_key(&self) -> &Key {
        return match &self.key {
            Some(x) => x,
            None => panic!()
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
