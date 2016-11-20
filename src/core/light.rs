use ::Vec3f;

pub struct Light {
    position: Vec3f,
    ambient: Vec3f,
    diffuse: Vec3f,
    intensity: f32
}

impl Light {
    pub fn new() -> Light {
        Light {
            position: Vec3f::new(0.0, 0.0, 0.0),
            ambient: Vec3f::new(0.0, 0.0, 0.0),
            diffuse: Vec3f::new(0.0, 0.0, 0.0),
            intensity: 1.0
        }
    }

    pub fn set_position(&mut self, position: Vec3f) {
        self.position = position;
    }

    pub fn ambient(&self) -> &Vec3f {
        &self.ambient
    }

    pub fn set_ambient(&mut self, ambient: Vec3f) {
        self.ambient = ambient;
    }

    pub fn diffuse(&self) -> &Vec3f {
        &self.diffuse
    }

    pub fn set_diffuse(&mut self, diffuse: Vec3f) {
        self.diffuse = diffuse;
    }

    pub fn intensity(&self) -> f32 {
        self.intensity
    }
}
