use raytracing::*;

pub trait Material {
    fn surface(&self, intersection: Intersection, scene: &Scene) -> Color;
}
