use ::Vec3f;
use ::Material;
use ::core::Ray;

pub trait Surface {
    fn intersect(&self, ray: &Ray, hit: &mut HitContext) -> bool;
    fn material(&self) -> &Material;
    fn set_material(&mut self, material: Box<Material>);
    fn normal(&self) -> Vec3f;
}

pub struct HitContext {
}
