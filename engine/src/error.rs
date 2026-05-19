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
  ImageError(#[from] image::ImageError)
}

pub type EngineResult<T> = Result<T, EngineError>;
