extern crate raytracer as rt;

use rt::sims::world::*;

fn main() {

    // Do graphics setup

    let params = WorldParams {
        max_species: 2
    };

    let mut world = God::generate_world(params);

    // God::populate(world, 20);

    loop {
        // world.update();
    }
}
