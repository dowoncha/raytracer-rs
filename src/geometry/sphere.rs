use ::types::Vec3f;
use ::Material;

pub struct Sphere<'a> {
    position: Vec3f,
    radius: f32,
    radius2: f32,        // radius * radius
    material: &'a Material
}
