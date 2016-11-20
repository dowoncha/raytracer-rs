/**
 * filename: engine.rs
 * author: Dowon Cha
 * description: Game engine
 * This should be starting point for engine.
 *
 */

use time;
use ::Vec3f;
use ::player::Player;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;

pub trait GameObject {
    fn position(&self) -> &Vec3f;
    fn update(&mut self, delta: f32);
    fn render(&self);
}

pub struct Texture {

}

pub struct Tile {

}

struct World {
    player: Box<Player>,
    map: Vec<&'static Tile>,
    tiles: HashMap<&'static str, Tile>,
    objects: Vec<Box<GameObject>>
}

pub struct Engine {
    world: Box<World>
    // renderer: Box<Renderer>
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            world: Box::new(World::new())
            // renderer: Box::new()_
        }
    }

    pub fn init(&mut self) {
        println!("Initializing engine");
    }

    pub fn run(&mut self) {
        let mut last_time = time::precise_time_ns();

        loop {
            let current = time::precise_time_ns();
            let elapsed = current - last_time;

            // process_input();

            self.update(elapsed);

            self.render();

            last_time = current;
        }
    }

    fn update(&mut self, delta: f32) {

    }

    fn render(&mut self) {
    }

    fn new_game(&mut self) {

    }

    fn load_game(&mut self) {

    }

    fn save_game(&self) {

    }

    fn load_config() {

    }

    fn save_config(&self, filename: String) {

    }
}
