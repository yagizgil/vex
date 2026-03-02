#[macro_use]
pub mod utils;

mod ast;
mod dbg;
mod engine;
mod lexer;
mod memory;
mod parser;

use std::env;
use std::fs;
use std::path::Path;
use std::process;
use std::time;
use std::time::Instant;



fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        help();
        return;
    }

    let command = args[1].as_str();

    match command {
        "run" | "check" | "build" => {
            raise_if_no_file(&args, command);
            let file_path = &args[2];
            let _source = read_file(file_path);
        }

        "dbg" => {
            if args.len() < 3 {
                eprintln!("Usage: vex dbg [-lex] [-pars] <file.vx>");
                process::exit(1);
            }

            dbg::lexpars(&args);
        }

        #[cfg(feature = "inspector")]
        "inspect" => {
            use crate::engine::Interpreter;
            use crate::engine::Resolver;
            use crate::lexer::Scanner;
            use crate::parser::Parser;

            raise_if_no_file(&args, "inspect");
            crate::utils::logger::REPORT_ENABLED.store(true, std::sync::atomic::Ordering::Relaxed);

            let file_path = &args[2];
            let source = read_file(file_path);

            let mut scanner = Scanner::new(source);
            let tokens = scanner.scan_tokens();

            let mut parser = Parser::new(tokens);
            let mut _ast = parser.parse();

            let mut resolver = Resolver::new();
            resolver.resolve_statements(&mut _ast);

            let mut interpreter = Interpreter::new();
            interpreter.interpret(&_ast);

            let file_name = Path::new(file_path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("output");

            let output_name = format!("{}_inspect.json", file_name);

            inspect_dump!(&output_name);
        }

        _ => {
            if command.ends_with(".vx") {
                let _source = read_file(command);
                println!("Vex: Running shortcut for '{}'...", command);
            } else {
                eprintln!("Unknown command: {}", command);
                help();
            }
        }
    }
}

fn help() {
    println!("Vex Programming Language v0.1.0");
    println!("\nUsage:");
    println!("  vex run <file.vx>    : Execute a Vex script");
    println!("  vex check <file.vx>  : Check for syntax errors");
    println!("  vex build <file.vx>  : Build a Vex script");
    println!("  vex dbg [flags] <file.vx> : Debug tools (-lex, -pars)");
    println!("  vex <file.vx>        : Shortcut for run");
}

pub fn read_file(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("Vex Error: Could not read file '{}'", path);
        eprintln!("Reason: {}", e);
        process::exit(1);
    })
}

fn raise_if_no_file(args: &[String], command: &str) {
    if args.len() < 3 {
        eprintln!("Error: No .vx file specified for '{}'", command);
        process::exit(1);
    }
}
