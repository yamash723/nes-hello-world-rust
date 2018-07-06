use super::*;

#[test]
fn DEY_test() {
    let mut registers = Registers::new();
    let opeland = 0x20;
    registers.Y = opeland + 1;

    Calculator::DEY(&mut registers);
    assert_eq!(registers.Y, opeland);
    assert_eq!(registers.P.negative, false);
    assert_eq!(registers.P.zero, false);
}

#[test]
fn DEY_update_zero_test() {
    let mut registers = Registers::new();
    let opeland = 0x00; // Zero operand
    registers.Y = opeland + 1;

    Calculator::DEY(&mut registers);
    assert_eq!(registers.Y, opeland);
    assert_eq!(registers.P.negative, false);
    assert_eq!(registers.P.zero, true);
}

#[test]
fn DEY_update_negative_test() {
    let mut registers = Registers::new();
    let opeland = 0x90; // Nagative opeland(over than 0x80)
    registers.Y = opeland + 1;

    Calculator::DEY(&mut registers);
    assert_eq!(registers.Y, opeland);
    assert_eq!(registers.P.negative, true);
    assert_eq!(registers.P.zero, false);
}