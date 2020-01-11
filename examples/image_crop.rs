use nae::prelude::*;

fn init(app: &mut App) -> Texture {
    app.load_file("./examples/assets/rust.png").unwrap()
}

fn draw(app: &mut App, logo: &mut Texture) {
    let draw = app.draw();
    draw.begin();
    draw.clear(Color::WHITE);
    draw.image_crop(logo, 0.0, 0.0, 0.0, 0.0, logo.width() * 0.5, 0.0);
    draw.image_crop(
        logo,
        logo.width() * 0.6,
        00.0,
        0.0,
        logo.height() * 0.5,
        0.0,
        logo.height() * 0.5,
    );
    draw.end();
}

#[nae::main]
fn main() {
    nae::init_with(init).draw(draw).build().unwrap();
}
