mod args;
mod command_maker;
mod methods;
mod flag_matcher;


pub use crate::commands::command_maker::make_commands;
pub use crate::commands::args::Args;
pub use crate::commands::methods::{rotate, resize, pixelate, blur};

