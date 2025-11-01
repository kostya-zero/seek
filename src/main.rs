use std::{io::Read, process::exit, time::Instant};

use clap::Parser;
use serde::Serialize;

use crate::{cli::Cli, terminal::highlight};

mod cli;
mod terminal;

fn main() {
    let cli = Cli::parse();

    if cli.pattern.is_none() {
        println!("Nothing to search.");
        exit(1);
    }

    let mut content = String::new();
    std::io::stdin().read_to_string(&mut content).unwrap();

    if content.is_empty() {
        exit(0);
    }

    let lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();
    if cli.json {
        print_json(&cli, &lines);
    } else {
        print_terminal(&cli, &lines);
    }
}

fn print_terminal(cli: &Cli, lines: &[String]) {
    let pattern = cli.pattern.as_ref().unwrap();
    let hightlight_color = cli.hightlight_color.as_str();
    let pattern_lenght = pattern.len();

    let start_timestamp = Instant::now();
    for (id, line) in lines.iter().enumerate() {
        for (index, _) in line.match_indices(pattern) {
            let end_index = index + pattern_lenght;
            let mut line_modified = line.clone();
            line_modified.insert_str(index, hightlight_color.into());
            line_modified.insert_str(end_index, "\x1b[0m");
            println!(
                "{}{line_modified}",
                if cli.show_line_numbers {
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

// Used only for JSON output.
#[derive(Serialize)]
struct SearchResult {
    line_number: usize,
    first_character: usize,
    last_character: usize,
}

fn print_json(cli: &Cli, lines: &[String]) {
    let mut results: Vec<SearchResult> = Vec::new();
    let pattern = cli.pattern.as_ref().unwrap();

    for (idx, line) in lines.iter().enumerate() {
        for (index, _) in line.match_indices(pattern) {
            let result = SearchResult {
                line_number: idx,
                first_character: index,
                last_character: pattern.len() + index,
            };

            results.push(result);
        }
    }

    let output = serde_json::to_string(&results).unwrap();
    println!("{output}");
}
