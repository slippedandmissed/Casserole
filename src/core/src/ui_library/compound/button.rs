use std::{rc::Weak, cell::RefCell};

use crate::{
    graphics::{Color, Position, Size},
    ui_library::{
        container::Container, expanded::Expanded, sized_box::SizedBox, CompoundWidget, Widget, Key, KeySegment, CompoundWidgetData,
    }, state::StateManager, widget_default_methods, compound_widget_default_methods,
};

use super::hoverable::Hoverable;


#[derive(Debug)]
pub struct Button {
    widget_data: CompoundWidgetData,

    background_color: Color,
    hovered_background_color: Color,
}

impl Button {
    pub fn new() -> Box<Self> {
        return Box::new(Self {
            widget_data: CompoundWidgetData::new(),
            background_color: Color { r: 0, g: 0, b: 255 },
            hovered_background_color: Color {
                r: 255,
                g: 255,
                b: 0,
            },
        });
    }
}

impl KeySegment for Button {
    fn key_segment(&self) -> String {
        return "Button".to_string();
    }
}

impl CompoundWidget for Button {
    
    compound_widget_default_methods!();

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
