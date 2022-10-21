use super::instruction::Instruction;
use super::instruction::{ADDHLTarget, ArithmeticTarget, IncDecTarget};
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

        Instruction::SBC(target) => match target {
            ArithmeticTarget::A => cpu.registers.a = sub(cpu, cpu.registers.a, true),
            ArithmeticTarget::B => cpu.registers.a = sub(cpu, cpu.registers.b, true),
            ArithmeticTarget::C => cpu.registers.a = sub(cpu, cpu.registers.c, true),
            ArithmeticTarget::D => cpu.registers.a = sub(cpu, cpu.registers.d, true),
            ArithmeticTarget::E => cpu.registers.a = sub(cpu, cpu.registers.e, true),
            ArithmeticTarget::H => cpu.registers.a = sub(cpu, cpu.registers.h, true),
            ArithmeticTarget::L => cpu.registers.a = sub(cpu, cpu.registers.l, true),
            ArithmeticTarget::HLI => todo!(),
            ArithmeticTarget::D8 => todo!(),
        },

        Instruction::AND(target) => match target {
            ArithmeticTarget::A => cpu.registers.a &= cpu.registers.a,
            ArithmeticTarget::B => cpu.registers.a &= cpu.registers.b,
            ArithmeticTarget::C => cpu.registers.a &= cpu.registers.c,
            ArithmeticTarget::D => cpu.registers.a &= cpu.registers.d,
            ArithmeticTarget::E => cpu.registers.a &= cpu.registers.e,
            ArithmeticTarget::H => cpu.registers.a &= cpu.registers.h,
            ArithmeticTarget::L => cpu.registers.a &= cpu.registers.l,
            ArithmeticTarget::HLI => todo!(),
            ArithmeticTarget::D8 => todo!(),
        },

        Instruction::XOR(target) => match target {
            ArithmeticTarget::A => cpu.registers.a ^= cpu.registers.a,
            ArithmeticTarget::B => cpu.registers.a ^= cpu.registers.b,
            ArithmeticTarget::C => cpu.registers.a ^= cpu.registers.c,
            ArithmeticTarget::D => cpu.registers.a ^= cpu.registers.d,
            ArithmeticTarget::E => cpu.registers.a ^= cpu.registers.e,
            ArithmeticTarget::H => cpu.registers.a ^= cpu.registers.h,
            ArithmeticTarget::L => cpu.registers.a ^= cpu.registers.l,
            ArithmeticTarget::HLI => todo!(),
            ArithmeticTarget::D8 => todo!(),
        },

        Instruction::OR(target) => match target {
            ArithmeticTarget::A => cpu.registers.a |= cpu.registers.a,
            ArithmeticTarget::B => cpu.registers.a |= cpu.registers.b,
            ArithmeticTarget::C => cpu.registers.a |= cpu.registers.c,
            ArithmeticTarget::D => cpu.registers.a |= cpu.registers.d,
            ArithmeticTarget::E => cpu.registers.a |= cpu.registers.e,
            ArithmeticTarget::H => cpu.registers.a |= cpu.registers.h,
            ArithmeticTarget::L => cpu.registers.a |= cpu.registers.l,
            ArithmeticTarget::HLI => todo!(),
            ArithmeticTarget::D8 => todo!(),
        },

        // like SUB but the result is discarded
        Instruction::CP(target) => match target {
            ArithmeticTarget::A => _ = sub(cpu, cpu.registers.a, false),
            ArithmeticTarget::B => _ = sub(cpu, cpu.registers.b, false),
            ArithmeticTarget::C => _ = sub(cpu, cpu.registers.c, false),
            ArithmeticTarget::D => _ = sub(cpu, cpu.registers.d, false),
            ArithmeticTarget::E => _ = sub(cpu, cpu.registers.e, false),
            ArithmeticTarget::H => _ = sub(cpu, cpu.registers.h, false),
            ArithmeticTarget::L => _ = sub(cpu, cpu.registers.l, false),
            ArithmeticTarget::HLI => todo!(),
            ArithmeticTarget::D8 => todo!(),
        },

        Instruction::INC(target) => match target {
            IncDecTarget::A => cpu.registers.a = inc_8bit(cpu, cpu.registers.a),
            IncDecTarget::B => cpu.registers.b = inc_8bit(cpu, cpu.registers.b),
            IncDecTarget::C => cpu.registers.c = inc_8bit(cpu, cpu.registers.c),
            IncDecTarget::D => cpu.registers.d = inc_8bit(cpu, cpu.registers.d),
            IncDecTarget::E => cpu.registers.e = inc_8bit(cpu, cpu.registers.e),
            IncDecTarget::H => cpu.registers.h = inc_8bit(cpu, cpu.registers.h),
            IncDecTarget::L => cpu.registers.l = inc_8bit(cpu, cpu.registers.l),
            IncDecTarget::HLI => todo!(),
            IncDecTarget::BC => {
                let value = cpu.registers.get_bc().wrapping_add(1);
                cpu.registers.set_bc(value);
            }
            IncDecTarget::DE => {
                let value = cpu.registers.get_de().wrapping_add(1);
                cpu.registers.set_de(value);
            }
            IncDecTarget::HL => {
                let value = cpu.registers.get_hl().wrapping_add(1);
                cpu.registers.set_hl(value);
            }
            IncDecTarget::SP => todo!(),
        },

        Instruction::DEC(target) => match target {
            IncDecTarget::A => cpu.registers.a = dec_8bit(cpu, cpu.registers.a),
            IncDecTarget::B => cpu.registers.b = dec_8bit(cpu, cpu.registers.b),
            IncDecTarget::C => cpu.registers.c = dec_8bit(cpu, cpu.registers.c),
            IncDecTarget::D => cpu.registers.d = dec_8bit(cpu, cpu.registers.d),
            IncDecTarget::E => cpu.registers.e = dec_8bit(cpu, cpu.registers.e),
            IncDecTarget::H => cpu.registers.h = dec_8bit(cpu, cpu.registers.h),
            IncDecTarget::L => cpu.registers.l = dec_8bit(cpu, cpu.registers.l),
            IncDecTarget::HLI => todo!(),
            IncDecTarget::BC => {
                let value = cpu.registers.get_bc().wrapping_sub(1);
                cpu.registers.set_bc(value);
            }
            IncDecTarget::DE => {
                let value = cpu.registers.get_de().wrapping_sub(1);
                cpu.registers.set_de(value);
            }
            IncDecTarget::HL => {
                let value = cpu.registers.get_hl().wrapping_sub(1);
                cpu.registers.set_hl(value);
            }
            IncDecTarget::SP => todo!(),
        },

        Instruction::DAA => cpu.registers.a = decimal_adjust(cpu, cpu.registers.a),

        Instruction::CPL => {
            cpu.registers.a = !cpu.registers.a;
            cpu.registers.f.subtract = true;
            cpu.registers.f.half_carry = true;
        }

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

fn inc_8bit(cpu: &mut CPU, value: u8) -> u8 {
    let new_value = value.wrapping_add(1);
    cpu.registers.f.zero = new_value == 0;
    cpu.registers.f.subtract = false;
    // Half Carry is set if the lower nibble of the value is equal to 0xF.
    // If the nibble is equal to 0xF (0b1111) that means incrementing the value
    // by 1 would cause a carry from the lower nibble to the upper nibble.
    cpu.registers.f.half_carry = value & 0xF == 0xF;
    new_value
}

fn dec_8bit(cpu: &mut CPU, value: u8) -> u8 {
    let new_value = value.wrapping_sub(1);
    cpu.registers.f.zero = new_value == 0;
    cpu.registers.f.subtract = true;
    // Half Carry is set if the lower nibble of the value is equal to 0x0.
    // If the nibble is equal to 0x0 (0b0000) that means decrementing the value
    // by 1 would cause a carry from the upper nibble to the lower nibble.
    cpu.registers.f.half_carry = value & 0xF == 0x0;
    new_value
}

fn decimal_adjust(cpu: &mut CPU, value: u8) -> u8 {
    let flags = cpu.registers.f;
    let mut carry = false;

    let result = if !flags.subtract {
        let mut result = value;
        if flags.carry || value > 0x99 {
            carry = true;
            result = result.wrapping_add(0x60);
        }
        if flags.half_carry || value & 0x0F > 0x09 {
            result = result.wrapping_add(0x06);
        }
        result
    } else if flags.carry {
        carry = true;
        let add = if flags.half_carry { 0x9A } else { 0xA0 };
        value.wrapping_add(add)
    } else if flags.half_carry {
        value.wrapping_add(0xFA)
    } else {
        value
    };

    cpu.registers.f.zero = result == 0;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = carry;

    result
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
    fn cp_set_carry() {
        let mut cpu = CPU::new();
        cpu.registers.c = 0x04;
        cpu.registers.a = 0x02;
        execute(&mut cpu, Instruction::CP(ArithmeticTarget::C));
        assert!(cpu.registers.f.carry);
        assert_eq!(cpu.registers.a, 0x02);
    }

    #[test]
    fn and_a_b() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0b11100110;
        cpu.registers.b = 0b11101001;
        execute(&mut cpu, Instruction::AND(ArithmeticTarget::B));
        assert_eq!(cpu.registers.a, 0b11100000);
    }

    #[test]
    fn xor_a_b() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0b11100110;
        cpu.registers.b = 0b11101001;
        execute(&mut cpu, Instruction::XOR(ArithmeticTarget::B));
        assert_eq!(cpu.registers.a, 0b00001111);
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
