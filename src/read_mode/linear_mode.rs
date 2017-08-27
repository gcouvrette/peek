use std::fs::File;
use std::io::{Read, SeekFrom, Seek};
use read_mode::read_mode::{ReadMode, ArrowDirection};

 // Reads char per char and output all in one linear
pub struct LinearMode {
    file: File,
    buffer: usize,
    index: u64,
    file_size: u64,
}

impl LinearMode {
    pub fn new(mut file: File, buffer: usize) -> Self {
        let file_size: u64 = (&mut file).seek(SeekFrom::End(0)).unwrap();
        LinearMode {
            file,
            buffer,
            index: 0,
            file_size,
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
                    self.index -= 1;
                }
            },
            ArrowDirection::RIGHT | ArrowDirection::DOWN => {
                if self.index + (self.buffer as u64) < self.file_size {
                    self.index += 1;
                }
            },
        };
    }
}
