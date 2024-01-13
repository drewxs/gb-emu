use crate::gpu::{GPU, VRAM_BEGIN, VRAM_END};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MemoryBus {
    pub memory: [u8; 0xFFFF],
    pub gpu: GPU,
}

impl MemoryBus {
    pub fn new() -> Self {
        MemoryBus {
            memory: [0; 0xFFFF],
            gpu: GPU::new(),
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        let addr = addr as usize;
        match addr {
            VRAM_BEGIN..=VRAM_END => self.gpu.read_vram(addr - VRAM_BEGIN),
            _ => panic!("TODO: support other areas of memory"),
        }
    }

    pub fn write_byte(&mut self, addr: u16, byte: u8) {
        let addr = addr as usize;
        match addr {
            VRAM_BEGIN..=VRAM_END => self.gpu.write_vram(addr - VRAM_BEGIN, byte),
            _ => panic!("TODO: support other areas of memory"),
        }
    }
}
