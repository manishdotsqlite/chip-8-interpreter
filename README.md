## Overview

This project is a CHIP-8 interpreter written in Rust. The CHIP-8 is a simple virtual machine that was used for running simple games in the 1970s. This interpreter aims to provide a functional implementation of the CHIP-8 specification, allowing you to run CHIP-8 programs.

## Features

- Supports basic CHIP-8 instructions.
- Emulates the CHIP-8 CPU, memory, and stack.
- Provides a simple command-line interface for testing and debugging.

## Getting Started

### Prerequisites

- Rust programming language installed. You can install Rust from [rustup.rs](https://rustup.rs/).

### Installation

1. Clone the repository:

   ```sh
        git clone https://github.com/yourusername/chip8-interpreter.git
        cd chip8-interpreter
   ```

2. Build the project:
   `sh
  cargo build
`

3. Run the interpreter:
   `sh
 cargo run -- <path_to_chip8_program>
 `
