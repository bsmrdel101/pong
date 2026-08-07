use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowId;
use crate::platform::window_attributes;
use crate::rendering::renderer::Renderer;
use crate::App;


impl ApplicationHandler for App {
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            WindowEvent::RedrawRequested => {
                if let Some(renderer) = self.renderer.as_mut() {
                    if let Err(error) = renderer.render() {
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
        // Prevent creating multiple windows if `resumed` is called again.
        if self.window.is_some() {
            return;
        }

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

            self.window = Some(window);

            wasm_bindgen_futures::spawn_local(async move {
                match Renderer::new(window_clone.clone()).await {
                    Ok(_renderer) => {
                        // Renderer initialization completed successfully.
                        //
                        // The renderer cannot be assigned to `self` here because
                        // this async task owns no mutable reference to `App`.
                        //
                        // See the note below.
                        window_clone.request_redraw();
                    }

                    Err(error) => {
                        web_sys::console::error_1(
                            &format!("Renderer initialization failed: {error}").into(),
                        );
                    }
                }
            });
        }
    }
}
