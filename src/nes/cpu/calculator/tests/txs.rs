use super::*;

#[test]
fn TXS_test() {
    let mut registers = Registers::new();
    registers.X = 0x89;
    registers.S = 0x00;

    Calculator::TXS(&mut registers);
    assert_eq!(registers.S, registers.X);
}