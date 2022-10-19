#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Instruction {
    ADD(ArithmeticTarget),
    ADDHL(ADDHLTarget),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ArithmeticTarget {
    A, B, C, D, E, H, L,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ADDHLTarget {
    BC, DE, HL,
}