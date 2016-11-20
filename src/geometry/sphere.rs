use ::Vec3f;
use ::Material;

pub struct Sphere<'a> {
    position: Vec3f,
    radius: f32,
    radius2: f32,        // radius * radius
    material: &'a Material
}

impl<'a> Sphere<'a> {
    pub fn new(position: Vec3f, radius: f32, material: &'a Material) -> Sphere {
        Sphere {
            position: position,
            radius: radius,
            radius2: radius * radius,
            material: material
        }
    }
}
