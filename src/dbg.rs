use std::time::Instant;
use std::process;

use crate::engine::Interpreter;
use crate::lexer::Scanner;
use crate::parser::Parser;

pub fn lexpars(args: &[String]) {
    let show_lex = args.iter().any(|a| a == "-lex");
    let show_pars = args.iter().any(|a| a == "-pars");
    let show_eng = args.iter().any(|a| a == "-eng");

    let file_path = args
        .iter()
        .skip(2) 
        .find(|a| !a.starts_with('-'))
        .unwrap_or_else(|| {
            eprintln!("Error: No .vx file specified.");
            process::exit(1);
        });

    let source = crate::read_file(file_path);
    let total_start = Instant::now();

    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    if show_lex {
        println!("\n[LEXER OUTPUT] for '{}':", file_path);
        println!("---------------------------------------");
        for token in &tokens {
            println!("{:?}", token);
        }
        print_timer("Lexer", total_start);
    }

    if show_pars || show_eng {
        let mut parser = Parser::new(tokens);
        let ast = parser.parse();

        if show_pars {
            println!("\n[PARSER AST OUTPUT] for '{}':", file_path);
            println!("---------------------------------------");
            for stmt in &ast {
                println!("{:#?}", stmt);
            }
            print_timer("Parser", total_start);
        }

        if show_eng {
            println!("\n[ENGINE EXECUTION]:");
            println!("---------------------------------------");
            let mut interpreter = Interpreter::new();

            interpreter.interpret(&ast);
            print_timer("Total Execution", total_start);
        }
    }

    if !show_lex && !show_pars && !show_eng {
        println!("Hint: Use -lex, -pars, or -eng flags to see debug information.");
    }
}

fn print_timer(label: &str, start: Instant) {
    let elapsed = start.elapsed();
    println!(
        "--- {} Finished: {}ms ({}μs) ---",
        label,
        elapsed.as_millis(),
        elapsed.as_micros()
    );
}