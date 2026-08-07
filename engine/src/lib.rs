pub mod error;
pub mod window;
pub mod platform;
pub mod rendering;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use error::EngineResult;
use rendering::renderer::Renderer;
use std::sync::Arc;
use winit::{
  event_loop::EventLoop,
  window::Window
};

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

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn start() {
  console_error_panic_hook::set_once();

  if let Err(error) = run() {
    web_sys::console::error_1(
      &format!("Engine failed: {error}").into()
    );
  }
}
