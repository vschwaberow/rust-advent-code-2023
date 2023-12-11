use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Error: Please provide a filename as a command line argument");
        return Ok(());
    }

    let file = File::open(&args[1])?;
    let reader = BufReader::new(file);
    let mut sum = 0;

    for line in reader.lines() {
        let line = line?;
        let first_digit = line.chars().find(|c| c.is_digit(10));
        let last_digit = line.chars().rev().find(|c| c.is_digit(10));

        if let (Some(first), Some(last)) = (first_digit, last_digit) {
            if let Ok(num) = format!("{}{}", first, last).parse::<i32>() {
                sum += num;
                println!("Current sum: {}", sum);
            }
        }
    }

    println!("Sum of calibration values: {}", sum);

    Ok(())
}
