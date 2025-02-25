use crate::extract_text::extract_numbers;
use crate::fibonacci::fibonacci;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let enable_fib = args.get(1).unwrap_or(&"true".to_string()).to_lowercase() == "true";
    let max_threshold: u8 = args
        .get(2)
        .unwrap_or(&"100".to_string())
        .parse()
        .unwrap_or(100);

    println!("FibBot application is running...");
    println!("Fibonacci Calculation Enabled: {}", enable_fib);
    println!("Max Threshold is: {}", max_threshold);

    let pr_content =
        "This pull_request fixes issue #42 and adds support for 7 new features. Also references ticket 13.";

    let numbers: Vec<u32> = extract_numbers(pr_content);

    println!("Extracted numbers: {:?}", numbers);

    let numbers = extract_numbers(pr_content);

    println!("Extracted numbers: {:?}", numbers);

    for number in numbers {
        let fib = fibonacci(number);
        println!("Fibonacci of {} is: {}", number, fib);
    }
}

mod extract_text;
mod fibonacci;
mod test;
