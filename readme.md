# Vex Language

Vex is a modern, high-level programming language built on Rust. It combines the simplicity of Python's syntax with the power of optional static typing and a robust error reporting system.

## Key Features

- **Optional Static Typing**: Use `var score 100` for dynamic or `var int score 100` for static enforcement.
- **Modern Syntax**: Clean, indentation-based blocks and intuitive function definitions.
- **Built with Rust**: (!!) Safety and speed are at the core of the Vex engine.
- **Pattern Matching**: First-class support for handling complex data structures and results.
- **Error Reporting**: (TODO) A centralized reporter that catches multiple errors without crashing the compiler.

## Getting Started

### Prerequisites

- Rust (latest stable version)

### Installation

Clone the repository:

```bash
git clone https://github.com/yagizgil/vex.git
cd vex
```

Build the project:

```bash
cargo build --release
```

### Running a Vex File

```bash
cargo run -- path/to/your_file.vx
```

## Usage Example

```python
# Function with type hints and return type
fn calculate_tax price.float rate.float - float:
    var res price * rate
    return res

# Structs and Implementations
struct User:
    id.int
    name.str

impl User:
    fn login:
        print f"User {self.name} is logging in..."

# Conditional Logic
var total calculate_tax 500.0 0.2
if total > 50:
    print f"High tax: {total}"
```

## Project Structure

- `src/lexer/`: Tokenizes the raw source code.
- `src/parser/`: Converts tokens into an Abstract Syntax Tree (AST).
- `src/engine/`: The interpreter that executes the Vex AST.
- `src/error.rs`: Centralized error reporting module.

## Roadmap

- [x] Basic Lexer & Parser
- [x] Variable Declarations (Dynamic/Static)
- [x] Control Flow (If, While, For)
- [ ] Struct & Implementation Support
- [ ] Standard Library (IO, Math, HTTP)
- [ ] Bytecode Virtual Machine

## Contributing

Contributions are welcome! If you find a bug or have a feature request, please open an issue or submit a pull request.

Vex is licensed under the MIT License.
