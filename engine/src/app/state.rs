use std::sync::Arc;
use winit::window::Window;
use crate::error::EngineResult;

pub struct AppState {
  pub window: Arc<Window>,
  // pub renderer: Renderer,
}

impl AppState {
  pub async fn new(window: Arc<Window>) -> EngineResult<Self> {
    Ok(
      Self {
        window
      }
    )
  }
}
