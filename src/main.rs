mod ascii;
mod commands;

use commands::{make_commands, Args};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {

    let _ = Args::match_command(&mut Args::new(), make_commands());

    Ok(())
}
