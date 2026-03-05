use crate::args::CliArgs;
use crate::metrics::{MetricsCollector, PhaseMetrics};

/// CompilerApp manages the whole process: Loading -> Lexing -> Parsing.
pub struct CompilerApp {
    args: CliArgs,
    metrics: Vec<PhaseMetrics>,   // Stores time/memory info for each step
    collector: MetricsCollector, // Helper to measure performance
}

impl CompilerApp {
    pub fn new(args: CliArgs) -> Self {
        Self {
            args,
            metrics: Vec::new(),
            collector: MetricsCollector::new(),
        }
    }

    /// Start the compilation based on user arguments.
    pub fn run(&mut self) {
        let path_str = self.args.file_path.to_string_lossy().to_string();

        // 1. Loading
        if self.args.stats { self.collector.start_phase(); }
        let file_id = match vex_loader::Loader::load_file(&path_str) {
            Ok(id) => id,
            Err(e) => {
                eprintln!("Error loading file '{}': {}", path_str, e);
                return;
            }
        };
        if self.args.stats { self.metrics.push(self.collector.end_phase("Loading")); }

        // 2. Inspector (if requested and enabled)
        #[cfg(feature = "inspector")]
        if self.args.inspect {
            if let Err(e) = vex_inspector::InspectorApp::run(file_id) {
                eprintln!("Inspector error: {}", e);
            }
            return;
        }

        if self.args.inspect {
            #[cfg(not(feature = "inspector"))]
            {
                use colored::*;
                println!("{}", "Warning: Inspector requested but not compiled in.".yellow());
                println!("Recompile with: cargo run --features inspector ...");
            }
        }

        // 3. Lexing
        if self.args.stats { self.collector.start_phase(); }
        let content = vex_loader::Loader::get_content(file_id);
        let mut lexer = vex_lexer::Lexer::new(file_id, content);
        let tokens = lexer.tokenize();
        if self.args.stats { self.metrics.push(self.collector.end_phase("Lexing")); }

        // 4. PreParsing
        // if self.args.stats { self.collector.start_phase(); }
        // let mut preparser = vex_parser::preparser::PreParser::new(tokens);
        // let _refined_tokens = preparser.process();
        // if self.args.stats { self.metrics.push(self.collector.end_phase("PreParsing")); }

        if self.args.stats { self.collector.start_phase(); }
        let mut preparser = vex_parser::parser::Parser::new(tokens);
        let astt = preparser.parse();
        if self.args.stats { self.metrics.push(self.collector.end_phase("Parsing")); }

        let diag = vex_diagnostic::diag!();
        if diag.has_errors() {
            diag.print_all();
        } else {
            use colored::*;
            println!("{} Compilation successful for '{}'", "Vex:".green().bold(), path_str);
        }

        // 6. Print Stats if requested
        if self.args.stats {
            MetricsCollector::print_stats(&self.metrics);
        }
    }
}
