extern crate nalgebra as na;
use raytracing::*;

#[derive(Clone)]
pub struct Intersection {
    pub hit_distance: f64,
    pub eye_ray: Ray,
    pub normal: na::Unit<na::Vector3<f64>>,
}

pub trait Trace {
    fn intersect(&self, ray: Ray) -> Option<Intersection>;
    fn material<'a>(&'a self) -> &'a Box<Material>;
}
