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
    INC(ArithmeticTarget),
    DEC(ArithmeticTarget),
    DAA,
    CPL,
    
    // Arithmetic/logical 16-bit
    ADDHL(ADDHLTarget),
    //INC
    //DEC
    ADDSP,
    LD,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ArithmeticTarget {
    A, B, C, D, E, H, L, HLI, D8,
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
    BC, DE, HL, SP,
}