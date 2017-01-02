use raytracing::*;
extern crate image;

pub trait Material {
    fn surface(&self, intersection: Intersection, scene: &Scene) -> image::Rgb<u8>;
}
