use super::*;

#[test]
fn JMP_test() {
    let mut registers = Registers::new();
    let opeland = 0x0090;

    Calculator::JMP(&mut registers, opeland);
    assert_eq!(opeland, registers.PC);
}