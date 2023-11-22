use clap::ArgMatches;
use dialoguer::Select;
use dialoguer::{theme::ColorfulTheme, Input};
use std::path::PathBuf;
use std::{error::Error, process};

use super::Args;

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
        let flags = &[true, false];

        if matches.get_flag("interactive") {
            loop {
                let command = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("$")
                    .default(0)
                    .items(&commands[..])
                    .interact()
                    .unwrap();

                let command = &commands[command];
                match command.get(..) {
                    Some("pixelate") => {
                        handle_input_file(self)?;
                        handle_output_file(self);
                        handle_pixel_size(self);
                        Args::pixelate(self)?;
                        println!()
                    }
                    Some("blur") => {
                        handle_input_file(self)?;
                        handle_output_file(self);
                        handle_blur_radius(self);
                        Args::blur(self)?;
                        println!()
                    }
                    Some("mirror") => {
                        handle_input_file(self)?;
                        handle_output_file(self);
                        Args::mirror(self)?;
                        println!()
                    }
                    Some("flip_vertical") => {
                        handle_input_file(self)?;
                        handle_output_file(self);
                        Args::flip_vertical(self)?;
                        println!()
                    }
                    Some("rotate") => {
                        handle_input_file(self)?;
                        handle_output_file(self);
                        Args::rotate(self)?;
                        println!()
                    }
                    Some("grayscale") => {
                        handle_input_file(self)?;
                        handle_output_file(self);
                        Args::grayscale(self)?;
                        println!()
                    }
                    Some("monochrome_ugly") => {
                        handle_input_file(self)?;
                        handle_output_file(self);
                        handle_threshold(self);
                        Args::monochrome_ugly(self)?;
                        println!()
                    }
                    Some("scale") => {
                        handle_input_file(self)?;
                        handle_output_file(self);
                        handle_scale_factor(self);
                        Args::resize(self)?;
                        println!()
                    }
                    Some("ascii") => {
                        handle_input_file(self)?;
                        self.set_file_ext(Some(String::from("txt")));
                        handle_output_file(self);

                        let flag = Select::with_theme(&ColorfulTheme::default())
                            .with_prompt("Render only in console or in file?")
                            .default(0)
                            .items(&flags[..])
                            .interact()
                            .unwrap();

                        let flag = &flags[flag];

                        Args::ascii(self, *flag)?;
                        println!()
                    }
                    Some("curse") => {
                        handle_input_file(self)?;
                        handle_output_file(self);
                        Args::curse(self)?;
                        println!()
                    }
                    Some("zxc") => {
                        handle_input_file(self)?;
                        handle_output_file(self);
                        Args::zxc(self)?;
                        println!()
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
        .with_prompt("Path to input image")
        .interact_text()
        .unwrap();

    set_input_filepath(path.into(), args);

    match set_input_extension(args) {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    }

    Ok(())
}

fn set_input_filepath(path: PathBuf, args: &mut Args) {
    args.set_filepath(PathBuf::from(path));
}

fn set_input_extension(args: &mut Args) -> Result<(), Box<dyn Error>> {
    let path = args.get_filepath();
    if path.extension().is_none() {
        return Err(
            "Output file must have an extension\nProgram is not supposed to read raw bytes".into(),
        );
    }
    let extension = path
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .unwrap()
        .to_string();

    args.set_file_ext(Some(extension));
    Ok(())
}

fn handle_output_file(args: &mut Args) {
    let path: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Path to output image")
        .default(
            args.get_filepath()
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
                + "_edited."
                + &args.get_file_ext().unwrap(),
        )
        .interact_text()
        .unwrap();

    set_output_filepath(path.into(), args);

    match set_output_extension(args) {
        Ok(_) => {}
        Err(e) => {
            println!("{}\nDefault extension (.jpg) will be used", e);
            args.set_output_name(PathBuf::from(
                args.get_output_name().clone().to_str().unwrap().to_string() + ".jpg",
            ));
            args.set_output_ext(Some("jpg".to_string()));
        }
    };
}

fn set_output_filepath(path: PathBuf, args: &mut Args) {
    args.set_output_name(PathBuf::from(path));
}

fn set_output_extension(args: &mut Args) -> Result<(), Box<dyn Error>> {
    let path = args.get_output_name();
    if path.extension().is_none() {
        return Err("Output file must have an extension".into());
    }
    let extension = path
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .unwrap()
        .to_string();

    args.set_output_ext(Some(extension));

    Ok(())
}

fn handle_pixel_size(args: &mut Args) {
    let size: u32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Pixel size of output image pixels")
        .interact_text()
        .unwrap();

    args.set_pixel(Some(size));
}

fn handle_blur_radius(args: &mut Args) {
    let radius: u32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Blur radius")
        .interact_text()
        .unwrap();

    args.set_blur_radius(Some(radius));
}

fn handle_threshold(args: &mut Args) {
    let threshold: f32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Threshold")
        .interact_text()
        .unwrap();

    args.set_threshold(Some(threshold));
}

fn handle_scale_factor(args: &mut Args) {
    let factor: u32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("New dimensions")
        .interact_text()
        .unwrap();

    args.set_resize(Some(factor));
}
