use std::{io::Read, process::exit};

use clap::Parser;

use crate::cli::Cli;

mod cli;

fn main() {
    let cli = Cli::parse();

    if cli.pattern.is_none() || cli.pattern.clone().unwrap().is_empty() {
        println!("Nothing to search.");
        exit(1);
    }

    let text = cli.pattern.unwrap();
    let mut content = String::new();
    std::io::stdin().read_to_string(&mut content).unwrap();

    if content.is_empty() {
        exit(0);
    }

    let lines = content.lines();
    for (id, line) in lines.enumerate() {
        if line.contains(&text) {
            let modified = line.replace(&text, &format!("\x1b[91m\x1b[1m{text}\x1b[0m"));
            println!(" {}:  {modified}", id + 1);
        }
    }
}
