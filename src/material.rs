pub use ::types::Vec4f;

pub struct GMaterial {
    ambient: Vec4f,
    diffuse: Vec4f,
    specular: Vec4f,
    spec_power: f32,
    reflectivity: f32
}

impl GMaterial {
    pub fn new(ambient: Vec4f, diffuse: Vec4f, specular: Vec4f, spec_power: f32, reflectivity: f32) -> GMaterial {
        GMaterial {
            ambient: ambient,
            diffuse: diffuse,
            specular: specular,
            spec_power: spec_power,
            reflectivity: reflectivity
        }
    }
}

impl Material for GMaterial {
    fn ambient(&self) -> &Vec4f {
        &self.ambient
    }

    fn diffuse(&self) -> &Vec4f {
        &self.diffuse
    }

    fn specular(&self) -> &Vec4f {
        &self.specular
    }

    fn specular_power(&self) -> f32 {
        self.spec_power
    }

    fn reflectivity(&self) -> f32 {
        self.reflectivity
    }
}

pub trait Material {
    fn ambient(&self) -> &Vec4f;
    fn diffuse(&self) -> &Vec4f;
    fn specular(&self) -> &Vec4f;
    fn specular_power(&self) -> f32;
    fn reflectivity(&self) -> f32;
}
