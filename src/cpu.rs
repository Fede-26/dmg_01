pub mod alu;
pub mod flags_register;
pub mod instruction;
pub mod registers;

use instruction::{ArithmeticTarget, Instruction};

use self::registers::Registers;

const STACK_SIZE: usize = 0xFF;

pub struct CPU {
    registers: Registers,
    pc: u16, //program counter
    sp: u16, //stack pointer
    stack: [u16; STACK_SIZE],
}

impl CPU {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            pc: 0x0,
            sp: 0x0,    //FIXME: change begin of stack pointer
            stack: [0x0; STACK_SIZE],
        }
    }

    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADD(target) => alu::execute(self, instruction),
            Instruction::ADDHL(target) => alu::execute(self, instruction),
            _ => { /*add support for more instructions*/ }
        }
    }

}
