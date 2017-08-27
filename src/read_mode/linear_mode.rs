use std::fs::File;
use std::io::{Read, SeekFrom, Seek};
use read_mode::read_mode::{ReadMode, ArrowDirection};

 // Reads char per char and output all in one linear
pub struct LinearMode {
    file: File,
    buffer: usize,
    index: u64,
}

impl LinearMode {
    pub fn new(file: File, buffer: usize) -> Self {
        LinearMode {
            file,
            buffer,
            index: 0,
        }
    }
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
