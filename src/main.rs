// src/main.rs

mod cpu;
mod io;
mod memory;

use cpu::CPU;
use io::IOController;
use memory::MemoryManagementUnit;

fn main() {
    println!("Virtual Machine Initializing...");
    let io_controller = IOController::new();
    let mmu = MemoryManagementUnit::new();
    let mut cpu = CPU::new(io_controller, mmu);

    // Load a simple program into memory
    let program = vec![
        0x46, 0x00, // INPUT R0
        0x47, 0x01, // OUTPUT R1
        0x40, 0x10, // ADD R1, R0
        0x47, 0x01, // OUTPUT R1
        0x4E, 0x00, // CMP R0, R0
        0x50, 0x00, // JE 0 (Loop back to start)
    ];

    cpu.load_program(&program);
    cpu.run();
}
