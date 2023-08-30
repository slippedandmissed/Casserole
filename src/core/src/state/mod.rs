use std::{any::Any, collections::HashMap};

use crate::ui_library::Key;

pub struct StateManager {
    states: HashMap<Key, Box<dyn Any>>,
}

impl StateManager {
    pub fn new() -> Self {
        return Self {
            states: HashMap::new(),
        };
    }

    pub fn get_state<T: State>(&mut self, key: &Key, factory: &dyn Fn() -> Box<T>) -> &mut T {
        let x = self.states.entry(key.clone()).or_insert_with(|| factory());
        return x.as_mut().downcast_mut::<T>().unwrap();
    }
}

pub trait State: Any {}
