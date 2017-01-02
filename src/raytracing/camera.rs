extern crate nalgebra as na;
use na::Cross;
use raytracing::Ray;

pub struct Camera {
    fov: (f64, f64),
    position: na::Point3<f64>,
    forward: na::Unit<na::Vector3<f64>>,
    right: na::Unit<na::Vector3<f64>>,
    up: na::Unit<na::Vector3<f64>>,
}

// Convention: y is up
fn calculate_basis_vectors(look: na::Unit<na::Vector3<f64>>)
                           -> (na::Unit<na::Vector3<f64>>, na::Unit<na::Vector3<f64>>) {
    let straight_up = na::Unit::new(&na::Vector3::new(0f64, 1f64, 0f64));
    let right = look.unwrap().cross(&straight_up.unwrap());
    (na::Unit::new(&right.cross(&look.unwrap())), na::Unit::new(&right))
}

fn calculate_fov_components(diagonal_fov: f64, aspect_ratio: f64) -> (f64, f64) {
    let vfov = ((diagonal_fov / 2f64).tan() / (1f64 + aspect_ratio.powi(2)).sqrt()).atan() * 2f64;
    let hfov = ((diagonal_fov / 2f64).tan() / (1f64 + aspect_ratio.powi(-2)).sqrt()).atan() * 2f64;
    (hfov, vfov)
}

impl Camera {
    pub fn new(diagonal_fov: f64,
           aspect_ratio: f64,
           position: na::Point3<f64>,
           look: na::Unit<na::Vector3<f64>>)
           -> Camera {
        let (up, right) = calculate_basis_vectors(look);
        Camera {
            fov: calculate_fov_components(diagonal_fov, aspect_ratio),
            position: position,
            forward: look,
            up: up,
            right: right,
        }
    }

    pub fn get_ray(&self, x: f64, y: f64) -> Ray {
        Ray {
            origin: self.position,
            direction: na::Unit::new(&(self.up.unwrap() * (self.fov.0 * x).tan() +
                                       self.right.unwrap() * (self.fov.1 * y).tan() +
                                       self.forward.unwrap())),
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate nalgebra as na;
    use super::{calculate_basis_vectors, calculate_fov_components, Camera};
    use na::{Dot, Norm};
    use std::f64;

    #[test]
    fn camera_orthagonalization() {
        let look = na::Unit::new(&na::Vector3::new(1f64, 1f64, 0f64));
        let (up, right) = calculate_basis_vectors(look);
        print!("{:?}", up);
        assert!(na::approx_eq(&up.unwrap(),
                              &na::Vector3::new(-1f64, 1f64, 0f64).normalize()));
        assert!(na::approx_eq(&right.unwrap(),
                              &na::Vector3::new(0f64, 0f64, 1f64).normalize()));
    }

    #[test]
    fn fov_components() {
        let dfov = f64::consts::PI / 2f64;
        let aspect = 4f64 / 3f64;
        let (hfov, vfov) = calculate_fov_components(dfov, aspect);
        assert!(hfov < dfov);
        assert!(vfov < dfov);
        assert!(vfov < hfov);
    }

    #[test]
    fn camera_init() {
        let forward = na::Unit::new(&na::Vector3::new(1f64, 1f64, 1f64));
        let position = na::Point3::new(0f64, 0f64, 1f64);
        let dfov = f64::consts::PI / 2f64;
        let camera = Camera::new(dfov, 4f64 / 3f64, position, forward);
        assert!(na::approx_eq(&0f64, &camera.up.unwrap().dot(&camera.right.unwrap())));
        assert!(na::approx_eq(&0f64, &forward.unwrap().dot(&camera.right.unwrap())));
        assert!(na::approx_eq(&0f64, &forward.unwrap().dot(&camera.up.unwrap())));
        assert!(camera.fov.0 < dfov);
        assert!(camera.fov.1 < camera.fov.0);
    }

    #[test]
    fn camera_rays() {
        let forward = na::Unit::new(&na::Vector3::new(1f64, 1f64, 1f64));
        let position = na::Point3::new(0f64, 0f64, 1f64);
        let camera = Camera::new(f64::consts::PI / 2f64, 4f64 / 3f64, position, forward);
        let center_ray = camera.get_ray(0f64, 0f64);
        println!("{:?}", camera.up);
        assert!(na::approx_eq(&center_ray.direction.unwrap(), &forward.unwrap()));
        assert!(na::approx_eq(&center_ray.origin, &position));
    }
}
