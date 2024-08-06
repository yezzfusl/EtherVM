// src/memory.rs

pub struct MemoryManagementUnit {
    memory: Vec<u8>,
}

impl MemoryManagementUnit {
    pub fn new() -> Self {
        MemoryManagementUnit {
            memory: vec![0; 65536], // 64KB of memory
        }
    }

    pub fn read_byte(&self, address: usize) -> u8 {
        self.memory[address]
    }

    pub fn write_byte(&mut self, address: usize, value: u8) {
        self.memory[address] = value;
    }

    pub fn read_word(&self, address: usize) -> u32 {
        let bytes = &self.memory[address..address + 4];
        u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
    }

    pub fn write_word(&mut self, address: usize, value: u32) {
        let bytes = value.to_le_bytes();
        self.memory[address..address + 4].copy_from_slice(&bytes);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_write_byte() {
        let mut mmu = MemoryManagementUnit::new();
        mmu.write_byte(0, 42);
        assert_eq!(mmu.read_byte(0), 42);
    }

    #[test]
    fn test_read_write_word() {
        let mut mmu = MemoryManagementUnit::new();
        mmu.write_word(0, 0x12345678);
        assert_eq!(mmu.read_word(0), 0x12345678);
    }

    #[test]
    fn test_memory_persistence() {
        let mut mmu = MemoryManagementUnit::new();
        mmu.write_byte(100, 1);
        mmu.write_byte(101, 2);
        mmu.write_byte(102, 3);
        mmu.write_byte(103, 4);

        assert_eq!(mmu.read_word(100), 0x04030201);
    }
}
