mod framebuffer;
mod line;

use framebuffer::Framebuffer;
use line::line;
use raylib::prelude::*;

fn main() {
    let width = 800;
    let height = 600;

    let mut framebuffer = Framebuffer::new(width, height);

    framebuffer.set_background_color(Color::new(50, 50, 100, 255));
    framebuffer.clear();

    framebuffer.set_current_color(Color::GREEN);
    line(
        &mut framebuffer,
        Vector2::new(50.0, 50.0),
        Vector2::new(350.0, 350.0),
    );

    framebuffer.set_current_color(Color::RED);
    line(
        &mut framebuffer,
        Vector2::new(350.0, 50.0),
        Vector2::new(50.0, 350.0),
    );

    framebuffer.render_to_file("lines.bmp");
}