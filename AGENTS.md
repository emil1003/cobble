# Cobble Project Overview

Cobble is an 8-bit Instruction Set Architecture (ISA) project implemented in Rust. It includes a complete pipeline for processing instructions, including a parser, a compiler, and an interpreter.

## Project Structure

- `cobble/src/assembler`: Contains the logic for parsing and processing assembly language.
- `cobble/src/compiler`: Handles the transformation of source code/assembly into machine-readable instructions.
- `cobble/src/interpreter`: Responsible for executing the instructions produced by the compiler.
- `cobble/src/lib.rs`: The main library entry point.
- `cobble/src/main.rs`: The main executable entry point.

## Tech Stack

- **Language**: Rust
- **Build System**: Cargo
- **Environment**: The Rust toolchain is pinned by `devenv` (via `devenv.nix` and `devenv.yaml`).

## Key Concepts for Agents

- **ISA**: The project centers around an 8-bit architecture.
- **Pipeline**: Understanding the flow from assembly -> compiler -> interpreter is crucial for navigating the codebase.
- **Modular Design**: The separation into `assembler`, `compiler`, and `interpreter` modules allows for isolated testing and development.

## Development Workflow

- Run `cargo build` and `cargo test` for standard Rust workflows.
- Refer to `cobble/examples` for usage demonstrations and integration tests.
