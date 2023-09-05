use clap::{arg, command, value_parser, ArgAction, Command};
// use dialoguer::{theme::ColorfulTheme, Select};
use image::imageops;
use image::io::Reader as ImageReader;
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};
use indicatif::{ProgressBar, ProgressStyle};
use std::error::Error;
use std::path::PathBuf;
use std::time::Duration;

type Image = ImageBuffer<Rgba<u8>, Vec<u8>>;
struct Args {
    filename: PathBuf,
    radius: Option<u32>,
    pixel: Option<u32>,
    resize: Option<u32>,
    output: Option<PathBuf>,
    file_ext: Option<String>,
}

impl Args {
    fn format_output_name(&self) -> PathBuf {
        let output_name = self
            .output
            .as_ref()
            .unwrap_or(&self.filename)
            .with_extension(self.file_ext.as_deref().unwrap_or("jpg"));
        output_name
    }
}

fn rotate(img: &Image) -> Image {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.green} {msg}")
            .unwrap()
            .tick_strings(&[
                "[    ]", "[=   ]", "[==  ]", "[=== ]", "[ ===]", "[  ==]", "[   =]", "[    ]",
                "[   =]", "[  ==]", "[ ===]", "[====]", "[=== ]", "[==  ]", "[=   ]",
            ]),
    );
    let out_img = imageops::rotate90(img);
    out_img
}

fn resize(img: &Image, new_dims: (u32, u32)) -> Image {
    let (old_width, old_height) = img.dimensions();
    let (new_width, new_height) = new_dims;

    let mut resized = ImageBuffer::new(new_width, new_height);

    for (new_x, new_y, pixel) in resized.enumerate_pixels_mut() {
        let old_x = (new_x as f32 * (old_width as f32 / new_width as f32)) as u32;
        let old_y = (new_y as f32 * (old_height as f32 / new_height as f32)) as u32;

        if let Some(old_pixel) = img.get_pixel_checked(old_x, old_y) {
            *pixel = *old_pixel;
        } else {
            println!("({old_x} -> {new_x}, {old_y} -> {new_y})");
        }
    }

    resized
}

fn pixelate(img: &DynamicImage, new_dims: (u32, u32)) -> Image {
    let old_dims = img.dimensions();

    let img = img.to_rgba8();

    let small = resize(&img, ((old_dims.0 / new_dims.0), (old_dims.1 / new_dims.1)));

    let pixelated = resize(&small, old_dims);
    pixelated
}

fn blur(img: &Image, radius: u32) -> Image {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.green} {msg}")
            .unwrap()
            // For more spinners check out the cli-spinners project:
            // https://github.com/sindresorhus/cli-spinners/blob/master/spinners.json
            .tick_strings(&[
                "[    ]", "[=   ]", "[==  ]", "[=== ]", "[ ===]", "[  ==]", "[   =]", "[    ]",
                "[   =]", "[  ==]", "[ ===]", "[====]", "[=== ]", "[==  ]", "[=   ]",
            ]),
    );
    pb.set_message("\x1b[33mBlurring...\x1b[0m");
    let img = imageops::blur(img, radius as f32);
    pb.finish_with_message("\x1b[32mDone:\x1b[0m");
    img
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut args = Args {
        filename: PathBuf::from(""),
        radius: None,
        pixel: None,
        output: None,
        resize: None,
        file_ext: None,
    };
    let matches = command!() // requires `cargo` feature
        .arg(
            arg!([filename] "File path")
                .required(true)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(
                -o --output <FILENAME> "Output filename"
            )
            .value_parser(value_parser!(PathBuf))
            .action(ArgAction::Set),
        )
        .subcommand(
            Command::new("pixelate")
                .about("Pixelates the image with a given pixel size")
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
        .get_matches();

    if let Some(path) = matches.get_one::<PathBuf>("filename") {
        args.filename = path.clone();
        let file_ext_name = path
            .clone()
            .to_str()
            .unwrap()
            .to_string()
            .split(".")
            .last()
            .unwrap()
            .to_string();
        args.file_ext = Some(".".to_string() + &file_ext_name);
    }

    if let Some(name) = matches.get_one::<PathBuf>("output") {
        args.output = Some(name.clone());
    } else {
        args.output = Some(
            matches
                .get_one::<PathBuf>("filename")
                .unwrap()
                .to_path_buf(),
        );
    }

    if let Some(matches) = matches.subcommand_matches("blur") {
        if let Some(r) = matches.get_one::<u32>("blur_radius") {
            args.radius = Some(r.clone());
            let img_result = blur(
                &ImageReader::open(args.filename.clone())?
                    .decode()?
                    .into_rgba8(),
                *r,
            );
            let _ = img_result.save(args.format_output_name());
        }
    }

    if let Some(matches) = matches.subcommand_matches("pixelate") {
        let img = ImageReader::open(args.filename.clone())?.decode()?;
        if let Some(s) = matches.get_one::<u32>("pixel_size") {
            args.pixel = Some(s.clone());
            let img_result = pixelate(&img, (*s, *s));
            let _ = img_result.save(args.format_output_name());
        }
    }

    if let Some(matches) = matches.subcommand_matches("scale") {
        let img = ImageReader::open(args.filename.clone())?.decode()?;
        if let Some(s) = matches.get_one::<u32>("scale") {
            args.resize = Some(s.clone());
            let img_result = resize(&img.into_rgba8(), (*s, *s));
            let _ = img_result.save(args.format_output_name());
        }
    }

    if let Some(matches) = matches.subcommand_matches("rotate"){
        let img_result = rotate(&ImageReader::open(args.filename.clone())?.decode()?.into_rgba8());
        let _ = img_result.save(args.format_output_name());
    }

    if let Some(matches) = matches.subcommand_matches("mirror") {
        let img = ImageReader::open(args.filename.clone())?.decode()?;
        let img_result = img.fliph();
        let _ = img_result.save(args.format_output_name());
    }

    if let Some(matches) = matches.subcommand_matches("flip_vertical") {
        let img = ImageReader::open(args.filename.clone())?.decode()?;
        let img_result = img.flipv();
        let _ = img_result.save(args.format_output_name());
    }

    Ok(())
}
