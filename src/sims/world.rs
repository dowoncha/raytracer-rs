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

pub enum Biome {
    Tundra,
    Grassland,
    Rainforest,
}

pub struct WorldParams {
    max_species: i32
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
