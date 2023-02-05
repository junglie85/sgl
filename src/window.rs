use sgl_math::v2;
use winit::{
    dpi::{LogicalSize, PhysicalSize},
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::EventLoop,
    platform::run_return::EventLoopExtRunReturn,
    window::WindowBuilder,
};

use crate::{Key, SglError, View};

pub struct Window {
    pub(crate) pixel_size: PhysicalSize<u32>,
    closed: bool,
    event_loop: EventLoop<()>,
    pub(crate) native_window: winit::window::Window,
    pub(crate) view: View,
    key_pressed: [bool; 1],
}

impl Window {
    pub fn new<S>(
        width: u32,
        height: u32,
        title: S,
        pixel_width: u32,
        pixel_height: u32,
    ) -> Result<Self, SglError>
    where
        S: Into<String>,
    {
        let event_loop = EventLoop::new();

        let pixel_size = PhysicalSize::new(pixel_width, pixel_height);
        let logical_size = LogicalSize::new(width * pixel_size.width, height * pixel_size.height);
        let window_builder = WindowBuilder::new()
            .with_title(title)
            .with_resizable(false)
            .with_inner_size(logical_size)
            .with_visible(true);
        let native_window = window_builder
            .build(&event_loop)
            .map_err(|e| SglError::General(e.to_string()))?;
        let view = View::new(
            v2(
                logical_size.width as f32 / 2.0,
                logical_size.height as f32 / 2.0,
            ),
            logical_size.width as f32,
            logical_size.height as f32,
        );

        Ok(Self {
            pixel_size,
            closed: false,
            event_loop,
            native_window,
            view,
            key_pressed: [false; 1],
        })
    }

    pub fn closed(&self) -> bool {
        self.closed
    }

    pub fn view(&self) -> View {
        self.view
    }

    pub fn key_down(&self, key: Key) -> bool {
        self.key_pressed[key as usize]
    }

    pub fn update(&mut self) {
        self.event_loop.run_return(|event, _, control_flow| {
            control_flow.set_poll();
            match event {
                Event::MainEventsCleared => {
                    control_flow.set_exit();
                }
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    self.closed = true;
                }
                Event::WindowEvent { window_id, event } if self.native_window.id() == window_id => {
                    match event {
                        WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    virtual_keycode: Some(virtual_code),
                                    state: ElementState::Pressed,
                                    ..
                                },
                            ..
                        } => match virtual_code {
                            VirtualKeyCode::Escape => {
                                self.key_pressed[Key::Escape as usize] = true;
                            }
                            _ => (),
                        },
                        _ => (),
                    }
                }

                _ => (),
            }
        });
    }
}
