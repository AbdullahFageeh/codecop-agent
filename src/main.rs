use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
struct Cli {
    prompt: String,

    #[arg(short, long)]
    file: Vec<String>,

    #[arg(long)]
    scan_src: bool,
}

fn should_include_file(path: &Path) -> bool {
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        matches!(ext, "rs" | "toml" | "md")
    } else {
        false
    }
}

fn collect_files(dir: &Path, files: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();

            if path.is_dir() {
                collect_files(&path, files);
            } else if path.is_file() && should_include_file(&path) {
                files.push(path);
            }
        }
    }
}

fn append_file_to_context(path: &Path, context: &mut String) {
    let file_name = path.display().to_string();

    if let Ok(meta) = fs::metadata(path) {
        if meta.len() > 20_000 {
            context.push_str(&format!(
                "\n\n--- {} ---\nSkipped: file too large",
                file_name
            ));
            return;
        }
    }

    if let Ok(content) = fs::read_to_string(path) {
        context.push_str(&format!("\n\n--- {} ---\n{}", file_name, content));
    } else {
        context.push_str(&format!(
            "\n\n--- {} ---\nCould not read file",
            file_name
        ));
    }
}

fn read_project_files(extra_files: &[String], scan_src: bool) -> String {
    let mut context = String::new();

    for file in ["Cargo.toml", "README.md", "src/main.rs"] {
        append_file_to_context(Path::new(file), &mut context);
    }

    if scan_src {
        let mut src_files = Vec::new();
        collect_files(Path::new("src"), &mut src_files);

        for path in src_files {
            if path != PathBuf::from("src/main.rs") {
                append_file_to_context(&path, &mut context);
            }
        }
    }

    for file in extra_files {
        append_file_to_context(Path::new(file), &mut context);
    }

    context
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let api_key = std::env::var("GROQ_API_KEY").expect("GROQ_API_KEY not set");
    let context = read_project_files(&args.file, args.scan_src);

    let full_prompt = format!(
        "Here are the project files:\n{}\n\nQuestion: {}",
        context, args.prompt
    );

    let client = reqwest::Client::new();
    let body = serde_json::json!({
        "model": "llama-3.3-70b-versatile",
        "messages": [
            {
                "role": "system",
                "content": "You are a Rust codebase assistant. Answer questions based on the provided files."
            },
            {
                "role": "user",
                "content": full_prompt
            }
        ]
    });

    let res = client
        .post("https://api.groq.com/openai/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&body)
        .send()
        .await
        .expect("Request failed");

    let json: serde_json::Value = res.json().await.expect("Bad JSON");
    let answer = &json["choices"][0]["message"]["content"];
    println!("{}", answer);
}