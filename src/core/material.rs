extern crate nalgebra as na;
use na::{Vector3};

type Vec3f = Vector3<f32>;

pub struct Material {
    ambient: Vec3f,
    diffuse: Vec3f,
    specular: Vec3f,
    specular_pow: f32,
    reflectivity: f32
}

impl Material {
    pub fn new() -> Material {
        Material {
            ambient: Vec3f::new(0.0, 0.0, 0.0),
            diffuse: Vec3f::new(0.0, 0.0, 0.0),
            specular: Vec3f::new(0.0, 0.0, 0.0),
            specular_pow: 0.0,
            reflectivity: 0.0
        }
    }
}
