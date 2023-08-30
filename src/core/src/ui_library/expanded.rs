use std::{rc::Weak, cell::RefCell};

use crate::{graphics::{Position, Size}, state::StateManager};

use super::{Widget, Key, KeySegment};

use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Expanded {
    key: Option<Key>,
    #[derivative(Debug = "ignore")]
    state_manager: Weak<RefCell<StateManager>>,
    position: Position,
    available_space: Size,
}

impl Expanded {
    pub fn new() -> Box<Self> {
        return Box::new(Self {
            key: None,
            state_manager: Weak::new(),
            position: Position::origin(),
            available_space: Size::zero(),
        });
    }
}

impl KeySegment for Expanded {
    fn key_segment(&self) -> String {
        return "Expanded".to_string();
    }
}

impl Widget for Expanded {
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
        return true;
    }

    fn set_layout(&mut self, position: Position, available_space: Size) {
        self.position = position;
        self.available_space = available_space;
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
