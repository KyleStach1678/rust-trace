extern crate image;
pub use raytracing::*;

pub struct AmbientMaterial {
    pub color: image::Rgb<u8>,
}

impl Material for AmbientMaterial {
    fn surface(&self, _: Intersection, _: &Scene) -> image::Rgb<u8> {
        self.color
    }
}
