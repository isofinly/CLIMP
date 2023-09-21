use clap::{arg, command, value_parser, ArgAction, ArgMatches, Command};
use std::path::PathBuf;

pub fn make_commands() -> ArgMatches {
    let matches = command!() // requires `cargo` feature
        .arg(
            arg!([filepath] "File path")
                .required(true)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(
                -o --output <filepath> "Output filepath"
            )
            .value_parser(value_parser!(PathBuf))
            .action(ArgAction::Set),
        )
        .subcommand(
            Command::new("pixelate")
                .about("Pixelate the image with a given pixel size")
                .arg_required_else_help(true)
                .arg(
                    arg!(-p --pixel_size <VALUE>)
                        .required(true)
                        .value_parser(value_parser!(u32))
                        .action(ArgAction::Set),
                ),
        )
        .subcommand(
            Command::new("blur")
                .about("Blurs the image with a given radius")
                .arg_required_else_help(true)
                .arg(
                    arg!(-r --blur_radius <VALUE>)
                        .required(true)
                        .value_parser(value_parser!(u32))
                        .action(ArgAction::Set),
                ),
        )
        .subcommand(Command::new("mirror").about("Mirrors the image"))
        .subcommand(Command::new("flip_vertical").about("Flips the image vertically"))
        .subcommand(Command::new("rotate").about("Rotates the image 90 degrees"))
        .subcommand(
            Command::new("scale")
                .about("Scales the image")
                .arg_required_else_help(true)
                .arg(
                    arg!(-s --scale <VALUE>)
                        .required(true)
                        .value_parser(value_parser!(u32))
                        .action(ArgAction::Set),
                ),
        )
        .subcommand(
            Command::new("ascii")
                .about("Renders image as an ASCII art with a given charset")
                .arg(
                    arg!(--width <VALUE>)
                        .value_parser(value_parser!(u32))
                        .action(ArgAction::Set),
                )
                .arg(
                    arg!(--height <VALUE>)
                        .value_parser(value_parser!(u32))
                        .action(ArgAction::Set),
                )
                .arg(
                    arg!(--colored <BOOL>)
                        .value_parser(value_parser!(bool))
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    arg!(--invert <BOOL>)
                        .value_parser(value_parser!(bool))
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    arg!(--charset <SET>)
                        .value_parser(value_parser!(String))
                        .action(ArgAction::Set),
                ),
        )
        .get_matches();
    matches
}
