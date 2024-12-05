use std::fs;
use std::io::{BufRead, BufReader};

pub struct FileStats {
    pub file_name: String,
    pub bytes: usize,
    pub lines: i32,
    pub words: usize,
    pub characters: usize,
}

impl FileStats {
    pub fn new(file_name: String) -> FileStats {
        FileStats {
            file_name: file_name,
            bytes: 0,
            lines: 0,
            words: 0,
            characters: 0,
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
