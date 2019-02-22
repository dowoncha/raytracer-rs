use std::sync::Arc;

use ::types::{Vec3f, Mat4f};
use ::Material;
use ::ray::Ray;

pub struct Intersection {
    ray: Ray,               // Ray that intersected
    pub time: f64,
    normal: Vec3f,
    object: Arc<Object> 
}

pub struct ObjectData {
    // texture
    transformation: Mat4f,
    inv_trans: Mat4f
}

pub trait Object {
    /// Return whether ray intersected object
    fn intersection(&self, ray: &Ray) -> Option<Intersection> {
        let internal = self.internal();
        // 1. Transform the ray by the inverse transformation
        let transformed = ray.transform(internal.inv_trans);
        
        // if let Some(intersection) = self.inters
        
        None
    }
    
    /// Return whether point is inside object
    fn is_inside(&self, point: &Vec3f) -> bool;

    fn internal(&self) -> &ObjectData;
    // A world coordinate bounding box computed for the object
    // fn bounding_box(&self, transformation: Mat4f) -> AABB;

    // fn texture
    // fn interior
    // fn transformation
    // fn children
    // pre computed
    // trans_inverse
    // aabb
}
