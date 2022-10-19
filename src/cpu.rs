pub mod alu;
pub mod flags_register;
pub mod instruction;
pub mod registers;

use instruction::{ArithmeticTarget, Instruction};

use self::registers::Registers;

const STACK_SIZE: usize = 0xFF;

pub struct CPU {
    pub registers: Registers,
    pub pc: u16, //program counter
    pub sp: u16, //stack pointer
    pub stack: [u16; STACK_SIZE],
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
            Instruction::ADD(_) => alu::execute(self, instruction),
            Instruction::ADC(_) => alu::execute(self, instruction),
            Instruction::SUB(_) => alu::execute(self, instruction),
            Instruction::SBC(_) => alu::execute(self, instruction),
            Instruction::AND(_) => alu::execute(self, instruction),
            Instruction::XOR(_) => alu::execute(self, instruction),
            Instruction::OR(_) => alu::execute(self, instruction),
            Instruction::CP(_) => alu::execute(self, instruction),
            Instruction::INC(_) => alu::execute(self, instruction),
            Instruction::DEC(_) => alu::execute(self, instruction),
            Instruction::DAA => alu::execute(self, instruction),
            Instruction::CPL => alu::execute(self, instruction),

            Instruction::ADDHL(_) => alu::execute(self, instruction),
            Instruction::ADDSP => alu::execute(self, instruction),
            Instruction::LD => alu::execute(self, instruction),
            // _ => { /*add support for more instructions*/ }
        }
    }

}
