use ::types::Vec3f;
use ::Material;
use ::ray::Ray;

use ::HitResult;

pub trait Surface {
    fn intersect(&self, ray: &Ray) -> Option<HitResult>;
    fn material(&self) -> &str;
    fn set_material(&mut self, material: &str);
    fn normal(&self) -> Vec3f;
}
