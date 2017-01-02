extern crate nalgebra as na;

#[derive(Clone)]
pub struct Ray {
    pub origin: na::Point3<f64>,
    pub direction: na::Unit<na::Vector3<f64>>,
}
