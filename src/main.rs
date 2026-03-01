#[macro_use]
pub mod utils;

mod ast;
mod lexer;
mod parser;
mod dbg;
mod engine;
mod memory;

use std::env;
use std::fs;
use std::process;
use std::time;
use std::time::Instant;

use crate::lexer::Scanner;
use crate::parser::Parser;


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