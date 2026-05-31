use reqwest::blocking::Client;
use serde_json::json;
use std::env;
use std::fs;
use std::path::Path;

fn should_include_file(path: &Path) -> bool {
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        matches!(ext, "rs" | "toml" | "md")
    } else {
        false
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!("Usage: cargo run -- \"your question\"");
        std::process::exit(1);
    }

    let prompt = args.join(" ");
    let mut project_files = String::new();

    let cargo_toml =
        fs::read_to_string("Cargo.toml").unwrap_or_else(|_| "Could not read Cargo.toml".to_string());
    project_files.push_str(&format!("--- Cargo.toml ---\n{}\n\n", cargo_toml));

    if let Ok(entries) = fs::read_dir("src") {
        for entry in entries.flatten() {
            let path = entry.path();

            if path.is_file() && should_include_file(&path) {
                let file_name = path.display().to_string();

                let metadata = fs::metadata(&path);
                if let Ok(meta) = metadata {
                    if meta.len() > 20_000 {
                        project_files.push_str(&format!(
                            "--- {} ---\nSkipped: file too large\n\n",
                            file_name
                        ));
                        continue;
                    }
                }

                let content = fs::read_to_string(&path)
                    .unwrap_or_else(|_| format!("Could not read {}", file_name));

                project_files.push_str(&format!("--- {} ---\n{}\n\n", file_name, content));
            }
        }
    }

    let readme =
        fs::read_to_string("README.md").unwrap_or_else(|_| "Could not read README.md".to_string());
    project_files.push_str(&format!("--- README.md ---\n{}\n\n", readme));

    let full_prompt = format!(
        "User question: {prompt}\n\nProject files:\n\n{project_files}"
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