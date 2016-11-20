/**
 * Core modules
 */
mod material;
mod light;
mod scene;
mod ray;
mod image;

pub use self::ray::*;

pub use self::scene::Scene;

pub fn render(scene: &Scene, buffer: &mut [u8]) {
    unimplemented!();
}

pub fn set_sample_rate(rate: i32) {
    unimplemented!();
}
/**
 * Core exports
 */
pub use self::light::Light as Light;
pub use self::material::Material as Material;
