use raylib::prelude::*;

pub struct Framebuffer {
    width: i32,
    height: i32,
    image: Image,
    current_color: Color,
}

impl Framebuffer {
    pub fn new(width: i32, height: i32) -> Self {
        let image = Image::gen_image_color(width, height, Color::BLANK);
        Self {
            width,
            height,
            image,
            current_color: Color::WHITE,
        }
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.image = Image::gen_image_color(self.width, self.height, color);
    }

    pub fn clear(&mut self) {
        // Ya fue "borrado" al reinicializar la imagen
    }

    pub fn point(&mut self, x: f32, y: f32) {
        if x >= 0.0 && x < self.width as f32 && y >= 0.0 && y < self.height as f32 {
            self.image.draw_pixel(x as i32, y as i32, self.current_color);
        }
    }

    pub fn draw_line(&mut self, x0: f32, y0: f32, x1: f32, y1: f32) {
        // Algoritmo de Bresenham adaptado
        let mut x0 = x0 as i32;
        let mut y0 = y0 as i32;
        let x1 = x1 as i32;
        let y1 = y1 as i32;

        let dx = (x1 - x0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let dy = -(y1 - y0).abs();
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            self.point(x0 as f32, y0 as f32);

            if x0 == x1 && y0 == y1 {
                break;
            }

            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }
    }

    pub fn draw_poligono_border(&mut self, vertices: &[Vector2]) {
        for i in 0..vertices.len() {
            let a = vertices[i];
            let b = vertices[(i + 1) % vertices.len()];
            self.draw_line(a.x, a.y, b.x, b.y);
        }
    }

    pub fn render_to_file(&mut self, filename: &str) {
        self.image.export_image(filename);
        println!("Imagen guardada en {}", filename);
    }
}
