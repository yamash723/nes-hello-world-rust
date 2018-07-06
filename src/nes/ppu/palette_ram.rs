use std::ops::Range;
use nes::ram::Ram;
use super::palette::PaletteGroup;

pub enum PaletteType {
    Sprite,
    Background,
}

pub struct PaletteRam(Ram);
impl PaletteRam {
    pub fn new() -> Self {
        PaletteRam(Ram::new(vec![0; 0x20]))
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        self.0.write(addr, data);
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.0.read(addr)
    }

    pub fn read_range(&self, range: Range<usize>) -> &[u8] {
        &self.0.read_range(range)
    }

    pub fn get_palettes(&self, palette_id: u8, palette_type: PaletteType) -> PaletteGroup {
        let offset = match palette_type {
            PaletteType::Sprite => 0x10,
            PaletteType::Background => 0x00,
        };

        let start_idx = (palette_id * 4 + offset) as usize;
        let end_idx = start_idx + 4;

        let palette_numbers = self.read_range(start_idx..end_idx);
        let palette_numbers = array_ref!(palette_numbers, 0, 4);
        PaletteGroup::build(palette_numbers)
    }
}

#[cfg(test)]
mod palette_ram_test {
    use super::*;

    #[test]
    fn can_be_read_and_write_test() {
        let mut ram = PaletteRam::new();
        ram.write(0x0002, 0x4F);

        assert_eq!(ram.read(0x0002), 0x4F);
    }

    #[test]
    fn read_range_test() {
        let mut ram = PaletteRam::new();
        ram.write(0x0000, 0x00);
        ram.write(0x0001, 0x01);
        ram.write(0x0002, 0x02);

        assert_eq!(ram.read_range(0..3), &[0x00, 0x01, 0x02]);
    }

    #[test]
    fn get_background_palettes_test() {
        let mut ram = PaletteRam::new();
        ram.write(0x0004, 0x01);
        ram.write(0x0005, 0x02);
        ram.write(0x0006, 0x03);
        ram.write(0x0007, 0x04);

        let palette_group = ram.get_palettes(1, PaletteType::Background);
        let expect_palettes = PaletteGroup::build(&[0x01, 0x02, 0x03, 0x04]);
        assert_eq!(palette_group, expect_palettes);
    }

    #[test]
    fn get_sprite_palettes_test() {
        let mut ram = PaletteRam::new();
        ram.write(0x0014, 0x01);
        ram.write(0x0015, 0x02);
        ram.write(0x0016, 0x03);
        ram.write(0x0017, 0x04);

        let palette_group = ram.get_palettes(1, PaletteType::Sprite);
        let expect_palettes = PaletteGroup::build(&[0x01, 0x02, 0x03, 0x04]);
        assert_eq!(palette_group, expect_palettes);
    }
}