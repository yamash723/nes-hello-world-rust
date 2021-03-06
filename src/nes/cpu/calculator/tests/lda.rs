use super::*;

#[test]
fn LDA_test() {
    let mut registers = Registers::new();
    let mut bus = BusMock::new();

    let addr = 0x0010;
    let opeland = 0x20;
    bus.write(addr, opeland);

    Calculator::LDA(&mut registers, &mut bus, addr);
    assert_eq!(registers.A, opeland);
    assert_eq!(registers.P.negative, false);
    assert_eq!(registers.P.zero, false);
}

#[test]
fn LDA_update_zero_test() {
    let mut registers = Registers::new();
    let mut bus = BusMock::new();

    let addr = 0x0010;
    let opeland = 0x00; // Zero operand
    bus.write(addr, opeland);

    Calculator::LDA(&mut registers, &mut bus, addr);
    assert_eq!(registers.A, opeland);
    assert_eq!(registers.P.negative, false);
    assert_eq!(registers.P.zero, true);
}

#[test]
fn LDA_update_negative_test() {
    let mut registers = Registers::new();
    let mut bus = BusMock::new();

    let addr = 0x0010;
    let opeland = 0x90; // Nagative opeland(over than 0x80)
    bus.write(addr, opeland);

    Calculator::LDA(&mut registers, &mut bus, addr);
    assert_eq!(registers.A, opeland);
    assert_eq!(registers.P.negative, true);
    assert_eq!(registers.P.zero, false);
}

#[test]
fn LDA_immediate_test() {
    let mut registers = Registers::new();
    let opeland = 0x0010;

    Calculator::LDA_immediate(&mut registers, opeland);
    assert_eq!(registers.A, opeland as u8);
    assert_eq!(registers.P.negative, false);
    assert_eq!(registers.P.zero, false);
}

#[test]
fn LDA_immediate_update_zero_test() {
    let mut registers = Registers::new();
    let opeland = 0x0000; // Zero operand

    Calculator::LDA_immediate(&mut registers, opeland);
    assert_eq!(registers.A, opeland as u8);
    assert_eq!(registers.P.negative, false);
    assert_eq!(registers.P.zero, true);
}

#[test]
fn LDA_immediate_update_negative_test() {
    let mut registers = Registers::new();
    let opeland = 0x90; // Nagative opeland(over than 0x80)

    Calculator::LDA_immediate(&mut registers, opeland);
    assert_eq!(registers.A, opeland as u8);
    assert_eq!(registers.P.negative, true);
    assert_eq!(registers.P.zero, false);
}
