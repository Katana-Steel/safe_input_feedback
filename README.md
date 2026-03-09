This is a silly prof of concept that you could have input feedback on a console prompt without flagging the length of that input.

# how to build
```
cmake -B build/ -G Ninja .
cmake --build build/
# run the POC
build/SIF_app
```
# prerequisites

## C++ build
* GNU Readline
* C++ compiler

## Rust build
* Rust toolchain (rustc, cargo)

# Rust implementation

A Rust rewrite is available in the `rust/` directory. It replaces the GNU Readline
dependency with `crossterm` for cross-platform terminal handling, and can be used
as both a standalone binary and a library crate.

```
cd rust
cargo build
cargo run
```

To use as a library in another Rust project:
```toml
[dependencies]
safe_input_feedback = { path = "path/to/rust" }
```
