use super::bus::CpuBus;
use super::registers::Registers;
use super::AddressingMode;

pub struct Controller;

impl Controller {
    pub fn reset<T: CpuBus>(registers: &mut Registers, bus: &mut T) {
        registers.reset();
        registers.PC = bus.read_twice(0xFFFC);
    }

    pub fn fetch<T: CpuBus>(registers: &mut Registers, bus: &mut T) -> u8 {
        let code = bus.read(registers.PC);
        registers.PC += 1;
        code
    }

    pub fn fetch_opeland<T: CpuBus>(registers: &mut Registers, bus: &mut T, mode: &AddressingMode) -> u16 {
        match *mode {
            AddressingMode::Implied => 0x0000,
            AddressingMode::Accumulator => 0x0000,
            AddressingMode::Immediate => Controller::fetch(registers, bus) as u16,
            AddressingMode::Relative => Controller::fetch_relative(registers, bus),
            AddressingMode::ZeroPage => Controller::fetch(registers, bus) as u16,
            AddressingMode::ZeroPageX => Controller::fetch_page_zero_x(registers, bus),
            AddressingMode::ZeroPageY => Controller::fetch_page_zero_y(registers, bus),
            AddressingMode::Absolute => Controller::fetch_absolute(registers, bus),
            AddressingMode::AbsoluteX => Controller::fetch_absolute_x(registers, bus),
            AddressingMode::AbsoluteY => Controller::fetch_absolute_y(registers, bus),
            AddressingMode::PreIndexedIndirect => Controller::fetch_pre_indexed_indirect(registers, bus),
            AddressingMode::PostIndexedIndirect => Controller::fetch_post_indexed_indirect(registers, bus),
            AddressingMode::IndirectAbsolute => Controller::fetch_indirect_absolute(registers, bus),
        }
    }

    pub fn fetch_relative<T: CpuBus>(registers: &mut Registers, bus: &mut T) -> u16 {
        let offset = Controller::fetch(registers, bus) as u16;

        if offset < 0x80 {
            registers.PC + offset
        } else {
            registers.PC + offset - 0x100
        }
    }

    pub fn fetch_page_zero_x<T: CpuBus>(registers: &mut Registers, bus: &mut T) -> u16 {
        (Controller::fetch(registers, bus) + registers.X) as u16
    }

    pub fn fetch_page_zero_y<T: CpuBus>(registers: &mut Registers, bus: &mut T) -> u16 {
        (Controller::fetch(registers, bus) + registers.X) as u16
    }

    pub fn fetch_absolute<T: CpuBus>(registers: &mut Registers, bus: &mut T) -> u16 {
        let lower = Controller::fetch(registers, bus) as u16;
        let upper = Controller::fetch(registers, bus) as u16;
        lower | upper << 8
    }

    pub fn fetch_absolute_x<T: CpuBus>(registers: &mut Registers, bus: &mut T) -> u16 {
        Controller::fetch_absolute(registers, bus) + registers.X as u16
    }

    pub fn fetch_absolute_y<T: CpuBus>(registers: &mut Registers, bus: &mut T) -> u16 {
        Controller::fetch_absolute(registers, bus) + registers.Y as u16
    }

    pub fn fetch_pre_indexed_indirect<T: CpuBus>(registers: &mut Registers, bus: &mut T) -> u16 {
        let fetch_data = Controller::fetch(registers, bus);
        let addr = (fetch_data + registers.X) as u16;
        bus.read_twice(addr)
    }

    pub fn fetch_post_indexed_indirect<T: CpuBus>(registers: &mut Registers, bus: &mut T) -> u16 {
        let addr = Controller::fetch(registers, bus) as u16;
        bus.read_twice(addr) + registers.Y as u16
    }

    pub fn fetch_indirect_absolute<T: CpuBus>(registers: &mut Registers, bus: &mut T) -> u16 {
        let lower_address = Controller::fetch_absolute(registers, bus);
        let lower = bus.read(lower_address) as u16;

        // address of upper address
        //   0x0004 -> 0x0005 (next address of lower)
        //   0x01FF -> 0x0100 (not carry to upper address)
        let lower_masked_address = lower_address & 0xFF00;
        let upper_masked_address = lower_address & 0x00FF;
        let upper_address = lower_masked_address | ((upper_masked_address + 1) & 0x00FF);
        let upper = bus.read(upper_address) as u16;

        lower | upper << 8
    }
}

#[cfg(test)]
mod controller_test {
    use super::*;
    use nes::cpu::CpuBus;
    use nes::cpu::registers::Registers;

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
    fn fetch_test() {
        let mut registers = Registers::new();
        let mut bus = BusMock::new();

        registers.PC = 0x0000;
        bus.ram[0x0000] = 0x10;

        assert_eq!(Controller::fetch(&mut registers, &mut bus), 0x10);
        assert_eq!(registers.PC, 0x0001);
    }

    #[test]
    fn reset_test() {
        let mut bus = BusMock::new();
        bus.write(0xFFFD, 0x56); // upper address
        bus.write(0xFFFC, 0x78); // lower address

        let mut registers = Registers::new();
        // set no default values.
        registers.A = 0x01;
        registers.X = 0x01;
        registers.Y = 0x01;
        registers.S = 0x01;
        registers.P.negative = !registers.P.negative;
        registers.P.overflow = !registers.P.overflow;
        registers.P.reserved = !registers.P.reserved;
        registers.P.break_mode = !registers.P.break_mode;
        registers.P.decimal = !registers.P.decimal;
        registers.P.interrupt = !registers.P.interrupt;
        registers.P.zero = !registers.P.zero;
        registers.P.carry = !registers.P.carry;
        registers.PC = 0x0001;

        let mut expect_registers = Registers::new();
        expect_registers.PC = 0x5678;

        Controller::reset(&mut registers, &mut bus);
        assert_eq!(registers, expect_registers);
    }

    #[test]
    fn fetch_absolute_test() {
        let mut registers = Registers::new();
        let mut bus = BusMock::new();

        registers.PC = 0x0000;
        bus.ram[0x0000] = 0x34; // lower address
        bus.ram[0x0001] = 0x12; // upper address

        assert_eq!(Controller::fetch_absolute(&mut registers, &mut bus), 0x1234);
        assert_eq!(registers.PC, 0x0002);
    }

    #[test]
    fn fetch_pre_indexed_indirect_test() {
        let mut registers = Registers::new();
        let mut bus = BusMock::new();

        registers.X = 0x05;
        registers.PC = 0x0000;
        bus.ram[0x0000] = 0x02;

        // bus.ram[0x0001] + registers.X = `0x0007`
        bus.ram[0x0007] = 0x78; // lower address
        bus.ram[0x0008] = 0x56; // upper address

        assert_eq!(Controller::fetch_pre_indexed_indirect(&mut registers, &mut bus), 0x5678);
        assert_eq!(registers.PC, 0x0001);
    }

    #[test]
    fn fetch_post_indexed_indirect_test() {
        let mut registers = Registers::new();
        let mut bus = BusMock::new();

        registers.Y = 0x05;
        registers.PC = 0x0000;
        bus.ram[0x0000] = 0x10;

        bus.ram[0x0010] = 0x73; // lower address
        bus.ram[0x0011] = 0x56; // upper address

        // expect value is 0x5673(read from bus) + 0x10(read from Y of the registers)
        assert_eq!(Controller::fetch_post_indexed_indirect(&mut registers, &mut bus), 0x5678);
        assert_eq!(registers.PC, 0x0001);
    }

    #[test]
    fn fetch_relative_positive_test() {
        let mut registers = Registers::new();
        let mut bus = BusMock::new();

        // offset number. 0x00 - 0x7F -> positive
        let offset = 0x10;

        registers.PC = 0x0100;
        bus.ram[0x0100] = offset;

        // Program counter + offset number.
        let except_number = (registers.PC + 1) + offset as u16;

        assert_eq!(Controller::fetch_relative(&mut registers, &mut bus), except_number);
        assert_eq!(registers.PC, 0x0101);
    }

    #[test]
    fn fetch_relative_negative_test() {
        let mut registers = Registers::new();
        let mut bus = BusMock::new();

        // offset number. 0x80 - 0xFF -> negative
        let offset = 0x90;

        registers.PC = 0x0200;
        bus.ram[0x0200] = offset;

        // Program counter + offset number.
        let real_offset = offset as i16 - 0x0100;
        let program_counter = (registers.PC + 1) as i16;
        let except_number = (program_counter + real_offset) as u16;

        assert_eq!(Controller::fetch_relative(&mut registers, &mut bus), except_number);
        assert_eq!(registers.PC, 0x0201);
    }

    #[test]
    fn fetch_indirect_absolute_test() {
        let mut registers = Registers::new();
        let mut bus = BusMock::new();

        registers.PC = 0x0000;

        bus.ram[0x0000] = 0x04;
        bus.ram[0x0001] = 0x00;

        bus.ram[0x0004] = 0x20; // lower address
        bus.ram[0x0005] = 0x00; // upper address

        assert_eq!(Controller::fetch_indirect_absolute(&mut registers, &mut bus), 0x0020);
        assert_eq!(registers.PC, 0x0002);
    }

    #[test]
    fn fetch_indirect_absolute_not_carry_up_test() {
        let mut registers = Registers::new();
        let mut bus = BusMock::new();

        registers.PC = 0x0000;

        bus.ram[0x0000] = 0xFF;
        bus.ram[0x0001] = 0x01;

        bus.ram[0x01FF] = 0x20; // lower address
        bus.ram[0x0100] = 0x00; // upper address(not carry up to upper address)

        assert_eq!(Controller::fetch_indirect_absolute(&mut registers, &mut bus), 0x0020);
        assert_eq!(registers.PC, 0x0002);
    }
}