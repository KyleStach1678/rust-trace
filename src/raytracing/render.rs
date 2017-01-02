extern crate image;
use raytracing::*;
use std::path::Path;

pub fn render(scene: &Scene, camera: &Camera, size: (u32, u32)) {
    let mut image = image::ImageBuffer::new(size.0, size.1);
    for i in 0..size.0 {
        for j in 0..size.1 {
            let x = (i * 2) as f64 / (size.0 as f64) - 1f64;
            let y = (j * 2) as f64 / (size.1 as f64) - 1f64;
            image.put_pixel(i, j, trace(scene, camera.get_ray(x, y)));
        }
    }
    image.save(Path::new("test.png"));
}
