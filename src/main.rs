#![warn(clippy::all, clippy::pedantic)]

mod parser;

use clap::Parser;
use std::path::PathBuf;

use parser::parse_vtt_file;

#[derive(Parser)]
#[clap(author,version,about,long_about=None)]
struct Cli {
    path: PathBuf,

    #[clap(value_parser)]
    #[clap(short, long)]
    output: std::path::PathBuf,
}

fn main() {
    let cli = &Cli::parse();
    let input_path = cli.path.as_path();
    let output_path = cli.output.as_path();
    parse_vtt_file(input_path, output_path, false);
}
