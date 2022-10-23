#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Instruction {
    // http://bgb.bircd.org/pandocs.htm#cpuinstructionset

    // Arithmetic/logical 8-bit
    ADD(ArithmeticTarget),
    ADC(ArithmeticTarget),
    SUB(ArithmeticTarget),
    SBC(ArithmeticTarget),
    AND(ArithmeticTarget),
    XOR(ArithmeticTarget),
    OR(ArithmeticTarget),
    CP(ArithmeticTarget),
    INC(IncDecTarget),
    DEC(IncDecTarget),
    DAA,
    CPL,

    // Arithmetic/logical 16-bit
    ADDHL(ADDHLTarget),
    //INC
    //DEC
    ADDSP,

    //Load
    LD(LoadType),

    //JUMPS
    JP(JumpTest),
    JR(JumpTest),
    JPI,

    //Stack manipulation
    PUSH(StackTarget),
    POP(StackTarget),
    CALL(JumpTest),
    RET(JumpTest),
    RETI,
    RST(RSTLocation),

    //Carry flag (in alu)
    CCF,
    SCF,

    //Rotate instructions
    RRA,
    RLA,
    RRCA,
    RLCA,

    // Prefix Instructions
    BIT(PrefixTarget, BitPosition),
    RES(PrefixTarget, BitPosition),
    SET(PrefixTarget, BitPosition),
    SRL(PrefixTarget),
    RR(PrefixTarget),
    RL(PrefixTarget),
    RRC(PrefixTarget),
    RLC(PrefixTarget),
    SRA(PrefixTarget),
    SLA(PrefixTarget),
    SWAP(PrefixTarget),

    // System management
    HALT,
    NOP,
    DI,
    EI,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HLI,
    D8,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum IncDecTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HLI,
    BC,
    DE,
    HL,
    SP,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ADDHLTarget {
    BC,
    DE,
    HL,
    SP,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum JumpTest {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LoadByteTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HLI,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LoadByteSource {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    D8,
    HLI,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LoadWordTarget {
    BC,
    DE,
    HL,
    SP,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Indirect {
    BCIndirect,
    DEIndirect,
    HLIndirectMinus,
    HLIndirectPlus,
    WordIndirect,
    LastByteIndirect,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LoadType {
    Byte(LoadByteTarget, LoadByteSource),
    Word(LoadWordTarget),
    AFromIndirect(Indirect),
    IndirectFromA(Indirect),
    AFromByteAddress,
    ByteAddressFromA,
    SPFromHL,
    HLFromSPN,
    IndirectFromSP,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum StackTarget {
    AF,
    BC,
    DE,
    HL,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PrefixTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HLI,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BitPosition {
    B0,
    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
}

impl std::convert::From<BitPosition> for u8 {
    fn from(position: BitPosition) -> u8 {
        match position {
            BitPosition::B0 => 0,
            BitPosition::B1 => 1,
            BitPosition::B2 => 2,
            BitPosition::B3 => 3,
            BitPosition::B4 => 4,
            BitPosition::B5 => 5,
            BitPosition::B6 => 6,
            BitPosition::B7 => 7,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RSTLocation {
    X00,
    X08,
    X10,
    X18,
    X20,
    X28,
    X30,
    X38,
}
impl RSTLocation {
    pub fn to_hex(&self) -> u16 {
        match self {
            RSTLocation::X00 => 0x00,
            RSTLocation::X08 => 0x08,
            RSTLocation::X10 => 0x10,
            RSTLocation::X18 => 0x18,
            RSTLocation::X20 => 0x20,
            RSTLocation::X28 => 0x28,
            RSTLocation::X30 => 0x30,
            RSTLocation::X38 => 0x38,
        }
    }
}

/* --- OPCODES --- */

impl Instruction {
    pub fn from_byte(byte: u8, prefixed: bool) -> Option<Instruction> {
        if prefixed {
            Instruction::from_byte_prefixed(byte)
        } else {
            Instruction::from_byte_not_prefixed(byte)
        }
    }

    fn from_byte_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x00 => None, //TODO: fill all instructions
            _ => None,
        }
    }

    fn from_byte_not_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x3c => Some(Instruction::INC(IncDecTarget::A)),
            0x04 => Some(Instruction::INC(IncDecTarget::B)),
            0x14 => Some(Instruction::INC(IncDecTarget::D)),
            0x24 => Some(Instruction::INC(IncDecTarget::H)),
            0x0c => Some(Instruction::INC(IncDecTarget::C)),
            0x1c => Some(Instruction::INC(IncDecTarget::E)),
            0x2c => Some(Instruction::INC(IncDecTarget::L)),
            0x34 => Some(Instruction::INC(IncDecTarget::HLI)),
            0x03 => Some(Instruction::INC(IncDecTarget::BC)),
            0x13 => Some(Instruction::INC(IncDecTarget::DE)),
            0x23 => Some(Instruction::INC(IncDecTarget::HL)),
            0x33 => Some(Instruction::INC(IncDecTarget::SP)),

            0x3d => Some(Instruction::DEC(IncDecTarget::A)),
            0x05 => Some(Instruction::DEC(IncDecTarget::B)),
            0x0d => Some(Instruction::DEC(IncDecTarget::C)),
            0x15 => Some(Instruction::DEC(IncDecTarget::D)),
            0x1d => Some(Instruction::DEC(IncDecTarget::E)),
            0x25 => Some(Instruction::DEC(IncDecTarget::H)),
            0x2d => Some(Instruction::DEC(IncDecTarget::L)),
            0x35 => Some(Instruction::DEC(IncDecTarget::HLI)),
            0x0b => Some(Instruction::DEC(IncDecTarget::BC)),
            0x1b => Some(Instruction::DEC(IncDecTarget::DE)),
            0x2b => Some(Instruction::DEC(IncDecTarget::HL)),
            0x3b => Some(Instruction::DEC(IncDecTarget::SP)),

            0x87 => Some(Instruction::ADD(ArithmeticTarget::A)),
            0x80 => Some(Instruction::ADD(ArithmeticTarget::B)),
            0x81 => Some(Instruction::ADD(ArithmeticTarget::C)),
            0x82 => Some(Instruction::ADD(ArithmeticTarget::D)),
            0x83 => Some(Instruction::ADD(ArithmeticTarget::E)),
            0x84 => Some(Instruction::ADD(ArithmeticTarget::H)),
            0x85 => Some(Instruction::ADD(ArithmeticTarget::L)),
            0x86 => Some(Instruction::ADD(ArithmeticTarget::HLI)),
            0xc6 => Some(Instruction::ADD(ArithmeticTarget::D8)),

            0x09 => Some(Instruction::ADDHL(ADDHLTarget::BC)),
            0x19 => Some(Instruction::ADDHL(ADDHLTarget::DE)),
            0x29 => Some(Instruction::ADDHL(ADDHLTarget::HL)),
            0x39 => Some(Instruction::ADDHL(ADDHLTarget::SP)),

            0x8f => Some(Instruction::ADC(ArithmeticTarget::A)),
            0x88 => Some(Instruction::ADC(ArithmeticTarget::B)),
            0x89 => Some(Instruction::ADC(ArithmeticTarget::C)),
            0x8a => Some(Instruction::ADC(ArithmeticTarget::D)),
            0x8b => Some(Instruction::ADC(ArithmeticTarget::E)),
            0x8c => Some(Instruction::ADC(ArithmeticTarget::H)),
            0x8d => Some(Instruction::ADC(ArithmeticTarget::L)),
            0x8e => Some(Instruction::ADC(ArithmeticTarget::HLI)),
            0xce => Some(Instruction::ADC(ArithmeticTarget::D8)),

            0x97 => Some(Instruction::SUB(ArithmeticTarget::A)),
            0x90 => Some(Instruction::SUB(ArithmeticTarget::B)),
            0x91 => Some(Instruction::SUB(ArithmeticTarget::C)),
            0x92 => Some(Instruction::SUB(ArithmeticTarget::D)),
            0x93 => Some(Instruction::SUB(ArithmeticTarget::E)),
            0x94 => Some(Instruction::SUB(ArithmeticTarget::H)),
            0x95 => Some(Instruction::SUB(ArithmeticTarget::L)),
            0x96 => Some(Instruction::SUB(ArithmeticTarget::HLI)),
            0xd6 => Some(Instruction::SUB(ArithmeticTarget::D8)),

            0x9f => Some(Instruction::SBC(ArithmeticTarget::A)),
            0x98 => Some(Instruction::SBC(ArithmeticTarget::B)),
            0x99 => Some(Instruction::SBC(ArithmeticTarget::C)),
            0x9a => Some(Instruction::SBC(ArithmeticTarget::D)),
            0x9b => Some(Instruction::SBC(ArithmeticTarget::E)),
            0x9c => Some(Instruction::SBC(ArithmeticTarget::H)),
            0x9d => Some(Instruction::SBC(ArithmeticTarget::L)),
            0x9e => Some(Instruction::SBC(ArithmeticTarget::HLI)),
            0xde => Some(Instruction::SBC(ArithmeticTarget::D8)),

            0xa7 => Some(Instruction::AND(ArithmeticTarget::A)),
            0xa0 => Some(Instruction::AND(ArithmeticTarget::B)),
            0xa1 => Some(Instruction::AND(ArithmeticTarget::C)),
            0xa2 => Some(Instruction::AND(ArithmeticTarget::D)),
            0xa3 => Some(Instruction::AND(ArithmeticTarget::E)),
            0xa4 => Some(Instruction::AND(ArithmeticTarget::H)),
            0xa5 => Some(Instruction::AND(ArithmeticTarget::L)),
            0xa6 => Some(Instruction::AND(ArithmeticTarget::HLI)),
            0xe6 => Some(Instruction::AND(ArithmeticTarget::D8)),

            0xb7 => Some(Instruction::OR(ArithmeticTarget::A)),
            0xb0 => Some(Instruction::OR(ArithmeticTarget::B)),
            0xb1 => Some(Instruction::OR(ArithmeticTarget::C)),
            0xb2 => Some(Instruction::OR(ArithmeticTarget::D)),
            0xb3 => Some(Instruction::OR(ArithmeticTarget::E)),
            0xb4 => Some(Instruction::OR(ArithmeticTarget::H)),
            0xb5 => Some(Instruction::OR(ArithmeticTarget::L)),
            0xb6 => Some(Instruction::OR(ArithmeticTarget::HLI)),
            0xf6 => Some(Instruction::OR(ArithmeticTarget::D8)),

            0xaf => Some(Instruction::XOR(ArithmeticTarget::A)),
            0xa8 => Some(Instruction::XOR(ArithmeticTarget::B)),
            0xa9 => Some(Instruction::XOR(ArithmeticTarget::C)),
            0xaa => Some(Instruction::XOR(ArithmeticTarget::D)),
            0xab => Some(Instruction::XOR(ArithmeticTarget::E)),
            0xac => Some(Instruction::XOR(ArithmeticTarget::H)),
            0xad => Some(Instruction::XOR(ArithmeticTarget::L)),
            0xae => Some(Instruction::XOR(ArithmeticTarget::HLI)),
            0xee => Some(Instruction::XOR(ArithmeticTarget::D8)),

            0xbf => Some(Instruction::CP(ArithmeticTarget::A)),
            0xb8 => Some(Instruction::CP(ArithmeticTarget::B)),
            0xb9 => Some(Instruction::CP(ArithmeticTarget::C)),
            0xba => Some(Instruction::CP(ArithmeticTarget::D)),
            0xbb => Some(Instruction::CP(ArithmeticTarget::E)),
            0xbc => Some(Instruction::CP(ArithmeticTarget::H)),
            0xbd => Some(Instruction::CP(ArithmeticTarget::L)),
            0xbe => Some(Instruction::CP(ArithmeticTarget::HLI)),
            0xfe => Some(Instruction::CP(ArithmeticTarget::D8)),

            0xe8 => Some(Instruction::ADDSP),
            //TODO: fill all instructions
            _ => None,
        }
    }
}
