use wasm_bindgen::JsCast;
use winit::{ platform::web::WindowAttributesExtWebSys, window::WindowAttributes };


pub fn window_attributes() -> WindowAttributes {
  let mut attrs = WindowAttributes::default();
  let window = web_sys::window().unwrap();
  let document = window.document().unwrap();
  let canvas = document.get_element_by_id("canvas").unwrap();
  let canvas = canvas.unchecked_into();
  attrs = attrs.with_canvas(Some(canvas));

  attrs
}
