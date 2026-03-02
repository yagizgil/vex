use std::process;
use std::time::Instant;

use crate::engine::Interpreter;
use crate::lexer::Scanner;
use crate::parser::Parser;
use crate::engine::Resolver;

use crate::utils::logger::error::Reporter;

pub fn lexpars(args: &[String]) {
    let _all = args.iter().any(|a| a == "-all");
    let show_lex = args.iter().any(|a| a == "-lex") || _all;
    let show_pars = args.iter().any(|a| a == "-pars") || _all;
    let show_eng = args.iter().any(|a| a == "-eng") || _all;
    let mut show_report: bool = false;

    let mut _report = || {
        if args.iter().any(|a| a == "-r") {
            show_report = true;
            Reporter::display();
        }
    };

    crate::utils::logger::REPORT_ENABLED.store(true, std::sync::atomic::Ordering::Relaxed);

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

    let mut resolver = Resolver::new();
    // resolver.begin_scope();

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
        let mut ast = parser.parse();
        resolver.resolve_statements(&mut ast);

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

    _report();

    let check_cli_usage = || {
    let flags = [show_lex, show_pars, show_eng, show_report, _all];
    
    if flags.iter().all(|&f| !f) {
        println!("--------------------------------------------------");
        println!("Hint: You didn't specify any output flags.");
        println!("   Try running with -all for full debug output.");
        println!("   Or use specific: -lex, -pars, -eng, -r");
        println!("--------------------------------------------------");
    }
};

// Program sonunda tek satırda çağır
check_cli_usage();
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
