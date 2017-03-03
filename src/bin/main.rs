#[macro_use]
extern crate clap;
extern crate raytracer as rt;
extern crate image;

use std::process;
use rt::{Scene, RenderConfig};
use clap::{Arg, SubCommand, ArgMatches};

fn run_render(matches: &ArgMatches) -> Result<(), String> {
    Ok(())
}

/**
 * run
 * Owns the clap app
 */
fn run(matches: ArgMatches) -> Result<(), String> {
    match matches.subcommand() {
       ("render", Some(sub_m)) => run_render(sub_m),
       _ => Ok(())
   }
}

/**
 * main
 * Commands:
 * new-scene create a new scene
 * load-scene
 * render the currently loaded scene
 */
fn main() {
    let matches = clap::App::new("Raytracer")
        .version("1.0")
        .author("Dowon C. <dowon1014@gmail.com>")
        .subcommand(SubCommand::with_name("render")
            .help("Render the current scene")
            .arg(Arg::with_name("debug")
                .short("d")
                .long("debug")
                .help("Enable debug mode"))
            .arg(Arg::with_name("workers"))
            .arg(Arg::with_name("config"))
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
        .subcommand(SubCommand::with_name("scenes")
            .help("List all scenes")
            .arg(Arg::with_name("FILE")
                .required(true)
            )
        )
        .get_matches();

    if let Err(e) = run(matches) {
        println!("Raytracer error: {}", e);
        process::exit(1);
    }
}
