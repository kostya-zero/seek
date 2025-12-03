use std::{io::Read, process::exit, time::Instant};

use clap::Parser;
use serde::Serialize;

use crate::{
    cli::Cli,
    terminal::{HighlightColor, highlight_to_code, print_error},
};

mod cli;
mod terminal;

fn main() {
    let cli = Cli::parse();

    let pattern = cli.pattern.trim();
    if pattern.is_empty() {
        print_error("Empty pattern.");
        exit(1);
    }

    let mut content = String::new();
    std::io::stdin().read_to_string(&mut content).unwrap();

    if content.is_empty() {
        exit(0);
    }

    if cli.json {
        print_json(pattern, &content);
    } else {
        print_terminal(pattern, &cli, &content);
    }
}

fn print_terminal(pattern: &str, cli: &Cli, content: &str) {
    let highlight_color: HighlightColor = cli.hightlight_color.as_str().into();
    let highlight = highlight_to_code(&highlight_color);
    let reset = "\x1b[0m";

    let start_timestamp = Instant::now();

    for (id, line) in content.lines().enumerate() {
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

fn print_json(pattern: &str, content: &str) {
    let mut results: Vec<SearchResult> = Vec::new();

    for (idx, line) in content.lines().enumerate() {
        if !line.contains(pattern) {
            continue;
        }
        for (index, _) in line.match_indices(pattern) {
            let result = SearchResult {
                line_number: idx + 1,
                first_character: index,
                last_character: pattern.len() + index,
            };

            results.push(result);
        }
    }

    let output = serde_json::to_string(&results).unwrap();
    println!("{output}");
}
