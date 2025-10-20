use std::{
    io::Read,
    process::exit,
    time::{Duration, Instant},
};

use clap::Parser;

use crate::{cli::Cli, terminal::highlight};

mod cli;
mod terminal;

fn main() {
    let cli = Cli::parse();

    if cli.pattern.is_none() {
        println!("Nothing to search.");
        exit(1);
    }

    let text = cli.pattern.unwrap();
    let show_line_number = cli.show_line_numbers;
    let mut content = String::new();
    std::io::stdin().read_to_string(&mut content).unwrap();

    if content.is_empty() {
        exit(0);
    }

    let lines = content.lines();
    let hightlight_color = cli.hightlight_color.as_str();

    let start_timestamp = Instant::now();
    for (id, line) in lines.enumerate() {
        if line.contains(&text) {
            let modified_display =
                line.replace(&text, highlight(&text, hightlight_color.into()).as_str());
            println!(
                "{}{modified_display}",
                if show_line_number {
                    format!(" {}: ", id + 1)
                } else {
                    String::new()
                }
            );
        }
    }
    let elapsed = start_timestamp.elapsed().as_millis();
    if cli.metrics {
        println!("Done in {} ms", elapsed);
    }
}
