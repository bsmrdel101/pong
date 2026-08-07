use thiserror::Error;

#[derive(Debug, Error)]
pub enum EngineError {
  #[error(transparent)]
  EventLoop(#[from] winit::error::EventLoopError),

  #[error(transparent)]
  RequestDeviceError(#[from] wgpu::RequestDeviceError),

  #[error(transparent)]
  SurfaceError(#[from] wgpu::SurfaceError),

  #[error(transparent)]
  RequestAdapterError(#[from] wgpu::RequestAdapterError),

  #[error(transparent)]
  CreateSurfaceError(#[from] wgpu::CreateSurfaceError),

  #[error(transparent)]
  IOError(#[from] std::io::Error),

  #[error(transparent)]
  ImageError(#[from] image::ImageError),
  
  #[cfg(target_arch = "wasm32")]
  #[error("Javascript error: {0}")]
  Js(String)
}

pub type EngineResult<T> = Result<T, EngineError>;

#[cfg(target_arch = "wasm32")]
impl From<wasm_bindgen::JsValue> for EngineError {
  fn from(value: wasm_bindgen::JsValue) -> Self {
    EngineError::Js(
      value
        .as_string()
        .unwrap_or_else(|| format!("{:?}", value))
    )
  }
}
