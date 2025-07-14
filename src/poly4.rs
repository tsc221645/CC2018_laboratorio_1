use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use crate::fill::fill_polygon;

pub const POLY4V: [Vector2; 8] = [
    Vector2::new(337.0, 118.0),
    Vector2::new(321.0, 135.0),
    Vector2::new(356.0, 222.0),
    Vector2::new(383.0, 203.0),
    Vector2::new(337.0, 118.0),
    Vector2::new(337.0, 118.0),
    Vector2::new(337.0, 118.0),
    Vector2::new(337.0, 118.0),
];

pub const POLY5V: [Vector2; 4] = [
    Vector2::new(345.0, 150.0),
    Vector2::new(352.0, 160.0),
    Vector2::new(344.0, 169.0),
    Vector2::new(337.0, 160.0),
];

pub fn draw_poligono4(fb: &mut Framebuffer) {
    fill_polygon(fb, &POLY4V);
    fb.set_current_color(Color::new(50, 50, 100, 255)); 
    fill_polygon(fb, &POLY5V);
}
