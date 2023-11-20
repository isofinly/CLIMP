mod args;
mod command_creator;
mod arg_matcher;
mod interactive_matcher;
mod methods;

pub use crate::commands::args::Args;
pub use crate::commands::command_creator::make_commands;
pub use crate::commands::methods::{
    blur, curse, grayscale, monochrome_ugly, pixelate, resize, rotate, zxc,
};
