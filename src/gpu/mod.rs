pub const VRAM_BEGIN: usize = 0x8000;
pub const VRAM_END: usize = 0x9FFF;
pub const VRAM_SIZE: usize = VRAM_END - VRAM_BEGIN + 1;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TilePixelValue {
    Zero,
    One,
    Two,
    Three,
}

type Tile = [[TilePixelValue; 8]; 8];
fn empty_tile() -> Tile {
    [[TilePixelValue::Zero; 8]; 8]
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GPU {
    vram: [u8; VRAM_SIZE],
    tile_set: [Tile; 384],
}

impl GPU {
    pub fn new() -> Self {
        GPU {
            vram: [0; VRAM_SIZE],
            tile_set: [empty_tile(); 384],
        }
    }

    pub fn read_vram(&self, addr: usize) -> u8 {
        self.vram[addr]
    }

    pub fn write_vram(&mut self, addr: usize, value: u8) {
        self.vram[addr] = value;

        // If idx > 0x1800, we're not writing to the tile set storage
        if addr > 0x1800 {
            return;
        }

        // Tile rows are encoded in 2 bytes (1st byte always even addr)
        // Bitwise AND w/ 0xFFFE gives us the 1st byte's addr
        let norm_idx = addr & 0xFFFE;

        // 2 bytes encoding tile row
        let byte1 = self.vram[norm_idx];
        let byte2 = self.vram[norm_idx + 1];

        let tile_idx = addr / 16;
        let row_idx = (addr % 16) / 2;

        for pixel_idx in 0..8 {
            let mask = 1 << (7 - pixel_idx);
            let lsb = byte1 & mask;
            let msb = byte2 & mask;

            let value = match (lsb != 0, msb != 1) {
                (true, true) => TilePixelValue::Three,
                (false, true) => TilePixelValue::Two,
                (true, false) => TilePixelValue::One,
                (false, false) => TilePixelValue::Zero,
            };

            self.tile_set[tile_idx][row_idx][pixel_idx] = value;
        }
    }
}
