use crate::ascii::{self, render_to_file};
use clap::ArgMatches;
use dialoguer::Select;
use dialoguer::{theme::ColorfulTheme, Input};
use image::io::Reader as ImageReader;
use image::ImageFormat;
use std::io;
use std::path::PathBuf;
use std::{error::Error, process};

use super::{blur, curse, grayscale, monochrome_ugly, pixelate, resize, rotate, zxc, Args};
use ascii::{from_str, render, RenderOptions};

impl Args {
    pub fn interactive_matcher(&mut self, matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        let commands = &[
            "pixelate",
            "blur",
            "mirror",
            "flip_vertical",
            "rotate",
            "grayscale",
            "monochrome_ugly",
            "scale",
            "ascii",
            "curse",
            "zxc",
            "exit",
        ];

        if matches.get_flag("interactive") {
            println!("Type exit to exit | Help for help");
            loop {
                // if let Ok(cmd) = Input::<String>::with_theme(&ColorfulTheme::default())
                //     .with_prompt("$")
                //     .interact_text()
                // {
                //     match cmd.as_str() {
                //         "exit" => {
                //             process::exit(0);
                //         }
                //         _ => {
                //             // println!("{}", cmd);
                //         }
                //     }
                // }
                let command = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Pick your flavor")
                    .default(0)
                    .items(&commands[..])
                    .interact()
                    .unwrap();

                let command = &commands[command];
                match command.get(..) {
                    Some("pixelate") => {
                        handle_input_file(self)?;
                        println!("pixelate: {:?}{:?}", self.get_filepath(), self.get_file_ext());
                        handle_output_file(self)?;
                        println!("pixelate: {:?}{:?}", self.get_output_name(), self.get_output_ext());
                    }
                    Some("blur") => {
                        println!("blur")
                    }
                    Some("mirror") => {
                        println!("mirror")
                    }
                    Some("flip_vertical") => {
                        println!("flip_vertical")
                    }
                    Some("rotate") => {
                        println!("rotate")
                    }
                    Some("grayscale") => {
                        println!("grayscale")
                    }
                    Some("monochrome_ugly") => {
                        println!("monochrome_ugly")
                    }
                    Some("scale") => {
                        println!("scale")
                    }
                    Some("ascii") => {
                        println!("ascii")
                    }
                    Some("curse") => {
                        println!("curse")
                    }
                    Some("zxc") => {
                        println!("zxc")
                    }
                    Some("exit") => {
                        process::exit(0);
                    }
                    _ => {
                        println!("Unknown command");
                    }
                }
            }
        }
        Ok(())
    }
}

fn handle_input_file(args: &mut Args) -> Result<(), Box<dyn Error>> {
    let path: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter path to input image")
        .interact_text()
        .unwrap();

    set_input_filepath(path.into(), args);

    set_input_extension(args)?;

    Ok(())
}

fn set_input_filepath(path: PathBuf, args: &mut Args) {
    args.set_filepath(PathBuf::from(path));
}

fn set_input_extension(args: &mut Args) -> Result<(), Box<dyn Error>> {
    let path = args.get_filepath();
    if path.extension().and_then(std::ffi::OsStr::to_str).is_none() {
        return Err("File with no extension provided".into());
    }
    let extension = path
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .unwrap()
        .to_string();

    args.set_file_ext(Some(extension));

    Ok(())
}

fn handle_output_file(args: &mut Args) -> Result<(), Box<dyn Error>> {
    let path: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter path to output image")
        .interact_text()
        .unwrap();

    set_output_filepath(path.into(), args);

    set_output_extension(args)?;

    Ok(())
}

fn set_output_filepath(path: PathBuf, args: &mut Args) {
    args.set_output_name(PathBuf::from(path));
}

fn set_output_extension(args: &mut Args) -> Result<(), Box<dyn Error>> {
    let path = args.get_output_name();
    if path.extension().and_then(std::ffi::OsStr::to_str).is_none() {
        return Err("File with no extension provided".into());
    }
    let extension = path
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .unwrap()
        .to_string();

    args.set_output_ext(Some(extension));

    Ok(())
}
