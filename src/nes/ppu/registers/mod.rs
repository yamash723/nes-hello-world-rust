mod ppu_ctrl;
mod ppu_mask;
mod ppu_scroll;
mod ppu_addr;
mod ppu_data;

use self::ppu_ctrl::PpuCtrl;
use self::ppu_mask::PpuMask;
use self::ppu_scroll::PpuScroll;
use self::ppu_addr::PpuAddr;
use self::ppu_data::PpuData;

use nes::ram::Ram;
use nes::ppu::PpuContext;

pub struct Registers {
    pub ppu_ctrl: PpuCtrl,
    pub ppu_mask: PpuMask,
    pub ppu_addr: PpuAddr,
    pub ppu_data: PpuData,
    pub ppu_scroll: PpuScroll,

    // unimplemented!
    // pub ppu_status:,
    // pub oam:,
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            ppu_ctrl: PpuCtrl::new(),
            ppu_mask: PpuMask::new(),
            ppu_scroll: PpuScroll::new(),
            ppu_addr: PpuAddr::new(),
            ppu_data: PpuData::new(),
        }
    }

    pub fn write(&mut self, addr: u16, data: u8, ppu_context: &mut PpuContext) {
        match addr {
            0x0000 => self.ppu_ctrl.write(data),
            0x0001 => self.ppu_mask.write(data),
            0x0005 => self.ppu_scroll.write(data),
            0x0006 => self.ppu_addr.write(data as u16),
            0x0007 => self.ppu_data_write(data, ppu_context),
            _ => unimplemented!()
        }
    }

    pub fn read(&mut self, addr: u16, ppu_context: &mut PpuContext) -> u8 {
        match addr {
            0x0007 => self.ppu_data_read(ppu_context),
            _ => unimplemented!(),
        }
    }

    fn ppu_data_read(&mut self, ppu_context: &mut PpuContext) -> u8 {
        let addr = self.ppu_addr.read();
        let data = self.ppu_data.read(addr, ppu_context);
        self.increment_vram();
        
        data
    }

    fn ppu_data_write(&mut self, data: u8, ppu_context: &mut PpuContext) {
        let addr = self.ppu_addr.read();
        self.ppu_data.write(addr, data, ppu_context);
        self.increment_vram();
    }

    pub fn get_nametable_id(&self) -> u8 {
        self.ppu_ctrl.get_nametable_id()
    }

    fn increment_vram(&mut self) {
        let offset = self.ppu_ctrl.get_vram_increment_offset();
        self.ppu_addr.update(offset);
    }
}

#[cfg(test)]
mod registers_test {
    use super::*;
    use nes::ppu::palette_ram::PaletteRam;

    fn dummy_ppu_context() -> PpuContext {
        PpuContext {
            vram: Ram::new(vec![0;0x20]),
            cram: Ram::new(vec![0;0x20]),
            palette_ram: PaletteRam::new(),
        }
    }

    #[test]
    fn increment_vram_test() {
        let mut registers = Registers::new();
        registers.increment_vram();
        assert_eq!(registers.ppu_addr.read(), 0x0000 + 1); // increment 1

        let mut registers = Registers::new();
        registers.ppu_ctrl.write(0b00000100);
        registers.increment_vram();
        assert_eq!(registers.ppu_addr.read(), 0x0000 + 32); // increment 32
    }

    #[test]
    fn write_ppu_ctrl_test() {
        let mut ppu_context = dummy_ppu_context();
        let mut registers = Registers::new();
        registers.write(0x0000, 0xFF, &mut ppu_context);
        assert_eq!(registers.ppu_ctrl.read(), 0xFF);
    }

    #[test]
    fn write_ppu_mask_test() {
        let mut ppu_context = dummy_ppu_context();
        let mut registers = Registers::new();
        registers.write(0x0001, 0xFF, &mut ppu_context);
        assert_eq!(registers.ppu_mask.read(), 0xFF);
    }

    #[test]
    fn write_ppu_scroll_test() {
        let mut ppu_context = dummy_ppu_context();
        let mut registers = Registers::new();

        registers.write(0x0005, 0xFF, &mut ppu_context);
        registers.write(0x0005, 0xEE, &mut ppu_context);
        assert_eq!(registers.ppu_scroll.x, 0xFF);
        assert_eq!(registers.ppu_scroll.y, 0xEE);
    }

    #[test]
    fn write_ppu_addr_test() {
        let mut ppu_context = dummy_ppu_context();
        let mut registers = Registers::new();

        registers.write(0x0006, 0xFF, &mut ppu_context);
        assert_eq!(registers.ppu_addr.read(), 0xFF00);
    }

    #[test]
    fn write_ppu_data_test() {
        let mut ppu_context = dummy_ppu_context();
        let mut registers = Registers::new();

        registers.ppu_data.buf = 0x10;
        registers.write(0x0007, 0xFF, &mut ppu_context);
        assert_eq!(registers.ppu_addr.read(), 0x00 + 1); // incremented

        assert_eq!(registers.ppu_data.read(0x0000, &mut ppu_context), 0x10); // read PpuData buf
        assert_eq!(registers.ppu_data.read(0x0000, &mut ppu_context), 0xFF); // read wrote data
    }

    #[test]
    fn read_ppu_data_test() {
        let mut ppu_context = dummy_ppu_context();
        let mut registers = Registers::new();
        registers.ppu_addr.update(0x0F);
        registers.ppu_data.write(0x000F, 0xEE, &mut ppu_context);

        assert_eq!(registers.read(0x0007, &mut ppu_context), 0x00); // read PpuData buf
        assert_eq!(registers.ppu_addr.read(), 0x0F + 1);            // incremented
        assert_eq!(registers.read(0x0007, &mut ppu_context), 0xEE); // read wrote data
        assert_eq!(registers.ppu_addr.read(), 0x0F + 2);            // incremented
    }
}