use ::types::{Mat4f, Vec3f};

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    position: Vec3f,
    direction: Vec3f
}

impl Ray {
    pub fn new(position: Vec3f, direction: Vec3f) -> Ray {
        Ray {
            position,
            direction
        }
    }

    pub fn transform(self, transform: Mat4f) -> Self {
        unimplemented!()
    }

    pub fn evaluate(&self, t: f32) -> Vec3f {
        self.position + self.direction * t
    }

    pub fn position(&self) -> &Vec3f {
        &self.position
    }

    pub fn direction(&self) -> &Vec3f {
        &self.direction
    }
}
