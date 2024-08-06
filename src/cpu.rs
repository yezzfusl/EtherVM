// src/cpu.rs

use std::collections::HashMap;
mod memory;
use memory::MemoryManagementUnit;
use crate::io::IOController;

pub struct CPU {
    registers: [u32; 8],
    program_counter: usize,
    mmu: MemoryManagementUnit,
    io_controller: IOController,
    instruction_set: HashMap<u8, fn(&mut CPU, u8, u8, u8)>,
    flags: u8, // New: Flags register for comparison results
}

impl CPU {
    pub fn new(io_controller: IOController) -> Self {
        let mut cpu = CPU {
            registers: [0; 8],
            program_counter: 0,
            mmu: MemoryManagementUnit::new(),
            io_controller,
            instruction_set: HashMap::new(),
            flags: 0,
        };
        cpu.initialize_instruction_set();
        cpu
    }

    fn initialize_instruction_set(&mut self) {
        self.instruction_set.insert(0x00, CPU::add);
        self.instruction_set.insert(0x01, CPU::sub);
        self.instruction_set.insert(0x02, CPU::mul);
        self.instruction_set.insert(0x03, CPU::div);
        self.instruction_set.insert(0x04, CPU::load);
        self.instruction_set.insert(0x05, CPU::store);
        self.instruction_set.insert(0x06, CPU::input);
        self.instruction_set.insert(0x07, CPU::output);
        // New instructions
        self.instruction_set.insert(0x08, CPU::and);
        self.instruction_set.insert(0x09, CPU::or);
        self.instruction_set.insert(0x0A, CPU::xor);
        self.instruction_set.insert(0x0B, CPU::not);
        self.instruction_set.insert(0x0C, CPU::shl);
        self.instruction_set.insert(0x0D, CPU::shr);
        self.instruction_set.insert(0x0E, CPU::cmp);
        self.instruction_set.insert(0x0F, CPU::jmp);
        self.instruction_set.insert(0x10, CPU::je);
        self.instruction_set.insert(0x11, CPU::jne);
        self.instruction_set.insert(0x12, CPU::jg);
        self.instruction_set.insert(0x13, CPU::jl);
    }

    pub fn run(&mut self) {
        loop {
            let opcode = self.fetch();
            self.decode_and_execute(opcode);
        }
    }

    fn fetch(&mut self) -> u8 {
        let instruction = self.mmu.read_byte(self.program_counter);
        self.program_counter += 1;
        instruction
    }

    fn decode_and_execute(&mut self, opcode: u8) {
        let (op, r1, r2, r3) = self.decode(opcode);
        if let Some(instruction) = self.instruction_set.get(&op) {
            instruction(self, r1, r2, r3);
        } else {
            panic!("Unknown opcode: {:02X}", op);
        }
    }

    fn decode(&self, opcode: u8) -> (u8, u8, u8, u8) {
        let op = (opcode & 0xF0) >> 4;
        let r1 = (opcode & 0x0C) >> 2;
        let r2 = opcode & 0x03;
        let r3 = 0; // For future use
        (op, r1, r2, r3)
    }

    // Existing arithmetic operations...

    fn and(&mut self, r1: u8, r2: u8, _r3: u8) {
        self.registers[r1 as usize] &= self.registers[r2 as usize];
    }

    fn or(&mut self, r1: u8, r2: u8, _r3: u8) {
        self.registers[r1 as usize] |= self.registers[r2 as usize];
    }

    fn xor(&mut self, r1: u8, r2: u8, _r3: u8) {
        self.registers[r1 as usize] ^= self.registers[r2 as usize];
    }

    fn not(&mut self, r1: u8, _r2: u8, _r3: u8) {
        self.registers[r1 as usize] = !self.registers[r1 as usize];
    }

    fn shl(&mut self, r1: u8, r2: u8, _r3: u8) {
        self.registers[r1 as usize] <<= self.registers[r2 as usize];
    }

    fn shr(&mut self, r1: u8, r2: u8, _r3: u8) {
        self.registers[r1 as usize] >>= self.registers[r2 as usize];
    }

    fn cmp(&mut self, r1: u8, r2: u8, _r3: u8) {
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

    fn jmp(&mut self, r1: u8, _r2: u8, _r3: u8) {
        self.program_counter = self.registers[r1 as usize] as usize;
    }

    fn je(&mut self, r1: u8, _r2: u8, _r3: u8) {
        if self.flags & 0b0001 != 0 {
            self.program_counter = self.registers[r1 as usize] as usize;
        }
    }

    fn jne(&mut self, r1: u8, _r2: u8, _r3: u8) {
        if self.flags & 0b0001 == 0 {
            self.program_counter = self.registers[r1 as usize] as usize;
        }
    }

    fn jg(&mut self, r1: u8, _r2: u8, _r3: u8) {
        if self.flags & 0b0011 == 0 {
            self.program_counter = self.registers[r1 as usize] as usize;
        }
    }

    fn jl(&mut self, r1: u8, _r2: u8, _r3: u8) {
        if self.flags & 0b0010 != 0 {
            self.program_counter = self.registers[r1 as usize] as usize;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::MockIOController;

    // Existing tests...

    #[test]
    fn test_logical_operations() {
        let io_controller = MockIOController::new();
        let mut cpu = CPU::new(io_controller);
        
        cpu.registers[0] = 0b1100;
        cpu.registers[1] = 0b1010;
        
        cpu.and(0, 1, 0);
        assert_eq!(cpu.registers[0], 0b1000);
        
        cpu.registers[0] = 0b1100;
        cpu.or(0, 1, 0);
        assert_eq!(cpu.registers[0], 0b1110);
        
        cpu.registers[0] = 0b1100;
        cpu.xor(0, 1, 0);
        assert_eq!(cpu.registers[0], 0b0110);
        
        cpu.registers[0] = 0b1100;
        cpu.not(0, 0, 0);
        assert_eq!(cpu.registers[0], 0xFFFFFFF3);
    }

    #[test]
    fn test_shift_operations() {
        let io_controller = MockIOController::new();
        let mut cpu = CPU::new(io_controller);
        
        cpu.registers[0] = 0b1100;
        cpu.registers[1] = 2;
        
        cpu.shl(0, 1, 0);
        assert_eq!(cpu.registers[0], 0b110000);
        
        cpu.registers[0] = 0b110000;
        cpu.shr(0, 1, 0);
        assert_eq!(cpu.registers[0], 0b1100);
    }

    #[test]
    fn test_compare_and_jump() {
        let io_controller = MockIOController::new();
        let mut cpu = CPU::new(io_controller);
        
        cpu.registers[0] = 10;
        cpu.registers[1] = 10;
        cpu.registers[2] = 100; // Jump target
        
        cpu.cmp(0, 1, 0);
        assert_eq!(cpu.flags & 0b0001, 0b0001); // Zero flag should be set
        
        cpu.je(2, 0, 0);
        assert_eq!(cpu.program_counter, 100);
        
        cpu.registers[1] = 11;
        cpu.cmp(0, 1, 0);
        assert_eq!(cpu.flags & 0b0010, 0b0010); // Negative flag should be set
        
        cpu.program_counter = 0;
        cpu.jl(2, 0, 0);
        assert_eq!(cpu.program_counter, 100);
    }
}
