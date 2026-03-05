# Vex Programming Language

Vex is a simple and powerful programming language. It is built with Rust.
It has a special Inspector tool to see how the compiler works step-by-step.

## Requirements

You need Rust and Cargo on your computer.

## How to Run Vex

To run a Vex file (.vx), use this command:

```bash
cargo run -p vex-cli -- path/to/your_file.vx
```

## Compilation Statistics

You can see how long each compiler step (Lexing, PreParsing, etc.) takes and how much RAM they use.

### Phase Speed Test:

```bash
cargo run -p vex-cli -- path/to/your_file.vx --stats
```

### Phase RAM Usage:

To see memory (RAM) used by each step, you need the `performance-stats` feature:

```bash
cargo run --features performance-stats -p vex-cli -- path/to/your_file.vx --stats
```

## Using the Vex Inspector (Debugger)

The Inspector is a powerful tool to see how the compiler works. You must use the `inspector` feature for it to work.

### GUI Inspector (Recommended)

Our new graphical inspector provides a side-by-side view of Lexer tokens, PreParser results, AST nodes, and Source code highlighting.

**To open the GUI Inspector:**

```bash
cargo run --features inspector -p vex-cli -- path/to/your_file.vx --inspect --gui
```

**Key Features:**

- **Full View:** Complete pipeline visualization in a single screen.
- **Execution Trace:** Real-time call stack tracking of the Rust parser algorithms.
- **Cross-Highlighting:** Clicking a token highlights it in all other lists and the source code.
- **Automatic Sync:** AST selection automatically scrolls and highlights related tokens.

### TUI Inspector (Terminal)

If you prefer the terminal, you can use the classic TUI inspector:

```bash
cargo run --features inspector -p vex-cli -- --inspect path/to/your_file.vx
```

#### Inspector Shortcuts:

- **SPACE**: Move to the next step.
- **Skip Phase**: Fast-forward to the next compiler phase.
- **Reload (R)**: Reload the source file and restart inspection.
- **Focus (F)**: Re-center the view on the current selection.
- **Q**: Quit the Inspector.

## Exporting Data

If you want to save the token list, press E in the Inspector.
It will create a file named report*...*.md. You can open it in VS Code to see a nice table of your code.

## For Contributors

We love help! To add a new feature:

1. Look at vex-lexer to change how characters are read.
2. Look at vex-parser to change the language rules.
3. Use the Inspector to test your changes. It helps you find bugs easily.

### Project Structure:

- vex-core: Basic types (Tokens, Spans).
- vex-lexer: Converts code text into tokens.
- vex-parser: Checks the code structure.
- vex-inspector: The TUI debugger tool.
- vex-cli: The main program.

Happy coding with Vex!
