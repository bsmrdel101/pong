#[cfg(target_arch = "wasm32")]
use std::{cell::RefCell, rc::Rc};

use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowId;
use crate::platform::window_attributes;
use crate::rendering::renderer::Renderer;
use crate::App;


impl ApplicationHandler for App {
  fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
    match event {
      WindowEvent::CloseRequested => {
        event_loop.exit();
      }
      WindowEvent::RedrawRequested => {
        #[cfg(target_arch = "wasm32")]
        {
          if self.renderer.is_none() {
            if let Some(renderer_ready) = &self.renderer_ready {
              if let Some(result) = renderer_ready.borrow_mut().take() {
                match result {
                  Ok(renderer) => {
                    web_sys::console::log_1(&"Moving renderer into App".into());
                    self.renderer = Some(renderer);
                  }
                  Err(error) => {
                    web_sys::console::error_1(
                      &format!("Renderer initialization failed: {error}").into()
                    );
                  }
                }
              }
            }
          }
        }

        if let Some(renderer) = self.renderer.as_mut() {
          if let Err(error) = renderer.render() {
            #[cfg(target_arch = "wasm32")]
            web_sys::console::error_1(&format!("Render failed: {error}").into());

            #[cfg(not(target_arch = "wasm32"))]
            eprintln!("{error}");
          }

          if let Some(window) = self.window.as_ref() {
            window.request_redraw();
          }
        }
      }
      WindowEvent::Resized(size) => {
        if let Some(renderer) = self.renderer.as_mut() {
          renderer.resize(size.width, size.height);
        }
      }
      _ => {}
    }
  }

  fn resumed(&mut self, event_loop: &ActiveEventLoop) {
    if self.window.is_some() { return; }

    let window = match event_loop.create_window(window_attributes()) {
      Ok(window) => Arc::new(window),
      Err(error) => {
        eprintln!("{error}");
        event_loop.exit();
        return;
      }
    };

    #[cfg(not(target_arch = "wasm32"))]
    {
      let renderer = match pollster::block_on(Renderer::new(window.clone())) {
        Ok(renderer) => renderer,
        Err(error) => {
          eprintln!("{error}");
          event_loop.exit();
          return;
        }
      };

      self.renderer = Some(renderer);
      self.window = Some(window.clone());

      window.request_redraw();
    }

    #[cfg(target_arch = "wasm32")]
    {
      let window_clone = window.clone();
      let renderer_ready = Rc::new(RefCell::new(None));
      let renderer_ready_clone = renderer_ready.clone();

      self.window = Some(window);
      self.renderer_ready = Some(renderer_ready);

      web_sys::console::log_1(
        &"Window created".into()
      );

      wasm_bindgen_futures::spawn_local(async move {
        match Renderer::new(window_clone.clone()).await {
          Ok(renderer) => {
            web_sys::console::log_1(&"Renderer initialized successfully!".into());
            *renderer_ready_clone.borrow_mut() = Some(Ok(renderer));
            window_clone.request_redraw();
          }
          Err(error) => {
            web_sys::console::error_1(
                &format!("Renderer initialization failed: {error}").into()
            );
          }
        }
      });
    }
  }
}
