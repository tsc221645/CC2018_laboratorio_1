mod framebuffer;
mod fill;
mod poly2;

use raylib::prelude::*;
use framebuffer::Framebuffer;
use poly2::draw_poligono2;

fn main() {
    let width = 800;
    let height = 600;
    let mut framebuffer = Framebuffer::new(width, height);

    framebuffer.set_background_color(Color::new(50, 50, 100, 255));
    framebuffer.clear();

    framebuffer.set_current_color(Color::BLUE); // relleno
    draw_poligono2(&mut framebuffer);

    framebuffer.set_current_color(Color::WHITE); // borde
    framebuffer.draw_poligono_border(&poly2::POLY2V);

    framebuffer.render_to_file("out.bmp");
}