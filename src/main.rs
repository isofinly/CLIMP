// use dialoguer::{theme::ColorfulTheme, Select};



mod commands;
mod ascii;

use std::error::Error;
use std::path::PathBuf;
// use crate::commands::Args;
use commands::{make_commands, Args};



fn main() -> Result<(), Box<dyn Error>> {
    let args = Args {
        filepath: PathBuf::from(""),
        radius: None,
        pixel: None,
        output: None,
        resize: None,
        file_ext: None,
        width: None,
        height: None,
        colored: false,
        invert: false,
        charset: "default".to_string(),
    };

    let matches = make_commands();

    let _ = Args::match_flags(args, matches);

    Ok(())
}
