use std::fs::File;
use std::io::{Seek, SeekFrom, BufReader, BufRead};
use read_mode::read_mode::{ReadMode, ArrowDirection};

 // Iterates line by line
pub struct MultilineMode {
    file: File,
    width: usize,
    height: usize,
    line_index: usize,
    char_index: usize,
}

impl MultilineMode {
    pub fn new(file: File, width: usize, height: usize) -> Self {
        MultilineMode {
            file,
            width,
            height,
            line_index: 0,
            char_index: 0,
        }
    }
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
