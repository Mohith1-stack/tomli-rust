# Architecture of Python Tomli

## Module Responsibilities
- `__init__.py`: Provides the public API (`loads`, `load`, `TOMLDecodeError`).
- `_parser.py`: The core parsing engine. It handles reading characters, managing state, checking syntax rules, and creating the final output structure. Contains the `Flags` and `NestedDict` structures for managing state and immutability.
- `_re.py`: Regex definitions and parsing for datetimes, local times, and numbers (binary, octal, hex, and floating-point). Maps regex matches to native types.
- `_types.py`: Minimal type aliases (`ParseFloat`, `Key`, `Pos`) to aid type checking.

## Parser & Tokenizer Workflow
Tomli does **not** have a traditional multi-pass lexer/tokenizer. Instead, it is a single-pass scanner that operates directly on the source string.
- It iterates character-by-character using an integer index (`pos`).
- It uses structural pattern matching on individual characters (e.g., `if char == '['`) and string slicing/`startswith` for lookaheads.
- Helper functions like `skip_chars` and `skip_until` advance the `pos` index.

## Data Flow
1. **Input**: A string (or bytes decoded to string).
2. **State Management**: The parser initializes an `Output` object, which contains:
   - `NestedDict`: A mutable tree of dictionaries/lists that builds the final data structure.
   - `Flags`: A nested structure tracking immutability (e.g., inline tables and arrays cannot be mutated once closed, namespaces cannot be redefined).
3. **Loop**: The main loop in `loads()` processes statements (key-value pairs, table headers `[table]`, or array-of-table headers `[[array]]`) one by one.
4. **Values**: Value parsing is recursively dispatched based on the initial character of the value.
5. **Output**: Once the end of the document is reached, the underlying `dict` inside `NestedDict` is returned.

## Dependencies
Tomli is completely dependency-free, relying only on the standard library:
- `re` for complex value matching (datetimes, numbers).
- `datetime` for date and time objects.
- `sys` to get recursion limits.
- `collections.abc` / `typing` for type hints.

## Error Handling
- Uses a custom `TOMLDecodeError` (extending `ValueError`).
- Parsing errors store the original document and the failure position (`pos`).
- The `lineno` and `colno` are dynamically computed based on `\n` characters before the error position.

## Important Algorithms
- **Recursion Limits**: To prevent crash-inducing stack overflows (especially related to `mypyc`), nested inline arrays/tables are capped by `sys.getrecursionlimit()`.
- **Immutability Flags**: `Flags.is_` recursively traverses the parsed key hierarchy to ensure TOML immutability rules (like not adding keys to inline tables or redefining tables) are properly enforced.
- **Lazy Module Loading**: `_re.py` is loaded lazily since it contains expensive regex compilations, improving start time if a document doesn't contain complex values.
