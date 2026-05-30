use reqwest::blocking::Client;
use serde_json::json;
use std::env;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!("Usage: cargo run -- \"your question\"");
        std::process::exit(1);
    }

    let prompt = args.join(" ");

    let cargo_toml =
        fs::read_to_string("Cargo.toml").unwrap_or_else(|_| "Could not read Cargo.toml".to_string());
    let main_rs =
        fs::read_to_string("src/main.rs").unwrap_or_else(|_| "Could not read src/main.rs".to_string());

    let full_prompt = format!(
        "User question: {prompt}\n\nProject files:\n\n--- Cargo.toml ---\n{cargo_toml}\n\n--- src/main.rs ---\n{main_rs}"
    );

    let api_key = env::var("GROQ_API_KEY")?;
    let client = Client::new();

    let body = json!({
        "model": "llama-3.1-8b-instant",
        "messages": [
            {
                "role": "system",
                "content": "You explain Rust projects clearly and briefly."
            },
            {
                "role": "user",
                "content": full_prompt
            }
        ]
    });

    let json_response: serde_json::Value = client
        .post("https://api.groq.com/openai/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&body)
        .send()?
        .json()?;

    let answer = &json_response["choices"][0]["message"]["content"];

    println!("{}", answer.as_str().unwrap_or("No response text found"));

    Ok(())
}