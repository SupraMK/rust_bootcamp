use clap::Parser;
use std::collections::HashMap;
use std::io::{self, Read};

#[derive(Parser)]
#[command(name = "wordfreq", about = "Count word frequency in text")]
struct Args {
    #[arg(value_name = "TEXT")]
    text: Option<String>,

    #[arg(long, default_value_t = 10)]
    top: usize,

    #[arg(long, default_value_t = 1)]
    min_length: usize,

    #[arg(long)]
    ignore_case: bool,
}

fn main() {
    let args = Args::parse();

    let input_text = match args.text {
        Some(t) => t,
        None => read_all_stdin(),
    };

    let frequencies = count_word_frequencies(&input_text, args.min_length, args.ignore_case);

    let mut list: Vec<(String, usize)> = Vec::new();
    for (word, count) in frequencies {
        list.push((word, count));
    }

    list.sort_by(|a, b| {
        let count_a = a.1;
        let count_b = b.1;

        let by_count = count_b.cmp(&count_a);
        if by_count != std::cmp::Ordering::Equal {
            return by_count;
        }

        a.0.cmp(&b.0)
    });

    let n = args.top;
    println!("Top {} words:\n", n.min(list.len()));

    for i in 0..list.len() {
        if i >= n {
            break;
        }
        let (word, count) = &list[i];
        println!("{}: {}", word, format_number(*count));
    }
}

fn read_all_stdin() -> String {
    let mut text = String::new();
    io::stdin()
        .read_to_string(&mut text)
        .expect("Failed to read stdin");
    text
}

fn count_word_frequencies(text: &str, min_length: usize, ignore_case: bool) -> HashMap<String, usize> {
    let mut map: HashMap<String, usize> = HashMap::new();

    let mut current_word = String::new();

    for ch in text.chars() {
        if ch.is_alphanumeric() {
            current_word.push(ch);
        } else {
            add_word_if_valid(&mut map, &current_word, min_length, ignore_case);
            current_word.clear();
        }
    }

    add_word_if_valid(&mut map, &current_word, min_length, ignore_case);

    map
}

fn add_word_if_valid(
    map: &mut HashMap<String, usize>,
    word: &str,
    min_length: usize,
    ignore_case: bool,
) {
    if word.is_empty() {
        return;
    }

    if word.chars().count() < min_length {
        return;
    }

    let final_word = if ignore_case {
        word.to_lowercase()
    } else {
        word.to_string()
    };

    let entry = map.entry(final_word).or_insert(0);
    *entry += 1;
}

fn format_number(n: usize) -> String {
    let s = n.to_string();
    let mut result = String::new();

    let mut count_from_end = 0;
    for ch in s.chars().rev() {
        if count_from_end == 3 {
            result.push(',');
            count_from_end = 0;
        }
        result.push(ch);
        count_from_end += 1;
    }

    result.chars().rev().collect()
}