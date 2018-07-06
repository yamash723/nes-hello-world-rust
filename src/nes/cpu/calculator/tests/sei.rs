use super::*;

#[test]
fn SEI_test() {
    let mut registers = Registers::new();
    registers.P.interrupt = false;

    Calculator::SEI(&mut registers);
    assert_eq!(registers.P.interrupt, true);
}