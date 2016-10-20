use ::core::{Ray, Material};

pub trait Surface {
    fn intersect(ray: &Ray, tMax: f32) -> bool;
}

pub struct Sphere {
    position: Vec3f,
    radius: f32,
    radius2: f32,        // radius * radius
    material: &'a Material
}
