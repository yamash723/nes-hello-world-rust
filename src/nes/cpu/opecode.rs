use std::collections::HashMap;

pub struct Opecode {
    pub command: Command,
    pub mode: AddressingMode,
    pub cycle: usize,
}


#[derive(PartialEq, Debug)]
pub enum AddressingMode {
    Implied,
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectAbsolute,
    PreIndexedIndirect,
    PostIndexedIndirect,
}

#[derive(Debug)]
pub enum Command {
    BNE,
    DEY,
    INX,
    JMP,
    LDA,
    LDX,
    LDY,
    SEI,
    STA,
    TXS,
}

lazy_static! {
    pub static ref OPECODE_MAP: HashMap<u8, Opecode> = {
        let cycles: Vec<usize> =
            vec![7, 6, 2, 8, 3, 3, 5, 5, 3, 2, 2, 2, 4, 4, 6, 6, 2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7,
                 4, 4, 6, 7, 6, 6, 2, 8, 3, 3, 5, 5, 4, 2, 2, 2, 4, 4, 6, 6, 2, 5, 2, 8, 4, 4, 6, 6,
                 2, 4, 2, 7, 4, 4, 6, 7, 6, 6, 2, 8, 3, 3, 5, 5, 3, 2, 2, 2, 3, 4, 6, 6, 2, 5, 2, 8,
                 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 6, 7, 6, 6, 2, 8, 3, 3, 5, 5, 4, 2, 2, 2, 5, 4, 6, 6,
                 2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 6, 7, 2, 6, 2, 6, 3, 3, 3, 3, 2, 2, 2, 2,
                 4, 4, 4, 4, 2, 6, 2, 6, 4, 4, 4, 4, 2, 4, 2, 5, 5, 4, 5, 5, 2, 6, 2, 6, 3, 3, 3, 3,
                 2, 2, 2, 2, 4, 4, 4, 4, 2, 5, 2, 5, 4, 4, 4, 4, 2, 4, 2, 4, 4, 4, 4, 4, 2, 6, 2, 8,
                 3, 3, 5, 5, 2, 2, 2, 2, 4, 4, 6, 6, 2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 7, 7,
                 2, 6, 3, 8, 3, 3, 5, 5, 2, 2, 2, 2, 4, 4, 6, 6, 2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7,
                 4, 4, 7, 7];

        let mut m = HashMap::new();
        m.insert(0xA9, Opecode { command: Command::LDA, mode: AddressingMode::Immediate, cycle: cycles[0xA9] });
        m.insert(0xA5, Opecode { command: Command::LDA, mode: AddressingMode::ZeroPage, cycle: cycles[0xA5] });
        m.insert(0xAD, Opecode { command: Command::LDA, mode: AddressingMode::Absolute, cycle: cycles[0xAD] });
        m.insert(0xB5, Opecode { command: Command::LDA, mode: AddressingMode::ZeroPageX, cycle: cycles[0xB5] });
        m.insert(0xBD, Opecode { command: Command::LDA, mode: AddressingMode::AbsoluteX, cycle: cycles[0xBD] });
        m.insert(0xB9, Opecode { command: Command::LDA, mode: AddressingMode::AbsoluteY, cycle: cycles[0xB9] });
        m.insert(0xA1, Opecode { command: Command::LDA, mode: AddressingMode::PreIndexedIndirect, cycle: cycles[0xA1] });
        m.insert(0xB1, Opecode { command: Command::LDA, mode: AddressingMode::PostIndexedIndirect, cycle: cycles[0xB1] });
        m.insert(0xA2, Opecode { command: Command::LDX, mode: AddressingMode::Immediate, cycle: cycles[0xA2] });
        m.insert(0xA6, Opecode { command: Command::LDX, mode: AddressingMode::ZeroPage, cycle: cycles[0xA6] });
        m.insert(0xAE, Opecode { command: Command::LDX, mode: AddressingMode::Absolute, cycle: cycles[0xAE] });
        m.insert(0xB6, Opecode { command: Command::LDX, mode: AddressingMode::ZeroPageY, cycle: cycles[0xB6] });
        m.insert(0xBE, Opecode { command: Command::LDX, mode: AddressingMode::AbsoluteY, cycle: cycles[0xBE] });
        m.insert(0xA0, Opecode { command: Command::LDY, mode: AddressingMode::Immediate, cycle: cycles[0xA0] });
        m.insert(0xA4, Opecode { command: Command::LDY, mode: AddressingMode::ZeroPage, cycle: cycles[0xA4] });
        m.insert(0xAC, Opecode { command: Command::LDY, mode: AddressingMode::Absolute, cycle: cycles[0xAC] });
        m.insert(0xB4, Opecode { command: Command::LDY, mode: AddressingMode::ZeroPageX, cycle: cycles[0xB4] });
        m.insert(0xBC, Opecode { command: Command::LDY, mode: AddressingMode::AbsoluteX, cycle: cycles[0xBC] });
        m.insert(0x85, Opecode { command: Command::STA, mode: AddressingMode::ZeroPage, cycle: cycles[0x85] });
        m.insert(0x8D, Opecode { command: Command::STA, mode: AddressingMode::Absolute, cycle: cycles[0x8D] });
        m.insert(0x95, Opecode { command: Command::STA, mode: AddressingMode::ZeroPageX, cycle: cycles[0x95] });
        m.insert(0x9D, Opecode { command: Command::STA, mode: AddressingMode::AbsoluteX, cycle: cycles[0x9D] });
        m.insert(0x99, Opecode { command: Command::STA, mode: AddressingMode::AbsoluteY, cycle: cycles[0x99] });
        m.insert(0x81, Opecode { command: Command::STA, mode: AddressingMode::PreIndexedIndirect, cycle: cycles[0x81] });
        m.insert(0x91, Opecode { command: Command::STA, mode: AddressingMode::PostIndexedIndirect, cycle: cycles[0x91] });
        m.insert(0x9A, Opecode { command: Command::TXS, mode: AddressingMode::Implied, cycle: cycles[0x9A] });
        m.insert(0xE8, Opecode { command: Command::INX, mode: AddressingMode::Implied, cycle: cycles[0xE8] });
        m.insert(0x88, Opecode { command: Command::DEY, mode: AddressingMode::Implied, cycle: cycles[0x88] });
        m.insert(0x78, Opecode { command: Command::SEI, mode: AddressingMode::Implied, cycle: cycles[0x78] });
        m.insert(0x4C, Opecode { command: Command::JMP, mode: AddressingMode::Absolute, cycle: cycles[0x4C] });
        m.insert(0x6C, Opecode { command: Command::JMP, mode: AddressingMode::IndirectAbsolute, cycle: cycles[0x6C] });
        m.insert(0xD0, Opecode { command: Command::BNE, mode: AddressingMode::Relative, cycle: cycles[0xD0] });
        m
    };
}