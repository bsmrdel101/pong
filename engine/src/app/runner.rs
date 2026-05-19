use std::sync::Arc;
use winit::{
  application::ApplicationHandler,
  event_loop::{ ActiveEventLoop, EventLoop },
  window::Window,
  event::WindowEvent,
  window::WindowId
};
use crate::{ app::state::{App, AppState}, error::EngineResult, platform::window_attributes, rendering::renderer::Renderer };

impl ApplicationHandler<()> for App {
  fn resumed(&mut self, event_loop: &ActiveEventLoop) {
    let attributes = window_attributes();
    let window = match event_loop.create_window(attributes) {
      Ok(window) => Arc::new(window),
      Err(error) => {
        eprintln!("{error}");
        event_loop.exit();
        return;
      }
    };
    
    let state = match pollster::block_on(create_state(window)) {
      Ok(state) => state,
      Err(error) => {
        eprintln!("{error}");
        event_loop.exit();
        return;
      }
    };

    self.state = Some(state);
  }

  fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
    match event {
      WindowEvent::CloseRequested => {
        event_loop.exit();
      }
      _ => {}
    }
  }
}


pub async fn create_state(window: Arc<Window>) -> EngineResult<AppState> {
  let renderer = Renderer::new(window.clone()).await?;
  AppState::new(window, renderer).await
}

pub fn run() -> EngineResult<()> {
  let event_loop = EventLoop::new()?;
  let mut app = App::new();
  event_loop.run_app(&mut app)?;

  Ok(())
}
