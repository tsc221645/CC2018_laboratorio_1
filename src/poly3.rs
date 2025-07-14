use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use crate::fill::fill_polygon;

pub const POLY3V: [Vector2; 4] = [
    Vector2::new(456.0, 146.0),
    Vector2::new(464.0, 183.0),
    Vector2::new(502.0, 177.0),
    Vector2::new(495.0, 138.0),
];

pub fn draw_poligono3(fb: &mut Framebuffer) {
    fill_polygon(fb, &POLY3V);
}