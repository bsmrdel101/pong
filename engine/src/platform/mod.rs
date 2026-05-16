#[cfg(not(target_arch = "wasm32"))]
pub mod desktop;

#[cfg(target_arch = "wasm32")]
pub mod web;

#[cfg(not(target_arch = "wasm32"))]
pub use desktop::window_attributes;

#[cfg(target_arch = "wasm32")]
pub use web::window_attributes;
