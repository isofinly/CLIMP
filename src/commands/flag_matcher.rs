use std::path::PathBuf;
use crate::ascii;
use clap::ArgMatches;
use image::io::Reader as ImageReader;
use std::error::Error;
use std::io;
use unicode_segmentation::UnicodeSegmentation;

use super::{blur, pixelate, resize, rotate, Args};
use ascii::{from_str, render, RenderOptions};

impl Args {
    pub fn match_flags(mut args: Args, matches: ArgMatches) -> Result<(), Box<dyn Error>> {
        if let Some(path) = matches.get_one::<PathBuf>("filepath") {
            let file_ext_name = std::path::Path::new(&path)
                .extension()
                .and_then(std::ffi::OsStr::to_str)
                .unwrap_or("jpg")
                .to_string();
            args.filepath = path.clone();
            args.file_ext = Some(/*".".to_string() + */ file_ext_name);
        }

        if let Some(name) = matches.get_one::<PathBuf>("output") {
            args.output = Some(name.clone());
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

        if matches
            .subcommand_matches("ascii")
            .unwrap()
            .get_flag("colored")
        {
            args.colored = true
        };
        if matches
            .subcommand_matches("ascii")
            .unwrap()
            .get_flag("invert")
        {
            args.invert = true
        };
        if let Some(name) = matches
            .subcommand_matches("ascii")
            .unwrap()
            .get_one::<String>("charset")
        {
            args.charset = name.to_string()
        };
        if let Some(name) = matches
            .subcommand_matches("ascii")
            .unwrap()
            .get_one::<u32>("width")
        {
            args.width = Some(*name as u32)
        };
        if let Some(name) = matches
            .subcommand_matches("ascii")
            .unwrap()
            .get_one::<u32>("height")
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
                }
            }
            Some(("pixelate", sub_matches)) => {
                let img = ImageReader::open(args.filepath.clone())?.decode()?;
                if let Some(s) = sub_matches.get_one::<u32>("pixel_size") {
                    args.pixel = Some(s.clone());
                    let img_result = pixelate(&img, (*s, *s));
                    let _ = img_result.save(args.format_output_name());
                }
            }
            Some(("scale", sub_matches)) => {
                let img = ImageReader::open(args.filepath.clone())?.decode()?;
                if let Some(s) = sub_matches.get_one::<u32>("scale") {
                    args.resize = Some(s.clone());
                    let img_result = resize(&img.into_rgba8(), (*s, *s));
                    let _ = img_result.save(args.format_output_name());
                }
            }
            Some(("rotate", _sub_matches)) => {
                let img_result = rotate(
                    &ImageReader::open(args.filepath.clone())?
                        .decode()?
                        .into_rgba8(),
                );
                let _ = img_result.save(args.format_output_name());
            }
            Some(("mirror", _sub_matches)) => {
                let img = ImageReader::open(args.filepath.clone())?.decode()?;
                let img_result = img.fliph();
                let _ = img_result.save(args.format_output_name());
            }
            Some(("flip_vertical", _sub_matches)) => {
                let img = ImageReader::open(args.filepath.clone())?.decode()?;
                let img_result = img.flipv();
                let _ = img_result.save(args.format_output_name());
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