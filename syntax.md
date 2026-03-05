# Vex Language Syntax Guide

Vex is designed to be a clean, minimalist, and readable programming language. It removes unnecessary punctuation like parentheses `()`, curly braces `{}`, and equal signs `=` in favor of clear spacing and indentation.

This guide outlines the core rules and syntax of Vex in simple English.

---

## 1. Variables and Assignments

Vex does not use the equal sign (`=`) for assigning values to variables. You simply use a space between the variable name and its value.

**Declaring a new variable:**
Use the `var` keyword, followed by the variable name, and then its value.

```vex
var limit 100
var message "Hello, World!"
```

**Updating an existing variable:**
Just write the variable name, a space, and the new value.

```vex
var num 2
num 10       # Updates num to 10
num num + 1  # Adds 1 cleanly

# Increment and Decrement shorthands
var counter 0
counter++    # Auto increments by 1
counter--    # Auto decrements by 1
```

**Constants:**
Constants are declared using the `const` keyword.

```vex
const PI 3.14159
```

**Multiple statements on one line:**
You can use a semicolon (`;`) to separate multiple lines of code on the same line if you want to.

```vex
var a 10; var b 15
print a; print b
```

---

## 2. Types and Data Structures

If you want to explicitly declare a type, put the type name immediately after the `var` keyword.

```vex
var int i 0
var float score 95.5
var bool is_valid true
```

**Lists (Arrays):**
Use the `list` keyword to define an array, wrapped in square brackets `[]`.

```vex
var list levels ["Beginner", "Pro", "Expert"]
```

**Dictionaries:**
Use the `dict` keyword to define key-value pairs, wrapped in curly braces `{}`. Key-value pairs inside a dictionary still use the standard JSON-like colon `:` syntax.

**Type Casting:**
Vex relies on the `as` keyword for casting and type conversions.

```vex
var int a 1

fn calc para.float - int:
     var c para * 1.5
     return c as int

# Dynamic cast mapping combined with function parameters
var b calc a as float
```

Vex allows three ways to access data within lists and dictionaries:

1. **Bracket Access:** Standard index or key lookup using `[]`
2. **Static Access:** Dot notation lookup using `.`
3. **Dynamic Access:** Resolving fields from a variable evaluating at runtime using `./`

```vex
var dict config {"theme": "dark", "port": 8080, "retry": 3}
var list users ["admin", "guest"]

# 1. Standard Bracket Access
var current_port config["port"]
var first_user users[0]

# 2. Static Dot Access (Cleanest for known keys)
var same_port config.port
var same_user users.0

# 3. Dynamic Access (Reading via a string/int variable)
var user_choice "theme"
var active_theme config./user_choice

var user_index 1
var second_user users./user_index

# 4. Safe / Optional Chaining Access (Returns null instead of panic if undefined/null)
var current_env my_server.?metadata.?env

var dynamic_safe config.?/user_choice
```

**Dictionary Methods:**
You can call built-in methods on dictionaries using the `::` operator.

```vex
var dict queues {"inbox": 150, "outbox": 20, "errors": 5}

var all_queues queues::keys
var total_queues queues::len

print f"Total queues in system: {total_queues}"

for q_name in all_queues:
    var message_count queues./q_name

    if message_count > 100:
        print f"WARNING: Queue {q_name} overloaded! ({message_count} messages)"
```

---

## 3. Operations and Logic

Math and logic operations use standard symbols.

- **Math:** `+`, `-`, `*`, `/`
- **Comparisons:** `>`, `<`, `>=`, `<=`, `==`, `!=`
- **Logical:** `and`, `or`, `!` (not)

```vex
var total 5 * 2
var is_approved (total > 5) and (limit == 100)
```

---

## 4. Blocks and Indentation

Vex uses **Python-style indentation** instead of curly braces to define blocks of code (like what happens inside an `if` statement or a `while` loop).

A block starts with a colon (`:`) at the end of the line, and the code inside the block must be indented (pushed right) with spaces.

### If / Elif / Else

```vex
var score 85

if score > 90:
    print "Excellent"
elif score > 70:
    print "Good"
else:
    print "Needs improvement"
```

### While Loop

```vex
var i 0
while i < 3:
    print f"Counter: {i}"
    i i + 1
```

### For Loop

Vex supports iterating over lists or destructuring (unpacking) items from dictionaries.

```vex
for level in levels:
    print f"Level: {level}"

for key, value in config:
    print f"Config {key}: {value}"
```

---

## 5. Functions

Vex functions are very minimal. They do not use parentheses `()` for defining parameters or returning types.

### Defining a Function

- Use the `fn` keyword.
- Parameters are written as `name.type`. Both `str` and `string` are valid type annotations for strings.
- The return type is specified with a hyphen `-` or an arrow `->` followed by the type.
- The function header ends with a colon `:`.

```vex
fn calculate_bonus current.int multiplier.float - float:
    var res current * multiplier
    return res

# Both `->` and `str`/`string` are also perfectly valid!
fn get_user_name id.int -> string:
    return "John"
```

### Calling a Function

When calling a function, do not use parentheses. Separate the arguments with spaces.

```vex
var float bonus calculate_bonus 50 1.5
print bonus
```

---

## 6. Strings and Formatting

Vex has rich support for strings, including multi-line text and string interpolation (injecting variables into text).

### Normal Strings

```vex
var greet "Hello"
var single 'World'
```

### Multi-line Strings

Use triple-quotes `"""` or `'''` to create strings that span across multiple lines.

```vex
var description """
This is a long text.
It keeps the formatting and newlines.
"""
```

### F-Strings (Interpolation)

Prefix a string with `f` (or use backticks `` ` ``) to inject variables directly into the text using `{}`.

```vex
var app "Vex"
var version 1.0

print f"Welcome to {app} version {version}!"
print `Math logic: {10 + 20}`
```

---

## 7. Comments

Comments are lines of text ignored by the compiler.

- **Single-line comments:** Start with `#` or `//`.
- **Multi-line block comments:** Wrap the comment in `/*` and `*/`.

```vex
# This is a Python-style comment
// This is a C-style comment

/*
   This is a large block
   of text that spans
   multiple lines.
*/
```

---

## 8. Pattern Matching and Error Handling

Vex supports robust pattern matching functionality as a cleaner alternative to long `if-elif` chains. It is also natively used to extract `Result` values like `Ok` and `Err`.

```vex
# Standard value matching
var status "error"
match status:
    "ok":
        print "System stable"
    "error":
        print "System failure"
    _:
        print "Unknown status"  # The underscore acts as a default/catch-all

# Result matching (Error Handling without Try/Catch)
var result Ok("Successfully connected!")

match result:
    Ok(msg):
        print f"Success: {msg}"
    Err(code):
        print f"Failed with error code: {code}"
```

---

## 9. Object-Oriented Structures (Planned/Experimental)

Vex supports building custom complex structures and states.

**Enums:**

```vex
pub enum ConnectionState:
    Disconnected
    Connecting
    Connected
```

**Structs, Generics, and Implementations (`impl`):**
Structs and Enums can take generic type parameters using square brackets `[]`. Struct fields define their type with a dot (`.`), similar to function parameters. You can group functions associated with a struct inside an `impl` block.

```vex
# A Queue struct that accepts a generic type 'T'
pub struct Queue[T]:
    items.list
    limit.int

impl Queue[T]:
    pub fn push item.T -> bool:
        if self.items::len >= self.limit:
            return false
        self.items::append item
        return true

# Instantiating a generic struct (A Queue that accepts 'str')
var str_queue Queue[str] [] 100
str_queue::push "message"

pub struct Server:
    ip.string # Both 'str' and 'string' work
    metadata.dict

impl Server:
    # Constructor utilizing `self` assignment
    pub fn new target_ip.str -> Server:
        var meta {"env": "prod", "region": "eu-central"}
        return Server target_ip meta

    pub fn get_region - string:
        return self.metadata.region

# Instantiating a struct
var my_server Server::new "192.168.1.10"

# Accessing fields and methods
var region my_server.get_region
var ip my_server.ip
```

**Checking State using Native Accessors:**
Utilize native struct and dictionary accessor patterns alongside minimal execution functions implicitly returning types.

```vex
var dict active_user {"id": 101, "role": "admin", "status": "active"}

fn update_user_field field_name.str new_value.str - bool:
    # Use native :: method calls
    var is_valid active_user::has field_name

    match is_valid:
        true:
            active_user./field_name new_value
            print f"Success: {field_name} updated to {new_value}."
            return true
        false:
            print f"Error: User does not contain field '{field_name}'!"
            return false

# Function execution chaining
var success update_user_field "status" "suspended"
```
