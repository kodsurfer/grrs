#![allow(unused)]

use clap::{arg, Parser};
use log::{info, warn};

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    env_logger::init();
    info!("start up");
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path).expect("could not read file");

    find_matches(&args, content);
    info!("ended");
}

fn find_matches(args: &args, content: String) {
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
