# Kria Programming Language

A custom programming language interpreter written in Rust.

## Features

- File extension: `.krx`
- Interpreted language
- Modular architecture with lexer, parser, and interpreter

## Building

Ensure you have Rust installed. Then:

```bash
cargo build
```

## Running

```bash
cargo run
```

## Project Structure

- `src/main.rs` - Main executable entry point
- `src/lib.rs` - Core library with module declarations
- `src/lexer.rs` - Tokenization logic for .krx files
- `src/parser.rs` - AST parsing implementation
- `src/interpreter.rs` - Language execution engine

## License

MIT
