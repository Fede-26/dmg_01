use super::instruction::Instruction;
use super::instruction::{ADDHLTarget, ArithmeticTarget};
use crate::cpu::CPU;

pub fn execute(cpu: &mut CPU, instruction: Instruction) {
    match instruction {
        Instruction::ADD(target) => match target {
            ArithmeticTarget::A => cpu.registers.a = add(cpu, cpu.registers.a, false),
            ArithmeticTarget::B => cpu.registers.a = add(cpu, cpu.registers.b, false),
            ArithmeticTarget::C => cpu.registers.a = add(cpu, cpu.registers.c, false),
            ArithmeticTarget::D => cpu.registers.a = add(cpu, cpu.registers.d, false),
            ArithmeticTarget::E => cpu.registers.a = add(cpu, cpu.registers.e, false),
            ArithmeticTarget::H => cpu.registers.a = add(cpu, cpu.registers.h, false),
            ArithmeticTarget::L => cpu.registers.a = add(cpu, cpu.registers.l, false),
            ArithmeticTarget::HLI => todo!(),
            ArithmeticTarget::D8 => todo!(),
        },

        Instruction::ADC(target) => match target {
            ArithmeticTarget::A => cpu.registers.a = add(cpu, cpu.registers.a, true),
            ArithmeticTarget::B => cpu.registers.a = add(cpu, cpu.registers.b, true),
            ArithmeticTarget::C => cpu.registers.a = add(cpu, cpu.registers.c, true),
            ArithmeticTarget::D => cpu.registers.a = add(cpu, cpu.registers.d, true),
            ArithmeticTarget::E => cpu.registers.a = add(cpu, cpu.registers.e, true),
            ArithmeticTarget::H => cpu.registers.a = add(cpu, cpu.registers.h, true),
            ArithmeticTarget::L => cpu.registers.a = add(cpu, cpu.registers.l, true),
            ArithmeticTarget::HLI => todo!(),
            ArithmeticTarget::D8 => todo!(),
        },

        Instruction::SUB(target) => match target {
            ArithmeticTarget::A => cpu.registers.a = sub(cpu, cpu.registers.a, false),
            ArithmeticTarget::B => cpu.registers.a = sub(cpu, cpu.registers.b, false),
            ArithmeticTarget::C => cpu.registers.a = sub(cpu, cpu.registers.c, false),
            ArithmeticTarget::D => cpu.registers.a = sub(cpu, cpu.registers.d, false),
            ArithmeticTarget::E => cpu.registers.a = sub(cpu, cpu.registers.e, false),
            ArithmeticTarget::H => cpu.registers.a = sub(cpu, cpu.registers.h, false),
            ArithmeticTarget::L => cpu.registers.a = sub(cpu, cpu.registers.l, false),
            ArithmeticTarget::HLI => todo!(),
            ArithmeticTarget::D8 => todo!(),
        },

        Instruction::SBC(target) => todo!(),

        Instruction::AND(target) => todo!(),

        Instruction::XOR(target) => todo!(),

        Instruction::OR(target) => todo!(),

        Instruction::CP(target) => todo!(),

        Instruction::INC(target) => todo!(),

        Instruction::DEC(target) => todo!(),

        Instruction::DAA => todo!(),

        Instruction::CPL => todo!(),

        Instruction::ADDHL(target) => match target {
            ADDHLTarget::BC => {
                let value = add_hl(cpu, cpu.registers.get_bc());
                cpu.registers.set_hl(value);
            }
            ADDHLTarget::DE => {
                let value = add_hl(cpu, cpu.registers.get_de());
                cpu.registers.set_hl(value);
            }
            ADDHLTarget::HL => {
                let value = add_hl(cpu, cpu.registers.get_hl());
                cpu.registers.set_hl(value);
            }
            ADDHLTarget::SP => {
                let value = add_hl(cpu, cpu.registers.get_hl());
                cpu.sp = value;
            }
        },

        Instruction::ADDSP => todo!(),
        Instruction::LD => todo!(),
        // _ => { /*add support for more instructions*/ }
    }
}

fn add(cpu: &mut CPU, value: u8, with_carry: bool) -> u8 {
    let additional_carry = if with_carry && cpu.registers.f.carry {
        1
    } else {
        0
    };

    let (new_value, did_overflow) = cpu.registers.a.overflowing_add(value);
    let (new_value2, did_overflow2) = new_value.overflowing_add(additional_carry);
    cpu.registers.f.zero = new_value2 == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.carry = did_overflow || did_overflow2;
    // TODO: check if the carry need's to be set to zero;

    // Half Carry is set if adding the lower nibbles of the value and register A
    // together result in a value bigger than 0xF. If the result is larger than 0xF
    // than the addition caused a carry from the lower nibble to the upper nibble.
    cpu.registers.f.half_carry = ((cpu.registers.a & 0xF) + (value & 0xF) + additional_carry) > 0xF;
    new_value2
}

fn sub(cpu: &mut CPU, value: u8, with_carry: bool) -> u8 {
    let additional_carry = if with_carry && cpu.registers.f.carry {
        1
    } else {
        0
    };

    let (new_value, did_overflow) = cpu.registers.a.overflowing_sub(value);
    let (new_value2, did_overflow2) = new_value.overflowing_sub(additional_carry);
    cpu.registers.f.zero = new_value2 == 0;
    cpu.registers.f.subtract = true;
    cpu.registers.f.carry = did_overflow || did_overflow2;
    // TODO: check if the carry need's to be set to zero;

    // Half Carry is set if adding the lower nibbles of the value and register A
    // together result in a value bigger than 0xF. If the result is larger than 0xF
    // than the addition caused a carry from the lower nibble to the upper nibble.
    cpu.registers.f.half_carry = (cpu.registers.a & 0xF) < (value & 0xF) + additional_carry;
    new_value2
}

fn add_hl(cpu: &mut CPU, value: u16) -> u16 {
    let (new_value, did_overflow) = cpu.registers.get_hl().overflowing_add(value);
    cpu.registers.f.zero = new_value == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.carry = did_overflow;
    new_value
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

    #[test]
    fn sub_set_carry() {
        let mut cpu = CPU::new();
        cpu.registers.c = 0x04;
        cpu.registers.a = 0x02;
        execute(&mut cpu, Instruction::SUB(ArithmeticTarget::C));
        assert!(cpu.registers.f.carry);
        assert_eq!(cpu.registers.a, 0xFE);
    }

    #[test]
    fn add_hl_to_bc() {
        let mut cpu = CPU::new();
        cpu.registers.set_hl(0x0004);
        cpu.registers.set_bc(0xFFFE);
        execute(&mut cpu, Instruction::ADDHL(ADDHLTarget::BC));
        assert!(cpu.registers.f.carry);
        assert_eq!(cpu.registers.get_hl(), 0x0002);
    }
}
