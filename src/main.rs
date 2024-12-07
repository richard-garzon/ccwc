use ccwc::FileStats;

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
fn main() {
    let cli = Cli::parse();

    let mut file_stats = FileStats::new(&cli.file_name.unwrap());

    file_stats.populate_data();

    // if all args passed, lwmc. if none, lwc
    println!(
        "l: {} w: {} c: {} m: {}",
        file_stats.lines, file_stats.words, file_stats.bytes, &file_stats.chars
    );
}
