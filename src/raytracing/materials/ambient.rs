pub use raytracing::*;

pub struct AmbientMaterial {
    pub color: Color,
}

impl Material for AmbientMaterial {
    fn surface(&self, _: Intersection, _: &Scene) -> Color {
        self.color
    }
}
