use clap::{arg, command, value_parser, ArgAction, ArgMatches, Command};
use std::path::PathBuf;

/// Creates command line arguments
///
/// Returns ArgMatches container for parse results
pub fn make_commands() -> ArgMatches {
    let matches = command!() // requires `cargo` feature
        .arg(
            arg!([filepath] "File path to image you want to edit")
                .required(true)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(
                -o --output <filepath> "Output filepath"
            )
            .value_parser(value_parser!(PathBuf))
            .help("Can be set to one of ImageFormat's values: Png, Jpeg, Gif, WebP, Pnm, Tiff, Tga, Dds,  Bmp, Ico, Hdr, OpenExr, Farbfeld, Avif, Qoi. But \x1b[31mascii\x1b[0m command produces files without any extension")
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
                .about("Blur the image with a given radius")
                .arg_required_else_help(true)
                .arg(
                    arg!(-r --blur_radius <VALUE>)
                        .required(true)
                        .value_parser(value_parser!(u32))
                        .action(ArgAction::Set),
                ),
        )
        .subcommand(Command::new("mirror").about("Mirror the image"))
        .subcommand(Command::new("flip_vertical").about("Flip the image vertically"))
        .subcommand(Command::new("rotate").about("Rotate an image 90 degrees clockwise"))
        .subcommand(Command::new("grayscale").about("Make the image grayscale"))
        .subcommand(
            Command::new("monochrome_ugly")
                .about("Make the image monochrome")
                .arg(
                    arg!(-t --threshold <VALUE>)
                        .default_value("128.0")
                        .value_parser(value_parser!(f32))
                        .action(ArgAction::Set),
                ),
        )
        .subcommand(
            Command::new("scale")
                .about("Scale the image")
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
                .about("Render image as an ASCII art with a given charset")
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
                ).arg(
                    arg!(-v --verbose_only <BOOL>)
                        .value_parser(value_parser!(bool))
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("curse").about("Curse the image")                
        )
        .subcommand(
            Command::new("zxc").about("Ultimate zxc dead inside the image")                
        )
        .get_matches();
    matches
}
