// #![feature(proc_macro)]
#![allow(dead_code)]
#![allow(unused_variables)]
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
mod geometry;
