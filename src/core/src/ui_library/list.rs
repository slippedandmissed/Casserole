use std::{cell::RefCell, rc::Weak};

use crate::{
    graphics::{Position, Size},
    platform::Platform,
    state::StateManager,
    widget_default_methods,
};

use super::{Key, Widget, WidgetData};

use key_segment::KeySegment;
use key_segment_derive::KeySegment;

#[derive(Debug)]
pub enum MainAxisAlignment {
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceAround,
}

#[derive(Debug)]
pub enum CrossAxisAlignment {
    Start,
    Center,
    End,
}

#[derive(Debug)]
pub enum MainAxisSize {
    Min,
    Max,
}

#[derive(Debug)]
pub enum CrossAxisSize {
    Min,
    Max,
}

#[derive(Debug)]
pub enum ListDirection {
    Column,
    Row,
}

#[derive(Debug, KeySegment)]
pub struct List {
    widget_data: WidgetData,
    direction: ListDirection,
    main_axis_alignment: MainAxisAlignment,
    cross_axis_alignment: CrossAxisAlignment,
    main_axis_size: MainAxisSize,
    cross_axis_size: CrossAxisSize,
    children: Vec<Box<dyn Widget>>,
}

impl List {
    pub fn new(
        direction: ListDirection,
        main_axis_alignment: MainAxisAlignment,
        cross_axis_alignment: CrossAxisAlignment,
        main_axis_size: MainAxisSize,
        cross_axis_size: CrossAxisSize,
        children: Vec<Box<dyn Widget>>,
    ) -> Box<Self> {
        return Box::new(Self {
            widget_data: WidgetData::new(),
            direction,
            main_axis_alignment,
            cross_axis_alignment,
            main_axis_size,
            cross_axis_size,
            children,
        });
    }
}

impl Widget for List {
    widget_default_methods!();

    fn set_layout(&mut self, position: Position, available_space: Size) {
        self.widget_data.position = position;
        self.widget_data.available_space = available_space;

        let mut child_sizes: Vec<Size> = self
            .children
            .iter()
            .map(|child| child.get_size(&self.widget_data.available_space))
            .collect();

        let mut total_child_size = child_sizes
            .iter()
            .map(|x| x.clone())
            .reduce(match self.direction {
                ListDirection::Column => |a: Size, s: Size| Size {
                    width: if a.width >= s.width { a.width } else { s.width },
                    height: a.height + s.height,
                },
                ListDirection::Row => |a: Size, s: Size| Size {
                    width: a.width + s.width,
                    height: if a.height >= s.height {
                        a.height
                    } else {
                        s.height
                    },
                },
            })
            .unwrap();

        for (i, child) in self.children.iter().enumerate() {
            // We assume that there is at most one expanding child
            if child.does_expand() {
                match &self.direction {
                    ListDirection::Column => {
                        child_sizes[i].height = self.widget_data.available_space.height
                            - total_child_size.height
                            + child_sizes[i].height;
                        total_child_size.height = self.widget_data.available_space.height;
                    }
                    ListDirection::Row => {
                        child_sizes[i].width = self.widget_data.available_space.width
                            - total_child_size.width
                            + child_sizes[i].width;
                        total_child_size.width = self.widget_data.available_space.width;
                    }
                }
                break;
            }
        }

        let total_child_main_axis_size: f64;
        let total_child_cross_axis_size: f64;
        let main_axis_available_space: f64;
        let cross_axis_available_space: f64;

        match self.direction {
            ListDirection::Column => {
                total_child_main_axis_size = total_child_size.height;
                total_child_cross_axis_size = total_child_size.width;
                main_axis_available_space = self.widget_data.available_space.height;
                cross_axis_available_space = self.widget_data.available_space.width;
            }
            ListDirection::Row => {
                total_child_main_axis_size = total_child_size.width;
                total_child_cross_axis_size = total_child_size.height;
                main_axis_available_space = self.widget_data.available_space.width;
                cross_axis_available_space = self.widget_data.available_space.height;
            }
        };
        let main_axis_real_size = match self.main_axis_size {
            MainAxisSize::Max => main_axis_available_space,
            MainAxisSize::Min => total_child_main_axis_size,
        };

        let cross_axis_real_size = match self.cross_axis_size {
            CrossAxisSize::Max => cross_axis_available_space,
            CrossAxisSize::Min => total_child_cross_axis_size,
        };

        let main_axis_extra_space: f64 = main_axis_real_size - total_child_main_axis_size;

        let mut used_main_axis_space = 0.;
        let child_positions: Vec<(Position, Size)> = (0..self.children.len())
            .map(|i| {
                let child_size = &child_sizes[i];
                let child_main_axis_size: f64;
                let child_cross_axis_size: f64;

                match self.direction {
                    ListDirection::Column => {
                        child_main_axis_size = child_size.height;
                        child_cross_axis_size = child_size.width;
                    }
                    ListDirection::Row => {
                        child_main_axis_size = child_size.width;
                        child_cross_axis_size = child_size.height;
                    }
                }

                let cross_axis_pos = match self.cross_axis_alignment {
                    CrossAxisAlignment::Start => 0.,
                    CrossAxisAlignment::Center => {
                        cross_axis_real_size / 2. - child_cross_axis_size / 2.
                    }
                    CrossAxisAlignment::End => cross_axis_real_size - child_cross_axis_size,
                };

                let mut used_main_axis_extra_space = 0.;
                let main_axis_pos = match self.main_axis_alignment {
                    MainAxisAlignment::Start => used_main_axis_space,
                    MainAxisAlignment::Center => {
                        main_axis_real_size / 2. - total_child_main_axis_size / 2.
                            + used_main_axis_space
                    }
                    MainAxisAlignment::End => {
                        main_axis_real_size - total_child_main_axis_size + used_main_axis_space
                    }
                    MainAxisAlignment::SpaceBetween => {
                        used_main_axis_extra_space = if i == 0 {
                            0.
                        } else {
                            main_axis_extra_space / (self.children.len() - 1) as f64
                        };
                        used_main_axis_space + used_main_axis_extra_space
                    }
                    MainAxisAlignment::SpaceAround => {
                        used_main_axis_extra_space =
                            main_axis_extra_space / (self.children.len() + 1) as f64;
                        used_main_axis_space + used_main_axis_extra_space
                    }
                };

                let x: f64;
                let y: f64;

                let real_child_size: Size;

                match self.direction {
                    ListDirection::Column => {
                        x = cross_axis_pos;
                        y = main_axis_pos;
                        real_child_size = Size {
                            width: child_cross_axis_size,
                            height: child_main_axis_size,
                        };
                    }
                    ListDirection::Row => {
                        x = main_axis_pos;
                        y = cross_axis_pos;
                        real_child_size = Size {
                            width: child_main_axis_size,
                            height: child_cross_axis_size,
                        };
                    }
                }

                used_main_axis_space += child_main_axis_size + used_main_axis_extra_space;
                return (Position { x, y }, real_child_size);
            })
            .collect();

        for (i, child) in self.children.iter_mut().enumerate() {
            let (position, real_child_size) = &child_positions[i];

            child.set_layout(position.clone(), real_child_size.clone());
        }
    }

    fn draw(&self, parent_position: Position, platform: &dyn Platform) -> () {
        for child in &self.children {
            child.draw(
                parent_position.clone() + self.widget_data.position.clone(),
                platform,
            );
        }
    }

    fn get_size(&self, available_space: &Size) -> Size {
        let child_sizes = self
            .children
            .iter()
            .map(|child| child.get_size(available_space));

        let total_child_size = child_sizes
            .clone()
            .reduce(match self.direction {
                ListDirection::Column => |a: Size, s: Size| Size {
                    width: if a.width >= s.width { a.width } else { s.width },
                    height: a.height + s.height,
                },
                ListDirection::Row => |a: Size, s: Size| Size {
                    width: a.width + s.width,
                    height: if a.height >= s.height {
                        a.height
                    } else {
                        s.height
                    },
                },
            })
            .unwrap();

        let total_child_main_axis_size: f64;
        let total_child_cross_axis_size: f64;
        let main_axis_available_space: f64;
        let cross_axis_available_space: f64;

        match self.direction {
            ListDirection::Column => {
                total_child_main_axis_size = total_child_size.height;
                total_child_cross_axis_size = total_child_size.width;
                main_axis_available_space = available_space.height;
                cross_axis_available_space = available_space.width;
            }
            ListDirection::Row => {
                total_child_main_axis_size = total_child_size.width;
                total_child_cross_axis_size = total_child_size.height;
                main_axis_available_space = available_space.width;
                cross_axis_available_space = available_space.height;
            }
        };
        let main_axis_real_size = match self.main_axis_size {
            MainAxisSize::Max => main_axis_available_space,
            MainAxisSize::Min => total_child_main_axis_size,
        };

        let cross_axis_real_size = match self.cross_axis_size {
            CrossAxisSize::Max => cross_axis_available_space,
            CrossAxisSize::Min => total_child_cross_axis_size,
        };

        return match self.direction {
            ListDirection::Column => Size {
                width: cross_axis_real_size,
                height: main_axis_real_size,
            },
            ListDirection::Row => Size {
                width: main_axis_real_size,
                height: cross_axis_real_size,
            },
        };
    }

    fn get_width(&self, available_space: &Size) -> f64 {
        return self.get_size(available_space).width;
    }

    fn get_height(&self, available_space: &Size) -> f64 {
        return self.get_size(available_space).height;
    }

    fn get_children_mut(&mut self) -> Vec<&mut dyn Widget> {
        let mut result: Vec<&mut dyn Widget> = vec![];
        for child in &mut self.children {
            result.push(child.as_mut());
        }
        return result;
    }
    fn get_children(&self) -> Vec<&dyn Widget> {
        let mut result: Vec<&dyn Widget> = vec![];
        for child in &self.children {
            result.push(child.as_ref());
        }
        return result;
    }
}
