use std::{fs, io};

use clap::Parser;

#[derive(Parser)]
#[command(name = "ccwc")]
#[command(about = "wc impl", long_about = None)]
struct Cli {
    file_name: Option<String>,

    #[arg(short = 'c', long = "")]
    is_bytes: bool,
}
fn main() {
    let cli = Cli::parse();

    let contents = match &cli.file_name {
        Some(file_name) => fs::read_to_string(file_name).unwrap(),
        None => io::read_to_string(io::stdin()).unwrap(),
    };

    println!("{} {}", contents.len(), cli.file_name.unwrap_or_default());
}
