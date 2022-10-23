const MEMORY_SIZE: usize = 0xFFFF;

pub struct MemoryBus {
    pub memory: [u8; MEMORY_SIZE],
}

impl MemoryBus {
    pub fn new() -> Self {
        Self {
            memory: [0x0; MEMORY_SIZE],
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        let address = address as usize;
        self.memory[address] = value;
    }
}
