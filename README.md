# tomli-rust

A production-quality Rust port of the Python `Tomli` library, built for the Port Mortem hackathon.

## Goals
- Preserve original behavior exactly.
- Provide a memory-safe, fast, and dependency-free TOML parser.
- Avoid regex in favor of high-performance custom byte parsers.

## Usage
```rust
use tomli_rust::parse;

fn main() {
    let toml_str = r#"
        [server]
        port = 8080
    "#;
    
    let ast = parse(toml_str).expect("Valid TOML");
    println!("{:#?}", ast);
}
```

## Architecture & Decisions
See `ARCHITECTURE.md`, `MIGRATION_PLAN.md`, and `DECISIONS.md` for deep dives into how and why the architecture evolved from Python to Rust.
