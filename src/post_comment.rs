use reqwest::Client;
use std::env;

async fn post_comment(pr_content: &str) -> Result<(), reqwest::Error> {
    let repo = env::var("GITHUB_REPOSITORY").expect("GITHUB_REPOSITORY not set");
    let pr_number = env::var("PR_NUMBER")
        .expect("PR_NUMBER not set")
        .parse::<u32>()
        .expect("Invalid PR_NUMBER");

    let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");

    let url = format!(
        "https://api.github.com/repos/{}/issues/{}/comments",
        repo, pr_number
    );

    let client = Client::new();
    let response = client
        .post(&url)
        .header("Authorization", format!("token {}", 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))
        .header("User-Agent", "FibBot")
        .json(&serde_json::json!({ "body": pr_content }))
        .send()
        .await?;

    if response.status().is_success() {
        println!("✅ Comment posted successfully.");
    } else {
        eprintln!("❌ Failed to post comment: {:?}", response.text().await?);
    }

    Ok(())
}
