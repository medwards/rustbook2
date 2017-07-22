use std::env;
use std::collections::HashMap;
use std::process::exit;

fn main() {
    // TODO:
    // * negative values

    // Could have done this as a loop /w extra variables but wanted to see if it could do
    // it inline
    let input = env::args().skip(1).map(|x| {
                match x.parse() {
                    Ok(num) => num,
                    Err(_) => {
                        print_help();
                        exit(1);
                    }
                }
            }
        )
        .collect::<Vec<_>>();
    // for example this check could happen in the previouse "line" if it was a loop instead
    if input.len() == 0 {
        print_help();
        exit(1);
    }
    println!("mean: {}", mean(&input));
    println!("median: {}", median(&input));
    println!("mode: {}", mode(&input));
}

fn print_help() {
    println!("Usage: stats NUM [NUM [NUM ...]]");
    println!("NUM is a number (floating point or integer), at least one is required");
    println!("NUM cannot currently be negative");
}

fn mean(numbers: &Vec<f32>) -> f32 {
    // can overflow, looping version with checked_add is trivial though
    let sum: f32 = numbers.iter().sum();
    sum / numbers.len() as f32
}

fn median(numbers: &Vec<f32>) -> f32 {
    let median_index = (numbers.len() - 1) / 2;
    numbers[median_index]
}

fn mode(numbers: &Vec<f32>) -> f32 {
    let mut counter = HashMap::new();
    for number in numbers.iter() {
        // awkward cast to string since f32 can't be a key
        let count = counter.entry(format!("{}", number)).or_insert(0);
        *count += 1;
    }

    let mut max_key = &String::new();
    let mut max_value = &0;
    for (key, value) in counter.iter() {
        if value > max_value {
            max_value = value;
            max_key = key;
        }
    }

    max_key.parse::<f32>().unwrap()
}
