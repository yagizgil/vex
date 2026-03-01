pub mod core;
pub mod error;

pub use core::REPORT_ENABLED;
pub use error::{ErrorCode, Reporter, VexError};
use std::sync::atomic::{Ordering};

// GENERAL ERROR MACROS
#[macro_export]
macro_rules! vex_err {
    ($line:expr, $code:expr, $detail:expr) => {
        if $crate::utils::logger::REPORT_ENABLED.load(std::sync::atomic::Ordering::Relaxed) {
            $crate::utils::logger::error::Reporter::add(
                $crate::utils::logger::error::VexError::General(
                    $line,
                    $code,
                    $detail.into(), 
                ),
            );
        }
    };
}

#[macro_export]
macro_rules! vex_panic {
    ($line:expr, $code:expr, $detail:expr) => {
        $crate::vex_err!($line, $code, $detail);
        if $crate::utils::logger::REPORT_ENABLED.load(std::sync::atomic::Ordering::Relaxed) {
            $crate::utils::logger::error::Reporter::display();
        }
        std::process::exit(1);
    };
}

// LEXER ERROR MACROS
#[macro_export]
macro_rules! vex_lex_err {
    ($line:expr, $code:expr, $detail:expr) => {
        if $crate::utils::logger::REPORT_ENABLED.load(std::sync::atomic::Ordering::Relaxed) {
            $crate::utils::logger::error::Reporter::add($crate::utils::logger::error::VexError::Lexer(
                $line,
                $code,
                $detail.into(),
            ));
        }
    };
    ($line:expr, $code:expr) => {
        if $crate::utils::logger::REPORT_ENABLED.load(std::sync::atomic::Ordering::Relaxed) {
            $crate::utils::logger::error::Reporter::add($crate::utils::logger::error::VexError::Lexer(
                $line, $code, None,
            ));
        }
    };
}

#[macro_export]
macro_rules! vex_lex_panic {
    ($line:expr, $code:expr, $detail:expr) => {
        $crate::vex_lex_err!($line, $code, $detail);
        if $crate::utils::logger::REPORT_ENABLED.load(std::sync::atomic::Ordering::Relaxed) {
            $crate::utils::logger::error::Reporter::display();
        }
        std::process::exit(1);
    };
}

// PARSER ERROR MACROS
#[macro_export]
macro_rules! vex_pars_err {
    ($line:expr, $code:expr, $detail:expr) => {
        if $crate::utils::logger::REPORT_ENABLED.load(std::sync::atomic::Ordering::Relaxed) {
            $crate::utils::logger::error::Reporter::add($crate::utils::logger::error::VexError::Parser(
                $line,
                $code,
                $detail.into(),
            ));
        }
    };
    ($line:expr, $code:expr) => {
        if $crate::utils::logger::REPORT_ENABLED.load(std::sync::atomic::Ordering::Relaxed) {
            $crate::utils::logger::error::Reporter::add($crate::utils::logger::error::VexError::Parser(
                $line, $code, None,
            ));
        }
    };
}

#[macro_export]
macro_rules! vex_pars_panic {
    ($line:expr, $code:expr, $detail:expr) => {
        $crate::vex_pars_err!($line, $code, $detail);
        if $crate::utils::logger::REPORT_ENABLED.load(std::sync::atomic::Ordering::Relaxed) {
            $crate::utils::logger::error::Reporter::display();
        }
        std::process::exit(1);
    };
}

// INTERPRETER ERROR MACROS
#[macro_export]
macro_rules! vex_int_err {
    ($line:expr, $code:expr, $detail:expr) => {
        if $crate::utils::logger::REPORT_ENABLED.load(std::sync::atomic::Ordering::Relaxed) {
            $crate::utils::logger::error::Reporter::add(
                $crate::utils::logger::error::VexError::Interpreter(
                    $line,
                    $code,
                    $detail.into(),
                ),
            );
        }
    };
    ($line:expr, $code:expr) => {
        if $crate::utils::logger::REPORT_ENABLED.load(std::sync::atomic::Ordering::Relaxed) {
            $crate::utils::logger::error::Reporter::add(
                $crate::utils::logger::error::VexError::Interpreter($line, $code, None),
            );
        }
    };
}

#[macro_export]
macro_rules! vex_int_panic {
    ($line:expr, $code:expr, $detail:expr) => {
        $crate::vex_int_err!($line, $code, $detail);
        if $crate::utils::logger::REPORT_ENABLED.load(std::sync::atomic::Ordering::Relaxed) {
            $crate::utils::logger::error::Reporter::display();
        }
        std::process::exit(1);
    };
    ($line:expr, $code:expr) => {
        $crate::vex_int_err!($line, $code);
        if $crate::utils::logger::REPORT_ENABLED.load(std::sync::atomic::Ordering::Relaxed) {
            $crate::utils::logger::error::Reporter::display();
        }
        std::process::exit(1);
    };
}