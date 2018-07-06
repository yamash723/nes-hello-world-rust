use super::*;

#[test]
fn STA_test() {
    let mut registers = Registers::new();
    let mut bus = BusMock::new();
    let opeland = 0x90;

    registers.A = 0x89;
    Calculator::STA(&registers, &mut bus, opeland);

    let actual = bus.read(opeland as u16);
    assert_eq!(actual, registers.A);
}