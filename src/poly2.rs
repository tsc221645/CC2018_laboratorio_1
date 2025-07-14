use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use crate::fill::fill_polygon;

pub const POLY2V: [Vector2; 4] = [
    Vector2::new(321.0, 335.0),
    Vector2::new(288.0, 286.0),
    Vector2::new(339.0, 251.0),
    Vector2::new(374.0, 302.0),
];

pub fn draw_poligono2(fb: &mut Framebuffer) {
    fill_polygon(fb, &POLY2V);
}
