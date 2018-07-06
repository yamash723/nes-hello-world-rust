use nes::cassette::NesCassette;
use nes::ppu::Ppu;
use nes::ram::Ram;

pub struct Bus<'a, T: 'a> where T: NesCassette {
    cassette: &'a T,
    ppu: &'a mut Ppu,
    wram: &'a mut Ram,
}

pub trait CpuBus {
    fn read(&mut self, addr: u16) -> u8;
    fn read_twice(&mut self, addr: u16) -> u16;
    fn write(&mut self, addr: u16, data: u8);
}

impl <'a, T: 'a> Bus<'a, T> where T: NesCassette {
    pub fn new(cassette: &'a T, ppu: &'a mut Ppu, wram: &'a mut Ram) -> Self {
        Self {
            cassette: cassette,
            ppu: ppu,
            wram: wram,
        }
    }
}

impl <'a, T: 'a> CpuBus for Bus<'a, T> where T: NesCassette {
    fn read(&mut self, addr: u16) -> u8 {
        match addr {
            0x0000..0x07FF => self.wram.read(addr),
            0x0800..0x1FFF => self.wram.read(addr - 0x0800),
            0x2000..0x2007 => self.ppu.read(addr),
            0x2008..0x3FFF => self.ppu.read(addr - 0x2008),
            // 0x4000..0x401F => unimplemented!(), // APU I/O Keypad
            // 0x4020..0x5FFF => unimplemented!(), // Expantion Rom
            // 0x6000..0x7FFF => unimplemented!(), // Expantion Ram
            0x8000..0xBFFF => {
                // ToDo: refactoring
                if self.cassette.program_rom_length() <= 0x4000 {
                    self.cassette.read_program_rom(addr - 0xC000)
                } else {
                    self.cassette.read_program_rom(addr - 0x8000)
                }
            },
            0xC000..0xFFFF => self.cassette.read_program_rom(addr - 0x8000),
            _ => panic!("unexpected memory area access!"),
        }
    }

    fn read_twice(&mut self, addr: u16) -> u16 {
        let lower = self.read(addr) as u16;
        let upper = self.read(addr + 1) as u16;

        (upper << 8) | lower
    }

    fn write(&mut self, addr: u16, data: u8) {
        match addr {
            0x0000..0x07FF => self.wram.write(addr, data),
            0x0800..0x1FFF => self.wram.write(addr - 0x0800, data),
            0x2000..0x3FFF => self.ppu.write(addr - 0x2000, data),
            // 0x4000..0x401F => unimplemented!(), // APU I/O Keypad
            // 0x4020..0x5FFF => unimplemented!(), // Expantion Rom
            // 0x6000..0x7FFF => unimplemented!(), // Expantion Ram
            _ => panic!("unexpected memory area access!"),
        }
    }
}

#[cfg(test)]
mod cpu_bus_test {
    use super::*;

    struct CassetteMock {
        program_rom: Vec<u8>,
        character_rom: Vec<u8>,
    }

    impl CassetteMock {
        fn new() -> Self {
            CassetteMock {
                program_rom: vec![0; 0x5000],
                character_rom: vec![0; 0x5000],
            }
        }
    }

    impl NesCassette for CassetteMock {
        fn read_program_rom(&self, addr: u16) -> u8 {
            self.program_rom[addr as usize]
        }

        fn read_character_rom(&self, addr: u16) -> u8 {
            self.character_rom[addr as usize]
        }

        fn program_rom_length(&self) -> usize {
            self.program_rom.len()
        }
    }

    #[test]
    fn read_from_wram_address() {
        let cassette = CassetteMock::new();
        let mut ppu = Ppu::new(cassette.character_rom.clone());
        let mut ram = Ram::new(vec![0; 2048]);
        ram.write(0x0002, 0x4F);

        let mut cpu_bus = Bus::new(
            &cassette,
            &mut ppu,
            &mut ram,
        );

        assert_eq!(cpu_bus.read(0x0002), 0x4F);
    }

    #[test]
    fn read_from_program_rom_address() {
        let mut cassette = CassetteMock::new();
        let mut ppu = Ppu::new(cassette.character_rom.clone());
        let mut ram = Ram::new(vec![0; 2048]);
        cassette.program_rom[0x0000] = 0x78;

        let mut cpu_bus = Bus::new(
            &cassette,
            &mut ppu,
            &mut ram,
        );

        assert_eq!(cpu_bus.read(0x8000), 0x78);
    }

    #[test]
    fn write_wram_address() {
        let cassette = CassetteMock::new();
        let mut ppu = Ppu::new(cassette.character_rom.clone());
        let mut ram = Ram::new(vec![0; 2048]);
        let mut cpu_bus = Bus::new(
            &cassette,
            &mut ppu,
            &mut ram,
        );

        cpu_bus.write(0x0002, 0x4F);
        assert_eq!(cpu_bus.read(0x0002), 0x4F);
    }

    #[test]
    fn read_twice_test() {
        let mut cassette = CassetteMock::new();
        let mut ppu = Ppu::new(cassette.character_rom.clone());
        let mut ram = Ram::new(vec![0; 2048]);
        cassette.program_rom[0x0000] = 0x78;
        cassette.program_rom[0x0001] = 0x56;

        let mut cpu_bus = Bus::new(
            &cassette,
            &mut ppu,
            &mut ram,
        );

        assert_eq!(cpu_bus.read_twice(0x8000), 0x5678);
    }
}