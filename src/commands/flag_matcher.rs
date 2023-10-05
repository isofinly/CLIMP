use crate::ascii;
use clap::ArgMatches;
use image::io::Reader as ImageReader;
use std::error::Error;
use std::io;
use std::path::PathBuf;
use unicode_segmentation::UnicodeSegmentation;

use super::{blur, grayscale, monochrome_ugly, pixelate, resize, rotate, Args};
use ascii::{from_str, render, RenderOptions};

impl Args {
    pub fn match_flags(mut args: Args, matches: ArgMatches) -> Result<(), Box<dyn Error>> {
        if let Some(path) = matches.get_one::<PathBuf>("filepath") {
            let file_ext_name = std::path::Path::new(&path)
                .extension()
                .and_then(std::ffi::OsStr::to_str)
                .unwrap_or("jpg")
                .to_string();
            args.filepath = PathBuf::from(path);
            args.file_ext = Some(file_ext_name.clone());
        }

        if let Some(name) = matches.get_one::<PathBuf>("output") {
            args.output = Some(PathBuf::from(name));
            args.file_ext = Some(String::from(
                args.output
                    .clone()
                    .unwrap()
                    .extension()
                    .unwrap()
                    .to_str()
                    .unwrap(),
            ));
        } else {
            let filename = args
                .filepath
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
                + "_edited";

            args.output = Some(
                args.filepath
                    .parent()
                    .unwrap()
                    .join(filename)
                    .with_extension(args.file_ext.as_deref().unwrap_or("jpg")),
            );
        }

        if matches.subcommand_matches("ascii").is_some()
            && matches
                .subcommand_matches("ascii")
                .unwrap()
                .get_flag("colored")
        {
            args.colored = true
        };
        if matches.subcommand_matches("ascii").is_some()
            && matches
                .subcommand_matches("ascii")
                .unwrap()
                .get_flag("invert")
        {
            args.invert = true
        };
        if let Some(name) = matches
            .subcommand_matches("ascii")
            .and_then(|m| m.get_one::<String>("charset"))
        {
            args.charset = name.to_string()
        }
        if let Some(name) = matches
            .subcommand_matches("ascii")
            .and_then(|m| m.get_one::<u32>("width"))
        {
            args.width = Some(*name as u32)
        };
        if let Some(name) = matches
            .subcommand_matches("ascii")
            .and_then(|m| m.get_one::<u32>("height"))
        {
            args.height = Some(*name as u32)
        };

        match matches.subcommand() {
            Some(("blur", sub_matches)) => {
                if let Some(r) = sub_matches.get_one::<u32>("blur_radius") {
                    args.radius = Some(r.clone());
                    let img_result = blur(
                        &ImageReader::open(args.filepath.clone())?
                            .decode()?
                            .into_rgba8(),
                        *r,
                    );
                    let _ = img_result.save(args.format_output_name());
                    println!("Blurred image saved as {:?}", args.format_output_name());
                }
            }
            Some(("pixelate", sub_matches)) => {
                let img = ImageReader::open(args.filepath.clone())?.decode()?;
                if let Some(s) = sub_matches.get_one::<u32>("pixel_size") {
                    args.pixel = Some(s.clone());
                    let img_result = pixelate(&img, (*s, *s));
                    let _ = img_result.save(args.format_output_name());
                    println!("Pixelated image saved as {:?}", args.format_output_name());
                }
            }
            Some(("scale", sub_matches)) => {
                let img = ImageReader::open(args.filepath.clone())?.decode()?;
                if let Some(s) = sub_matches.get_one::<u32>("scale") {
                    args.resize = Some(s.clone());
                    let img_result = resize(&img.into_rgba8(), (*s, *s));
                    let _ = img_result.save(args.format_output_name());
                    println!("Scaled image saved as {:?}", args.format_output_name());
                }
            }
            Some(("rotate", _sub_matches)) => {
                let img_result = rotate(
                    &ImageReader::open(args.filepath.clone())?
                        .decode()?
                        .into_rgba8(),
                );
                let _ = img_result.save(args.format_output_name());
                println!("Rotated image saved as {:?}", args.format_output_name());
            }
            Some(("mirror", _sub_matches)) => {
                let img = ImageReader::open(args.filepath.clone())?.decode()?;
                let img_result = img.fliph();
                let _ = img_result.save(args.format_output_name());
                println!("Mirrored image saved as {:?}", args.format_output_name());
            }
            Some(("flip_vertical", _sub_matches)) => {
                let img = ImageReader::open(args.filepath.clone())?.decode()?;
                let img_result = img.flipv();
                let _ = img_result.save(args.format_output_name());
                println!("Flipped image saved as {:?}", args.format_output_name());
            }
            Some(("monochrome_ugly", sub_matches)) => {
                if let Some(t) = sub_matches.get_one::<f32>("threshold") {
                    args.threshold = Some(t.clone());
                    let img_result = monochrome_ugly(
                        &ImageReader::open(args.filepath.clone())?
                            .decode()?
                            .into_rgba8(),
                        *t,
                    );
                    let _ = img_result.save(args.format_output_name());
                    println!("Monochrome image saved as {:?}", args.format_output_name());
                }
            }
            Some(("grayscale", _sub_matches)) => {
                let img_result = grayscale(
                    &ImageReader::open(args.filepath.clone())?
                        .decode()?
                        .into_rgba8(),
                );
                let _ = img_result.save(args.format_output_name());
                println!("Grayscale image saved as {:?}", args.format_output_name());
            }
            Some(("ascii", _sub_matches)) => {
                let clusters =
                    UnicodeSegmentation::graphemes(args.charset.as_str(), true).collect::<Vec<_>>();
                let charset = from_str(args.charset.as_str()).unwrap_or(clusters.as_slice());

                if args.width.is_none() && args.height.is_none() {
                    args.width = Some(80);
                }

                let path = args.filepath.clone();

                render(
                    path.to_str().unwrap(),
                    &mut io::stdout(),
                    &RenderOptions {
                        width: args.width,
                        height: args.height,
                        colored: args.colored,
                        invert: args.invert,
                        charset,
                    },
                )?;
            }
            _ => println!("Unidentified subcommand."),
        }
        Ok(())
    }
}
