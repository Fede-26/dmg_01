pub mod alu;
pub mod flags_register;
pub mod instruction;
pub mod load;
pub mod registers;

use crate::memory_bus::MemoryBus;

use instruction::Instruction;
use instruction::{JumpTest, StackTarget};

use self::registers::Registers;

pub struct CPU {
    pub registers: Registers,
    pub pc: u16, //program counter
    pub sp: u16, //stack pointer
    pub bus: MemoryBus,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            pc: 0x0,
            sp: 0x0, //FIXME: change begin of stack pointer
            bus: MemoryBus::new(),
        }
    }

    fn step(&mut self) {
        let mut instruction_byte = self.bus.read_byte(self.pc);
        let prefixed = instruction_byte == 0xCB;
        if prefixed {
            instruction_byte = self.bus.read_byte(self.pc + 1);
        }

        let (next_pc, cycles) =
            if let Some(instruction) = Instruction::from_byte(instruction_byte, prefixed) {
                self.execute(instruction)
            } else {
                let description = format!(
                    "0x{}{:x}",
                    if prefixed { "cb" } else { "" },
                    instruction_byte
                );
                panic!("Unkown instruction found for: {}", description)
            };

        self.pc = next_pc;
    }

    pub fn execute(&mut self, instruction: Instruction) -> (u16, u8) {
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

            Instruction::LD(_) => load::execute(self, instruction),

            Instruction::JP(test) => {
                let jump_condition = match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::Always => true,
                };
                self.jump(jump_condition)
            }
            Instruction::JR(test) => {
                let jump_condition = match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::Always => true,
                };
                self.jump_relative(jump_condition)
            }
            Instruction::JPI => (self.registers.get_hl(), 4),

            Instruction::PUSH(target) => {
                let value = match target {
                    StackTarget::AF => self.registers.get_af(),
                    StackTarget::BC => self.registers.get_bc(),
                    StackTarget::DE => self.registers.get_de(),
                    StackTarget::HL => self.registers.get_hl(),
                };
                self.push(value);
                (self.pc.wrapping_add(1), 16)
            }
            Instruction::POP(target) => {
                let result = self.pop();
                match target {
                    StackTarget::AF => self.registers.set_af(result),
                    StackTarget::BC => self.registers.set_bc(result),
                    StackTarget::DE => self.registers.set_de(result),
                    StackTarget::HL => self.registers.set_hl(result),
                };
                (self.pc.wrapping_add(1), 12)
            }
            // _ => { /*add support for more instructions*/ }
        }
    }

    fn jump(&self, jump_condition: bool) -> (u16, u8) {
        if jump_condition {
            (self.read_next_word(), 16)
        } else {
            // If we don't jump we need to still move the program
            // counter forward by 3 since the jump instruction is
            // 3 bytes wide (1 byte for tag and 2 bytes for jump address)
            (self.pc.wrapping_add(3), 12)
        }
    }

    fn jump_relative(&self, should_jump: bool) -> (u16, u8) {
        let next_step = self.pc.wrapping_add(2);
        if should_jump {
            let offset = self.read_next_byte() as i8;
            let pc = if offset >= 0 {
                next_step.wrapping_add(offset as u16)
            } else {
                next_step.wrapping_sub(offset.abs() as u16)
            };
            (pc, 16)
        } else {
            (next_step, 12)
        }
    }

    fn push(&mut self, value: u16) {
        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, ((value & 0xFF00) >> 8) as u8);

        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, (value & 0xFF) as u8);
    }

    fn pop(&mut self) -> u16 {
        let lsb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        let msb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        (msb << 8) | lsb
    }

    pub fn read_next_byte(&self) -> u8 {
        self.bus.read_byte(self.pc + 1) as u8
    }

    pub fn read_next_word(&self) -> u16 {
        // Gameboy is little endian so read pc + 2 as most significant bit
        // and pc + 1 as least significant bi
        let least_significant_byte = self.bus.read_byte(self.pc + 1) as u16;
        let most_significant_byte = self.bus.read_byte(self.pc + 2) as u16;
        (most_significant_byte << 8) | least_significant_byte
    }
}
