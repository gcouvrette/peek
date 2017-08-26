extern crate getch;

use std::env::Args;
use std::str::FromStr;
use std::collections::VecDeque;
use std::fs::File;
use std::io::Read;
use getch::Getch;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::BufReader;
use std::io::BufRead;

enum ArrowDirection {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

trait ReadMode {
    fn read(&mut self);
    fn on_scroll(&mut self, direction: ArrowDirection);
}

fn get_read_mode(args: Args) -> Box<ReadMode> {
    let mut args: VecDeque<String> = args.skip(1).collect();
    // First, fetch the path. It is always the last argument, so we pop it.
    let path = args.pop_back().unwrap_or_else(||help());
    let file = File::open(&path).expect(&format!("File not found: {}", &path));
    // Then, we parse the read mode:
    match args.pop_front().as_ref().map(|s| &s[..]) {
        Some("-l") | None => {
            let buffer = usize::from_str(&args.pop_front().unwrap_or("3200".to_owned())).unwrap_or_else(|_|help());
            Box::new(LinearMode {
                file,
                buffer,
                index: 0,
            })
        },
        Some("-m") => {
            let width = usize::from_str(&args.pop_front().unwrap_or("80".to_owned())).unwrap_or_else(|_|help());
            let height = usize::from_str(&args.pop_front().unwrap_or("40".to_owned())).unwrap_or_else(|_|help());
            Box::new(MultilineMode {
                file,
                width,
                height,
                line_index: 0,
                char_index: 0,
            })
        },
        _ => {
            help();
        },
    }
}

 // Reads char per char and output all in one linear
struct LinearMode {
    file: File,
    buffer: usize,
    index: u64,
}

impl ReadMode for LinearMode {
    fn read(&mut self) {
        let mut buf = vec![0u8;self.buffer];
        self.file.seek(SeekFrom::Start(self.index));
        self.file.read(buf.as_mut_slice());
         // read then backtrack to stay at the same spot.
        println!("{}", String::from_utf8_lossy(&buf));
    }
    fn on_scroll(&mut self, direction: ArrowDirection) {
        match direction {
            ArrowDirection::LEFT | ArrowDirection::UP => {
                if self.index > 0 {
                    self.index -= 10;
                }
            },
            ArrowDirection::RIGHT | ArrowDirection::DOWN => {
                    self.index += 10;
            },
        };
    }
}
 // Iterates line by line
struct MultilineMode {
    file: File,
    width: usize,
    height: usize,
    line_index: usize,
    char_index: usize,
}

impl ReadMode for MultilineMode {
    fn read(&mut self) {
        let mut seek_total: i64 = 0;
        { // Immutable borrow of the file in this scope;
            let mut reader = BufReader::new(&self.file);
            let mut buf = String::new();
            for i in 0..self.height {
                if let Ok(n) = reader.read_line(&mut buf) {
                    seek_total -= n as i64;
                }
                println!("{}", &buf.chars().skip(self.char_index).take(self.width).collect::<String>());
            }
        // File borrow released here.
        }
        // Mutable borrow here to seek.
        panic!(format!("read: {}", seek_total));
        self.file.seek(SeekFrom::Current(seek_total)); // FIXME: The buffer is reading more than seek_total, the file is not scrolling properly.
    }
    fn on_scroll(&mut self, direction: ArrowDirection) {
        match direction {
            ArrowDirection::LEFT => {
                if self.char_index > 0 {
                    self.char_index -= 1;
                }
            },
            ArrowDirection::RIGHT => {
                    self.char_index += 1;
            },
            ArrowDirection::UP => {
                if self.line_index > 0 {
                    self.line_index -= 1;
                }
            },
            ArrowDirection::DOWN => {
                    self.line_index += 1;
            },
        };
    }
}

fn main() {
    let mut read_mode = get_read_mode(std::env::args());

    let getch = Getch::new().expect("Unable to read from console byte-by-byte.");
    loop {
        clear_screen();
        read_mode.read();

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
