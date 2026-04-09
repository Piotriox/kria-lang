use std::env;
use std::fs;
use std::process;

use kria::lexer::Lexer;
use kria::parser::Parser;
use kria::compiler::Compiler;
use kria::vm::VM;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <filename.krx>", args[0]);
        process::exit(1);
    }
    
    let filename = &args[1];
    
    // Read file
    let source = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", filename, e);
            process::exit(1);
        }
    };
    
    // Lexer: tokenize
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize();
    
    // Parser: parse into AST
    let mut parser = Parser::new(tokens);
    let statements = match parser.parse() {
        Ok(stmts) => stmts,
        Err(e) => {
            eprintln!("Parse error: {}", e);
            process::exit(1);
        }
    };
    
    // Compiler: generate bytecode from AST
    let compiler = Compiler::new();
    let bytecode = match compiler.compile(&statements) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Compile error: {}", e);
            process::exit(1);
        }
    };

    // VM: execute bytecode
    let mut vm = VM::new();
    if let Err(e) = vm.execute(&bytecode) {
        eprintln!("Runtime error: {}", e);
        process::exit(1);
    }
}
