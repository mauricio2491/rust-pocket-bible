# Rust Pocket Bible 🦀

A quick guide and reference for Rust basic features and fundamentals.

## Overview

This project serves as a comprehensive yet concise guide to learning the essential features of the Rust programming language. It covers core concepts, syntax, and best practices for beginners and those looking for a quick reference.

## Contents

Currently covered topics:

- **Declarative Macros** - Using `macro_rules!` to create custom macros
- **Variables and Mutability** - Understanding `let`, `mut`, and immutability
- **Control Flow** - Working with `if`, `else if`, `else` expressions
- **Loops** - Mastering `loop`, `for`, and `while` loops
- **Loop Labels** - Breaking out of nested loops with labeled break statements
- **Break and Continue** - Controlling loop flow with break and continue statements

Coming soon:
- **Ownership and Borrowing** - Understanding Rust's unique memory management system
- **Pattern Matching** - Powerful control flow with `match` expressions
- **Error Handling** - Working with `Result` and `Option` types
- **Structs and Enums** - Creating custom data structures
- **Traits** - Defining shared behavior across types
- **Lifetimes** - Ensuring references are valid
- **Modules and Packages** - Organizing code effectively
- **Testing** - Writing and running tests

## Getting Started

### IDE Setup

This project is developed using Visual Studio Code with the Rust Analyzer extension installed.

Rust Analyzer provides IntelliSense, code navigation, inline errors, and improved Rust development support directly in VS Code.

To install it in VS Code:

1. Open the Extensions panel
2. Search for `rust-analyzer`
3. Install the extension
4. Make sure the Rust toolchain is installed on your system

## Prerequisites

- Rust installed on your system
- Cargo included with the Rust toolchain

You can verify the installation with:

```bash
rustc --version
cargo --version
```

### Create a new Rust Cargo project

To create a new project from scratch:

```bash
cargo new my_project
```

This creates a new folder named `my_project` with the standard Rust project structure:

```text
my_project/
├── Cargo.toml
├── src/
│   └── main.rs
```

To create the project in the current folder instead:

```bash
cargo init
```

### Build the project

Compile the project without running it:

```bash
cargo build
```

Build for production/release mode:

```bash
cargo build --release
```

### Run the project

Run the application:

```bash
cargo run
```

### Check for compiler errors

Validate the project without producing a binary:

```bash
cargo check
```

This is often faster than a full build while you are editing code.

### Format the project

Format Rust source files according to the standard Rust style:

```bash
cargo fmt
```

You can also format and then run the project in one flow:

```bash
cargo fmt && cargo check && cargo run
```

## Running Tests

```bash
cargo test
```

## Author
Jorge Mauricio Amador Mendez

Created as a learning resource for Rust fundamentals.
