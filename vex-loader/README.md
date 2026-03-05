# Vex Loader

Vex Loader is a simple tool to read files from your computer and give them to the compiler.

## Features

- **File Reading**: Reads `.vx` files as text strings.
- **SourceMap Registration**: Automatically adds every file to a central `SourceMap`.
- **Unique IDs**: Gives a unique ID to every file so the compiler can track them easily.

## Usage

This module is the first step when starting the compiler.

```rust
use vex_loader::Loader;

let file_id = Loader::load_file("my_code.vx").unwrap();
let content = Loader::get_content(file_id);
```
