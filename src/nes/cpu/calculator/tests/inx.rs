use super::*;

#[test]
fn INX_test() {
    let mut registers = Registers::new();
    let opeland = 0x20;
    registers.X = opeland - 1;

    Calculator::INX(&mut registers);
    assert_eq!(registers.X, opeland);
    assert_eq!(registers.P.negative, false);
    assert_eq!(registers.P.zero, false);
}

// INX is never set zero to registers.
// (INX is incrementing value of X before set registers)

// #[test]
// fn INX_update_zero_test() {
//     let mut registers = Registers::new();
//     let opeland = 0x00; // Zero operand
//     registers.X = opeland - 1;

//     Calculator::INX(&mut registers);
//     assert_eq!(registers.X, opeland);
//     assert_eq!(registers.P.negative, false);
//     assert_eq!(registers.P.zero, true);
// }

#[test]
fn INX_update_negative_test() {
    let mut registers = Registers::new();
    let opeland = 0x90; // Nagative opeland(over than 0x80)
    registers.X = opeland - 1;

    Calculator::INX(&mut registers);
    assert_eq!(registers.X, opeland);
    assert_eq!(registers.P.negative, true);
    assert_eq!(registers.P.zero, false);
}