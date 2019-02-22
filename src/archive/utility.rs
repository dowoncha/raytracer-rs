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

/**
 * Number theory functions
 */

use std::ops::Rem;
// use std::core::cmp::PartialEq;-

/**
 * check_even
 * More efficient % 2
 */
pub fn check_even<T: Rem + PartialEq>(n: T) -> bool {
    // (n % 2) as T == 0
    false
}

/**
 * gcd
 * Calculate the gcd of two integers,
 * Binary method
 * Returns g*2^d
 */
pub fn gcd(mut a: u64, mut b: u64) -> (u64, u64){
    let mut d = 0;
    while a % 2 == 0 && b & 2 == 0 {
        a = a / 2;
        b = b / 2;
        d += 1;
    }

    while a != b {
        if check_even(a) {
            a /= 2;
        } else if check_even(b) {
            b /= 2;
        } else if a > b {
            a = (a - b) / 2;
        } else {
            b = (b - a) / 2;
        }
    }

    (a, d)
}


#[cfg(test)]
mod tests {
    use super::mul_mod;

    #[test]
    fn mul_mod_test() {
        assert_eq!(2, mul_mod(2, 2, 5));
    }
}

/**
 * mul_mod
 * Computes a * b (mod m)
 * why did I write this?
 */
pub fn mul_mod(mut a: u64, mut b: u64, mut m: u64) -> u64 {
    let mut out = 0u64; // final result
    let mp2 = m >> 1;

    if a >= m {
        a %= m;
    }

    if b >= m {
        b %= m;
    }

    /**
     * voodoo magic
     */
    for i in 0..64 {
        out = if out > mp2 { (out << 1u64) - m } else { out << 1u64 };

        if a & 0x8000000000000000u64 != 0 {
            out += b;
        }
        if d > m {
            out -= m;
        }
        a <<= 1;
    }

    out
}
