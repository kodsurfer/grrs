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

    find_matches(&content, &args.pattern, &mut std::io::stdout());
    info!("ended");
}

fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            println!("{}", line);
        }
    }
}

#[test]
fn find_a_match() {
    let mut res = Vec::new();
    find_matches("pam param pam", "param", &mut res);
    assert_eq!()
}