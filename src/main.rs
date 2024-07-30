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

  let file_name = cli.file_name;

    let contents = match file_name {
        Some(ref file_name) => fs::read_to_string(file_name).unwrap(),
        None => io::read_to_string(io::stdin()).unwrap(),
    };

    println!("{} {}", contents.len(), file_name.unwrap());
}
