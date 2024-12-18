use ccwc::FileStats;
use std::fs::File;

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
    let mut result = String::new();

    if !(is_bytes || is_words || is_lines || is_chars) {
        result.push_str(&format!(
            "{} {} {} {}",
            file_stats.lines.to_string(),
            file_stats.words.to_string(),
            file_stats.bytes.to_string(),
            file_name
        ));
        return result;
    }

    if is_lines {
        result.push_str(&format!("  {}", file_stats.lines.to_string()))
    }
    if is_words {
        result.push_str(&format!("  {}", file_stats.words.to_string()))
    }
    if is_chars {
        result.push_str(&format!("  {}", file_stats.chars.to_string()))
    }
    if is_bytes {
        result.push_str(&format!("  {}", file_stats.bytes.to_string()))
    }

    result.push_str(&format!("  {}", file_name));

    return result;
}

fn main() {
    let cli = Cli::parse();
    let file_name = cli.file_name.unwrap();

    let mut file_stats = FileStats::new();
    let file = File::open(&file_name).unwrap();

    file_stats.populate_data(file);

    let result = build_string(
        cli.is_bytes,
        cli.is_words,
        cli.is_lines,
        cli.is_chars,
        file_stats,
        &file_name,
    );

    println!("{}", result);
}
