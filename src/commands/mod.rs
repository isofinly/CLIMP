mod args;
mod command_creator;
mod methods;
mod command_matcher;

pub use crate::commands::command_creator::make_commands;
pub use crate::commands::args::Args;
pub use crate::commands::methods::{rotate, resize, pixelate, blur, monochrome_ugly, grayscale};