use ::types::Vec3f;
use ::Material;
use ::ray::Ray;

use ::HitContext;

pub trait Surface {
    fn intersect(&self, ray: &Ray, hit: &mut HitContext) -> bool;
    fn material(&self) -> &str;
    fn set_material(&mut self, material: &str);
    fn normal(&self) -> Vec3f;
}
