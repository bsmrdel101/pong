pub mod error;
pub mod window;
pub mod platform;
pub mod rendering;

use error::EngineResult;
use rendering::renderer::Renderer;
use std::sync::Arc;
use winit::{event_loop::EventLoop, window::Window};
use crate::platform::spawn_event_loop;


#[derive(Default)]
pub struct App {
  pub window: Option<Arc<Window>>,
  pub renderer: Option<Renderer>
}

pub fn run() -> EngineResult<()> {
  let event_loop = EventLoop::new().unwrap();
  spawn_event_loop(event_loop);
  Ok(())
}
