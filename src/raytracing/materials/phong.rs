extern crate nalgebra as na;
use na::Dot;
use raytracing::*;


pub struct PhongMaterial {
    pub ambient: Color,
    pub diffuse: Color,
    pub specular: Color,
    pub shiny: u32,
}

impl Material for PhongMaterial {
    fn surface(&self, intersection: Intersection, scene: &Scene) -> Color {
        if intersection.normal.unwrap().dot(&intersection.eye_ray.direction.unwrap()) > 0f64 {
            return Color {
                r: 0f64,
                g: 0f64,
                b: 0f64,
            };
        }
        let ambient_component = self.ambient;
        let mut diffuse_component = Color {
            r: 0f64,
            g: 0f64,
            b: 0f64,
        };
        let mut specular_component = Color {
            r: 0f64,
            g: 0f64,
            b: 0f64,
        };

        for light in &scene.lights {
            let to_light = na::Unit::new(&(light.pos -
                                           (intersection.hit_distance *
                                            intersection.eye_ray.direction.unwrap())
                .to_point()));
            let diffuse_intensity = to_light.unwrap().dot(&intersection.normal.unwrap()).max(0f64);
            let diffuse = self.diffuse * light.intensity * diffuse_intensity;

            let reflected_vector =
                na::Unit::new(&(2f64 * (to_light.unwrap().dot(&intersection.normal.unwrap())) *
                                intersection.normal.unwrap() -
                                to_light.unwrap()));
            // Invert the eye ray in this formula to get the direction from the hit point to the
            // viewer instead of viewer->hit
            let specular_intensity = reflected_vector.unwrap()
                .dot(&-intersection.eye_ray.direction.unwrap())
                .max(0f64)
                .powi(self.shiny as i32);
            let specular = self.specular * light.intensity * specular_intensity;

            diffuse_component = diffuse_component + diffuse;
            specular_component = specular_component + specular;
        }

        ambient_component + diffuse_component + specular_component
    }
}
