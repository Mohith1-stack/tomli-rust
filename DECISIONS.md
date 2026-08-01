# Architectural Decisions for `tomli-rust`

## 1. Modular Lexer/Tokenizer vs Monolithic Scanner
- **Why Python differs**: Python's `_parser.py` implements the scanner and parser simultaneously within a single monolithic file (over 800 lines). The text index position (`pos`) is manually advanced and passed across dozens of uncoupled helper functions.
- **Why Rust changed**: We broke this into `lexer.rs`, `tokenizer.rs`, and `parser.rs`. The `Lexer<'a>` struct explicitly and mutably tracks its own position state over `&'a str`.
- **Tradeoffs**: It introduces slight architectural overhead to maintain the extra structural boundaries between the modules, but provides robust memory safety guarantees and prevents out-of-bounds indexing panics.
- **Rejected alternatives**: Passing raw `&[u8]` slices and `&mut usize` pointers into every parsing function manually (like C). This was rejected because it is highly unergonomic and error-prone in Rust.

## 2. Hand-Rolled RFC 3339 Parser vs Regex
- **Why Python differs**: Python uses the standard library `re` module to compile expansive patterns like `RE_DATETIME` lazily on first execution.
- **Why Rust changed**: We implemented a zero-allocation byte slice parser in `datetime.rs` that checks positional digits in-place using iterators. 
- **Tradeoffs**: Hand-rolled parsers take longer to write and verify against edge cases, but they run significantly faster than `regex` (improving throughput and latency), and they completely eliminate a heavyweight crate dependency.
- **Rejected alternatives**: Using the `regex` crate or the `chrono` crate. Rejected to keep the library 100% dependency-free and lightweight, aligning with the requirement to prevent over-engineering.

## 3. `BTreeMap` for TOML Tables
- **Why Python differs**: Python's native `dict` maintains insertion order automatically since Python 3.7, meaning the parser naturally yields stable dictionaries.
- **Why Rust changed**: Rust's standard `HashMap` does not guarantee order and its iteration order is actively randomized (using SipHash) to prevent DoS attacks. We opted for `BTreeMap` in `value.rs`.
- **Tradeoffs**: `BTreeMap` has slightly slower insertions (`O(log n)`) compared to `HashMap` (`O(1)`), but provides deterministic iteration which is vital for passing differential fuzzing tests reliably and generating stable debugging output.
- **Rejected alternatives**: `HashMap` (rejected due to randomized instability), or pulling in the `indexmap` crate (rejected to avoid external dependencies).

## 4. `TomlError` Encapsulation
- **Why Python differs**: Raises `TOMLDecodeError` which inherits from `ValueError`, parsing line numbers via string manipulation at instantiation.
- **Why Rust changed**: We return a distinct `TomlError` struct that implements `std::error::Error`, returning it safely down the call stack using `Result<T, TomlError>`.
- **Tradeoffs**: Rust's method requires threading `Result` everywhere using the `?` operator, whereas Python exceptions bubble up automatically. However, Rust's explicit handling guarantees memory safety and eliminates uncaught exceptions crashing the runtime.
- **Rejected alternatives**: Using `panic!` on bad TOML. Strictly forbidden by Rust library standards for expected validation failures.
