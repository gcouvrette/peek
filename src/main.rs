extern crate peek;
extern crate getch;
use getch::Getch;
use peek::read_mode::read_mode::*;

fn main() {
    let mut read_mode = get_read_mode(std::env::args()).unwrap_or_else(|err| {
        println!("Error: {}", err);
        help()
        });

    let getch = Getch::new().expect("Unable to read from console byte-by-byte.");

    read_mode.read();
    loop {
        let key: Option<ArrowDirection> = match getch.getch().unwrap() {
            75u8 => Some(ArrowDirection::LEFT),
            72u8 => Some(ArrowDirection::UP),
            77u8 => Some(ArrowDirection::RIGHT),
            80u8 => Some(ArrowDirection::DOWN),
            113u8 /* q */ => {break;},
            _ => None,
        };
        if let Some(key) = key {
            read_mode.on_scroll(key);
            clear_screen();
            read_mode.read();
        }
    }
}

/// Displays a help message for this command, then quit:
fn help() -> ! {
    println!("-- peek --
    This tool is used to peek at the content of a file
    without reading all of it in memory. Very useful
    for big files.

    Possible options:
    -m [width, height] : Multiline - Reads the file as a multiline file.
                                     Peek will read until `height` lines are found
                                     and will display the first `width` characters
                                     of each line. In this mode, it is possible
                                     to move the reading area using the arrow keys.
                                     Width and height are optional. Defaults are
                                     `width`: 80, `height`: 40

   -l [length] : Linear (*default) - Reads the file in a linear pattern, reading only
                                     `length` chars at a time. Use arrow keys to
                                     move in the file. Left/Up goes back in the buffer,
                                     right/down moves forward. Default is
                                     `length`: 3200");
    std::process::exit(0); // quit the app.
}

fn clear_screen() {
    print!("{}[2J", 27 as char);
}
