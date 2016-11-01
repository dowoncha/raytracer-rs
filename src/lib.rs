#![allow(dead_code)]
#![allow(unused_variables)]

/**
 * Main library start point
 */


extern crate nalgebra as na;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

// pub use ::core::Engine as Engine;
pub use ::core::material::Material as Material;
pub use ::core::light::Light as Light;
pub use ::core::scene::Scene as Scene;

pub use ::common::types as types;
pub use ::geometry::Surface as Surface;

mod core;
mod common;
mod particles;
mod geometry;

pub mod sims;

use std::collections::HashMap;
use ::core::image::Image as Image;
use types::Vec4f;

pub struct Engine<'a> {
    scene: Box<Scene>,
    materials: HashMap<&'a str, &'a Material>
}

impl<'a> Engine<'a> {
    fn new() -> Engine<'a> {
        Engine {
            scene: Box::new(Scene::new()),
            materials: HashMap::new()
        }
    }

    fn render<I: Image>(&self, image: &mut I) {
        println!("Rendering");
    }

    fn trace(depth: i32) -> Vec4f {
        Vec4f::new(0.0, 0.0, 0.0, 0.0)
    }
}
