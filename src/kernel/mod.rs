// Intersection
// EXPORTS
struct PathRadiance { }

impl PathRadiance {
    fn new() -> Self {
        Self {

        }
    }
}

struct PathState { }

impl PathState {
    fn new() -> Self {
        Self {

        }
    }
}

pub struct KernelGlobals {
    /* textures */
    // name: texture<type>

    /* if osl */

    /* Run-time data */
    // transparent_shadow_intersection: Box<Intersection>,

    /* split kernel */

}

impl KernelGlobals {
    pub fn new() -> KernelGlobals {
        KernelGlobals {

        }
    }
}

pub use self::KernelGlobals as Globals;

fn kernel_path_trace_setup(
    kg: &KernelGlobals,
    sample: i32,
    x: i32,
    y: i32,
    /* rng_hash */
) {

}

fn kernel_path_scene_intersect(
    kg: &KernelGlobals, 
    state: &PathState, 
    ray: &Ray, 
    L: &PathRadiance
    ) -> (bool, Intersection) {
        
        let intersection = Intersection {

        };

        (true, intersection)
    }

fn kernel_path_integrate(
    kg: &KernelGlobals,
    state: &PathState,
    throughput: &[f32; 3],
    ray: &Ray,
    L: &PathRadiance,
    buffer: &[f32]
    // emission_sd: ShaderData
    ) {
    /* path iteration */ 
    loop {

        let (hit, intersection) = kernel_path_scene_intersect(kg, state, ray, L);

        // Find intersection with lamps and compute emission
        // kernel_path

        // If cfg(volume) integration

        /* Shade background */
        if !hit {
            // kernel_path_background()
        } else /*if path_state_ao_bounce()*/ {
            break;
        }

        /* Setup shader data */
        // shader_setup_from_ray()

        // evaluate Shader
    }
}

pub fn kernel_path_trace(kg: &KernelGlobals,
    buffer: &[f32],
    sample: i32,
    x: i32,
    y: i32,
    offset: i32,
    stride: i32) {

    /* buffer offset */
    let index = offset + x + y * stride;

    // Notes: where is kernel_data coming from?
    let pass_stride = 1; // kernel_data.fil.pass_stride;

    let buffer = &buffer[(index * pass_stride) as usize..];

    /* Initialize random numbers and sample ray */
    let ray = kernel_path_trace_setup(kg, sample, x, y);

    /* Check ray contact time */

    /* Initialize state. */
    let throughput = &[1.0, 1.0, 1.0];

    /* Shading */
    let L = PathRadiance::new(/* kernel_data.film.use_light_pass */);

    /* Shader Data Storage */

    /* Initialize path state */
    let state = PathState::new(kg, emission_sd, rng_hash, sample, &ray);

    /* That sweet calculus */
    kernel_path_integrate(kg, &state, throughput, ray, &L, buffer);

    /* Let's write the result */
    // kernel_write_result(kg, buffer, sample, L);
}

pub use self::kernel_path_trace as path_trace;