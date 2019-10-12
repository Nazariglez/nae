mod graphics;
mod window;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

pub struct App {
    window: window::Window,
    renderer: graphics::renderer::Renderer,
}

pub struct AppBuilder<S> {
    state: Option<S>,
}

impl<S> AppBuilder<S> {
    pub fn build(&self) -> Result<App, String> {
        let win = window::Window::new();
        let renderer = graphics::renderer::Renderer::new(win.window())?;

        Ok(App {
            window: win,
            renderer: renderer
        })
    }
}

pub fn init<S>(state: S) -> AppBuilder<S> {
    AppBuilder { state: Some(state) }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn wasm_main() {
    main();
}

fn main() {
    println!("Hello, world!");
    let app = init({}).build().unwrap();
}
