#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MemoryBus {
    pub memory: [u8; 0xFFFF],
}

impl MemoryBus {
    pub fn new() -> Self {
        MemoryBus {
            memory: [0; 0xFFFF],
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
}
