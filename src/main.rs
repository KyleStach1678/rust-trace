extern crate nalgebra as na;
extern crate image;

mod raytracing;
use raytracing::*;
use raytracing::shapes::*;
use std::f64;

fn main() {
    let scene = Scene {
        objects: vec![Box::new(Sphere {
                          origin: na::Point3::new(0f64, 0f64, 0f64),
                          radius: 1f64,
                          material: Box::new(materials::PhongMaterial {
                              ambient: Color {
                                  r: 0.05f64,
                                  g: 0.05f64,
                                  b: 0.05f64,
                              },
                              diffuse: Color {
                                  r: 1f64,
                                  g: 0f64,
                                  b: 0.5f64,
                              },
                              specular: Color {
                                  r: 1f64,
                                  g: 1f64,
                                  b: 1f64,
                              },
                              shiny: 5,
                          }),
                      })],
        lights: vec![Box::new(Light {
                         pos: na::Point3::new(3f64, 3f64, 3f64),
                         intensity: Color {
                             r: 1f64,
                             g: 0.5f64,
                             b: 0.5f64,
                         },
                     })],
    };

    let camera = Camera::new(f64::consts::PI / 20f64,
                             3f64 / 2f64,
                             na::Point3::new(10f64, 0f64, 0f64),
                             na::Unit::new(&na::Vector3::new(-1f64, 0f64, 0f64)));

    render(&scene, &camera, (600, 400));

    println!("{:?}",
             trace(&scene,
                   Ray {
                       origin: na::Point3::new(2f64, 0f64, 0f64),
                       direction: na::Unit::new(&na::Vector3::new(-1f64, 0f64, 0f64)),
                   }));
}
