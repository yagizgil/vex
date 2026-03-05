# Vex Parser

Vex Parser is the part of the compiler that checks the logic of your code. It creates an Abstract Syntax Tree (AST) so we can run or compile it.

## Process

The parsing happens in two main steps:

1. **PreParser**: It cleans up the tokens from the Lexer. It handles indentation (Indent/Dedent) and removes unnecessary spaces.
2. **Parser (Pratt Parser)**: It uses the Pratt Parsing method to handle complex math and logic expressions based on operator priority.

## Features

- **Indentation Rules**: Supports Python-like indentation or C-like curly braces `{}`.
- **Type Checking (Basic)**: Validates variable and function type declarations.
- **Expression Parsing**: Handles literals, variables, unary, binary, and grouping operations.
- **Declarations**: Supports variables, functions, structures, and classes.

## Usage

The parser takes tokens and returns a tree of nodes (AST).

```rust
use vex_parser::Parser;

let mut parser = Parser::new(tokens);
let ast = parser.parse();
```
