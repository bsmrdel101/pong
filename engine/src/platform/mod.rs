#[cfg(not(target_arch = "wasm32"))]
pub mod desktop;

#[cfg(target_arch = "wasm32")]
pub mod web;

#[cfg(not(target_arch = "wasm32"))]
pub use desktop::{
  spawn_event_loop,
  get_device_required_limits,
  get_instance,
  window_attributes,
  load_bytes
};

#[cfg(target_arch = "wasm32")]
pub use web::{
  spawn_event_loop,
  get_device_required_limits,
  get_instance,
  window_attributes,
  load_bytes
};
