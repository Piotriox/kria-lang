# Kria Programming Language

A custom programming language written in Rust.

## Features

- File extension: `.krx`
- Bytecode VM-based execution
- Dynamic + strong typing
- Variable assignment with `set`
- Arithmetic operations (+, -, *, /)
- Comparison operators: `==`, `!=`, `>`, `<`, `>=`, `<=`
- Logical operators: `and`, `or`, `not`
- Conditional branches: `if`, `elseif`, `else`
- Loops: `while`
- Block scope with `{}`
- Line comments using `//`
- Print function
- Null handling for undefined variables
- Newline-based statement termination (no semicolons)

## Architecture

Kria now compiles source into bytecode before execution using a simple stack-based VM.
The runtime pipeline is:

```text
Source → Lexer → Parser → Compiler → Bytecode VM
```

## Building

Ensure you have Rust installed. Then:

```bash
cargo build
```

## Running

```bash
cargo run -- test.krx
```

Example `test.krx`:
```kria
set x = 5
set x = 4
set y = 3
set text = "test"
print(x + y)
print(text)

set mynum = true
if (mynum == true) {
   print("Mynum is true")
} elseif (mynum == false) {
   print("Mynum is false")
} else {
   print("mynum must be a boolean")
}

set counter = 0
while (counter < 5) {
    print(counter)
    set counter = counter + 1
}

// Line comments start with // and continue to the end of the line.
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
kria test.krx
```

## License
MIT