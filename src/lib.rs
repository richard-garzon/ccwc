use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct FileStats {
    pub bytes: usize,
    pub lines: i32,
    pub words: usize,
    pub chars: usize,
}

const BUFSIZE: usize = 512;

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

            lines += 1;
            bytes += bytes_read;
            words += buffer.split_whitespace().count();
            characters += buffer.len();
            buffer.clear();
        }

        self.bytes = bytes;
        self.lines = lines;
        self.words = words;
        self.chars = characters;
    }
}
