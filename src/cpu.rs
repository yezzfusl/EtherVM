// src/cpu.rs

use std::collections::HashMap;
use crate::io::IOController;
use crate::memory::MemoryManagementUnit;

pub struct CPU {
    registers: [u32; 8],
    program_counter: usize,
    mmu: MemoryManagementUnit,
    io_controller: IOController,
    instruction_set: HashMap<u8, fn(&mut CPU, u8, u8)>,
    flags: u8,
    halted: bool,
}

impl CPU {
    pub fn new(io_controller: IOController, mmu: MemoryManagementUnit) -> Self {
        let mut cpu = CPU {
            registers: [0; 8],
            program_counter: 0,
            mmu,
            io_controller,
            instruction_set: HashMap::new(),
            flags: 0,
            halted: false,
        };
        cpu.initialize_instruction_set();
        cpu
    }

    fn initialize_instruction_set(&mut self) {
        self.instruction_set.insert(0x40, CPU::add);
        self.instruction_set.insert(0x41, CPU::sub);
        self.instruction_set.insert(0x42, CPU::mul);
        self.instruction_set.insert(0x43, CPU::div);
        self.instruction_set.insert(0x44, CPU::load);
        self.instruction_set.insert(0x45, CPU::store);
        self.instruction_set.insert(0x46, CPU::input);
        self.instruction_set.insert(0x47, CPU::output);
        self.instruction_set.insert(0x48, CPU::and);
        self.instruction_set.insert(0x49, CPU::or);
        self.instruction_set.insert(0x4A, CPU::xor);
        self.instruction_set.insert(0x4B, CPU::not);
        self.instruction_set.insert(0x4C, CPU::shl);
        self.instruction_set.insert(0x4D, CPU::shr);
        self.instruction_set.insert(0x4E, CPU::cmp);
        self.instruction_set.insert(0x4F, CPU::jmp);
        self.instruction_set.insert(0x50, CPU::je);
        self.instruction_set.insert(0x51, CPU::jne);
        self.instruction_set.insert(0x52, CPU::jg);
        self.instruction_set.insert(0x53, CPU::jl);
        self.instruction_set.insert(0xFF, CPU::halt);
    }

    pub fn load_program(&mut self, program: &[u8]) {
        for (i, &byte) in program.iter().enumerate() {
            self.mmu.write_byte(i, byte);
        }
    }

    pub fn run(&mut self) {
        self.halted = false;
        while !self.halted {
            let opcode = self.fetch();
            self.decode_and_execute(opcode);
        }
        println!("CPU halted. Final register state:");
        self.print_registers();
    }

    fn fetch(&mut self) -> u8 {
        let instruction = self.mmu.read_byte(self.program_counter);
        self.program_counter += 1;
        instruction
    }

    fn decode_and_execute(&mut self, opcode: u8) {
        let r1 = self.fetch();
        let r2 = self.fetch();
        if let Some(instruction) = self.instruction_set.get(&opcode) {
            instruction(self, r1, r2);
        } else {
            panic!("Unknown opcode: {:02X}", opcode);
        }
    }

    fn print_registers(&self) {
        for (i, reg) in self.registers.iter().enumerate() {
            println!("R{}: {:08X}", i, reg);
        }
        println!("Flags: {:08b}", self.flags);
    }

    // Instruction implementations

    fn add(&mut self, r1: u8, r2: u8) {
        self.registers[r1 as usize] = self.registers[r1 as usize].wrapping_add(self.registers[r2 as usize]);
    }

    fn sub(&mut self, r1: u8, r2: u8) {
        self.registers[r1 as usize] = self.registers[r1 as usize].wrapping_sub(self.registers[r2 as usize]);
    }

    fn mul(&mut self, r1: u8, r2: u8) {
        self.registers[r1 as usize] = self.registers[r1 as usize].wrapping_mul(self.registers[r2 as usize]);
    }

    fn div(&mut self, r1: u8, r2: u8) {
        if self.registers[r2 as usize] != 0 {
            self.registers[r1 as usize] /= self.registers[r2 as usize];
        } else {
            panic!("Division by zero");
        }
    }

    fn load(&mut self, r1: u8, r2: u8) {
        let address = self.registers[r2 as usize] as usize;
        self.registers[r1 as usize] = self.mmu.read_word(address);
    }

    fn store(&mut self, r1: u8, r2: u8) {
        let address = self.registers[r2 as usize] as usize;
        self.mmu.write_word(address, self.registers[r1 as usize]);
    }

    fn input(&mut self, r1: u8, _r2: u8) {
        self.registers[r1 as usize] = self.io_controller.input();
    }

    fn output(&mut self, r1: u8, _r2: u8) {
        self.io_controller.output(self.registers[r1 as usize]);
    }

    fn and(&mut self, r1: u8, r2: u8) {
        self.registers[r1 as usize] &= self.registers[r2 as usize];
    }

    fn or(&mut self, r1: u8, r2: u8) {
        self.registers[r1 as usize] |= self.registers[r2 as usize];
    }

    fn xor(&mut self, r1: u8, r2: u8) {
        self.registers[r1 as usize] ^= self.registers[r2 as usize];
    }

    fn not(&mut self, r1: u8, _r2: u8) {
        self.registers[r1 as usize] = !self.registers[r1 as usize];
    }

    fn shl(&mut self, r1: u8, r2: u8) {
        self.registers[r1 as usize] <<= self.registers[r2 as usize];
    }

    fn shr(&mut self, r1: u8, r2: u8) {
        self.registers[r1 as usize] >>= self.registers[r2 as usize];
    }

    fn cmp(&mut self, r1: u8, r2: u8) {
        let (result, overflow) = self.registers[r1 as usize].overflowing_sub(self.registers[r2 as usize]);
        self.flags = 0;
        if result == 0 {
            self.flags |= 0b0001; // Zero flag
        }
        if result & 0x80000000 != 0 {
            self.flags |= 0b0010; // Negative flag
        }
        if overflow {
            self.flags |= 0b0100; // Overflow flag
        }
    }

    fn jmp(&mut self, r1: u8, _r2: u8) {
        self.program_counter = self.registers[r1 as usize] as usize;
    }

    fn je(&mut self, r1: u8, _r2: u8) {
        if self.flags & 0b0001 != 0 {
            self.program_counter = self.registers[r1 as usize] as usize;
        }
    }

    fn jne(&mut self, r1: u8, _r2: u8) {
        if self.flags & 0b0001 == 0 {
            self.program_counter = self.registers[r1 as usize] as usize;
        }
    }

    fn jg(&mut self, r1: u8, _r2: u8) {
        if self.flags & 0b0011 == 0 {
            self.program_counter = self.registers[r1 as usize] as usize;
        }
    }

    fn jl(&mut self, r1: u8, _r2: u8) {
        if self.flags & 0b0010 != 0 {
            self.program_counter = self.registers[r1 as usize] as usize;
        }
    }

    fn halt(&mut self, _r1: u8, _r2: u8) {
        self.halted = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::MockIOController;

    #[test]
    fn test_program_execution() {
        let io_controller = MockIOController::new();
        let mmu = MemoryManagementUnit::new();
        let mut cpu = CPU::new(io_controller, mmu);

        let program = vec![
            0x46, 0x00, // INPUT R0
            0x40, 0x10, // ADD R1, R0
            0x47, 0x01, // OUTPUT R1
            0xFF, 0x00, // HALT
        ];

        cpu.load_program(&program);
        cpu.io_controller.set_next_input(5);
        cpu.run();

        assert_eq!(cpu.registers[0], 5);
        assert_eq!(cpu.registers[1], 5);
        assert_eq!(cpu.io_controller.get_last_output(), 5);
        assert!(cpu.halted);
    }
}
