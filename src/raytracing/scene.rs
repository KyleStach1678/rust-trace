extern crate nalgebra as na;
use raytracing::*;

pub struct Light {
    pos: na::Vector3<f64>,
}

pub struct Scene {
    pub objects: Vec<Box<Trace>>,
    pub lights: Vec<Box<Light>>,
}
