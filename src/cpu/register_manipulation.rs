/* TODO:
 * - Add HLI and D8 targets implementation (waiting for read bytes function in cpu)
 * - Add ADDSP instruction implementation
 * - Add more tests
 * - Add comments
 */

use super::instruction::Instruction;
use super::instruction::{BitPosition, PrefixTarget};
use crate::cpu::CPU;

pub fn execute(cpu: &mut CPU, instruction: Instruction) -> (u16, u8) {
    match instruction {
        Instruction::RRA => {
            cpu.registers.a = rotate_right_through_carry(cpu, cpu.registers.a, false);
            (cpu.pc.wrapping_add(1), 4)
        }
        Instruction::RLA => {
            cpu.registers.a = rotate_left_through_carry(cpu, cpu.registers.a, false);
            (cpu.pc.wrapping_add(1), 4)
        }
        Instruction::RRCA => {
            cpu.registers.a = rotate_right(cpu, cpu.registers.a, false);
            (cpu.pc.wrapping_add(1), 4)
        }
        Instruction::RLCA => {
            cpu.registers.a = rotate_left(cpu, cpu.registers.a, false);
            (cpu.pc.wrapping_add(1), 4)
        }

        Instruction::BIT(prefix, bit_position) => {
            let register = match prefix {
                PrefixTarget::A => cpu.registers.a,
                PrefixTarget::B => cpu.registers.b,
                PrefixTarget::C => cpu.registers.c,
                PrefixTarget::D => cpu.registers.d,
                PrefixTarget::E => cpu.registers.e,
                PrefixTarget::H => cpu.registers.h,
                PrefixTarget::L => cpu.registers.l,
                PrefixTarget::HLI => {
                    let address = cpu.registers.get_hl();
                    cpu.bus.read_byte(address)
                }
            };
            bit_test(cpu, register, bit_position);

            match prefix {
                PrefixTarget::HLI => (cpu.pc.wrapping_add(2), 16),
                _ => (cpu.pc.wrapping_add(2), 8),
            }
        }

        Instruction::RES(prefix, bit_position) => {
            match prefix {
                PrefixTarget::A => cpu.registers.a = reset_bit(cpu.registers.a, bit_position),
                PrefixTarget::B => cpu.registers.b = reset_bit(cpu.registers.b, bit_position),
                PrefixTarget::C => cpu.registers.c = reset_bit(cpu.registers.c, bit_position),
                PrefixTarget::D => cpu.registers.d = reset_bit(cpu.registers.d, bit_position),
                PrefixTarget::E => cpu.registers.e = reset_bit(cpu.registers.e, bit_position),
                PrefixTarget::H => cpu.registers.h = reset_bit(cpu.registers.h, bit_position),
                PrefixTarget::L => cpu.registers.l = reset_bit(cpu.registers.l, bit_position),
                PrefixTarget::HLI => {
                    let address = cpu.registers.get_hl();
                    let value = cpu.bus.read_byte(address);
                    let result = reset_bit(value, bit_position);
                    cpu.bus.write_byte(address, result);
                }
            };
            match prefix {
                PrefixTarget::HLI => (cpu.pc.wrapping_add(2), 16),
                _ => (cpu.pc.wrapping_add(2), 8),
            }
        }

        Instruction::SET(prefix, bit_position) => {
            match prefix {
                PrefixTarget::A => cpu.registers.a = set_bit(cpu.registers.a, bit_position),
                PrefixTarget::B => cpu.registers.b = set_bit(cpu.registers.b, bit_position),
                PrefixTarget::C => cpu.registers.c = set_bit(cpu.registers.c, bit_position),
                PrefixTarget::D => cpu.registers.d = set_bit(cpu.registers.d, bit_position),
                PrefixTarget::E => cpu.registers.e = set_bit(cpu.registers.e, bit_position),
                PrefixTarget::H => cpu.registers.h = set_bit(cpu.registers.h, bit_position),
                PrefixTarget::L => cpu.registers.l = set_bit(cpu.registers.l, bit_position),
                PrefixTarget::HLI => {
                    let address = cpu.registers.get_hl();
                    let value = cpu.bus.read_byte(address);
                    let result = set_bit(value, bit_position);
                    cpu.bus.write_byte(address, result);
                }
            };
            match prefix {
                PrefixTarget::HLI => (cpu.pc.wrapping_add(2), 16),
                _ => (cpu.pc.wrapping_add(2), 8),
            }
        }

        Instruction::SRL(prefix) => {
            match prefix {
                PrefixTarget::A => cpu.registers.a = shift_right_logical(cpu, cpu.registers.a),
                PrefixTarget::B => cpu.registers.b = shift_right_logical(cpu, cpu.registers.b),
                PrefixTarget::C => cpu.registers.c = shift_right_logical(cpu, cpu.registers.c),
                PrefixTarget::D => cpu.registers.d = shift_right_logical(cpu, cpu.registers.d),
                PrefixTarget::E => cpu.registers.e = shift_right_logical(cpu, cpu.registers.e),
                PrefixTarget::H => cpu.registers.h = shift_right_logical(cpu, cpu.registers.h),
                PrefixTarget::L => cpu.registers.l = shift_right_logical(cpu, cpu.registers.l),
                PrefixTarget::HLI => {
                    let address = cpu.registers.get_hl();
                    let value = cpu.bus.read_byte(address);
                    let result = shift_right_logical(cpu, value);
                    cpu.bus.write_byte(address, result);
                }
            };
            match prefix {
                PrefixTarget::HLI => (cpu.pc.wrapping_add(2), 16),
                _ => (cpu.pc.wrapping_add(2), 8),
            }
        }

        Instruction::SLA(prefix) => {
            match prefix {
                PrefixTarget::A => cpu.registers.a = shift_left_arithmetic(cpu, cpu.registers.a),
                PrefixTarget::B => cpu.registers.b = shift_left_arithmetic(cpu, cpu.registers.b),
                PrefixTarget::C => cpu.registers.c = shift_left_arithmetic(cpu, cpu.registers.c),
                PrefixTarget::D => cpu.registers.d = shift_left_arithmetic(cpu, cpu.registers.d),
                PrefixTarget::E => cpu.registers.e = shift_left_arithmetic(cpu, cpu.registers.e),
                PrefixTarget::H => cpu.registers.h = shift_left_arithmetic(cpu, cpu.registers.h),
                PrefixTarget::L => cpu.registers.l = shift_left_arithmetic(cpu, cpu.registers.l),
                PrefixTarget::HLI => {
                    let address = cpu.registers.get_hl();
                    let value = cpu.bus.read_byte(address);
                    let result = shift_left_arithmetic(cpu, value);
                    cpu.bus.write_byte(address, result);
                }
            };
            match prefix {
                PrefixTarget::HLI => (cpu.pc.wrapping_add(2), 16),
                _ => (cpu.pc.wrapping_add(2), 8),
            }
        }

        Instruction::SRA(prefix) => {
            match prefix {
                PrefixTarget::A => cpu.registers.a = shift_right_arithmetic(cpu, cpu.registers.a),
                PrefixTarget::B => cpu.registers.b = shift_right_arithmetic(cpu, cpu.registers.b),
                PrefixTarget::C => cpu.registers.c = shift_right_arithmetic(cpu, cpu.registers.c),
                PrefixTarget::D => cpu.registers.d = shift_right_arithmetic(cpu, cpu.registers.d),
                PrefixTarget::E => cpu.registers.e = shift_right_arithmetic(cpu, cpu.registers.e),
                PrefixTarget::H => cpu.registers.h = shift_right_arithmetic(cpu, cpu.registers.h),
                PrefixTarget::L => cpu.registers.l = shift_right_arithmetic(cpu, cpu.registers.l),
                PrefixTarget::HLI => {
                    let address = cpu.registers.get_hl();
                    let value = cpu.bus.read_byte(address);
                    let result = shift_right_arithmetic(cpu, value);
                    cpu.bus.write_byte(address, result);
                }
            };
            match prefix {
                PrefixTarget::HLI => (cpu.pc.wrapping_add(2), 16),
                _ => (cpu.pc.wrapping_add(2), 8),
            }
        }

        Instruction::RR(prefix) => {
            match prefix {
                PrefixTarget::A => {
                    cpu.registers.a = rotate_right_through_carry(cpu, cpu.registers.a, true)
                }

                PrefixTarget::B => {
                    cpu.registers.b = rotate_right_through_carry(cpu, cpu.registers.b, true)
                }

                PrefixTarget::C => {
                    cpu.registers.c = rotate_right_through_carry(cpu, cpu.registers.c, true)
                }

                PrefixTarget::D => {
                    cpu.registers.d = rotate_right_through_carry(cpu, cpu.registers.d, true)
                }

                PrefixTarget::E => {
                    cpu.registers.e = rotate_right_through_carry(cpu, cpu.registers.e, true)
                }

                PrefixTarget::H => {
                    cpu.registers.h = rotate_right_through_carry(cpu, cpu.registers.h, true)
                }

                PrefixTarget::L => {
                    cpu.registers.l = rotate_right_through_carry(cpu, cpu.registers.l, true)
                }

                PrefixTarget::HLI => {
                    let address = cpu.registers.get_hl();
                    let value = cpu.bus.read_byte(address);
                    let result = rotate_right_through_carry(cpu, value, true);
                    cpu.bus.write_byte(address, result);
                }
            };
            match prefix {
                PrefixTarget::HLI => (cpu.pc.wrapping_add(2), 16),
                _ => (cpu.pc.wrapping_add(2), 8),
            }
        }

        Instruction::RL(prefix) => {
            match prefix {
                PrefixTarget::A => {
                    cpu.registers.a = rotate_left_through_carry(cpu, cpu.registers.a, true)
                }
                PrefixTarget::B => {
                    cpu.registers.b = rotate_left_through_carry(cpu, cpu.registers.b, true)
                }
                PrefixTarget::C => {
                    cpu.registers.c = rotate_left_through_carry(cpu, cpu.registers.c, true)
                }
                PrefixTarget::D => {
                    cpu.registers.d = rotate_left_through_carry(cpu, cpu.registers.d, true)
                }
                PrefixTarget::E => {
                    cpu.registers.e = rotate_left_through_carry(cpu, cpu.registers.e, true)
                }
                PrefixTarget::H => {
                    cpu.registers.h = rotate_left_through_carry(cpu, cpu.registers.h, true)
                }
                PrefixTarget::L => {
                    cpu.registers.l = rotate_left_through_carry(cpu, cpu.registers.l, true)
                }
                PrefixTarget::HLI => {
                    let address = cpu.registers.get_hl();
                    let value = cpu.bus.read_byte(address);
                    let result = rotate_left_through_carry(cpu, value, true);
                    cpu.bus.write_byte(address, result);
                }
            };
            match prefix {
                PrefixTarget::HLI => (cpu.pc.wrapping_add(2), 16),
                _ => (cpu.pc.wrapping_add(2), 8),
            }
        }

        Instruction::RRC(prefix) => {
            match prefix {
                PrefixTarget::A => cpu.registers.a = rotate_right(cpu, cpu.registers.a, true),
                PrefixTarget::B => cpu.registers.b = rotate_right(cpu, cpu.registers.b, true),
                PrefixTarget::C => cpu.registers.c = rotate_right(cpu, cpu.registers.c, true),
                PrefixTarget::D => cpu.registers.d = rotate_right(cpu, cpu.registers.d, true),
                PrefixTarget::E => cpu.registers.e = rotate_right(cpu, cpu.registers.e, true),
                PrefixTarget::H => cpu.registers.h = rotate_right(cpu, cpu.registers.h, true),
                PrefixTarget::L => cpu.registers.l = rotate_right(cpu, cpu.registers.l, true),
                PrefixTarget::HLI => {
                    let address = cpu.registers.get_hl();
                    let value = cpu.bus.read_byte(address);
                    let result = rotate_right(cpu, value, true);
                    cpu.bus.write_byte(address, result);
                }
            };
            match prefix {
                PrefixTarget::HLI => (cpu.pc.wrapping_add(2), 16),
                _ => (cpu.pc.wrapping_add(2), 8),
            }
        }

        Instruction::RLC(prefix) => {
            match prefix {
                PrefixTarget::A => cpu.registers.a = rotate_left(cpu, cpu.registers.a, true),
                PrefixTarget::B => cpu.registers.b = rotate_left(cpu, cpu.registers.b, true),
                PrefixTarget::C => cpu.registers.c = rotate_left(cpu, cpu.registers.c, true),
                PrefixTarget::D => cpu.registers.d = rotate_left(cpu, cpu.registers.d, true),
                PrefixTarget::E => cpu.registers.e = rotate_left(cpu, cpu.registers.e, true),
                PrefixTarget::H => cpu.registers.h = rotate_left(cpu, cpu.registers.h, true),
                PrefixTarget::L => cpu.registers.l = rotate_left(cpu, cpu.registers.l, true),
                PrefixTarget::HLI => {
                    let address = cpu.registers.get_hl();
                    let value = cpu.bus.read_byte(address);
                    let result = rotate_left(cpu, value, true);
                    cpu.bus.write_byte(address, result);
                }
            };

            match prefix {
                PrefixTarget::HLI => (cpu.pc.wrapping_add(2), 16),
                _ => (cpu.pc.wrapping_add(2), 8),
            }
        }

        Instruction::SWAP(prefix) => {
            match prefix {
                PrefixTarget::A => cpu.registers.a = swap_nibbles(cpu, cpu.registers.a),
                PrefixTarget::B => cpu.registers.b = swap_nibbles(cpu, cpu.registers.b),
                PrefixTarget::C => cpu.registers.c = swap_nibbles(cpu, cpu.registers.c),
                PrefixTarget::D => cpu.registers.d = swap_nibbles(cpu, cpu.registers.d),
                PrefixTarget::E => cpu.registers.e = swap_nibbles(cpu, cpu.registers.e),
                PrefixTarget::H => cpu.registers.h = swap_nibbles(cpu, cpu.registers.h),
                PrefixTarget::L => cpu.registers.l = swap_nibbles(cpu, cpu.registers.l),
                PrefixTarget::HLI => {
                    let address = cpu.registers.get_hl();
                    let value = cpu.bus.read_byte(address);
                    let result = swap_nibbles(cpu, value);
                    cpu.bus.write_byte(address, result);
                }
            };
            match prefix {
                PrefixTarget::HLI => (cpu.pc.wrapping_add(2), 16),
                _ => (cpu.pc.wrapping_add(2), 8),
            }
        }

        _ => {
            /*ignore other instructions*/
            (0, 0)
        }
    }
}

fn rotate_right_through_carry(cpu: &mut CPU, value: u8, set_zero: bool) -> u8 {
    let carry_bit = if cpu.registers.f.carry { 1 } else { 0 } << 7;
    let new_value = carry_bit | (value >> 1);
    cpu.registers.f.zero = set_zero && new_value == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = value & 0b1 == 0b1;
    new_value
}

fn rotate_left_through_carry(cpu: &mut CPU, value: u8, set_zero: bool) -> u8 {
    let carry_bit = if cpu.registers.f.carry { 1 } else { 0 };
    let new_value = (value << 1) | carry_bit;
    cpu.registers.f.zero = set_zero && new_value == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = (value & 0x80) == 0x80;
    new_value
}

fn rotate_left(cpu: &mut CPU, value: u8, set_zero: bool) -> u8 {
    let carry = (value & 0x80) >> 7;
    let new_value = value.rotate_left(1) | carry;
    cpu.registers.f.zero = set_zero && new_value == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = carry == 0x01;
    new_value
}

fn rotate_right(cpu: &mut CPU, value: u8, set_zero: bool) -> u8 {
    let new_value = value.rotate_right(1);
    cpu.registers.f.zero = set_zero && new_value == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = value & 0b1 == 0b1;
    new_value
}

fn bit_test(cpu: &mut CPU, value: u8, bit_position: BitPosition) {
    let bit_position: u8 = bit_position.into();
    let result = (value >> bit_position) & 0b1;
    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = true;
}

fn reset_bit(value: u8, bit_position: BitPosition) -> u8 {
    let bit_position: u8 = bit_position.into();
    value & !(1 << bit_position)
}

fn set_bit(value: u8, bit_position: BitPosition) -> u8 {
    let bit_position: u8 = bit_position.into();
    value | (1 << bit_position)
}

fn shift_right_logical(cpu: &mut CPU, value: u8) -> u8 {
    let new_value = value >> 1;
    cpu.registers.f.zero = new_value == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = value & 0b1 == 0b1;
    new_value
}

fn shift_right_arithmetic(cpu: &mut CPU, value: u8) -> u8 {
    let msb = value & 0x80;
    let new_value = msb | (value >> 1);
    cpu.registers.f.zero = new_value == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = value & 0b1 == 0b1;
    new_value
}

fn shift_left_arithmetic(cpu: &mut CPU, value: u8) -> u8 {
    let new_value = value << 1;
    cpu.registers.f.zero = new_value == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = value & 0x80 == 0x80;
    new_value
}

fn swap_nibbles(cpu: &mut CPU, value: u8) -> u8 {
    let new_value = ((value & 0xf) << 4) | ((value & 0xf0) >> 4);
    cpu.registers.f.zero = new_value == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = false;
    new_value
}
