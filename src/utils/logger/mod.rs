pub mod error;
pub mod core;

pub use error::{ErrorCode, Reporter, VexError};

#[macro_export]
macro_rules! vex_err {
    ($line:expr, $code:expr, $detail:expr) => {
        $crate::utils::logger::error::Reporter::add(
            $crate::utils::logger::error::VexError::General($line, $code,Some($detail.to_string()))
        );
    };
}

#[macro_export]
macro_rules! vex_lex_err {
    ($line:expr, $code:expr, $detail:expr) => {
        $crate::utils::logger::error::Reporter::add(
            $crate::utils::logger::error::VexError::Lexer($line, $code, Some($detail.to_string()))
        )
    };
    ($line:expr, $code:expr) => {
        $crate::utils::logger::error::Reporter::add(
            $crate::utils::logger::error::VexError::Lexer($line, $code, None)
        )
    };
}

#[macro_export]
macro_rules! vex_pars_err {
    ($line:expr, $code:expr, $detail:expr) => {
        $crate::utils::logger::error::Reporter::add(
            $crate::utils::logger::error::VexError::Parser($line, $code, Some($detail.to_string()))
        )
    };
    ($line:expr, $code:expr) => {
        $crate::utils::logger::error::Reporter::add(
            $crate::utils::logger::error::VexError::Parser($line, $code, None)
        )
    };
}

#[macro_export]
macro_rules! vex_int_err {
    ($line:expr, $code:expr, $detail:expr) => {
        $crate::utils::logger::error::Reporter::add(
            $crate::utils::logger::error::VexError::Interpreter($line, $code, Some($detail.to_string()))
        )
    };
    ($line:expr, $code:expr) => {
        $crate::utils::logger::error::Reporter::add(
            $crate::utils::logger::error::VexError::Interpreter($line, $code, None)
        )
    };
}