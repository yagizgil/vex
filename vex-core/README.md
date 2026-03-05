# Vex Core

Vex Core contains the basic data structures and shared utilities for the Vex compiler.

## Components

- **Token**: Definitions for keywords, literals, and symbols.
- **AST**: Definitions for expressions and declarations in the Abstract Syntax Tree.
- **Span**: Utilities to track the location of code in the source files (line, column, start, and end).
- **SourceMap**: A central map to manage source files and their IDs.
- **Execution Trace**: A system to track function calls inside the compiler for debugging.

## Features

- **inspector**: Enables the tracing system. It uses thread-local storage to build a call stack tree.
- **serde**: Provides serialization support for AST and Trace structures.

## Usage

This crate is used by:

1. `vex-lexer`: To create tokens.
2. `vex-parser`: To build the AST.
3. `vex-inspector`: To visualize the compilation process.
