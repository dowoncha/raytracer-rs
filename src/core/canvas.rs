use ::common::bitmap::Bitmap;
use ::common::types::Mat3f;

pub struct Stroke {
    width: f32,                 // Width of stroke

    // Limiting ratio of diagonal length of am tier join to stroke width
    // If ratio exceeds limit, join will be drawn with bevel
    miter_limit: f32,
    add_cap: f32                // Adds a square cap at the ends
}

trait Canvas {
    fn save();
    fn restore();
    fn concat();
    fn translate(tx: f32, ty: f32);
    fn scale(sx: f32, sy: f32);
    fn rotate(radians: f32);
    fn clear();

    /* Drawing functions */
    fn fill_rect(rect: &Rect, color: &Color);
    fn fill_bitmap_rect(bmp: &Bitmap, rect: &Rect);

    fn shade_rect(rect: &Rect, shader: Box<Shader>);

    fn shade_convex_polygon(polygon: &Vec<Point>, shader: Box<Shader>);
}

pub struct GCanvas<'a> {
    bitmap: &'a Bitmap
    ctm: Mat3f              // Current transformation matrix
}

impl<'a> GCanvas<'a> {
    pub fn new(bitmap: &'a Bitmap) -> GCanvas {
        GCanvas {
            bitmap: ,
            ctm: na::new_identity(3)
        }
    }
}
