use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct FileStats {
    pub file_name: String,
    pub bytes: usize,
    pub lines: i32,
    pub words: usize,
    pub chars: usize,
}

pub fn read_lines<P: AsRef<Path>>(file_name: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(file_name)?;
    Ok(io::BufReader::new(file).lines())
}

impl FileStats {
    pub fn new(file_name: &String) -> FileStats {
        FileStats {
            file_name: file_name.clone(),
            bytes: 0,
            lines: 0,
            words: 0,
            chars: 0,
        }
    }

    pub fn populate_data(&mut self) {
        let mut bytes = 0;
        let mut characters = 0;
        let mut line_count = 0;
        let mut word_count = 0;

        if let Ok(lines) = read_lines(&self.file_name) {
            for l in lines {
                let line = l.unwrap();
                bytes += line.as_bytes().len();
                characters += line.chars().count();
                word_count += line.split_whitespace().count();
                line_count += 1;
                bytes += 1;
            }
        }

        self.bytes = bytes;
        self.lines = line_count;
        self.words = word_count;
        self.chars = characters;
    }
}
