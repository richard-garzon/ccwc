use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct FileStats {
    pub file_name: String,
    pub bytes: usize,
    pub lines: i32,
    pub words: usize,
    pub chars: usize,
}

const BUFSIZE: usize = 512;

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

        let file = File::open(&self.file_name).unwrap();
        let mut reader = BufReader::with_capacity(BUFSIZE, file);
        let mut in_word = false;

        loop {
            let length = {
                let buffer = reader.fill_buf().unwrap();

                for c in buffer {
                    bytes += 1;

                    if *c == b'\n' {
                        line_count += 1;
                    }

                    if c.is_ascii_whitespace() {
                        if in_word {
                            word_count += 1;
                            in_word = false;
                        }
                    } else {
                        in_word = true;
                    }
                }

                buffer.len()
            };

            if length == 0 {
                break;
            }
            reader.consume(length);
        }

        if in_word {
            word_count += 1;
        }

        self.bytes = bytes;
        self.lines = line_count;
        self.words = word_count;
        self.chars = characters;
    }
}
