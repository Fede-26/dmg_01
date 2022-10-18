pub mod alu;
pub mod flags_register;
pub mod instruction;
pub mod registers;

use instruction::{ArithmeticTarget, Instruction};

use self::registers::Registers;

pub struct CPU {
    registers: Registers,
    pc: u16, //program counter
}

impl CPU {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            pc: 0x0,
        }
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADD(target) => alu::execute(self, instruction),
            _ => { /*add support for more instructions*/ }
        }
    }

    fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        // Half Carry is set if adding the lower nibbles of the value and register A
        // together result in a value bigger than 0xF. If the result is larger than 0xF
        // than the addition caused a carry from the lower nibble to the upper nibble.
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        new_value
    }
}
