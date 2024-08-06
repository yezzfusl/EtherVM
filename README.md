# EtherVM
A lightweight, efficient virtual machine implementation in Rust, designed for educational purposes and as a foundation for more complex emulation projects.

## Features

### 1. CPU Emulation
- Basic CPU structure with registers and program counter
- Instruction fetch-decode-execute cycle
- Initial instruction set with basic arithmetic operations (ADD, SUB, MUL, DIV)
- Error handling for unknown opcodes and division by zero
- Unit tests for arithmetic operations

### 2. Memory Management Unit 
- Implementation of a Memory Management Unit (MMU)
- 64KB of emulated memory
- Byte and word (32-bit) read/write operations
- Integration of MMU with CPU for instruction fetching and data operations
- LOAD and STORE instructions for memory access
- Extended unit tests for memory operations

### 3. I/O Operations 
- IOController and IODevice trait for managing input/output operations
- ConsoleDevice implementation for basic console I/O
- INPUT and OUTPUT instructions added to the CPU
- MockIOController for testing I/O operations without actual console interaction
- Flexible architecture allowing easy addition of new I/O devices

### 4. Expanded Instruction Set 
- Logical operations: AND, OR, XOR, NOT
- Shift operations: SHL (shift left), SHR (shift right)
- Comparison operation (CMP) with flags register
- Control flow instructions:
  - JMP (unconditional jump)
  - JE (jump if equal)
  - JNE (jump if not equal)
  - JG (jump if greater)
  - JL (jump if less)
- Modified instruction decoding to accommodate the expanded set
- Comprehensive unit tests for new instructions and operations

## Project Structure
- src/
- ├── main.rs       # Entry point of the application
- ├── cpu.rs        # CPU implementation with instruction set
- ├── memory.rs     # Memory Management Unit implementation
- └── io.rs         # I/O Controller and devices

## Usage

- To build and run the virtual machine:
    - `cargo build`
    - `cargo run`

- To run the tests:
    - `cargo test` 

## Future Improvements

- Implement a simple assembler for easier program input
- Add more complex I/O devices (e.g., virtual disk, network interface)
- Implement interrupt handling and system calls
- Create a debugger interface for step-by-step execution and memory inspection

## Contributing
Contributions are welcome! Please feel free to submit a Pull Request.

