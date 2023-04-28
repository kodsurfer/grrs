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

    grrs::find_matches(&content, &args.pattern, &mut std::io::stdout());
    info!("ended");
}

#[test]
fn find_a_match() {
    let mut res = Vec::new();
    grrs::find_matches("pam param pam", "param", &mut res);
    assert_eq!(res, b"pam param pam\n")
}