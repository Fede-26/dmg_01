/* TODO:
 */

use super::instruction::{
    Indirect, Instruction, LoadByteSource, LoadByteTarget, LoadType, LoadWordTarget,
};
use crate::cpu::CPU;

pub fn execute(cpu: &mut CPU, instruction: Instruction) -> (u16, u8) {
    let load_type;
    if let Instruction::LD(lt) = instruction {
        load_type = lt;
    } else {
        panic!();
    }

    match load_type {
        // DESCRIPTION: load byte store in a particular register into another
        // particular register
        // WHEN: source is d8
        // PC:+2
        // Cycles: 8
        // WHEN: source is (HL)
        // PC:+1
        // Cycles: 8
        // ELSE:
        // PC:+1
        // Cycles: 4
        // Z:- N:- H:- C:-
        LoadType::Byte(target, source) => {
            let source_value = match source {
                LoadByteSource::A => cpu.registers.a,
                LoadByteSource::B => cpu.registers.b,
                LoadByteSource::C => cpu.registers.c,
                LoadByteSource::D => cpu.registers.d,
                LoadByteSource::E => cpu.registers.e,
                LoadByteSource::H => cpu.registers.h,
                LoadByteSource::L => cpu.registers.l,
                LoadByteSource::D8 => cpu.read_next_byte(),
                LoadByteSource::HLI => cpu.bus.read_byte(cpu.registers.get_hl()),
            };
            match target {
                LoadByteTarget::A => cpu.registers.a = source_value,
                LoadByteTarget::B => cpu.registers.b = source_value,
                LoadByteTarget::C => cpu.registers.c = source_value,
                LoadByteTarget::D => cpu.registers.d = source_value,
                LoadByteTarget::E => cpu.registers.e = source_value,
                LoadByteTarget::H => cpu.registers.h = source_value,
                LoadByteTarget::L => cpu.registers.l = source_value,
                LoadByteTarget::HLI => cpu.bus.write_byte(cpu.registers.get_hl(), source_value),
            };
            match source {
                LoadByteSource::D8 => (cpu.pc.wrapping_add(2), 8),
                LoadByteSource::HLI => (cpu.pc.wrapping_add(1), 8),
                _ => (cpu.pc.wrapping_add(1), 4),
            }
        }
        // DESCRIPTION: load next word in memory into a particular register
        // PC:+3
        // Cycles: 12
        // Z:- N:- H:- C:-
        LoadType::Word(target) => {
            let word = cpu.read_next_word();
            match target {
                LoadWordTarget::BC => cpu.registers.set_bc(word),
                LoadWordTarget::DE => cpu.registers.set_de(word),
                LoadWordTarget::HL => cpu.registers.set_hl(word),
                LoadWordTarget::SP => cpu.sp = word,
            };
            (cpu.pc.wrapping_add(3), 12)
        }
        // DESCRIPTION: load a particular value stored at the source address into A
        // WHEN: source is word indirect
        // PC:+3
        // Cycles: 16
        // ELSE:
        // PC:+1
        // Cycles: 8
        // Z:- N:- H:- C:-
        LoadType::AFromIndirect(source) => {
            cpu.registers.a = match source {
                Indirect::BCIndirect => cpu.bus.read_byte(cpu.registers.get_bc()),
                Indirect::DEIndirect => cpu.bus.read_byte(cpu.registers.get_de()),
                Indirect::HLIndirectMinus => {
                    let hl = cpu.registers.get_hl();
                    cpu.registers.set_hl(hl.wrapping_sub(1));
                    cpu.bus.read_byte(hl)
                }
                Indirect::HLIndirectPlus => {
                    let hl = cpu.registers.get_hl();
                    cpu.registers.set_hl(hl.wrapping_add(1));
                    cpu.bus.read_byte(hl)
                }
                Indirect::WordIndirect => cpu.bus.read_byte(cpu.read_next_word()),
                Indirect::LastByteIndirect => cpu.bus.read_byte(0xFF00 + cpu.registers.c as u16),
            };

            match source {
                Indirect::WordIndirect => (cpu.pc.wrapping_add(3), 16),
                _ => (cpu.pc.wrapping_add(1), 8),
            }
        }
        // DESCRIPTION: load the A register into memory at the source address
        // WHEN: instruction.source is word indirect
        // PC:+3
        // Cycles: 16
        // ELSE:
        // PC:+1
        // Cycles: 8
        // Z:- N:- H:- C:-
        LoadType::IndirectFromA(target) => {
            let a = cpu.registers.a;
            match target {
                Indirect::BCIndirect => {
                    let bc = cpu.registers.get_bc();
                    cpu.bus.write_byte(bc, a)
                }
                Indirect::DEIndirect => {
                    let de = cpu.registers.get_de();
                    cpu.bus.write_byte(de, a)
                }
                Indirect::HLIndirectMinus => {
                    let hl = cpu.registers.get_hl();
                    cpu.registers.set_hl(hl.wrapping_sub(1));
                    cpu.bus.write_byte(hl, a);
                }
                Indirect::HLIndirectPlus => {
                    let hl = cpu.registers.get_hl();
                    cpu.registers.set_hl(hl.wrapping_add(1));
                    cpu.bus.write_byte(hl, a);
                }
                Indirect::WordIndirect => {
                    let word = cpu.read_next_word();
                    cpu.bus.write_byte(word, a);
                }
                Indirect::LastByteIndirect => {
                    let c = cpu.registers.c as u16;
                    cpu.bus.write_byte(0xFF00 + c, a);
                }
            };

            match target {
                Indirect::WordIndirect => (cpu.pc.wrapping_add(3), 16),
                _ => (cpu.pc.wrapping_add(1), 8),
            }
        }
        // DESCRIPTION: Load the value in A into memory location located at 0xFF plus
        // an offset stored as the next byte in memory
        // PC:+2
        // Cycles: 12
        // Z:- N:- H:- C:-
        LoadType::ByteAddressFromA => {
            let offset = cpu.read_next_byte() as u16;
            cpu.bus.write_byte(0xFF00 + offset, cpu.registers.a);
            (cpu.pc.wrapping_add(2), 12)
        }
        // DESCRIPTION: Load the value located at 0xFF plus an offset stored as the next byte in memory into A
        // PC:+2
        // Cycles: 12
        // Z:- N:- H:- C:-
        LoadType::AFromByteAddress => {
            let offset = cpu.read_next_byte() as u16;
            cpu.registers.a = cpu.bus.read_byte(0xFF00 + offset);
            (cpu.pc.wrapping_add(2), 12)
        }
        // DESCRIPTION: Load the value in HL into SP
        // PC:+1
        // Cycles: 8
        // Z:- N:- H:- C:-
        LoadType::SPFromHL => {
            cpu.sp = cpu.registers.get_hl();
            (cpu.pc.wrapping_add(1), 8)
        }
        // DESCRIPTION: Load memory address with the contents of SP
        // PC:+3
        // Cycles: 20
        // Z:- N:- H:- C:-
        LoadType::IndirectFromSP => {
            let address = cpu.read_next_word();
            let sp = cpu.sp;
            cpu.bus.write_byte(address, (sp & 0xFF) as u8);
            cpu.bus
                .write_byte(address.wrapping_add(1), ((sp & 0xFF00) >> 8) as u8);
            (cpu.pc.wrapping_add(3), 20)
        }
        // DESCRIPTION: load HL with SP plus some specified byte
        // PC:+2
        // Cycles: 12
        // Z:0 N:0 H:? C:?
        LoadType::HLFromSPN => {
            let value = cpu.read_next_byte() as i8 as i16 as u16;
            let result = cpu.sp.wrapping_add(value);
            cpu.registers.set_hl(result);
            cpu.registers.f.zero = false;
            cpu.registers.f.subtract = false;
            // Half and whole carry are computed at the nibble and byte level instead
            // of the byte and word level like you might expect for 16 bit values
            cpu.registers.f.half_carry = (cpu.sp & 0xF) + (value & 0xF) > 0xF;
            cpu.registers.f.carry = (cpu.sp & 0xFF) + (value & 0xFF) > 0xFF;
            (cpu.pc.wrapping_add(2), 12)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn load_test__FIXME() {
        assert!(true)
    }
}
