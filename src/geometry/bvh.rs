use geometry::triangle::Triangle;
use geometry::aabb::AABB;

use std::rc::Rc;

struct BvhNode {
    left: Box<Option<KdNode>>,
    right: Box<Option<KdNode>>,
    object: Rc<Object>
    aabb: AABB
}

impl KdNode {
    pub fn new() -> KdNode {
        KdNode {
            left: Box::new(None),
            right: Box::new(None),
            triangles: Vec::new()
        }
    }
    
    /// Build the tree start at root,
    /// Root contains all triangles in the object and bbox surrounding the whole thing
    /// At each level down, split on a different axis
    /// For each level of the tree
    /// 1. Find the midpoint of all the triangles in the node
    /// 2. Find the longest axis of the bbox for that node
    /// 3. For each triangle check whether for the current axis it is less than or greater than the overall midpoint
    
    pub fn build(&self, tris: &mut Vec<&Triangle>, depth: i32) -> KdNode {
        let node = KdNode::new();
        node.triangles = tris.clone();
    }
}