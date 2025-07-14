mod framebuffer;
mod fill;
mod poly4;

use raylib::prelude::*;
use framebuffer::Framebuffer;
use poly4::draw_poligono4;

fn main() {
    let width = 800;
    let height = 600;
    let mut framebuffer = Framebuffer::new(width, height);

    framebuffer.set_background_color(Color::new(50, 50, 100, 255));
    framebuffer.clear();

    framebuffer.set_current_color(Color::GREEN); // relleno exterior
    draw_poligono4(&mut framebuffer);

    framebuffer.set_current_color(Color::WHITE); // bordes
    framebuffer.draw_poligono_border(&poly4::POLY4V);
    framebuffer.draw_poligono_border(&poly4::POLY5V);

    framebuffer.render_to_file("out.bmp");
}
