extern crate nalgebra as na;
use self::na::{Dot, Norm};
use raytracing::*;

pub struct Sphere {
    pub origin: na::Point3<f64>,
    pub radius: f64,
    pub material: Box<Material>,
}

impl Trace for Sphere {
    fn material<'a>(&'a self) -> &'a Box<Material> {
        &self.material
    }

    fn intersect(&self, ray: Ray) -> Option<Intersection> {
        // The point on the eye ray that is closest to the sphere's origin, and the distance form
        // the eye's origin to that point
        let dist_to_closest = ray.direction.unwrap().dot(&(self.origin - ray.origin));
        let closest_point = ray.origin + dist_to_closest * ray.direction.unwrap();

        // The origin, the closest point (above) and the intersection point form a right triangle
        // with the radius as the hypotenuse. Using the Pythagorean theorem, find the squared
        // distance (along the eye ray) between the closest point and the intersection point.
        let discriminant = self.radius.powi(2) - (self.origin - closest_point).norm_squared();

        // If the discriminant is negative, the ray misses the sphere. If the distance to the
        // closest point +/- the discriminant is also negative, the sphere is entirely behind the
        // view.
        if discriminant >= 0f64 && dist_to_closest + discriminant.sqrt() >= 0f64 {
            let mut dist_to_intersection = dist_to_closest - discriminant.sqrt();

            // The ray may emmenate from inside of the sphere in which case we want to use the
            // second intersection point.
            if dist_to_intersection < 0f64 {
                dist_to_intersection = dist_to_closest + discriminant.sqrt();
            }

            // The normal vector is just the intersection's position relative to the origin,
            // normalized.
            let normal = na::Unit::new(&(ray.origin +
                                         ray.direction.unwrap() * dist_to_intersection -
                                         self.origin));
            Some(Intersection {
                hit_distance: dist_to_intersection,
                eye_ray: ray,
                normal: normal,
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate image;
    use super::na;
    use super::Sphere;
    use raytracing::*;

    #[derive(Default)]
    struct DummyMaterial;
    impl Material for DummyMaterial {
        fn surface(&self, _: Intersection, _: &Scene) -> image::Rgb<u8> {
            image::Rgb([255u8, 0u8, 0u8])
        }
    }

    #[test]
    fn simple_intersect() {
        let sphere = Sphere {
            origin: na::Point3::new(0f64, 0f64, 0f64),
            radius: 1f64,
            material: Box::new(DummyMaterial::default()),
        };
        let ray = Ray {
            origin: na::Point3::new(-2f64, 0f64, 0f64),
            direction: na::Unit::new(&na::Vector3::new(1f64, 0f64, 0f64)),
        };

        if let Some(intersection) = sphere.intersect(ray) {
            assert!(na::approx_eq(&intersection.hit_distance, &1f64));
            assert!(na::approx_eq(&intersection.normal.unwrap(),
                                  &na::Vector3::new(1f64, 0f64, 0f64)));
        } else {
            panic!("Should intersect!");
        }
    }

    #[test]
    fn intersect_inside() {
        let sphere = Sphere {
            origin: na::Point3::new(0f64, 0f64, 0f64),
            radius: 1f64,
            material: Box::new(DummyMaterial::default()),
        };
        let ray = Ray {
            origin: na::Point3::new(0f64, 0f64, 0f64),
            direction: na::Unit::new(&na::Vector3::new(1f64, 0f64, 0f64)),
        };

        if let Some(intersection) = sphere.intersect(ray) {
            assert!(na::approx_eq(&intersection.hit_distance, &1f64));
            assert!(na::approx_eq(&intersection.normal.unwrap(),
                                  &na::Vector3::new(1f64, 0f64, 0f64)));
        } else {
            panic!("Should intersect!");
        }
    }

    #[test]
    fn intersect_translated() {
        let sphere = Sphere {
            origin: na::Point3::new(2f64, 0f64, 1f64),
            radius: 1f64,
            material: Box::new(DummyMaterial::default()),
        };
        let ray = Ray {
            origin: na::Point3::new(0f64, 0f64, 3f64),
            direction: na::Unit::new(&na::Vector3::new(2f64, 0f64, -2f64)),
        };

        if let None = sphere.intersect(ray) {
            panic!("Should intersect!");
        }
    }

    #[test]
    fn intersect_scaled() {
        let sphere = Sphere {
            origin: na::Point3::new(0f64, 0f64, 0f64),
            radius: 2f64,
            material: Box::new(DummyMaterial::default()),
        };
        let ray = Ray {
            origin: na::Point3::new(1f64, 0f64, 1.5f64),
            direction: na::Unit::new(&na::Vector3::new(-1f64, 0f64, 0f64)),
        };

        if let Some(intersection) = sphere.intersect(ray.clone()) {
            assert!(na::approx_eq(&(ray.origin +
                                    intersection.hit_distance * ray.direction.unwrap())
                                      .y,
                                  &0f64));
            assert!(na::approx_eq(&(ray.origin +
                                    intersection.hit_distance * ray.direction.unwrap())
                                      .z,
                                  &1.5f64));
        }
    }

    #[test]
    fn no_intersect_miss() {
        let sphere = Sphere {
            origin: na::Point3::new(0f64, 0f64, 0f64),
            radius: 1f64,
            material: Box::new(DummyMaterial::default()),
        };
        let ray = Ray {
            origin: na::Point3::new(2f64, 2f64, 0f64),
            direction: na::Unit::new(&na::Vector3::new(-1f64, 0f64, 0f64)),
        };

        if let Some(_) = sphere.intersect(ray) {
            panic!("Should not intersect!");
        }
    }

    #[test]
    fn no_intersect_behind() {
        let sphere = Sphere {
            origin: na::Point3::new(0f64, 0f64, 0f64),
            radius: 1f64,
            material: Box::new(DummyMaterial::default()),
        };
        let ray = Ray {
            origin: na::Point3::new(2f64, 0f64, 0f64),
            direction: na::Unit::new(&na::Vector3::new(1f64, 0f64, 0f64)),
        };

        if let Some(_) = sphere.intersect(ray) {
            panic!("Should not intersect!");
        }
    }
}
