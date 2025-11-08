use std::{io::Read, process::exit, time::Instant};

use clap::Parser;
use serde::Serialize;

use crate::{
    cli::Cli,
    terminal::{HighlightColor, highlight_to_code},
};

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
    let highlight_color: HighlightColor = cli.hightlight_color.as_str().into();
    let highlight = highlight_to_code(&highlight_color);
    let reset = "\x1b[0m";

    let start_timestamp = Instant::now();

    for (id, line) in lines.iter().enumerate() {
        if !line.contains(pattern) {
            continue;
        }
        let mut last = 0;
        let mut out = String::with_capacity(line.len() + 32);

        for (idx, _) in line.match_indices(pattern) {
            let end = idx + pattern.len();
            out.push_str(&line[last..idx]);
            out.push_str(highlight);
            out.push_str(&line[idx..end]);
            out.push_str(reset);
            last = end;
        }
        out.push_str(&line[last..]);

        println!(
            "{}{out}",
            if cli.show_line_numbers {
                format!(" {}: ", id + 1)
            } else {
                String::new()
            }
        );
    }

    if cli.metrics {
        let elapsed = start_timestamp.elapsed().as_millis();
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
