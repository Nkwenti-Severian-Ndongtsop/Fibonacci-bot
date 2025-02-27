use reqwest::blocking::Client;
use serde_json::Value;
use std::env;
use std::error::Error;

pub fn get_latest_pr_content() -> Result<String, Box<dyn Error>> {
    let repo = env::var("GITHUB_REPOSITORY")?;
    let token = env::var("GITHUB_TOKEN")?;

    let url = format!("https://api.github.com/repos/{}/pulls?state=open&sort=updated&direction=desc", repo);

    let client = Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "Rust PR Fetcher")
        .header("Authorization", format!("Bearer {}", token))
        .send()?
        .text()?;

    let json: Value = serde_json::from_str(&response)?;
    
    if let Some(pr) = json.as_array().and_then(|arr| arr.first()) {
        if let Some(body) = pr["body"].as_str() {
            return Ok(body.to_string());
        }
    }

    Err("No pull request body found".into())
}