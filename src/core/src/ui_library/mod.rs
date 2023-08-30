pub mod compound;
pub mod container;
pub mod expanded;
pub mod list;
pub mod padding;
pub mod sized_box;

use derivative::Derivative;
use key_segment::KeySegment;

use std::{cell::RefCell, fmt::Debug, rc::Weak};

use crate::{
    event_handlers::HandledEventInfo,
    graphics::{Position, Size},
    platform::Platform,
    state::{State, StateManager},
};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Key {
    pub value: String,
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct WidgetData {
    pub key: Option<Key>,
    #[derivative(Debug="ignore")]
    pub state_manager: Weak<RefCell<StateManager>>,
    pub position: Position,
    pub available_space: Size,
}

impl WidgetData {
    pub fn new() -> Self {
        return Self {
            key: None,
            state_manager: Weak::new(),
            position: Position::origin(),
            available_space: Size::zero(),
        };
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct CompoundWidgetData {
    pub key: Option<Key>,
    #[derivative(Debug="ignore")]
    pub state_manager: Weak<RefCell<StateManager>>,
    pub cached_build: Option<Box<dyn Widget>>,
    pub position: Position,
    pub available_space: Size,
}

impl CompoundWidgetData {
    pub fn new() -> Self {
        return Self {
            key: None,
            state_manager: Weak::new(),
            cached_build: None,
            position: Position::origin(),
            available_space: Size::zero(),
        };
    }
}

pub trait Widget: Debug + KeySegment {
    fn get_key(&self) -> &Key;
    fn set_key(&mut self, key: Key) -> ();
    fn get_state_manager(&self) -> Weak<RefCell<StateManager>>;
    fn set_state_manager(&mut self, state_manager: Weak<RefCell<StateManager>>) -> ();

    fn get_position(&self) -> &Position;
    fn set_position(&mut self, position: Position) -> ();
    fn get_available_space(&self) -> &Size;
    fn set_available_space(&mut self, available_space: Size) -> ();

    fn get_width(&self, available_space: &Size) -> f64;
    fn get_height(&self, available_space: &Size) -> f64;
    fn get_size(&self, available_space: &Size) -> Size {
        return Size {
            width: self.get_width(available_space),
            height: self.get_height(available_space),
        };
    }
    fn does_expand(&self) -> bool {
        for child in self.get_children() {
            if child.does_expand() {
                return true;
            }
        }
        return false;
    }

    fn get_children(&self) -> Vec<&dyn Widget>;
    fn get_children_mut(&mut self) -> Vec<&mut dyn Widget>;

    fn draw(&self, parent_position: Position, platform: &dyn Platform) -> ();
    fn set_layout(&mut self, position: Position, available_space: Size);

    fn get_cached_build(&self) -> Option<&dyn Widget> {
        return None;
    }

    fn get_cached_build_mut(&mut self) -> Option<&mut dyn Widget> {
        return None;
    }

    fn set_cached_build(&mut self, _cached_build: Option<Box<dyn Widget>>) -> () {}

    fn build(&mut self) -> Option<Box<dyn Widget>> {
        return None;
    }

    fn rebuild(&mut self) -> () {
        let build: Option<Box<dyn Widget>> = self.build();
        self.set_cached_build(build);
        let my_key = self.get_key().clone();
        let state_manager = self.get_state_manager();
        for (i, child) in self.get_children_mut().iter_mut().enumerate() {
            let mut new_parent_key_value = my_key.value.clone();
            new_parent_key_value.push_str(&format!("/{}", i));
            child.rebuild_with_key(new_parent_key_value, state_manager.clone());
        }
    }

    fn rebuild_with_key(
        &mut self,
        parent_key_value: String,
        state_manager: Weak<RefCell<StateManager>>,
    ) -> () {
        self.set_state_manager(state_manager.clone());
        let mut my_key = parent_key_value.clone();
        my_key.push_str(&format!("/{}", self.key_segment()));
        let my_key = Key { value: my_key };
        self.set_key(my_key.clone());
        let build: Option<Box<dyn Widget>> = self.build();
        self.set_cached_build(build);
        for (i, child) in self.get_children_mut().iter_mut().enumerate() {
            let mut new_parent_key_value = my_key.value.clone();
            new_parent_key_value.push_str(&format!("/{}", i));
            child.rebuild_with_key(new_parent_key_value, state_manager.clone());
        }
    }

    fn on_mouse_move(
        &mut self,
        _mouse_pos: &Position,
        _platform: &dyn Platform,
        _handled_event_info: &mut HandledEventInfo,
    ) -> bool {
        return false;
    }

    fn on_mouse_move_anywhere(
        &mut self,
        parent_position: Position,
        mouse_pos: &Position,
        platform: &dyn Platform,
        handled_event_info: &mut HandledEventInfo,
    ) -> bool {
        let my_relative_pos = self.get_position().clone();
        for child in self.get_children_mut() {
            let captured = child.on_mouse_move_anywhere(
                parent_position.clone() + my_relative_pos.clone(),
                mouse_pos,
                platform,
                handled_event_info,
            );
            if captured {
                return true;
            }
        }
        let my_pos = my_relative_pos + parent_position;
        let my_size = self.get_size(self.get_available_space());
        if my_pos.x <= mouse_pos.x
            && mouse_pos.x <= my_pos.x + my_size.width
            && my_pos.y <= mouse_pos.y
            && mouse_pos.y <= my_pos.y + my_size.height
        {
            return self.on_mouse_move(mouse_pos, platform, handled_event_info);
        }
        return false;
    }
}

pub trait CompoundWidget: Widget {
    fn get_key(&self) -> &Key;
    fn set_key(&mut self, key: Key) -> ();
    fn get_state_manager(&self) -> Weak<RefCell<StateManager>>;
    fn set_state_manager(&mut self, state_manager: Weak<RefCell<StateManager>>) -> ();

    fn get_position(&self) -> &Position;
    fn set_position(&mut self, position: Position) -> ();
    fn get_available_space(&self) -> &Size;
    fn set_available_space(&mut self, available_space: Size) -> ();

    fn get_cached_build(&self) -> Option<&dyn Widget>;
    fn get_cached_build_mut(&mut self) -> Option<&mut dyn Widget>;
    fn set_cached_build(&mut self, cached_build: Box<dyn Widget>) -> ();

    fn build(&self) -> Box<dyn Widget>;

    fn on_mouse_move(
        &mut self,
        _mouse_pos: &Position,
        _platform: &dyn Platform,
        _handled_event_info: &mut HandledEventInfo,
    ) -> bool {
        return false;
    }
}

impl<T: CompoundWidget> Widget for T {
    fn get_key(&self) -> &Key {
        return CompoundWidget::get_key(self);
    }
    fn set_key(&mut self, key: Key) -> () {
        return CompoundWidget::set_key(self, key);
    }
    fn get_state_manager(&self) -> Weak<RefCell<StateManager>> {
        return CompoundWidget::get_state_manager(self);
    }
    fn set_state_manager(&mut self, state_manager: Weak<RefCell<StateManager>>) -> () {
        return CompoundWidget::set_state_manager(self, state_manager);
    }

    fn get_position(&self) -> &Position {
        return CompoundWidget::get_position(self);
    }

    fn set_position(&mut self, position: Position) -> () {
        return CompoundWidget::set_position(self, position);
    }

    fn get_available_space(&self) -> &Size {
        return CompoundWidget::get_available_space(self);
    }

    fn set_available_space(&mut self, available_space: Size) -> () {
        return CompoundWidget::set_available_space(self, available_space);
    }

    fn get_cached_build(&self) -> Option<&dyn Widget> {
        return CompoundWidget::get_cached_build(self);
    }

    fn get_cached_build_mut(&mut self) -> Option<&mut dyn Widget> {
        return CompoundWidget::get_cached_build_mut(self);
    }

    fn set_cached_build(&mut self, cached_build: Option<Box<dyn Widget>>) -> () {
        match cached_build {
            None => (),
            Some(x) => CompoundWidget::set_cached_build(self, x),
        };
    }

    fn build(&mut self) -> Option<Box<dyn Widget>> {
        return Some(CompoundWidget::build(self));
    }

    fn get_children_mut(&mut self) -> Vec<&mut dyn Widget> {
        return vec![Widget::get_cached_build_mut(self).unwrap()];
    }
    fn get_children(&self) -> Vec<&dyn Widget> {
        return vec![Widget::get_cached_build(self).unwrap()];
    }

    fn set_layout(&mut self, position: Position, available_space: Size) {
        Widget::set_position(self, position);
        Widget::set_available_space(self, available_space.clone());
        Widget::get_cached_build_mut(self)
            .unwrap()
            .set_layout(Position::origin(), available_space);
    }

    fn draw(&self, parent_position: Position, platform: &dyn Platform) -> () {
        Widget::get_cached_build(self).unwrap().draw(
            parent_position + Widget::get_position(self).clone(),
            platform,
        );
    }

    fn get_height(&self, available_space: &Size) -> f64 {
        return Widget::get_cached_build(self)
            .unwrap()
            .get_height(available_space);
    }

    fn get_width(&self, available_space: &Size) -> f64 {
        return Widget::get_cached_build(self)
            .unwrap()
            .get_width(available_space);
    }

    fn get_size(&self, available_space: &Size) -> Size {
        return Widget::get_cached_build(self)
            .unwrap()
            .get_size(available_space);
    }

    fn on_mouse_move(
        &mut self,
        mouse_pos: &Position,
        platform: &dyn Platform,
        handled_event_info: &mut HandledEventInfo,
    ) -> bool {
        return CompoundWidget::on_mouse_move(self, mouse_pos, platform, handled_event_info);
    }
}

#[derive(Debug, Clone)]
pub struct StatefulWidgetEventHandlerInfo {
    pub was_captured: bool,
    pub needs_rebuild: bool,
}

impl StatefulWidgetEventHandlerInfo {
    pub fn new() -> Self {
        return Self {
            was_captured: false,
            needs_rebuild: false,
        };
    }
}

pub trait StatefulWidget: CompoundWidget {
    type T: State;

    fn get_key(&self) -> &Key;
    fn set_key(&mut self, key: Key) -> ();
    fn get_state_manager(&self) -> Weak<RefCell<StateManager>>;
    fn set_state_manager(&mut self, state_manager: Weak<RefCell<StateManager>>) -> ();

    fn get_position(&self) -> &Position;
    fn set_position(&mut self, position: Position) -> ();
    fn get_available_space(&self) -> &Size;
    fn set_available_space(&mut self, available_space: Size) -> ();

    fn get_cached_build(&self) -> Option<&dyn Widget>;
    fn get_cached_build_mut(&mut self) -> Option<&mut dyn Widget>;
    fn set_cached_build(&mut self, cached_build: Box<dyn Widget>) -> ();

    fn build(&self, state: &mut Self::T) -> Box<dyn Widget>;

    fn state_factory(&self) -> Box<Self::T>;

    fn on_mouse_move(
        &mut self,
        _state: &mut Self::T,
        _mouse_pos: &Position,
        _platform: &dyn Platform,
        _handled_event_info: &mut HandledEventInfo,
    ) -> StatefulWidgetEventHandlerInfo {
        return StatefulWidgetEventHandlerInfo::new();
    }
}

impl<U: StatefulWidget> CompoundWidget for U {
    fn get_key(&self) -> &Key {
        return StatefulWidget::get_key(self);
    }
    fn set_key(&mut self, key: Key) -> () {
        return StatefulWidget::set_key(self, key);
    }
    fn get_state_manager(&self) -> Weak<RefCell<StateManager>> {
        return StatefulWidget::get_state_manager(self);
    }
    fn set_state_manager(&mut self, state_manager: Weak<RefCell<StateManager>>) -> () {
        return StatefulWidget::set_state_manager(self, state_manager);
    }

    fn get_position(&self) -> &Position {
        return StatefulWidget::get_position(self);
    }

    fn set_position(&mut self, position: Position) -> () {
        return StatefulWidget::set_position(self, position);
    }

    fn get_available_space(&self) -> &Size {
        return StatefulWidget::get_available_space(self);
    }

    fn set_available_space(&mut self, available_space: Size) -> () {
        return StatefulWidget::set_available_space(self, available_space);
    }

    fn get_cached_build(&self) -> Option<&dyn Widget> {
        return StatefulWidget::get_cached_build(self);
    }

    fn get_cached_build_mut(&mut self) -> Option<&mut dyn Widget> {
        return StatefulWidget::get_cached_build_mut(self);
    }

    fn set_cached_build(&mut self, cached_build: Box<dyn Widget>) -> () {
        StatefulWidget::set_cached_build(self, cached_build);
    }

    fn build(&self) -> Box<dyn Widget> {
        let state_manager_rc = Weak::upgrade(&CompoundWidget::get_state_manager(self)).unwrap();
        let mut state_manager = (*state_manager_rc).borrow_mut();
        let state =
            state_manager.get_state(CompoundWidget::get_key(self), &|| self.state_factory());
        return StatefulWidget::build(self, state);
    }

    fn on_mouse_move(
        &mut self,
        mouse_pos: &Position,
        platform: &dyn Platform,
        handled_event_info: &mut HandledEventInfo,
    ) -> bool {
        let handler_info: StatefulWidgetEventHandlerInfo;
        {
            let state_manager_rc = Weak::upgrade(&CompoundWidget::get_state_manager(self)).unwrap();
            let mut state_manager = (*state_manager_rc).borrow_mut();
            let state =
                state_manager.get_state(CompoundWidget::get_key(self), &|| self.state_factory());

            handler_info =
                StatefulWidget::on_mouse_move(self, state, mouse_pos, platform, handled_event_info)
        }
        if handler_info.needs_rebuild {
            self.rebuild();
        }

        return handler_info.was_captured;
    }
}

#[macro_export]
macro_rules! widget_default_methods {
    () => {
        fn get_key(&self) -> &Key {
            return match &self.widget_data.key {
                Some(x) => x,
                None => panic!(),
            };
        }
    
        fn set_key(&mut self, key: Key) -> () {
            self.widget_data.key = Some(key);
        }
    
        fn get_state_manager(&self) -> Weak<RefCell<StateManager>> {
            return self.widget_data.state_manager.clone();
        }
    
        fn set_state_manager(&mut self, state_manager: Weak<RefCell<StateManager>>) -> () {
            self.widget_data.state_manager = state_manager;
        }
        
        fn get_position(&self) -> &Position {
            return &self.widget_data.position;
        }
    
        fn set_position(&mut self, position: Position) -> () {
            self.widget_data.position = position;
        }
    
        fn get_available_space(&self) -> &Size {
            return &self.widget_data.available_space;
        }
    
        fn set_available_space(&mut self, available_space: Size) -> () {
            self.widget_data.available_space = available_space;
        }
    
    }
}

#[macro_export]
macro_rules! compound_widget_default_methods {
    () => {
        
        widget_default_methods!();

        fn get_cached_build_mut(&mut self) -> Option<&mut dyn Widget> {
            return match &mut self.widget_data.cached_build {
                Some(x) => Some(x.as_mut()),
                None => None,
            };
        }
    
        fn get_cached_build(&self) -> Option<&dyn Widget> {
            return match &self.widget_data.cached_build {
                Some(x) => Some(x.as_ref()),
                None => None,
            };
        }
    
        fn set_cached_build(&mut self, cached_build: Box<dyn Widget>) -> () {
            self.widget_data.cached_build = Some(cached_build);
        }
        
    }
}

#[macro_export]
macro_rules! widget_default_fields {
    () => {
        key: Option<Key>,
        #[derivative(Debug="ignore")]
        state_manager: Weak<RefCell<StateManager>>,
        position: Position,
        available_space: Size,    
    };
}

#[macro_export]
macro_rules! compound_widget_default_fields {
    () => {
        widget_default_fields!();
        cached_build: Option<Box<dyn Widget>>,
    };
}