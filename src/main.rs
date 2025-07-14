mod framebuffer;
mod fill;
mod poly1;
mod poly2;
mod poly3;
mod poly4;

use raylib::prelude::*;
use framebuffer::Framebuffer;

fn main() {
    let width = 800;
    let height = 600;
    let mut framebuffer = Framebuffer::new(width, height);

    framebuffer.set_background_color(Color::new(50, 50, 100, 255));
    framebuffer.clear();

    framebuffer.set_current_color(Color::YELLOW);
    poly1::draw_poligono1(&mut framebuffer);
    framebuffer.set_current_color(Color::WHITE);
    framebuffer.draw_poligono_border(&poly1::POLY1V);

    framebuffer.set_current_color(Color::BLUE);
    poly2::draw_poligono2(&mut framebuffer);
    framebuffer.set_current_color(Color::WHITE);
    framebuffer.draw_poligono_border(&poly2::POLY2V);

    framebuffer.set_current_color(Color::RED);
    poly3::draw_poligono3(&mut framebuffer);
    framebuffer.set_current_color(Color::WHITE);
    framebuffer.draw_poligono_border(&poly3::POLY3V);

    framebuffer.set_current_color(Color::GREEN);
    poly4::draw_poligono4(&mut framebuffer);
    framebuffer.set_current_color(Color::WHITE);
    framebuffer.draw_poligono_border(&poly4::POLY4V);
    framebuffer.draw_poligono_border(&poly4::POLY5V);

    framebuffer.render_to_file("out.bmp");
}
