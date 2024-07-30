use std::fs;
use std::io::{BufReader, BufRead};

pub struct FileStats {
    pub file_name: String,
    pub bytes: usize,
    pub lines: i32,
    pub words: usize,
    pub characters: usize,
}

impl FileStats {
    pub fn new(file_name: String, bytes: usize, word_count: usize, line_count: i32) -> FileStats {
        FileStats { 
            file_name: file_name,
            bytes: bytes,
            lines: line_count,
            words: word_count,
            characters: bytes,
        }
    }
    pub fn get_stats(&mut self, file_name: String) {
        let file = fs::File::open(&file_name).unwrap();
        let reader = BufReader::new(file);
        let mut bytes = 0;
        let mut line_count = 0;
        let mut word_count = 0;

        for line in reader.lines() {
            let line = line.unwrap();
            bytes += line.as_bytes().len();
            word_count += line.split_whitespace().count();
            line_count += 1;
        }
        self.file_name = String::from(file_name);
        self.bytes = bytes;
        self.lines = line_count;
        self.words = word_count;
        self.characters = bytes;
    }
}