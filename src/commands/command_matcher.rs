use crate::ascii::{self, render_to_file};
use clap::ArgMatches;
use image::io::Reader as ImageReader;
use std::io;
use std::path::PathBuf;
use std::error::Error;
use unicode_segmentation::UnicodeSegmentation;

use super::{blur, grayscale, monochrome_ugly, pixelate, resize, rotate, Args};
use ascii::{from_str, render, RenderOptions};

impl Args {
    /// Matches command line arguments and calls the appropriate function
    /// based on matches.subcommand() method
    ///
    /// Takes Args struct and ArgMatches as input
    ///
    /// If no file extension is provided then jpg will be used
    pub fn match_command(&mut self, matches: ArgMatches) -> Result<(), Box<dyn Error>> {
        /* Someday I'll use this for batch processing
           .get_many::<String>("filepath")
           .unwrap_or_default()
           .map(|v| v.as_str())
           .collect::<Vec<_>>();
        */

        if let Some(path) = matches.get_one::<PathBuf>("filepath") {
            let file_ext_name = std::path::Path::new(&path)
                .extension()
                .and_then(std::ffi::OsStr::to_str)
                .unwrap_or("jpg")
                .to_string();
            self.set_filepath(PathBuf::from(path));
            self.set_file_ext(Some(file_ext_name));
        }

        if let Some(name) = matches.get_one::<PathBuf>("output") {
            self.set_output(Some(PathBuf::from(name)));
            self.set_file_ext(Some(String::from(
                self.get_output()
                    .unwrap()
                    .extension()
                    .and_then(std::ffi::OsStr::to_str)
                    .unwrap_or("jpg"),
            )));
        } else {
            let filename = self
                .get_filepath()
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
                + "_edited";

            self.set_output(Some(
                self.get_filepath()
                    .parent()
                    .unwrap()
                    .join(filename)
                    .with_extension(
                        self.get_file_ext()
                            .as_deref()
                            .unwrap_or(&String::from("jpg")),
                    ),
            ));
        }

        if matches.subcommand_matches("ascii").is_some()
            && matches
                .subcommand_matches("ascii")
                .unwrap()
                .get_flag("colored")
        {
            self.set_colored(true);
        };
        if matches.subcommand_matches("ascii").is_some()
            && matches
                .subcommand_matches("ascii")
                .unwrap()
                .get_flag("verbose_only")
        {
            self.set_verbose_only(true);
        };
        if matches.subcommand_matches("ascii").is_some()
            && matches
                .subcommand_matches("ascii")
                .unwrap()
                .get_flag("invert")
        {
            self.set_invert(true);
        };
        if let Some(name) = matches
            .subcommand_matches("ascii")
            .and_then(|m| m.get_one::<String>("charset"))
        {
            self.set_charset(name.to_string())
        }
        if let Some(name) = matches
            .subcommand_matches("ascii")
            .and_then(|m| m.get_one::<u32>("width"))
        {
            self.set_width(Some(*name as u32))
        };
        if let Some(name) = matches
            .subcommand_matches("ascii")
            .and_then(|m| m.get_one::<u32>("height"))
        {
            self.set_height(Some(*name as u32))
        };

        let out_name = self.format_output_name();

        match matches.subcommand() {
            Some(("blur", sub_matches)) => {
                if let Some(r) = sub_matches.get_one::<u32>("blur_radius") {
                    self.set_radius(Some(r.clone()));
                    let img_result = blur(
                        &ImageReader::open(self.get_filepath().clone())?
                            .decode()?
                            .into_rgba8(),
                        *r,
                    );
                    let _ = img_result.save(&out_name);
                    println!("Blurred image saved as {:?}", &out_name);
                }
            }
            Some(("pixelate", sub_matches)) => {
                let img = ImageReader::open(self.get_filepath().clone())?.decode()?;
                if let Some(s) = sub_matches.get_one::<u32>("pixel_size") {
                    self.set_pixel(Some(s.clone()));
                    let img_result = pixelate(&img, (*s, *s));
                    let _ = img_result.save(&out_name);
                    println!("Pixelated image saved as {:?}", &out_name);
                }
            }
            Some(("scale", sub_matches)) => {
                let img = ImageReader::open(self.get_filepath().clone())?.decode()?;
                if let Some(s) = sub_matches.get_one::<u32>("scale") {
                    self.set_resize(Some(s.clone()));
                    let img_result = resize(&img.into_rgba8(), (*s, *s));
                    let _ = img_result.save(&out_name);
                    println!("Scaled image saved as {:?}", &out_name);
                }
            }
            Some(("rotate", _sub_matches)) => {
                let img_result = rotate(
                    &ImageReader::open(self.get_filepath().clone())?
                        .decode()?
                        .into_rgba8(),
                );
                let _ = img_result.save(&out_name);
                println!("Rotated image saved as {:?}", &out_name);
            }
            Some(("mirror", _sub_matches)) => {
                let img = ImageReader::open(self.get_filepath().clone())?.decode()?;
                let img_result = img.fliph();
                let _ = img_result.save(&out_name);
                println!("Mirrored image saved as {:?}", &out_name);
            }
            Some(("flip_vertical", _sub_matches)) => {
                let img = ImageReader::open(self.get_filepath().clone())?.decode()?;
                let img_result = img.flipv();
                let _ = img_result.save(&out_name);
                println!("Flipped image saved as {:?}", &out_name);
            }
            Some(("monochrome_ugly", sub_matches)) => {
                if let Some(t) = sub_matches.get_one::<f32>("threshold") {
                    self.set_threshold(Some(t.clone()));
                    let img_result = monochrome_ugly(
                        &ImageReader::open(self.get_filepath().clone())?
                            .decode()?
                            .into_rgba8(),
                        *t,
                    );
                    let _ = img_result.save(&out_name);
                    println!("Monochrome image saved as {:?}", &out_name);
                }
            }
            Some(("grayscale", _sub_matches)) => {
                let img_result = grayscale(
                    &ImageReader::open(self.get_filepath().clone())?
                        .decode()?
                        .into_rgba8(),
                );
                let _ = img_result.save(&out_name);
                println!("Grayscale image saved as {:?}", &out_name);
            }
            Some(("ascii", sub_matches)) => {
                let clusters = UnicodeSegmentation::graphemes(self.get_charset().as_str(), true)
                    .collect::<Vec<_>>();
                let charset = from_str(self.get_charset().as_str()).unwrap_or(clusters.as_slice());

                let path = self.get_filepath().clone();

                if self.get_output().is_some() {
                    let out_name = out_name.file_stem().map(|stem| {
                        let mut new_path = out_name.clone();
                        new_path.set_file_name(stem);
                        new_path
                    });
                    if sub_matches.get_flag("verbose_only") {
                        render(
                            path.to_str().unwrap(),
                            &mut io::stdout(),
                            &RenderOptions {
                                width: self.get_width().or(Some(80)),
                                height: self.get_height(),
                                colored: self.is_colored(),
                                invert: self.is_invert(),
                                charset,
                            },
                        )?;
                    } else {
                        render_to_file(
                            path.to_str().unwrap(),
                            &out_name.unwrap(),
                            &RenderOptions {
                                width: self.get_width().or(Some(80)),
                                height: self.get_height(),
                                colored: self.is_colored(),
                                invert: self.is_invert(),
                                charset,
                            },
                        )?;
                    }
                }
            }
            _ => println!("Unidentified subcommand. \n Use '--help' for more information"),
        }
        Ok(())
    }
}
