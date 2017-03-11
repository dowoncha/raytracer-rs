#[macro_use]
extern crate clap;
extern crate raytracer as rt;
extern crate image;

use std::process;
use rt::{RenderConfig};
use clap::{Arg, SubCommand, ArgMatches};

fn run_render(matches: &ArgMatches) -> Result<(), String> {
    println!("Starting render");

    // Create the raytracer
    let app = rt::App::new();

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

    let render_config = RenderConfig {
        scene_name: "spheres",
        output_name: "spheres",
        width: width,
        height: height,
        sample_mode: rt::SampleMode::None,
        format: image::ImageFormat::PPM
    };

    let image_buffer = app.render(render_config);

    Ok(())
}

/**
 * Handles cli subcommands
 * Matches argument subcommands and calls respective function
 */ 
fn run(matches: ArgMatches) -> Result<(), String> {
    println!("Running");

    match matches.subcommand() {
       ("render", Some(sub_m)) => run_render(sub_m),
       _ => Ok(())
   }
}

/**
 * main CLI
 * Commands:
 * new-scene create a new scene
 * load-scene
 * render the currently loaded scene
 */
fn main() {
    let matches = clap::App::new("Raytracer")
        .version("1.0")
        .author("Dowon C. <dowon1014@gmail.com>")
        .subcommand(SubCommand::with_name("new")
            .about("Create a new project at <path>")
            .arg(Arg::with_name("path")
                .required(true))
            // .arg(Arg::with_name("name")
            )
        .subcommand(SubCommand::with_name("render")
            .about("Render the current scene")
            .arg(Arg::with_name("debug")
                .short("d")
                .long("debug")
                .help("Enable debug mode"))
            .arg(Arg::with_name("width")
                .short("W")
                .long("width")
                .help("Width of image (default=800)"))
            .arg(Arg::with_name("height")
                .short("H")
                .long("height")
                .help("Height of image (default=600)"))
            .arg(Arg::with_name("SCENE")
                .required(true)
                .help("Scene file to render")
            )
        )
        .subcommand(SubCommand::with_name("new-scene")
            .help("Create a new scene")
            .arg(Arg::with_name("NAME")
                .required(true)
            )
        )
        .get_matches();

    if let Err(e) = run(matches) {
        println!("Raytracer error: {}", e);
        process::exit(1);
    }
}