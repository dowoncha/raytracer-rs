/**
 * Software rendering of spheres with ray tracing
 */

extern crate raytracer;

use raytracer::core::{Material};

fn main() {
    // Make materials
    let mat_red = Material::new();

    let sphere1 = Sphere::new()
        .position(0.0, 0.0, -7.0)
        .radius(2.0)
        .material(mat_red);
}
