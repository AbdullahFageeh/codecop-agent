# \# codecop-agent

# 

# A simple Rust CLI that sends project files to Groq and answers questions about the codebase.

# 

# \## What it does

# 

# This project reads local project files, sends them to the Groq Chat Completions API, and returns an answer based on the code it sees.

# 

# Right now, it reads:

# 

# \- `Cargo.toml`

# \- `src/main.rs`

# 

# Example questions:

# 

# \- `what dependencies does this project use?`

# \- `explain this project`

# \- `explain src/main.rs step by step for a beginner`

# 

# \## Requirements

# 

# Before running the project, make sure you have:

# 

# \- Rust and Cargo installed

# \- a Groq API key

# \- the API key saved as an environment variable named `GROQ\_API\_KEY`

# 

# \## Run

# 

# From the project folder, run:

# 

# ```powershell

# cargo run -- "explain this project"

# ```

# 

# Another example:

# 

# ```powershell

# cargo run -- "what dependencies does this project use?"

# ```

# 

# \## Current dependencies

# 

# This project currently uses:

# 

# \- `reqwest`

# \- `serde\_json`

# 

# \## Notes

# 

# This is an early version of the tool.

# 

# Current limitations:

# 

# \- it only reads `Cargo.toml` and `src/main.rs`

# \- it does not yet scan all files in the project

# \- it sends the file contents directly in the prompt

# 

# \## Next ideas

# 

# Possible next improvements:

# 

# \- read all files inside `src/`

# \- skip large or unnecessary files

# \- improve prompt quality

# \- add better error handling

\- support more project types

## Vision
===

# 

# The goal of this project is to evolve from a simple Rust code-questioning CLI into a more capable AI agent.

# 

# Target direction:

# 

# \- Warp AI / Manus AI style task execution, planning, and orchestration

# \- Hermes-Agent style memory, tool use, and self-improving skills

# \- OpenClaw style local-first agent ideas and broader assistant capabilities

# 

# The plan is to build this step by step, starting from a narrow coding agent and gradually adding planning, tools, memory, and automation.

# 

# \## Author

# 

# Abdullah Fageeh

