mod cpu;
mod io;

use cpu::CPU;
use io::IOController;

fn main() {
    println!("Virtual Machine Initializing...");
    let io_controller = IOController::new();
    let mut cpu = CPU::new(io_controller);
    cpu.run();
}
