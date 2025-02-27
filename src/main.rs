use crate::{
    extract_text::extract_numbers, fibonacci::fibonacci, get_pull_request::get_pr_body,
    post_comment_to_github::post_comment, process_pr_result::process_pr_content_values,
};
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

    
    let pr_number = env::var("PR_NUMBER")
        .expect("PR_NUMBER not set")
        .parse::<u32>()
        .expect("Invalid PR_NUMBER");
    
    
    let pr_content = get_pr_body(pr_number).expect("Couldn't get the content of pull_request");
    let numbers = extract_numbers(pr_content.clone());
    println!("Extracted numbers: {:?}", numbers);
    
    let result = process_pr_content_values(pr_content);
    for number in numbers {
        let fib = fibonacci(number);
        println!("Fibonacci of {} is: {}", number, fib);
    }
    
    println!("{}", result);

    if let Err(e) = post_comment(&result).await {
        eprintln!("Error posting comment: {}", e);
    }
}

mod extract_text;
mod fibonacci;
mod get_pull_request;
mod post_comment_to_github;
mod process_pr_result;
mod test;
