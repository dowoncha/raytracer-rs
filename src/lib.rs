// @Author: Cha Dowon <dowon>
// @Date:   2016-11-20T11:01:12-05:00
// @Project: BeAM
// @Filename: lib.rs
// @Last modified by:   dowon
// @Last modified time: 2016-11-20T11:03:21-05:00



// #![feature(proc_macro)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
/**
 * Disable compiler warnings
 */

// Nalgebra for math
extern crate nalgebra as na;
/**
 * Serialization crate
 */
#[cfg(feature="serde_derive")]
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate time;
extern crate rand;
// mod nalgebra;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

// pub use ::core::Engine as Engine;
pub use ::core::material::Material as Material;
pub use ::core::light::Light as Light;
pub use ::core::scene::Scene as Scene;

/**
 * Public exports
 */
pub use core::*;
pub use geometry::*;
pub use common::*;
pub use engine::*;

/**
 * Library modules
 */
mod core;
mod common;
mod engine;
mod player;
mod particles;
pub mod geometry;

pub mod sims;
