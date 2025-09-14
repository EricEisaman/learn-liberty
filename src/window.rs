//! Window management module for Learn Liberty
//! 
//! This module provides window management interface
//! for the educational RPG application.

use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

pub struct WindowManager {
    window: winit::window::Window,
    event_loop: EventLoop<()>,
}

impl WindowManager {
    pub fn new(title: &str) -> Self {
        let event_loop = EventLoop::new().unwrap();
        let window = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(winit::dpi::LogicalSize::new(800, 600))
            .with_resizable(true)
            .build(&event_loop)
            .unwrap();

        Self { window, event_loop }
    }

    pub fn run<F>(self, mut update: F)
    where
        F: FnMut(&winit::window::Window) + 'static,
    {
        let _ = self.event_loop.run(move |event, elwt| {
            match event {
                Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                    elwt.exit();
                }
                Event::AboutToWait => {
                    update(&self.window);
                    self.window.request_redraw();
                }
                _ => (),
            }
        });
    }

    pub fn window(&self) -> &winit::window::Window {
        &self.window
    }
}
