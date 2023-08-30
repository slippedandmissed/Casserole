use crate::{
    graphics::{Color, Position, Size},
    ui_library::{
        container::Container, expanded::Expanded, sized_box::SizedBox, CompoundWidget, Widget,
    },
};

use super::hoverable::Hoverable;

#[derive(Debug)]
pub struct Button {
    cached_build: Option<Box<dyn Widget>>,
    position: Position,
    available_space: Size,
    background_color: Color,
    hovered_background_color: Color,
}

impl Button {
    pub fn new() -> Box<Self> {
        return Box::new(Self {
            cached_build: None,
            position: Position::origin(),
            available_space: Size::zero(),
            background_color: Color { r: 0, g: 0, b: 255 },
            hovered_background_color: Color {
                r: 255,
                g: 255,
                b: 0,
            },
        });
    }
}

impl CompoundWidget for Button {
    fn get_cached_build_mut(&mut self) -> Option<&mut dyn Widget> {
        return match &mut self.cached_build {
            Some(x) => Some(x.as_mut()),
            None => None,
        };
    }

    fn get_cached_build(&self) -> Option<&dyn Widget> {
        return match &self.cached_build {
            Some(x) => Some(x.as_ref()),
            None => None,
        };
    }

    fn set_cached_build(&mut self, cached_build: Box<dyn Widget>) -> () {
        self.cached_build = Some(cached_build);
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

    fn build(&self) -> Box<dyn Widget> {
        let background_color_clone = self.background_color.clone();
        let hovered_background_color_clone = self.hovered_background_color.clone();
        return Hoverable::new(Box::new(move |is_hovering| {
            SizedBox::new(
                Size {
                    width: 100.,
                    height: 40.,
                },
                Container::new(
                    if is_hovering {
                        hovered_background_color_clone.clone()
                    } else {
                        background_color_clone.clone()
                    },
                    Expanded::new(),
                ),
            )
        }));
    }
}
