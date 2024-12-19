use ccwc::FileStats;
use std::fs::File;
use std::io;
use std::io::BufReader;

use clap::Parser;

#[derive(Parser)]
#[command(name = "ccwc")]
#[command(about = "wc impl", long_about = None)]
struct Cli {
    file_name: Option<String>,

    #[arg(short = 'c', long = "bytes")]
    is_bytes: bool,

    #[arg(short = 'w', long = "words")]
    is_words: bool,

    #[arg(short = 'l', long = "lines")]
    is_lines: bool,

    #[arg(short = 'm', long = "chars")]
    is_chars: bool,
}

/*
    if all args passed, lwmc
    if no args, lwc
*/
fn build_string(
    is_bytes: bool,
    is_words: bool,
    is_lines: bool,
    is_chars: bool,
    file_stats: FileStats,
    file_name: &String,
) -> String {
    let mut parts = Vec::new();

    if !(is_bytes || is_words || is_lines || is_chars) {
        parts.push(file_stats.lines.to_string());
        parts.push(file_stats.words.to_string());
        parts.push(file_stats.bytes.to_string());
    } else {
        if is_lines {
            parts.push(file_stats.lines.to_string());
        }
        if is_words {
            parts.push(file_stats.words.to_string());
        }
        if is_chars {
            parts.push(file_stats.chars.to_string());
        }
        if is_bytes {
            parts.push(file_stats.bytes.to_string());
        }
    }

    parts.push(file_name.to_string());
    return parts.join(" ");
}

fn main() {
    let cli = Cli::parse();

    let (reader, printable_file_name): (BufReader<Box<dyn io::Read>>, String) = match cli.file_name
    {
        Some(file_name) => {
            let file = File::open(&file_name)
                .unwrap_or_else(|_| panic!("failed while opening file {}", file_name));
            (BufReader::new(Box::new(file)), file_name)
        }
        None => {
            let stdin = io::stdin();
            (BufReader::new(Box::new(stdin)), "".to_string())
        }
    };

    let mut file_stats = FileStats::new();

    file_stats.populate_data(reader);

    let result = build_string(
        cli.is_bytes,
        cli.is_words,
        cli.is_lines,
        cli.is_chars,
        file_stats,
        &printable_file_name,
    );

    println!("{}", result);
}
