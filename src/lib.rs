use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::EventLoop,
    platform::run_return::EventLoopExtRunReturn,
    window::{Window, WindowBuilder},
};

pub use crate::error::MehError;

mod error;

pub struct Screen {
    closed: bool,
    event_loop: EventLoop<()>,
    window: Window,
    key_pressed: [bool; 1],
}

impl Screen {
    pub fn new<S>(width: u32, height: u32, title: S) -> Result<Self, MehError>
    where
        S: Into<String>,
    {
        let event_loop = EventLoop::new();

        let logical_size = LogicalSize::new(width, height);
        let window_builder = WindowBuilder::new()
            .with_title(title)
            .with_resizable(false)
            .with_inner_size(logical_size)
            .with_visible(true);
        let window = window_builder
            .build(&event_loop)
            .map_err(|e| MehError::General(e.to_string()))?;

        Ok(Self {
            closed: false,
            event_loop,
            window,
            key_pressed: [false; 1],
        })
    }

    pub fn closed(&self) -> bool {
        self.closed
    }

    pub fn key_down(&self, key: Vk) -> bool {
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
                Event::WindowEvent { window_id, event } if self.window.id() == window_id => {
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
                                self.key_pressed[Vk::Escape as usize] = true;
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

#[derive(Debug, Clone, Copy)]
pub enum Vk {
    Escape,
}
