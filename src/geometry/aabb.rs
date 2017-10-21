use types::{Vec3f};
use ray::Ray;

struct AABB {
    min: Vec3f,
    max: Vec3f
}

impl AABB {
    pub fn new(min: Vec3f, max: Vec3f) -> Self {
        AABB {
            min,
            max
        }
    }
    
    /// Bounding box which contains nothing
    pub fn zero() -> Self {
        AABB {
            min: Vec3f::new(std::f32::INFINITY, std::f32::INFINITY, std::f32::INFINITY),
            max: Vec3f::new(std::f32::NEG_INFINITY, std::F32::NEG_INFINITY, std::F32::NEG_INFINITY)
        }
    }
    
    /// Bounding box which contains everything
    pub fn infinite() -> Self {
        Self {
            min: Vec3f::new(std::f32::NEG_INFINITY, std::f32::NEG_INFINITY, std::f32::NEG_INFINITY),
            max: Vec3f::new(std::f32::INFINITY, std::f32::INFINITY, std::f32::INFINITY)
        }
    }
   
    /// Return whether p is within the aabb
    pub fn contains(&self, p: &Vec3f) -> bool {
        if p.x < self.min.x || p.y < self.min.y || p.z < self.min.z
            || p.x > self.max.x || p.y > self.max.y || p.z > self.max.z {
                
            return false;
        }
        
        true
    }
   
    pub fn is_infinite(&self) -> bool {
        self.min.x.is_infinite()
    }
}