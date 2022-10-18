use super::instruction::{ArithmeticTarget, Instruction};
use crate::cpu::CPU;

pub fn execute(cpu: &mut CPU, instruction: Instruction) {
    match instruction {
        Instruction::ADD(target) => match target {
            ArithmeticTarget::A => todo!(),
            ArithmeticTarget::B => todo!(),
            ArithmeticTarget::C => {
                let value = cpu.registers.c;
                let new_value = add(cpu, value);
                cpu.registers.a = new_value;
            }
            ArithmeticTarget::D => todo!(),
            ArithmeticTarget::E => todo!(),
            ArithmeticTarget::H => todo!(),
            ArithmeticTarget::L => todo!(),
        },
        _ => { /*add support for more instructions*/ }
    }
}

fn add(cpu: &mut CPU, value: u8) -> u8 {
    let (new_value, did_overflow) = cpu.registers.a.overflowing_add(value);
    cpu.registers.f.zero = new_value == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.carry = did_overflow;
    // Half Carry is set if adding the lower nibbles of the value and register A
    // together result in a value bigger than 0xF. If the result is larger than 0xF
    // than the addition caused a carry from the lower nibble to the upper nibble.
    cpu.registers.f.half_carry = (cpu.registers.a & 0xF) + (value & 0xF) > 0xF;
    new_value
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_add_C() {
        let mut cpu = CPU::new();
        cpu.registers.c = 0x04;
        cpu.registers.a = 0x02;
        execute(&mut cpu, Instruction::ADD(ArithmeticTarget::C));
        assert_eq!(cpu.registers.a, 0x06);
    }

    #[test]
    fn add_set_carry() {
        let mut cpu = CPU::new();
        cpu.registers.c = 0x04;
        cpu.registers.a = 0xFE;
        execute(&mut cpu, Instruction::ADD(ArithmeticTarget::C));
        assert_eq!(cpu.registers.f.carry, true);
    }
}
