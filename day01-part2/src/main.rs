use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

const NUMS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Error: Please provide a filename as a command line argument");
        return Ok(());
    }

    let file = File::open(&args[1])?;
    let reader = BufReader::new(file);

    let total: usize = reader
        .lines()
        .map_while(Result::ok)
        .map(|line| calculate_total(&line))
        .sum();

    println!("{}", total);

    Ok(())
}

fn calculate_total(line: &str) -> usize {
    let mut total = 0;
    let bytes = line.as_bytes();
    let _len = bytes.len();

    let first_digit = find_first_digit(bytes);
    let last_digit = find_last_digit(bytes);

    if let (Some(first), Some(last)) = (first_digit, last_digit) {
        let two_digit_number = combine_digits(first, last);
        total += two_digit_number;
    }

    total
}

fn find_first_digit(line: &[u8]) -> Option<usize> {
    for i in 0..line.len() {
        if let Some(num) = num(line, i) {
            return Some(num);
        }
    }
    None
}

fn find_last_digit(line: &[u8]) -> Option<usize> {
    for i in (0..line.len()).rev() {
        if let Some(num) = num(line, i) {
            return Some(num);
        }
    }
    None
}

fn num(line: &[u8], i: usize) -> Option<usize> {
    if line[i].is_ascii_digit() {
        Some((line[i] - b'0') as usize)
    } else {
        NUMS.iter()
            .enumerate()
            .find(|(_, name)| line[i..].starts_with(name.as_bytes()))
            .map(|(num, _)| num + 1)
    }
}

fn combine_digits(first: usize, last: usize) -> usize {
    first * 10 + last
}
