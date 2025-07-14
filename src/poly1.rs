use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use crate::fill::fill_polygon;

pub const POLY1V: [Vector2; 12] = [
    Vector2::new(165.0, 380.0),
    Vector2::new(185.0, 360.0),
    Vector2::new(180.0, 330.0),
    Vector2::new(207.0, 345.0),
    Vector2::new(233.0, 330.0),
    Vector2::new(230.0, 360.0),
    Vector2::new(250.0, 380.0),
    Vector2::new(220.0, 385.0),
    Vector2::new(205.0, 410.0),
    Vector2::new(193.0, 383.0),
    Vector2::new(193.0, 383.0),
    Vector2::new(165.0, 380.0),
];

pub fn draw_poligono1(fb: &mut Framebuffer) {
    fill_polygon(fb, &POLY1V);
}
