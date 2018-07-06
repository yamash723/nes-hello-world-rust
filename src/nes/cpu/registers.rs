#[derive(Debug, PartialEq)]
pub struct Registers {
    pub A: u8,
    pub X: u8,
    pub Y: u8,
    pub S: u8,
    pub P: Status,
    pub PC: u16,
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            A: 0x00,
            X: 0x00,
            Y: 0x00,
            S: 0xFD,
            P: Status {
                negative: false,
                overflow: false,
                reserved: true,
                break_mode: true,
                decimal: false,
                interrupt: true,
                zero: false,
                carry: false,
            },
            PC: 0x8000,
        }
    }

    pub fn reset(&mut self) {
        let reg = Registers::new();
        self.A = reg.A;
        self.X = reg.X;
        self.Y = reg.Y;
        self.S = reg.S;
        self.P = reg.P;
        self.PC = reg.PC;
    }
}

#[derive(Debug, PartialEq)]
pub struct Status {
    pub negative: bool,
    pub overflow: bool,
    pub reserved: bool,
    pub break_mode: bool,
    pub decimal: bool,
    pub interrupt: bool,
    pub zero: bool,
    pub carry: bool,
}

impl Status {
    pub fn to_bit(&self) -> u8 {
        (self.negative as u8) << 7 |
        (self.overflow as u8) << 6 |
        (self.reserved as u8) << 5 |
        (self.break_mode as u8) << 4 |
        (self.decimal as u8) << 3 |
        (self.interrupt as u8) << 2 |
        (self.zero as u8) << 1 |
        (self.carry as u8)
    }

    pub fn set_by_bit(&mut self, status: u8) {
        self.negative = status & 0x80 == 0x80;
        self.overflow = status & 0x40 == 0x40;
        self.reserved = status & 0x20 == 0x20;
        self.break_mode = status & 0x10 == 0x10;
        self.decimal = status & 0x08 == 0x08;
        self.interrupt = status & 0x04 == 0x04;
        self.zero = status & 0x02 == 0x02;
        self.carry = status & 0x01 == 0x01;
    }
}

#[cfg(test)]
mod registers_test {
    use super::*;

    #[test]
    fn reset_registers() {
        let mut registers = Registers::new();
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
        registers.reset();

        assert_eq!(registers, Registers::new());
    }

    #[test]
    fn registers_p_to_bit() {
        let registers = Registers::new();
        // negative:   false => 0
        // overflow:   false => 0
        // reserved:   true =>  1
        // break_mode: true =>  1
        // decimal:    false => 0
        // interrupt:  true =>  1
        // zero:       false => 0
        // carry:      false => 0
        // 00110100 => 0x34
        assert_eq!(registers.P.to_bit(), 0x34);

        let mut registers = Registers::new();
        registers.P.negative = !registers.P.negative;
        registers.P.overflow = !registers.P.overflow;
        registers.P.reserved = !registers.P.reserved;
        registers.P.break_mode = !registers.P.break_mode;
        registers.P.decimal = !registers.P.decimal;
        registers.P.interrupt = !registers.P.interrupt;
        registers.P.zero = !registers.P.zero;
        registers.P.carry = !registers.P.carry;
        
        // negative:   true  => 1
        // overflow:   true  => 1
        // reserved:   false => 0
        // break_mode: false => 0
        // decimal:    true  => 1
        // interrupt:  false => 0
        // zero:       true  => 1
        // carry:      true  => 1
        // 11001011 => 0xCB
        assert_eq!(registers.P.to_bit(), 0xCB);
    }

    #[test]
    fn registers_p_set_by_bit() {
        let mut registers = Registers::new();
        registers.P.negative = true;
        registers.P.overflow = true;
        registers.P.reserved = false;
        registers.P.break_mode = false;
        registers.P.decimal = true;
        registers.P.interrupt = false;
        registers.P.zero = true;
        registers.P.carry = true;

        // negative:   false => 0
        // overflow:   false => 0
        // reserved:   true =>  1
        // break_mode: true =>  1
        // decimal:    false => 0
        // interrupt:  true =>  1
        // zero:       false => 0
        // carry:      false => 0
        // 00110100 => 0x34
        registers.P.set_by_bit(0x34);
        assert_eq!(registers.P.to_bit(), 0x34);
    }
}