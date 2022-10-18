#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Instruction {
    ADD(ArithmeticTarget),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ArithmeticTarget {
    A, B, C, D, E, H, L,
}