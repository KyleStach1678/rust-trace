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

struct IntersectedObject<'a> {
    intersection: Intersection,
    object: &'a Box<Trace>,
}

pub fn trace<'a>(scene: &'a Scene, eye: Ray) -> Color {
    let best = scene.objects
        .iter()
        .fold::<Option<IntersectedObject>, _>(None, |intersected_object, new_obj| {
            match (intersected_object, new_obj.intersect(eye.clone())) {
                (Some(intersected_object), Some(intersection)) => {
                    if intersected_object.intersection.hit_distance > intersection.hit_distance {
                        Some(IntersectedObject {
                            intersection: intersection,
                            object: &new_obj,
                        })
                    } else {
                        Some(intersected_object)
                    }
                }
                (None, Some(intersection)) => {
                    Some(IntersectedObject {
                        intersection: intersection,
                        object: &new_obj,
                    })
                }
                (Some(intersected_object), None) => Some(intersected_object),
                (None, None) => None,
            }
        });

    if let Some(IntersectedObject { intersection, object }) = best {
        object.material().surface(intersection, scene)
    } else {
        Color {
            r: 0f64,
            g: 0f64,
            b: 0f64,
        }
    }
}
