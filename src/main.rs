mod framebuffer;
mod fill;
mod poly3;

use raylib::prelude::*;
use framebuffer::Framebuffer;
use poly3::draw_poligono3;

fn main() {
    let width = 800;
    let height = 600;
    let mut framebuffer = Framebuffer::new(width, height);

    framebuffer.set_background_color(Color::new(50, 50, 100, 255));
    framebuffer.clear();

    framebuffer.set_current_color(Color::RED); // relleno
    draw_poligono3(&mut framebuffer);

    framebuffer.set_current_color(Color::WHITE); // borde
    framebuffer.draw_poligono_border(&poly3::POLY3V);

    framebuffer.render_to_file("out.bmp");
}