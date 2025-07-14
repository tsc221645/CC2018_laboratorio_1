use raylib::prelude::*;
use crate::framebuffer::Framebuffer;

pub fn fill_polygon(framebuffer: &mut Framebuffer, vertices: &[Vector2]) {
    if vertices.len() < 3 {
        return;
    }

    let mut min_y = vertices[0].y as i32;
    let mut max_y = vertices[0].y as i32;

    for v in vertices.iter() {
        let y = v.y as i32;
        if y < min_y {
            min_y = y;
        }
        if y > max_y {
            max_y = y;
        }
    }

    
    for y in min_y..=max_y {
        let mut intersections: Vec<i32> = Vec::new();

        //check for vertices
        for i in 0..vertices.len() {
            let v1 = vertices[i];
            let v2 = vertices[(i + 1) % vertices.len()];

            let (x0, y0) = (v1.x, v1.y);
            let (x1, y1) = (v2.x, v2.y);

            if y0 == y1 {
                continue;
            }

            // verifies if there is any line crossing
            if (y >= y0.min(y1) as i32) && (y < y0.max(y1) as i32) {
                let x = x0 + (y as f32 - y0) * (x1 - x0) / (y1 - y0);
                intersections.push(x as i32);
            }
        }

        intersections.sort();
        for i in (0..intersections.len()).step_by(2) {
            if i + 1 >= intersections.len() {
                break;
            }
            let x_start = intersections[i];
            let x_end = intersections[i + 1];
            for x in x_start..=x_end {
                framebuffer.point(x as f32, y as f32);
            }
        }
    }
}
