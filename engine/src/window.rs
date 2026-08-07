use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop};
use winit::window::{WindowAttributes, WindowId};
use crate::App;
use crate::rendering::renderer::Renderer;


impl ApplicationHandler for App {
  fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
    match event {
      WindowEvent::CloseRequested => {
        event_loop.exit();
      }
      WindowEvent::RedrawRequested => {
        if let Err(error) = self.renderer.as_mut().unwrap().render() {
          eprintln!("{error}");
        }
        self.window.as_ref().unwrap().request_redraw();
      }
      WindowEvent::Resized(size) => {
        self.renderer.as_mut().unwrap().resize(size.width, size.height);
      }
      _ => ()
    }
  }

  fn resumed(&mut self, event_loop: &ActiveEventLoop) {
    let window = match event_loop.create_window(WindowAttributes::default()) {
      Ok(window) => Arc::new(window),
      Err(error) => {
        eprintln!("{error}");
        event_loop.exit();
        return;
      }
    };

    let renderer = match pollster::block_on(Renderer::new(window.clone())) {
      Ok(renderer) => renderer,
      Err(error) => {
        eprintln!("{error}");
        event_loop.exit();
        return;
      }
    };

    self.renderer = Some(renderer);
    self.window = Some(window);

    self.window.as_ref().unwrap().request_redraw();
  }
}
