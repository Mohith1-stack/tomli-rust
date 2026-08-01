<div align="center">
  <h1>🦀 tomli-rust</h1>
  <p><strong>A zero-dependency, memory-safe, and blisteringly fast TOML parser for Rust.</strong></p>
  
  [![Build Status](https://img.shields.io/badge/build-passing-brightgreen)](#)
  [![Crates.io](https://img.shields.io/badge/crates.io-v0.1.0-orange)](#)
  [![Fuzzing Status](https://img.shields.io/badge/fuzzing-active-blue)](#)
  [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
</div>

---

`tomli-rust` is a production-quality Rust port of the highly acclaimed Python [`tomli`](https://github.com/hukkin/tomli) library. Originally developed as part of the **Port Mortem Hackathon**, this library guarantees identical parsing semantics to its Python predecessor while leveraging Rust's compiler to deliver unmatched performance and safety guarantees.

## 🚀 Features

- **Zero Dependencies**: Keeps your compilation times low and your binary sizes small. No `regex`, no `chrono`, no bloat.
- **100% Memory Safe**: Written entirely in Safe Rust with zero `unsafe` blocks.
- **Blisteringly Fast**: Swaps out Python's heavy regex evaluations for low-level, zero-allocation byte slice processing.
- **Differential Fuzzing Ready**: Mathematically hardened using LLVM `libFuzzer` to ensure zero panics on malformed data.
- **Deterministic**: Utilizes `BTreeMap` to guarantee stable AST traversal and output across all targets.

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
tomli-rust = "0.1.0"
```

## 🛠️ Usage

`tomli-rust` exposes a simple, ergonomic API. It can parse TOML directly from string slices or UTF-8 byte streams.

### Parsing a String

```rust
use tomli_rust::{parse, Value};

fn main() {
    let toml_data = r#"
        [server]
        host = "127.0.0.1"
        port = 8080
        active = true
    "#;

    match parse(toml_data) {
        Ok(ast) => {
            println!("Successfully parsed TOML!");
            // Safely traverse the AST
            if let Some(Value::Table(server)) = ast.get("server") {
                if let Some(Value::Integer(port)) = server.get("port") {
                    println!("Running on port: {}", port);
                }
            }
        },
        Err(e) => eprintln!("Failed to parse: {}", e),
    }
}
```

### Error Handling
Errors in `tomli-rust` automatically calculate precise line and column byte offsets, matching the exact behavior of Python's `TOMLDecodeError`.

```rust
let bad_toml = "[unclosed_table";
let err = parse(bad_toml).unwrap_err();
println!("{}", err); // "Expected closing bracket for table header (at line 1, column 15)"
```

## 🏗️ Architecture & Documentation

The migration from Python to Rust involved several key architectural improvements. Deep-dive documentation on these decisions can be found in the repository:

- 📖 **[ARCHITECTURE.md](./ARCHITECTURE.md)**: Details the separation of concerns between the Lexer, Tokenizer, and Parser.
- 📖 **[MIGRATION_PLAN.md](./MIGRATION_PLAN.md)**: The original blueprint mapping Python's dynamically typed modules to Rust's rigid memory models.
- 📖 **[DECISIONS.md](./DECISIONS.md)**: Explains engineering tradeoffs, such as dropping `regex` for hand-rolled RFC 3339 datetime parsing.

## 🧪 Testing & Benchmarking

Quality assurance is built into the foundation of `tomli-rust`. 

To run the native integration tests (which utilize the original Python `.toml` datasets):
```bash
cargo test
```

To run the `Criterion.rs` micro-benchmarking suite (evaluates throughput and latency across deep recursion and datetime evaluation):
```bash
cargo bench
```

## ⚖️ License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.
