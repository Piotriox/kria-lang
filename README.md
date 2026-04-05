# Kria Programming Language

A custom programming language written in Rust.

## Features

- File extension: `.krx`
- Interpreted language
- Dynamic + strong typing
- Variable assignment with `set`
- Arithmetic operations (+, -, *, /)
- Print function
- Null handling for undefined variables

## Building

Ensure you have Rust installed. Then:

```bash
cargo build
```

## Running

```bash
cargo run test.krx
```

## Installation

### Windows (creates kria-setup.exe)
```powershell
# Requires NSIS: https://nsis.sourceforge.io/
powershell scripts/build-windows.ps1
# Run kria-setup.exe to install
```

### Linux/macOS (creates kria-setup.sh)
```bash
./scripts/build-linux.sh
sudo ./dist/kria-setup.sh /usr/local
```

After installation, use:
```bash
kria run test.krx
```

## License
MIT