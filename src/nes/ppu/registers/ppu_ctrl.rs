pub struct PpuCtrl {
    // 0 = $2000; 1 = $2400; 2 = $2800; 3 = $2C00
    nametable_address: u8,
    // 0: add 1, going across; 1: add 32, going down
    vram_address_increment_ppudata: bool,
    // 0: $0000; 1: $1000; ignored in 8x16 mode
    sprite_pattern_table_address_8x8: bool,
    // 0: $0000; 1: $1000
    background_pattern_table_address: bool,
    // 0: 8x8; 1: 8x16
    sprite_size: bool,
    // 0: read backdrop from EXT pins; 1: output color on EXT pins
    ppu_select: bool,
    // 0: off; 1: on
    read_backdrop_from_ext: bool,
}

impl PpuCtrl {
    pub fn new() -> Self {
        PpuCtrl {
            nametable_address: 0x00,
            vram_address_increment_ppudata: false,
            sprite_pattern_table_address_8x8: false,
            background_pattern_table_address: false,
            sprite_size: false,
            ppu_select: false,
            read_backdrop_from_ext: false,
        }
    }

    pub fn write(&mut self, data: u8) {
        self.nametable_address                =  data & 0b00000011;
        self.vram_address_increment_ppudata   = (data & 0b00000100) >> 2 == 1;
        self.sprite_pattern_table_address_8x8 = (data & 0b00001000) >> 3 == 1;
        self.background_pattern_table_address = (data & 0b00010000) >> 4 == 1;
        self.sprite_size                      = (data & 0b00100000) >> 5 == 1;
        self.ppu_select                       = (data & 0b01000000) >> 6 == 1;
        self.read_backdrop_from_ext           = (data & 0b10000000) >> 7 == 1;
    }

    pub fn read(&self) -> u8 {
         self.nametable_address |
        (self.vram_address_increment_ppudata as u8)   << 2 |
        (self.sprite_pattern_table_address_8x8 as u8) << 3 |
        (self.background_pattern_table_address as u8) << 4 |
        (self.sprite_size as u8)                      << 5 |
        (self.ppu_select as u8)                       << 6 |
        (self.read_backdrop_from_ext as u8)           << 7
    }

    pub fn get_nametable_id(&self) -> u8 {
        self.nametable_address
    }

    pub fn get_vram_increment_offset(&self) -> u8 {
        if self.vram_address_increment_ppudata {
            32
        } else {
            1
        }
    }
}

#[cfg(test)]
mod ppu_ctrl_test {
    use super::*;

    #[test]
    fn write_test() {
        let mut ppu_ctrl = PpuCtrl::new();
        ppu_ctrl.write(0b10101010);

        assert_eq!(ppu_ctrl.nametable_address, 2);
        assert_eq!(ppu_ctrl.vram_address_increment_ppudata, false);
        assert_eq!(ppu_ctrl.sprite_pattern_table_address_8x8, true);
        assert_eq!(ppu_ctrl.background_pattern_table_address, false);
        assert_eq!(ppu_ctrl.sprite_size, true);
        assert_eq!(ppu_ctrl.ppu_select, false);
        assert_eq!(ppu_ctrl.read_backdrop_from_ext, true);
    }

    #[test]
    fn read_test() {
        let mut ppu_ctrl = PpuCtrl::new();
        ppu_ctrl.write(0b10101010);
        assert_eq!(ppu_ctrl.read(), 0b10101010);
    }
}