use ::types::Vec3f;
use ::Material;
use ::core::ray::Ray;

pub use self::sphere::Sphere as Sphere;
pub use self::plane::Plane as Plane;
pub use self::mesh::Mesh as Mesh;
pub use self::triangle::Triangle as Triangle;

mod sphere;
mod plane;
mod mesh;
mod triangle;

pub trait Surface {
    fn intersect(&self, ray: &Ray, hit: &mut HitContext) -> bool;
    fn material(&self) -> &Material;
    fn set_material(&mut self, material: Box<Material>);
    fn normal(&self) -> Vec3f;
}

pub struct HitContext {
}
