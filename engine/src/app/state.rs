use std::sync::Arc;
use winit::window::Window;
use crate::{ error::EngineResult, rendering::renderer::Renderer };

pub struct App {
  pub state: Option<AppState>
}

impl App {
  pub fn new() -> Self {
    Self {
      state: None
    }
  }
}

pub struct AppState {
  pub window: Arc<Window>,
  pub renderer: Renderer
}

impl AppState {
  pub async fn new(window: Arc<Window>, renderer: Renderer) -> EngineResult<Self> {
    Ok(
      Self {
        window,
        renderer
      }
    )
  }
}
