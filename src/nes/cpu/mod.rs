#![allow(non_snake_case)]

pub mod bus;
pub mod registers;

pub use super::cpu::bus::{Bus, CpuBus};
pub use super::cpu::registers::Registers;

mod opecode;
mod controller;
mod calculator;

use self::opecode::AddressingMode;
use self::calculator::Calculator;
use self::controller::Controller;

pub struct Cpu {
    pub registers: Registers,
}

impl Cpu {
    pub fn new() -> Self {
        Self { registers: Registers::new() }
    }

    pub fn run<T: CpuBus>(&mut self, bus: &mut T) -> usize {
        Calculator::execute(&mut self.registers, bus)
    }

    pub fn reset<T: CpuBus>(&mut self, bus: &mut T) {
        Controller::reset(&mut self.registers, bus);
    }
}

#[cfg(test)]
mod cpu_test {
    use super::*;

    struct BusMock {
        ram: Vec<u8>,
    }

    impl BusMock {
        fn new() -> Self {
            Self { ram: vec![0; 0xFFFF] }
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

    #[test]
    fn test_new() {
        let cpu = Cpu::new();
        assert_eq!(cpu.registers, Registers::new());
    }

    #[test]
    fn test_reset() {
        let mut cpu = Cpu::new();
        let mut bus = BusMock::new();
        bus.write(0xFFFC, 0x00);
        bus.write(0xFFFD, 0x80);

        cpu.reset(&mut bus);
        assert_eq!(cpu.registers.PC, 0x8000);
    }
}