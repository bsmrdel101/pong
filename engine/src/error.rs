use thiserror::Error;

#[derive(Debug, Error)]
pub enum EngineError {
  #[error("WINIT ERROR (EventLoopError)")]
  EventLoop(winit::error::EventLoopError),

  #[error("WGPU ERROR (RequestDeviceError)")]
  Rendering(wgpu::RequestDeviceError),
}

pub type EngineResult<T> = Result<T, EngineError>;

impl From<winit::error::EventLoopError> for EngineError {
  fn from(value: winit::error::EventLoopError) -> Self {
    EngineError::EventLoop(value)
  }
}

impl From<wgpu::RequestDeviceError> for EngineError {
  fn from(value: wgpu::RequestDeviceError) -> Self {
    EngineError::Rendering(value)
  }
}
