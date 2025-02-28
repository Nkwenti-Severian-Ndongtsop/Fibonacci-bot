use crate::extract_text::extract_numbers;
use crate::fibonacci::fibonacci;

pub fn process_pr_content_values(pr_content: &String) -> String {
    let numbers = extract_numbers(pr_content.to_string());

    if numbers.is_empty() {
        return "No numbers found in this pull_request.".to_string();
    }

    let mut response =
        String::from("#### Fibonacci output of each number in the pull_request is:\n");
    for &num in &numbers {
        let fib = fibonacci(num);
        response.push_str(&format!("- Fibonacci({}) = {}\n", num, fib));
    }

    response
}
