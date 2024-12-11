use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct FileStats {
    pub bytes: usize,
    pub lines: i32,
    pub words: usize,
    pub chars: usize,
}

impl FileStats {
    pub fn new() -> FileStats {
        FileStats {
            bytes: 0,
            lines: 0,
            words: 0,
            chars: 0,
        }
    }

    pub fn populate_data(&mut self, file: File) {
        let mut bytes = 0;
        let mut characters = 0;
        let mut lines = 0;
        let mut words = 0;

        let mut buffer = String::new();
        let mut reader = BufReader::new(&file);

        loop {
            let bytes_read = reader.read_line(&mut buffer).unwrap();
            if bytes_read == 0 {
                break;
            }
            // count lines
            lines += 1;

            // count bytes
            bytes += bytes_read;

            // count words
            words += buffer.split_whitespace().count();

            /*
            count characers - i took this from:
            https://github.com/llogiq/bytecount/blob/master/src/naive.rs
             */
            let as_bytes = buffer.as_bytes();
            characters += as_bytes.iter().filter(|&&byte| (byte >> 6) != 0b10).count();

            buffer.clear();
        }

        self.bytes = bytes;
        self.lines = lines;
        self.words = words;
        self.chars = characters;
    }
}
