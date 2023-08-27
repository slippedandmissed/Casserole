use casserole_core::event_handlers;
use casserole_core::graphics::{Color as CasseroleColor, GraphicsLibrary, Position, Size};
use crossbeam_channel::{unbounded, Receiver, Sender};
use once_cell::sync::Lazy;
use sdl2::event::{Event, WindowEvent};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::thread;
use std::time::Duration;

use crate::PLATFORM;

#[derive(Clone, Debug)]
pub enum DrawCommand {
    SetDrawColor { color: CasseroleColor },

    FillRect { position: Position, size: Size },

    UpdateDisplay,
}

pub struct SDL2GraphicsLibrary {
    pub get_screen_dims_sender: Sender<()>,
    pub get_screen_dims_receiver: Receiver<()>,
    pub screen_dims_sender: Sender<Size>,
    pub screen_dims_receiver: Receiver<Size>,

    pub draw_sender: Sender<DrawCommand>,
    pub draw_receiver: Receiver<DrawCommand>,
}

impl SDL2GraphicsLibrary {
    pub fn new() -> Self {
        let (draw_sender, draw_receiver) = unbounded();
        let (get_screen_dims_sender, get_screen_dims_receiver) = unbounded();
        let (screen_dims_sender, screen_dims_receiver) = unbounded();
        return Self {
            draw_sender,
            draw_receiver,
            get_screen_dims_sender,
            get_screen_dims_receiver,
            screen_dims_sender,
            screen_dims_receiver,
        };
    }

    pub fn init(&self) {
        let draw_receiver = self.draw_receiver.clone();
        let get_screen_dims_receiver = self.get_screen_dims_receiver.clone();
        let screen_dims_sender = self.screen_dims_sender.clone();
        thread::spawn(move || {
            let sdl_context = sdl2::init().unwrap();
            let video_subsystem = sdl_context.video().unwrap();

            let window = video_subsystem
                .window("Casserole", 800, 600)
                .opengl()
                .position_centered()
                .resizable()
                .build()
                .unwrap();
            let mut canvas = window.into_canvas().present_vsync().build().unwrap();
            let mut event_pump = sdl_context.event_pump().unwrap();

            let texture_creator = canvas.texture_creator();

            let size = canvas.output_size().unwrap();
            let mut texture = texture_creator
                .create_texture_target(texture_creator.default_pixel_format(), size.0, size.1)
                .unwrap();

            'running: loop {
                for event in event_pump.poll_iter() {
                    match event {
                        Event::Quit { .. } => {
                            event_handlers::on_quit();
                            break 'running;
                        }
                        Event::Window {
                            timestamp: _timestamp,
                            window_id: _window_id,
                            win_event,
                        } => match win_event {
                            WindowEvent::Resized(new_width, new_height) => {
                                texture = texture_creator
                                    .create_texture_target(
                                        texture_creator.default_pixel_format(),
                                        u32::try_from(new_width).unwrap(),
                                        u32::try_from(new_height).unwrap(),
                                    )
                                    .unwrap();
                                thread::spawn(|| {
                                    event_handlers::on_window_resize(&PLATFORM);
                                });
                            }
                            _ => (),
                        },
                        _ => (),
                    }
                }

                for cmd in draw_receiver.try_iter() {
                    match cmd {
                        DrawCommand::SetDrawColor { color } => {
                            canvas
                                .with_texture_canvas(&mut texture, |texture_canvas| {
                                    texture_canvas
                                        .set_draw_color(Color::RGB(color.r, color.g, color.b));
                                })
                                .unwrap();
                        }
                        DrawCommand::FillRect { position, size } => {
                            canvas
                                .with_texture_canvas(&mut texture, |texture_canvas| {
                                    texture_canvas
                                        .fill_rect(Rect::new(
                                            position.x,
                                            position.y,
                                            size.width,
                                            size.height,
                                        ))
                                        .unwrap();
                                })
                                .unwrap();
                        }
                        DrawCommand::UpdateDisplay => {
                            canvas.copy(&texture, None, None).unwrap();
                        }
                    }
                }

                for _ in get_screen_dims_receiver.try_iter() {
                    let size: (u32, u32) = canvas.output_size().unwrap();
                    screen_dims_sender
                        .send(Size {
                            width: size.0,
                            height: size.1,
                        })
                        .unwrap();
                }

                canvas.present();

                ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
            }
        });
    }
}

impl GraphicsLibrary for SDL2GraphicsLibrary {
    fn fill(&self, color: CasseroleColor) {
        self.fill_rect(Position { x: 0, y: 0 }, self.get_screen_dimensions(), color);
    }

    fn fill_rect(&self, position: Position, size: Size, color: CasseroleColor) {
        self.draw_sender
            .send(DrawCommand::SetDrawColor { color })
            .unwrap();
        self.draw_sender
            .send(DrawCommand::FillRect { position, size })
            .unwrap();
    }

    fn get_screen_dimensions(&self) -> Size {
        self.get_screen_dims_sender.send(()).unwrap();
        return self.screen_dims_receiver.recv().unwrap();
    }

    fn update(&self) {
        self.draw_sender.send(DrawCommand::UpdateDisplay).unwrap();
    }
}

pub static GRAPHICS_LIBRARY: Lazy<SDL2GraphicsLibrary> = Lazy::new(|| SDL2GraphicsLibrary::new());
