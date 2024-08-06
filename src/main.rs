mod cpu;

fn main() {
    println!("Virtual Machine Initializing...");
    let mut cpu = cpu::CPU::new();
    cpu.run();
}

