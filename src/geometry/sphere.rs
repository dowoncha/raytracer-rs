use ::types::Vec3f;
use ::Material;
use ::{Surface, HitContext};
use ::ray::Ray;

pub struct Sphere {
    position: Vec3f,
    radius: f32,
    radius2: f32,        // radius * radius
    material: String
}

impl Sphere {
    pub fn new(position: Vec3f, radius: f32, material: &str) -> Sphere {
        Sphere {
            position: position,
            radius: radius,
            radius2: radius * radius,
            material: String::from(material)
        }
    }
}

impl Surface for Sphere {
    fn intersect(&self, ray: &Ray, hit: &mut HitContext) -> bool {
        unimplemented!();
    }

    fn material(&self) -> &str {
        self.material.as_ref()
    }

    fn set_material(&mut self, material: &str) {
        self.material = String::from(material);
    }

    fn normal(&self) -> Vec3f {
        unimplemented!();
    }
}
