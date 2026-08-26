use std::collections::HashMap;
use std::env;
use std::fs;

pub mod lexer;
pub mod ast;
pub mod parser;
pub mod typecheck;
pub mod elaborator;
pub mod emitter;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 || args[1] != "compile" || args[3] != "-o" {
        eprintln!("Usage: scene_dsl compile input.dsl -o output.json");
        std::process::exit(1);
    }
    
    let input_file = &args[2];
    let output_file = &args[4];
    
    let source = fs::read_to_string(input_file).expect("Failed to read input file");
    
    // 1. Lex
    let tokens = lexer::lex(&source).expect("Lexer error");
    
    // 2. Parse
    let ast = parser::parse(&tokens).expect("Parser error");
    
    // 3. Typecheck
    typecheck::typecheck(&ast).expect("Typecheck error");
    
    // 4. Elaborate
    let ir = elaborator::elaborate(&ast).expect("Elaboration error");
    
    // 5. Emit
    let json = emitter::emit(&ir);
    
    fs::write(output_file, json).expect("Failed to write output file");
}

