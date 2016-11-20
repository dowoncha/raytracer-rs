// @Author: Cha Dowon <dowon>
// @Date:   2016-11-20T11:01:12-05:00
// @Project: BeAM
// @Filename: world.rs
// @Last modified by:   dowon
// @Last modified time: 2016-11-20T11:02:12-05:00



use std::collections::hash_map::HashMap;

use ::sims::specie::Specie;
use ::sims::consumeable::Consumeable;

pub enum WorldSpeed {
    Day,
    Week,
    Month,
    Year
}

pub struct WorldGenerator;

impl SpecieGenerator {
    fn randomize<'a>(&'a mut self) -> &'a mut SpecieGenerator {
        self
    }

    fn create(&self) -> Specie {
        unimplemented!();
    }
}

pub enum Biome {
    Tundra,
    Grassland,
    Rainforest,
}

pub struct WorldParams {
    pub max_species: i32
}

pub struct World {
    params: WorldParams,
    speed_of_time: i32,
    species: HashMap<&'static str, Specie>
}

impl World {
    pub fn add_specie(&mut self, specie: Specie) {
    // self.species.insert(specie.name(), specie);
    }

    pub fn update(&mut self) {
        for (name, mut specie) in &mut self.species {
            specie.update();
        }
    }
}
