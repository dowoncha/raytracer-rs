use na::{Dot};
use types::{Vec3f, Color, TColor};

trait Diffuse {
    fn diffuse(&self,
                light: Color, color: Color,
                ray: &Vec3f, normal: &Vec3f) -> Color 
    {
        Color::black()
    }
}

trait Specular {
    fn specular(
        &self, light: Color, color: Color, 
        ray: &Vec3f, normal: &Vec3f, viewer: &Vec3f) -> Color 
    {
        
        Color::black()
    }
}

trait Reflection {
    fn reflection(&self) -> Color {
        Color::black()
    }
}

pub trait Finish: Specular + Diffuse + Reflection {}

struct Lambertian {
    
}

impl Lambertian {
    pub fn new(diffuse: f64) -> Lambertian {
        Lambertian {
            
        }
    }
}

struct Phong {
    coeff: f64,
    exp: f64
}

impl Phong {
    fn new(specular: f64, exp: f64) -> Phong {
        Phong {
           coeff: specular,
           exp
        }
    }
}

impl Specular for Phong {
   fn specular(&self, light: Color, color: Color, ray: &Vec3f, normal: &Vec3f, viewer: &Vec3f) -> Color {
       let proj = (2.0 * ray.dot(normal)) * normal;
       let reflected = proj - ray;
       
       let mut specular_factor = reflected.dot(viewer) as f64;
       
       if specular_factor < 0.0 {
           return Color::black();
       }
       
       specular_factor = specular_factor.powf(self.exp);
       light.scalar(self.coeff * specular_factor) 
    }
}

struct BasicReflection {
    
}

impl BasicReflection {
    pub fn new(min: Color, max: Color, falloff: f64) -> BasicReflection {
        BasicReflection {
            
        }
    }
}