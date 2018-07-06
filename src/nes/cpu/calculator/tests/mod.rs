use super::*;

use nes::cpu::bus::CpuBus;
use nes::cpu::registers::Registers;

struct BusMock {
    ram: Vec<u8>,
}

impl BusMock {
    fn new() -> Self {
        Self { ram: vec![0; 0x100] }
    }
}

impl CpuBus for BusMock {
    fn read(&mut self, addr: u16) -> u8 {
        self.ram[addr as usize]
    }

    fn read_twice(&mut self, addr: u16) -> u16 {
        let lower = self.ram[addr as usize] as u16;
        let upper = self.ram[addr as usize + 1] as u16;
        lower | upper << 8
    }

    fn write(&mut self, addr: u16, data: u8) {
        self.ram[addr as usize] = data;
    }
}

mod bne;
mod dey;
mod inx;
mod jmp;
mod lda;
mod ldx;
mod ldy;
mod sei;
mod sta;
mod txs;