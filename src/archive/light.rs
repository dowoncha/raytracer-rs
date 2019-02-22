use ::types::Vec3f;
use ::types::Color;

pub trait Light {
    /// Light direction
    /// v: Point to illuminate
    /// Returns the direction of light rays pointing from v
    fn direction(&self, v: Vec3f) -> Vec3f; 
    
    /// Light illumincation
    /// v: Point to illuminate
    /// Returns the color of the light at the pointi v
    fn illumination(&self, v: Vec3f) -> Color;
    
    /// Light shadow
    /// Line index of the closest shadow ray intesection
    /// Where the point is in shadow
    fn shadow(&self, t: f64) -> bool;
}

pub struct PointLight {
    position: Vec3f,
    color: Color,
}

impl PointLight {
    pub fn new(position: Vec3f, color: Color) -> Self {
        Self {
            position,
            color
        }
    }

    pub fn set_position(&mut self, position: Vec3f) {
        self.position = position;
    }

    fn position(&self) -> &Vec3f {
        &self.position
    }
}

impl Light for PointLight {
    fn direction(&self, v: Vec3f) -> Vec3f {
        self.position - v
    }
    
    fn illumination(&self, v: Vec3f) -> Color {
        self.color
    }
    
    fn shadow(&self, t: f64) -> bool {
        t < 1.0
    }
}