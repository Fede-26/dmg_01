use super::instruction::Instruction;
use super::instruction::{ArithmeticTarget, ADDHLTarget};
use crate::cpu::CPU;

pub fn execute(cpu: &mut CPU, instruction: Instruction) {
    match instruction {
        Instruction::ADD(target) => match target {
            ArithmeticTarget::A => cpu.registers.a = add(cpu, cpu.registers.a),
            ArithmeticTarget::B => cpu.registers.a = add(cpu, cpu.registers.b),
            ArithmeticTarget::C => cpu.registers.a = add(cpu, cpu.registers.c),
            ArithmeticTarget::D => cpu.registers.a = add(cpu, cpu.registers.d),
            ArithmeticTarget::E => cpu.registers.a = add(cpu, cpu.registers.e),
            ArithmeticTarget::H => cpu.registers.a = add(cpu, cpu.registers.h),
            ArithmeticTarget::L => cpu.registers.a = add(cpu, cpu.registers.l),
        },
        Instruction::ADDHL(target) => match target {
            ADDHLTarget::BC => todo!(),
            ADDHLTarget::DE => todo!(),
            ADDHLTarget::HL => todo!(),
        }
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

fn add_hl(cpu: &mut CPU, value: u8) -> u8 {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_to_c() {
        let mut cpu = CPU::new();
        cpu.registers.c = 0x04;
        cpu.registers.a = 0x02;
        execute(&mut cpu, Instruction::ADD(ArithmeticTarget::C));
        assert_eq!(cpu.registers.a, 0x06);
        assert!(!cpu.registers.f.carry)
    }

    #[test]
    fn add_set_carry() {
        let mut cpu = CPU::new();
        cpu.registers.c = 0x04;
        cpu.registers.a = 0xFE;
        execute(&mut cpu, Instruction::ADD(ArithmeticTarget::C));
        assert!(cpu.registers.f.carry);
        assert_eq!(cpu.registers.a, 0x02);
    }
}
