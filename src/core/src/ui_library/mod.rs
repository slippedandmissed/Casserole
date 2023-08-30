pub mod compound;
pub mod container;
pub mod expanded;
pub mod list;
pub mod padding;
pub mod sized_box;

use std::fmt::Debug;

use crate::{
    event_handlers::HandledEventInfo,
    graphics::{Position, Size},
    platform::Platform,
};

pub trait Widget: Debug {
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
        let build = self.build();
        self.set_cached_build(build);
        for child in self.get_children_mut() {
            child.rebuild()
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
