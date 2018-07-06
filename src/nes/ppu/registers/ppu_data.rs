use nes::ram::Ram;
use nes::ppu::PpuContext;

enum MapType {
    PatternTable,
    Vram,
    VramMirror,
    Palette,
}

struct PpuMemoryMapRule;
impl PpuMemoryMapRule {
    fn address_to_map_type(addr: u16) -> MapType {
        match addr {
            0x0000..0x1FFF => MapType::PatternTable,
            0x2000..0x2FFF => MapType::Vram,
            0x3000..0x3EFF => MapType::VramMirror,
            0x3F00..0x3FFF => MapType::Palette, 
            _ => panic!("Access to out of PPU memory."),
        }
    }
}


pub struct PpuData {
    pub buf: u8,
}

impl PpuData {
    pub fn new() -> Self {
        PpuData { buf: 0 }
    }

    pub fn write(&mut self, addr: u16, data: u8, ppu_context: &mut PpuContext) {
        let calibrated_addr = self.calibrate_address(addr);

        match PpuMemoryMapRule::address_to_map_type(addr) {
            MapType::PatternTable => ppu_context.cram.write(calibrated_addr, data),
            MapType::Palette => ppu_context.palette_ram.write(calibrated_addr, data),
            MapType::Vram | MapType::VramMirror => ppu_context.vram.write(calibrated_addr, data),
        };
    }

    pub fn read(&mut self, addr: u16, ppu_context: &mut PpuContext) -> u8 {
        let buf = self.buf;
        let calibrated_addr = self.calibrate_address(addr);

        match PpuMemoryMapRule::address_to_map_type(addr) {
            MapType::PatternTable => self.buf = ppu_context.cram.read(calibrated_addr),
            MapType::Palette => {
                self.buf = ppu_context.vram.read(calibrated_addr);
                return ppu_context.palette_ram.read(calibrated_addr)
            },
            MapType::Vram | MapType::VramMirror => self.buf = ppu_context.vram.read(calibrated_addr),
        };

        buf
    }
    
    fn calibrate_address(&self, addr: u16) -> u16 {
        match PpuMemoryMapRule::address_to_map_type(addr) {
            MapType::PatternTable => addr,
            MapType::Vram => addr - 0x2000,
            MapType::VramMirror => addr - 0x3000,
            MapType::Palette => addr - 0x3F00,
        }
    }
}

#[cfg(test)]
mod ppu_data_test {
    use super::*;
    use nes::ppu::palette_ram::PaletteRam;

    #[test]
    fn read_pattern_test() {
        let mut ppu_context = PpuContext {
            cram: Ram::new(vec![0;0x20]),
            vram: Ram::new(vec![0;0x20]),
            palette_ram: PaletteRam::new(),
        };

        ppu_context.cram.write(0x00, 0xFF);

        let mut ppu_data = PpuData { buf: 0xEE };
        let read_data = ppu_data.read(0x0000, &mut ppu_context);

        assert_eq!(read_data, 0xEE);
        assert_eq!(ppu_data.buf, 0xFF);
    }

    #[test]
    fn read_vram_test() {
        let mut ppu_context = PpuContext {
            cram: Ram::new(vec![0;0x20]),
            vram: Ram::new(vec![0;0x20]),
            palette_ram: PaletteRam::new(),
        };

        ppu_context.vram.write(0x00, 0xFF);

        let mut ppu_data = PpuData { buf: 0xEE };
        let read_data = ppu_data.read(0x2000, &mut ppu_context);

        assert_eq!(read_data, 0xEE);
        assert_eq!(ppu_data.buf, 0xFF);
    }

    #[test]
    fn read_vram_mirror_test() {
        let mut ppu_context = PpuContext {
            cram: Ram::new(vec![0;0x20]),
            vram: Ram::new(vec![0;0x20]),
            palette_ram: PaletteRam::new(),
        };

        ppu_context.vram.write(0x00, 0xFF);

        let mut ppu_data = PpuData { buf: 0xEE };
        let read_data = ppu_data.read(0x3000, &mut ppu_context);

        assert_eq!(read_data, 0xEE);
        assert_eq!(ppu_data.buf, 0xFF);
    }

    #[test]
    fn read_palette_test() {
        let mut ppu_context = PpuContext {
            cram: Ram::new(vec![0;0x20]),
            vram: Ram::new(vec![0;0x20]),
            palette_ram: PaletteRam::new(),
        };

        ppu_context.vram.write(0x00, 0xFF);
        ppu_context.palette_ram.write(0x00, 0xEE);

        let mut ppu_data = PpuData::new();
        let read_data = ppu_data.read(0x3F00, &mut ppu_context);

        assert_eq!(read_data, 0xEE);
        assert_eq!(ppu_data.buf, 0xFF);
    }

    #[test]
    fn write_pattern_test() {
        let mut ppu_context = PpuContext {
            cram: Ram::new(vec![0;0x20]),
            vram: Ram::new(vec![0;0x20]),
            palette_ram: PaletteRam::new(),
        };

        let mut ppu_data = PpuData::new();
        ppu_data.write(0x0000, 0xFF, &mut ppu_context);

        assert_eq!(ppu_context.cram.read(0x0000), 0xFF);
    }

    #[test]
    fn write_vram_test() {
        let mut ppu_context = PpuContext {
            cram: Ram::new(vec![0;0x20]),
            vram: Ram::new(vec![0;0x20]),
            palette_ram: PaletteRam::new(),
        };

        let mut ppu_data = PpuData::new();
        ppu_data.write(0x2000, 0xFF, &mut ppu_context);

        assert_eq!(ppu_context.vram.read(0x0000), 0xFF);
    }

    #[test]
    fn write_vram_mirror_test() {
        let mut ppu_context = PpuContext {
            cram: Ram::new(vec![0;0x20]),
            vram: Ram::new(vec![0;0x20]),
            palette_ram: PaletteRam::new(),
        };

        let mut ppu_data = PpuData::new();
        ppu_data.write(0x3000, 0xFF, &mut ppu_context);

        assert_eq!(ppu_context.vram.read(0x0000), 0xFF);
    }

    #[test]
    fn write_palette_test() {
        let mut ppu_context = PpuContext {
            cram: Ram::new(vec![0;0x20]),
            vram: Ram::new(vec![0;0x20]),
            palette_ram: PaletteRam::new(),
        };

        let mut ppu_data = PpuData::new();
        ppu_data.write(0x3F00, 0xFF, &mut ppu_context);

        assert_eq!(ppu_context.palette_ram.read(0x0000), 0xFF);
    }
}
