use std::cmp;

pub fn clamp<T: cmp::Ord>(min: T, value: T, max: T) -> T {
    cmp::max(min, cmp::min(value, max))
}

/**
 * pin_to_unit
 * Normalize functoin
 */
pub fn pin_to_unit(x: f32) -> f32 {
    // cmp::max(0.0, cmp::min(1.0, x))
    0.0
}

pub fn floor_clamp(value: f32) -> f32 {
    value - value.floor()
}

/**
 * lerp
 * Linear interpolation function
 */
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    (1.0 - t) * a + t * b
}

/**
 * Efficiently convert a float into a u8
 */
pub fn float_to_byte(float: f32) -> u8 {
    0
    // let isx = (i32)(i * 255.99999f32);
}
