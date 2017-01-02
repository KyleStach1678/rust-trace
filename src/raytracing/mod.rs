mod ray;
pub use self::ray::Ray;

mod trace;
pub use self::trace::*;

mod material;
pub use self::material::Material;

mod scene;
pub use self::scene::*;

mod camera;
pub use self::camera::Camera;

mod render;
pub use self::render::render;

mod color;
pub use self::color::Color;

pub mod shapes;
pub mod materials;
