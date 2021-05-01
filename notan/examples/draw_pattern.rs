use notan::app::assets::*;
use notan::app::config::WindowConfig;
use notan::app::graphics::prelude::*;
use notan::app::{App, AppBuilder, DrawImages, DrawPattern, DrawShapes, Graphics, Plugins};
use notan::log;
use notan::prelude::*;

#[derive(AppState)]
struct State {
    img: Texture,
    count: f32,
    multi: f32,
}

impl State {
    pub fn count(&mut self, value: f32) {
        if self.count >= 200.0 || self.count <= 0.0 {
            self.multi *= -1.0;
        }

        self.count += value * self.multi;
    }
}

#[notan::main]
fn main() -> Result<(), String> {
    notan::init_with(init)
        .update(|app: &mut App, state: &mut State| state.count(1.0))
        .draw(draw)
        .build()
}

fn init(gfx: &mut Graphics) -> State {
    let img = TextureInfo::from_image(include_bytes!("assets/pattern.png")).unwrap();
    let texture = gfx.create_texture(img).unwrap();
    State {
        img: texture,
        count: 1.0,
        multi: 1.0,
    }
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw2();

    draw.pattern(&state.img)
        .size(800.0, 600.0)
        // .image_scale(2.0, 2.0)
        .image_offset(-state.count, -state.count);

    gfx.render(&draw);
}
