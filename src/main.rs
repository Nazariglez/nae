mod glm;
mod graphics;
mod window;
mod math;

use crate::graphics::color::{rgba, Color};
use crate::graphics::Vertex;
use std::rc::Rc;
use wasm_bindgen::__rt::core::cell::RefCell;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

pub struct App {
    window: window::Window,
    graphics: graphics::Context,
}

pub struct AppBuilder<S>
where
    S: 'static,
{
    state: Option<S>,
    draw_callback: Option<fn(&mut App, &mut S)>,
}

impl<S> AppBuilder<S> {
    pub fn build(&mut self) -> Result<(), String> {
        let win = window::Window::new();
        let gfx = graphics::Context::new(win.window())?;

        let mut app = App {
            window: win,
            graphics: gfx,
        };

        let mut state = self.state.take().unwrap();
        let mut draw_cb = self.draw_callback.take().unwrap_or(|_, _| {});

        //        let rc_app = Rc::new(RefCell::new(app));

        window::run(move || {
            draw_cb(&mut app, &mut state);
        });
        //cb(&mut app);

        //        Err("".to_string())
        Ok(())
    }

    pub fn draw(&mut self, cb: fn(&mut App, &mut S)) -> &mut Self {
        self.draw_callback = Some(cb);
        self
    }
}

pub fn init<S>(state: S) -> AppBuilder<S> {
    AppBuilder {
        state: Some(state),
        draw_callback: None,
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn wasm_main() {
    main();
}

//TODO think about this:
// - draw_2d() -> API easy like nannou api
//     draw_2d().transform().push(parentMatrix);
//     draw_2d().sprite(image)
//              .anchor(0.5, 0.5)
//              .rotation(1)
//              .scale(2, 2)
//              .filters([Filter::Blur, etc...])
//              .blend(BlendMode::Alpha)
//              .pos(100, 100);
// - draw() (or draw_raw())-> Stateful API like kha
//      gfx.begin(Some(Color::Red));
//      gfx.transform().push(matrix::scale(2, 2));
//      gfx.draw_image(image, 100, 100);
//      gfx.transform().pop();

fn draw_cb(app: &mut App, state: &mut State) {
    let gfx = &mut app.graphics;
    gfx.begin();
    gfx.clear(graphics::color::rgba(0.1, 0.2, 0.3, 1.0));
    gfx.set_color(Color::Red);
    gfx.transform().scale(0.5, 0.5);//.push(glm::scaling2d(&glm::vec2(0.5, 0.5)));
    gfx.draw_rect(0.0, 0.0, 100.0, 100.0);
    gfx.transform().pop();

    gfx.set_color(Color::Green);
//    gfx.transform().push(glm::scaling2d(&glm::vec2(2.0, 2.0)));
    gfx.transform().scale(2.0, 2.0);
    gfx.draw_triangle(200.0, 200.0, 300.0, 300.0, 100.0, 300.0);
    gfx.draw_vertex(&[
        Vertex::new(600.0, 200.0, Color::Red),
        Vertex::new(700.0, 300.0, Color::Green),
        Vertex::new(500.0, 300.0, Color::Blue),
    ]);
    gfx.set_color(Color::Red.with_alpha(0.3));
    gfx.stroke_triangle(600.0, 200.0, 700.0, 300.0, 500.0, 300.0, 10.0);
    gfx.transform().pop();

    let len = 50;
    for i in (0..len) {
        let n = i as f32;
        let r = (1.0 / len as f32) * n;
        let g = 0.5;
        let b = 1.0 - (1.0 / len as f32) * n;
        let a = 1.0;
        gfx.set_color(graphics::color::rgba(r, b, g, a));
        gfx.draw_rect(
            10.0 * n,
            10.0 * n,
            (100.0 / len as f32) * n,
            (100.0 / len as f32) * n,
        );
    }

    gfx.set_color(Color::Blue);
    gfx.draw_circle(200.0, 200.0, 50.0);
    gfx.stroke_circle(200.0, 200.0, 70.0, 10.0);
    gfx.set_color(Color::White);
    gfx.draw_line(200.0, 200.0, 300.0, 300.0, 10.0);
    gfx.draw_line(200.0, 300.0, 300.0, 200.0, 10.0);

    gfx.set_color(rgba(0.5, 0.5, 0.1, 1.0));
    gfx.draw_rounded_rect(300.0, 10.0, 200.0, 50.0, 20.0);
    gfx.set_color(rgba(1.0, 0.5, 0.5, 0.3));
    gfx.stroke_rounded_rect(300.0, 10.0, 200.0, 50.0, 20.0, 10.0);

    gfx.draw_rect(400.0, 100.0, 300.0, 80.0);
    gfx.set_color(Color::Green.with_alpha(0.3));
    gfx.stroke_rect(400.0, 100.0, 300.0, 80.0, 10.0);

    let (ww, hh) = (60.0, 60.0);
    gfx.set_color(Color::Red);
    gfx.set_alpha(0.5);
    gfx.transform().translate(430.0, 300.0);//.push(glm::translation2d(&glm::vec2(430.0, 300.0)));
    gfx.transform().rotate_deg(state.i as f32);
    gfx.draw_rect(-ww*0.5, -hh*0.5, ww, hh);
    gfx.transform().pop();
    gfx.transform().pop();

    gfx.set_color(Color::Blue);
    gfx.transform().translate(430.0, 300.0);
    gfx.transform().rotate_deg(state.i as f32 * 0.5);
    gfx.draw_rect(-ww*0.5, -hh*0.5, ww, hh);
    gfx.transform().pop();
    gfx.transform().pop();

    gfx.set_color(Color::Green);
    gfx.transform().translate(430.0, 300.0);
    gfx.transform().rotate_deg(-state.i as f32 * 0.5);
    gfx.draw_rect(-ww*0.5, -hh*0.5, ww, hh);
    gfx.transform().pop();
    gfx.transform().pop();
    gfx.set_alpha(1.0);


    gfx.end();

    state.i += 1;
}

struct State {
    pub i: i32,
}

fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    log("Hello, world!");
    let state = State { i: 0 };

    init(state)
        .draw(draw_cb)
        .build()
        .unwrap();
}

pub fn log(msg: &str) {
    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(msg));
}
