extern crate nalgebra as na;
use raytracing::*;

pub struct Light {
    pub pos: na::Point3<f64>,
    pub intensity: Color,
}

pub struct Scene {
    pub objects: Vec<Box<Trace>>,
    pub lights: Vec<Box<Light>>,
}
