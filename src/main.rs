use crate::{
    extract_text::extract_numbers, fibonacci::fibonacci,
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

    // let pr_number = env::var("PR_NUMBER")
    //     .expect("PR_NUMBER not set")
    //     .parse::<u32>()
    //     .expect("Invalid PR_NUMBER");

        let github_repository = env::var("GITHUB_REPOSITORY").unwrap_or_else(|_| "Nkwenti-Severian-Ndongtsop/Fibonacci-bot".to_string());
        let github_repository=  github_repository.split("/").collect::<Vec<&str>>();
        let owner = github_repository[0];
        let repo = github_repository[1];

        let pull_request = octocrab::instance().pulls( owner,  repo).list_files(1).await.expect("not found");
        let pr_content = &pull_request.items.first().unwrap().patch.clone().unwrap();
        
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
mod post_comment_to_github;
mod process_pr_result;
mod test;
