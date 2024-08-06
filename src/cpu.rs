use std::collections::HashMap;

pub struct CPU {
    registers: [u32; 8],
    program_counter: usize,
    memory: [u8; 4096],
    instruction_set: HashMap<u8, fn(&mut CPU, u8, u8, u8)>,
}

impl CPU {
    pub fn new() -> Self {
        let mut cpu = CPU {
            registers: [0; 8],
            program_counter: 0,
            memory: [0; 4096],
            instruction_set: HashMap::new(),
        };
        cpu.initialize_instruction_set();
        cpu
    }

    fn initialize_instruction_set(&mut self) {
        self.instruction_set.insert(0x00, CPU::add);
        self.instruction_set.insert(0x01, CPU::sub);
        self.instruction_set.insert(0x02, CPU::mul);
        self.instruction_set.insert(0x03, CPU::div);
    }

    pub fn run(&mut self) {
        loop {
            let opcode = self.fetch();
            self.decode_and_execute(opcode);
        }
    }

    fn fetch(&mut self) -> u8 {
        let instruction = self.memory[self.program_counter];
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
        let op = (opcode & 0xC0) >> 6;
        let r1 = (opcode & 0x38) >> 3;
        let r2 = (opcode & 0x07);
        let r3 = 0; // For future use
        (op, r1, r2, r3)
    }

    fn add(&mut self, r1: u8, r2: u8, _r3: u8) {
        self.registers[r1 as usize] += self.registers[r2 as usize];
    }

    fn sub(&mut self, r1: u8, r2: u8, _r3: u8) {
        self.registers[r1 as usize] = self.registers[r1 as usize].wrapping_sub(self.registers[r2 as usize]);
    }

    fn mul(&mut self, r1: u8, r2: u8, _r3: u8) {
        self.registers[r1 as usize] *= self.registers[r2 as usize];
    }

    fn div(&mut self, r1: u8, r2: u8, _r3: u8) {
        if self.registers[r2 as usize] != 0 {
            self.registers[r1 as usize] /= self.registers[r2 as usize];
        } else {
            panic!("Division by zero");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_instruction() {
        let mut cpu = CPU::new();
        cpu.registers[0] = 5;
        cpu.registers[1] = 10;
        cpu.add(0, 1, 0);
        assert_eq!(cpu.registers[0], 15);
    }

    #[test]
    fn test_sub_instruction() {
        let mut cpu = CPU::new();
        cpu.registers[0] = 10;
        cpu.registers[1] = 5;
        cpu.sub(0, 1, 0);
        assert_eq!(cpu.registers[0], 5);
    }

    #[test]
    fn test_mul_instruction() {
        let mut cpu = CPU::new();
        cpu.registers[0] = 3;
        cpu.registers[1] = 4;
        cpu.mul(0, 1, 0);
        assert_eq!(cpu.registers[0], 12);
    }

    #[test]
    fn test_div_instruction() {
        let mut cpu = CPU::new();
        cpu.registers[0] = 15;
        cpu.registers[1] = 3;
        cpu.div(0, 1, 0);
        assert_eq!(cpu.registers[0], 5);
    }

    #[test]
    #[should_panic(expected = "Division by zero")]
    fn test_div_by_zero() {
        let mut cpu = CPU::new();
        cpu.registers[0] = 15;
        cpu.registers[1] = 0;
        cpu.div(0, 1, 0);
    }
}

