use std::fs::File;
use std::io::{Seek, SeekFrom, BufReader, BufRead};
use read_mode::read_mode::{ReadMode, ArrowDirection};

 // Iterates line by line
pub struct MultilineMode {
    file: File,
    width: usize,
    height: usize,
    line_history: Vec<u64>,
    line_index: usize,
    char_index: usize,

}

impl MultilineMode {
    pub fn new(file: File, width: usize, height: usize) -> Self {
        MultilineMode {
            file,
            width,
            height,
            line_history: vec![0], // The first line is at position 0.
            line_index: 0,
            char_index: 0,
        }
    }
}

impl ReadMode for MultilineMode {
    fn read(&mut self) {
        {
            // Seek to the start of the line at line_index:
            let seek_idx = self.line_history[self.line_index];
            self.file.seek(SeekFrom::Start(seek_idx));
        }
        let mut reader = BufReader::new(&self.file);
        for i in 1..self.height {
            // read the line
            let mut buf = String::new();
            let newline_pos = reader.read_line(&mut buf).unwrap();
            // if we are reading a new line not in the history, add it:
            if self.line_index + i + 1 > self.line_history.len() {
                let last_seek_idx;
                    {last_seek_idx = self.line_history.last().unwrap().clone();}
                self.line_history.push(last_seek_idx + (newline_pos as u64)); // Push the next line's position
            }
            let line_to_print = &buf.chars()
                .skip(self.char_index) // skip the first X characters of the line
                .take(self.width) // Only take W letters following the index
                .filter(|c| {!String::from("\r\n").contains(*c)}) // take out any endlines
                .collect::<String>(); // return as a new string.
            println!("{}", line_to_print);
        }
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
