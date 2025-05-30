mod geometry;
mod material;
mod sphere;
mod scene;
mod ray_utils;
mod renderer;

use crate::geometry::Vec3;
use crate::renderer::cast_ray;
use std::fs::File;
use std::io::Write;

fn main() {
    const WIDTH: usize = 1024;
    const HEIGHT: usize = 768;
    const FOV: f32 = 1.05; // in radians

    let mut framebuffer = vec![Vec3::default(); WIDTH * HEIGHT];

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            // Normalized screen space [-1, 1]
            let dir_x = (2.0 * (x as f32 + 0.5) / WIDTH as f32 - 1.0) * (FOV / 2.0).tan() * WIDTH as f32 / HEIGHT as f32;
            let dir_y = -(2.0 * (y as f32 + 0.5) / HEIGHT as f32 - 1.0) * (FOV / 2.0).tan();
            let dir = Vec3::new(dir_x, dir_y, -1.0).normalized();
            
            framebuffer[y * WIDTH + x] = cast_ray(Vec3::new(0.0, 0.0, 0.0), dir, 0);
        }
    }

    // Save as PPM
    let mut file = File::create("out.ppm").expect("Failed to create file");
    write!(file, "P6\n{} {}\n255\n", WIDTH, HEIGHT).unwrap();

    for color in framebuffer.iter() {
        let max = color.x.max(color.y.max(color.z)).max(1.0);
        let r = (255.0 * color.x / max) as u8;
        let g = (255.0 * color.y / max) as u8;
        let b = (255.0 * color.z / max) as u8;
        file.write_all(&[r, g, b]).unwrap();
    }

    println!("Rendered ray-traced scene to out.ppm");
}
