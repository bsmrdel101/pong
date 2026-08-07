pub mod error;
pub mod window;
pub mod platform;
pub mod rendering;

use std::sync::Arc;
use winit::{event_loop::{ControlFlow, EventLoop}, window::Window};
use crate::{error::EngineResult, rendering::renderer::Renderer};


#[derive(Default)]
pub struct App {
  pub window: Option<Arc<Window>>,
  pub renderer: Option<Renderer>
}

pub fn run() -> EngineResult<()> {
  let event_loop = EventLoop::new().unwrap();
  event_loop.set_control_flow(ControlFlow::Poll);
  event_loop.run_app(&mut App::default()).unwrap();
  Ok(())
}
