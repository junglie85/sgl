use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::EventLoop,
    platform::run_return::EventLoopExtRunReturn,
    window::WindowBuilder,
};

use crate::{GraphicsDevice, Key, Renderer, Scene, SglError};

pub struct Window {
    closed: bool,
    event_loop: EventLoop<()>,
    pub(crate) native_window: winit::window::Window,
    gpu: GraphicsDevice,
    renderer: Renderer,
    key_pressed: [bool; 1],
}

impl Window {
    pub fn new<S>(width: u32, height: u32, title: S) -> Result<Self, SglError>
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
        let native_window = window_builder
            .build(&event_loop)
            .map_err(|e| SglError::General(e.to_string()))?;
        let gpu = GraphicsDevice::new(&native_window)?;
        let renderer = Renderer::new(&gpu, &native_window);

        Ok(Self {
            closed: false,
            event_loop,
            native_window,
            gpu,
            renderer,
            key_pressed: [false; 1],
        })
    }

    pub fn closed(&self) -> bool {
        self.closed
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

    pub fn display(&mut self, scene: Scene) {
        self.gpu.display(scene, &self.renderer);
    }
}