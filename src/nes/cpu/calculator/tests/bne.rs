use super::*;

#[test]
fn BNE_test() {
    let mut registers = Registers::new();
    registers.P.zero = false;

    let opeland = 0x0090;
    Calculator::BNE(&mut registers, opeland);

    assert_eq!(opeland, registers.PC);
}