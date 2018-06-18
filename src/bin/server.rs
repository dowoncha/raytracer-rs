#[macro_use]
extern crate clap;
use clap::{Arg, SubCommand, ArgMatches};

#[macro_use]
extern crate log;

extern crate raytracer as rt;
use rt::{Ray};

extern crate image;
extern crate error_chain;

use std::cell::RefCell;

struct Object {
}

struct Material {

}

struct Intersection {
    object: Option<&'static Object>
}

struct Scene {
    objects: Vec<Object>,
    materials: Vec<Material>,
    pub camera: Camera
}

impl Scene {
    pub fn intersection(&self) -> Option<Intersection> {
        Some(Intersection {
            object: None
        })
    }
}

struct Camera;

impl Camera {
    fn width(&self) -> usize {
        0
    }

    fn height(&self) -> usize {
        0
    }
}

struct Renderer<'s> {
    scene: &'s Scene
} 

impl<'s> Renderer<'s> {
    fn new(scene: &'s Scene) -> Renderer {
        Renderer {
            scene
        }
    } 

    fn render_frame(&self) -> Vec<u32> {
        let camera_width: usize = self.scene.camera.width();
        let camera_height: usize = self.scene.camera.height();

        let mut frame_buffer = Vec::with_capacity(camera_width * camera_height); 

        for y in 0..camera_height {
            for x in 0..camera_width {
                let dx = 0.0;
                let dy = 0.0;

                // Fix Ray
                // let ray = Ray::new(x as f32, y as f32, dx, dy);

                // if let Some(intersection) = self.scene.intersect(ray) {
                //     if let Some(object) = intersection.object {
                //         // Compute illumination
                //         // direction = lightPos - intersection.hit
                //         let shadow_ray = Ray::new(0.0, 0.0, 0.0, 0.0);
                        
                //         /* Shading */
                //         frame_buffer.push(255);
                //     }
                // } else {
                //     frame_buffer.push(0);
                // }
            }
        }

        frame_buffer
    }
}

fn run_render(matches: &ArgMatches) -> Result<(), String> {
    println!("Starting render");

    /**
     * width
     * Parse width or default to 800
     */
    let width: u32 = if let Some(str_width) = matches.value_of("width") {
        str_width.parse().unwrap()
    } else { 800 };

    /**
     * height
     * If a height is given, parse into a u32
     * or default to 600
     */
    let height: u32 = if let Some(str_height) = matches.value_of("height") {
        str_height.parse().unwrap()
    } else { 600 };
    

    Ok(())
}

/**
 * Handles cli subcommands
 * Matches argument subcommands and calls respective function
 */
fn run(matches: ArgMatches) -> Result<(), String> {
    // Find matching device

    // Initialize Task scheduler

    loop {
        // Stats
        // let device = Box::new(Device::create(device_info, stats, true))
        info!("Render server with device: {}", "CPU" /* device.info.description()*/)
        // device.server_run();
        // drop device;
    }

    // Exit task scheduler

    Ok(())
}

fn main() {
    let matches = clap::App::new("Raytracer")
        .version("1.0")
        .author("Dowon C. <dowon1014@gmail.com>")
        .args_from_usage(
            "--device=[DEVICENAME] 'Devices to use: '
            --list-devices 'List information about all available devices'
            --threads=[THREADS] 'Number of threads to use for CPU device'
            --debug 'Enable debug logging'
            -v... 'Set logger verbosity'"
        )
        .get_matches();

    if let Err(ref e) = run(matches) {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";
        
        writeln!(stderr, "error: {}", e).expect(errmsg);
        
        // for e in e.iter().skip(1) {
        //     writeln!(stderr, "caused by: {}", e).expect(errmsg);
        // }
        
        // // Backtrace
        // if let Some(backtrace) = e.backtrace() {
        //     writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        // }
        
        std::process::exit(1);
    }
}
