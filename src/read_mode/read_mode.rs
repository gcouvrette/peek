use std::env::Args;
use std::collections::VecDeque;
use std::fs::File;
use std::str::FromStr;
use read_mode::linear_mode::LinearMode;
use read_mode::multiline_mode::MultilineMode;

pub enum ArrowDirection {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

pub trait ReadMode {
    fn read(&mut self);
    fn on_scroll(&mut self, direction: ArrowDirection);
}

/// Tries to build a ReadMode using the parameters specified in the Args.
pub fn get_read_mode(args: Args) -> Result<Box<ReadMode>, &'static str> {
    let mut args: VecDeque<String> = args.skip(1).collect();
    // First, fetch the path. It is always the last argument, so we pop it.
    let path = args.pop_back().ok_or("File name is missing.")?;
    let file = File::open(&path).map_err(|_| "Cannot read file.")?;
    // Then, we parse the read mode:
    match args.pop_front().as_ref().map(|s| &s[..]) {
        Some("-l") | None => {
            let buffer = usize::from_str(&args.pop_front().unwrap_or("3200".to_owned())).map_err(|_|"Buffer length is not a number.")?;
            Ok(Box::new(LinearMode::new(file, buffer)))
        },
        Some("-m") => {
            let width = usize::from_str(&args.pop_front().unwrap_or("80".to_owned())).map_err(|_|"Width is not a number.")?;
            let height = usize::from_str(&args.pop_front().unwrap_or("40".to_owned())).map_err(|_|"Height is not a number.")?;
            Ok(Box::new(MultilineMode::new(file, width, height)))
        },
        _ => {
            Err("Unknown flag.")
        },
    }
}
