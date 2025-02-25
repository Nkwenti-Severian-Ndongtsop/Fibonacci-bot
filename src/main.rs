use crate::extract_text::extract_numbers;
use crate::fibonacci::fibonacci;
use crate::process_pr_result::process_pr_content;
use std::env;
use tokio;

#[tokio::main]
async fn main() {
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

    let result = process_pr_content(pr_content);

    println!("{}", result);

    let comment = process_pr_content(pr_content);

    if let Err(e) = post_comment(&comment).await {
        eprintln!("Error posting comment: {}", e);
    }
}

mod extract_text;
mod fibonacci;
mod post_comment;
mod process_pr_result;
mod test;
