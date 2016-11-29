use ::types::Vec3f;

#[derive(Clone)]
pub struct Ray {
    position: Vec3f,
    direction: Vec3f
}

impl Ray {
    pub fn new(position: Vec3f, direction: Vec3f) -> Ray {
        Ray {
            position: position,
            direction: direction
        }
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
