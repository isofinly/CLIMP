use super::Args;
use clap::ArgMatches;
use image::ImageFormat;
use std::error::Error;
use std::path::PathBuf;

impl Args {
    /// Matches command line arguments and calls the appropriate function
    /// based on matches.subcommand() method
    ///
    /// Takes Args struct and ArgMatches as input
    ///
    /// If no file extension is provided then jpg will be used
    pub fn match_arg(&mut self, matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
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
            self.set_output_name(PathBuf::from(name));
            self.set_output_ext(Some(String::from(
                match self
                    .get_output_name()
                    .extension()
                    .and_then(std::ffi::OsStr::to_str)
                {
                    Some(ext) => {
                        if ImageFormat::from_extension(ext).is_some() {
                            ext
                        } else {
                            "jpg"
                        }
                    }
                    None => "jpg",
                },
            )));
        } else if !matches.get_flag("interactive") {
            self.set_output_name(PathBuf::from(
                self.get_filepath()
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string()
                    + "_edited"
                    + "."
                    + self.get_file_ext().unwrap_or(&String::from("jpg")),
            ));
            self.set_output_ext(Some(String::from("jpg")));
        }

        if let Some(subcommand_matches) = matches.subcommand_matches("ascii") {
            if subcommand_matches.get_flag("colored") {
                self.set_colored(true);
            }
            if subcommand_matches.get_flag("verbose_only") {
                self.set_verbose_only(true);
            }
            if subcommand_matches.get_flag("invert") {
                self.set_invert(true);
            }
        }

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
            self.set_width(Some(*name))
        };
        if let Some(name) = matches
            .subcommand_matches("ascii")
            .and_then(|m| m.get_one::<u32>("height"))
        {
            self.set_height(Some(*name))
        };

        match matches.subcommand() {
            Some(("blur", sub_matches)) => {
                if let Some(r) = sub_matches.get_one::<u32>("blur_radius") {
                    self.set_blur_radius(Some(*r));
                    Args::blur(self)?;
                }
            }
            Some(("pixelate", sub_matches)) => {
                if let Some(s) = sub_matches.get_one::<u32>("pixel_size") {
                    self.set_pixel(Some(*s));
                    Args::pixelate(self)?;
                }
            }
            Some(("scale", sub_matches)) => {
                if let Some(s) = sub_matches.get_one::<u32>("scale") {
                    self.set_resize(Some(*s));
                    Args::resize(self)?;
                }
            }
            Some(("rotate", _sub_matches)) => {
                Args::rotate(self)?;
            }
            Some(("mirror", _sub_matches)) => {
                Args::mirror(self)?;
            }
            Some(("flip_vertical", _sub_matches)) => {
                Args::flip_vertical(self)?;
            }
            Some(("monochrome_ugly", sub_matches)) => {
                if let Some(t) = sub_matches.get_one::<f32>("threshold") {
                    self.set_threshold(Some(*t));
                    Args::monochrome_ugly(self)?;
                }
            }
            Some(("grayscale", _sub_matches)) => {
                Args::grayscale(self)?;
            }
            Some(("ascii", sub_matches)) => {
                let flag = sub_matches.get_flag("verbose_only");
                Args::ascii(self, flag)?;
            }
            Some(("curse", _sub_matches)) => {
                Args::curse(self)?;
            }
            Some(("zxc", _sub_matches)) => {
                Args::zxc(self)?;
            }
            _ => {
                if !matches.get_flag("interactive") {
                    println!("Unidentified subcommand. \nUse '--help' for more information")
                }
            }
        }
        Ok(())
    }
}
