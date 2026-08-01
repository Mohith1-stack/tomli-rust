# Migration Plan: Python Tomli to Rust

## `tomli/__init__.py`
↓
**`src/lib.rs`**
- **Responsibilities**: Provide the public API (`parse` or `from_str`), exposing the `Value` type and `Error` types.
- **Potential Redesigns**: Instead of `load` taking a file-like object and `loads` taking a string, Rust will use idiomatic `from_str` for strings and `from_slice` for bytes (or just `from_str`), returning a `Result<Table, ParseError>`.
- **Ownership/Lifetimes**: The parser will operate on `&'a str` to avoid allocations where possible.

## `tomli/_parser.py`
This large Python file handles all lexical analysis, parsing, and tree construction. In Rust, it will be split into multiple modules:

### **`src/lexer.rs`**
- **Responsibilities**: Consume raw characters, handle UTF-8, and manage positional state. Skip whitespace and comments.
- **Ownership/Lifetimes**: Operates over `&'a str`.
- **Structs**: `Lexer<'a>` containing the source string and current byte index.

### **`src/tokenizer.rs`**
- **Responsibilities**: Read basic TOML tokens from the lexer (strings, bare keys, operators like `[`, `]`, `{`, `}`, `=`).
- **Enums**: `Token<'a>` (e.g., `String(&'a str)`, `Equals`, `LBracket`, etc.).
- **Ownership/Lifetimes**: Yields tokens borrowing from the source string.

### **`src/parser.rs`**
- **Responsibilities**: Drive the tokenizer, enforce TOML structural rules, manage mutability flags (from `Flags`), and build the final document.
- **Potential Redesigns**: Replace Python's `NestedDict` and `Flags` with a robust Rust AST or DOM structure. We'll use a `Table` struct and handle immutability rules through state transitions rather than a secondary flag dictionary.
- **Structs**: `Parser<'a>`, `Table` (representing the root or nested tables).

### **`src/value.rs`**
- **Responsibilities**: Define the core TOML data types.
- **Enums**: `Value` (String, Integer, Float, Boolean, Datetime, Array, Table).
- **Traits**: `Display`, `Debug`, `PartialEq`.

### **`src/error.rs`**
- **Responsibilities**: Map `TOMLDecodeError` to a robust Rust error type.
- **Potential Redesigns**: Rust errors are enums. We will provide detailed variants (e.g., `InvalidString`, `DuplicateKey`) and attach line/column information.
- **Structs/Enums**: `ParseError`, implementing `std::error::Error`.

## `tomli/_re.py`
↓
**`src/datetime.rs`** & **`src/util.rs`** (for numbers)
- **Responsibilities**: Parse dates, times, and numeric formats (hex, octal, binary).
- **Potential Redesigns**: Python Tomli uses heavy regex (`_re.py`) to parse numbers and datetimes. In Rust, we will **avoid regex** completely. We will write hand-rolled parsers or use `std::str::parse` and a zero-allocation datetime parser to achieve benchmarkable performance.
- **Structs**: `Datetime`, `Date`, `Time`, `Offset`.
- **Traits**: `FromStr` for parsing datetimes.

## `tomli/_types.py`
- These types (`Key`, `Pos`, `ParseFloat`) will not be mapped directly to a single file.
- Rust's strong static typing eliminates the need for `_types.py`. Types will naturally reside in the modules where they are used.
