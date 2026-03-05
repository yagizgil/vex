# Vex Diagnostic

Vex Diagnostic is the error reporting system. It shows messages when the compiler finds a problem in your code.

## Features

- **Global Handler**: A single place (`once_cell`) to store and retrieve all errors during a session.
- **Error Codes**: Every error has a unique code (like `P001`) to help you find solutions easily.
- **Location Tracking**: Uses the `SourceMap` to point to the exact line and character where the error is.
- **Diagnostic Levels**: Supports three levels of messages:
  - **Error**: Compilation stops.
  - **Warning**: Important tips, but compilation continues.
  - **Note**: Extra information.

## Pretty Printing

Vex uses the `ariadne` crate to show errors in color. It highlights the problematic code directly in your terminal.

## Usage

You can throw errors using the `diag_emit!` macro.

```rust
use vex_diagnostic::diag_emit;

diag_emit!(Error, P001, "Expected ';'", span);
```
