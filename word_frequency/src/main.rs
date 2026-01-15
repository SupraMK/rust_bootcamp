use clap::Parser;
use std::collections::HashMap;
use std::io::{self, BufRead};

#[derive(Parser)]
struct Args {
    text: Option<String>,

    #[arg(long)]
    ignore_case: bool,

    #[arg(long, default_value = "1")]
    min_length: usize,

    #[arg(long)]
    top: Option<usize>,
}

fn main() {
    let args = Args::parse();

    let text = match args.text {
        Some(t) => t,
        None => {
            let mut input = String::new();
            for line in io::stdin().lock().lines() {
                input.push_str(&line.unwrap());
                input.push(' ');
            }
            input
        }
    };

    let text = if args.ignore_case {
        text.to_lowercase()
    } else {
        text
    };

    let mut counts: HashMap<String, u32> = HashMap::new();

    for word in text.split_whitespace() {
        if word.len() >= args.min_length {
            *counts.entry(word.to_string()).or_insert(0) += 1;
        }
    }

    let mut sorted: Vec<(String, u32)> = counts.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    let results: Vec<(String, u32)> = match args.top {
        Some(n) => sorted.into_iter().take(n).collect(),
        None => sorted,
    };

    for (word, count) in results {
        println!("{}: {}", word, count);
    }
}
