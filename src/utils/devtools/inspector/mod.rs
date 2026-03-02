use serde::Serialize;
use crate::lexer::token::Token;
use crate::ast::stmt::Stmt;

#[derive(Serialize, Clone, Debug)]
pub struct Snapshot {
    pub stage: String,
    pub message: String,
    pub tokens: Vec<Token>,
    pub ast: Vec<Stmt>,
}

#[derive(Serialize, Clone, Debug)]
pub struct InspectorHistory {
    pub snapshots: Vec<Snapshot>,
}

use std::sync::{Mutex, OnceLock};

static HISTORY: OnceLock<Mutex<InspectorHistory>> = OnceLock::new();

pub fn get_history() -> &'static Mutex<InspectorHistory> {
    HISTORY.get_or_init(|| Mutex::new(InspectorHistory { snapshots: Vec::new() }))
}

pub fn record(stage: &str, message: &str, tokens: &[Token], ast: &[Stmt]) {
    let mut history = get_history().lock().unwrap();
    history.snapshots.push(Snapshot {
        stage: stage.to_string(),
        message: message.to_string(),
        tokens: tokens.to_vec(),
        ast: ast.to_vec(),
    });
}


pub fn dump_to_file(filename: &str) {
    if let Ok(history) = get_history().lock() {
        match serde_json::to_string_pretty(&*history) {
            Ok(json) => {
                if std::fs::write(filename, json).is_ok() {
                    println!(
                        "\n\x1b[32m[VEX]\x1b[0m Inspection data successfully exported to: \x1b[34m'{}'\x1b[0m", 
                        filename
                    );
                } else {
                    eprintln!("\n\x1b[31m[ERROR]\x1b[0m Failed to write inspection data to '{}'", filename);
                }
            }
            Err(e) => eprintln!("\n\x1b[31m[ERROR]\x1b[0m Serialization failed: {}", e),
        }
    }
}