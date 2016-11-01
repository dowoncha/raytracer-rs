use std::collections::hash_map::HashMap;

use ::sims::consumeable::Consumeable;

pub enum EatingType {
    Herbivore,
    Carnivore,
    Omnivore
}

pub enum Attribute {
    Strength(f32),
    Agility(f32)
}

pub enum WorldSpeed {
    Day,
    Week,
    Month,
    Year
}

// pub trait Specie {
//     fn simulate(&mut self, dt: f32);
//     fn attributes(&self) -> &Vec<Attribute>;
//     fn aggressiveness(&self) -> f32;
//     fn consume(&mut self, food: f32);
// }

pub struct God;

pub struct WorldGenerator;
pub struct SpecieGenerator;

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

pub struct SpecieParams {
}

pub struct Specie {
    name: String,
    population: u64,
    eating_type: EatingType,
    food: Vec<Consumeable>,
    predators: Vec<Specie>
}

impl Specie {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn update(&mut self) {
        unimplemented!();
    }
}

impl God {
    pub fn generate_world(params: WorldParams) -> WorldGenerator {
        WorldGenerator { }
    }

    pub fn populate(world: &mut World, count: i32) {
        for i in 0..count {
            let specie = God::generate_specie().randomize().create();

            world.add_specie(specie);
        }
    }

    pub fn generate_specie() -> SpecieGenerator {
        SpecieGenerator { }
    }
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
        unimplemented!();
        // self.species.insert(&specie.name(), specie);
    }

    pub fn update(&mut self) {
        unimplemented!();
        for (name, specie) in &self.species {
        }
    }
}
