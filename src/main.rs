mod commands;
mod ascii;

use std::error::Error;
use commands::{make_commands, Args};

fn main() -> Result<(), Box<dyn Error>> {

    let matches = make_commands();

    let _ = Args::match_command(&mut Args::new(), matches);

    Ok(())
}
