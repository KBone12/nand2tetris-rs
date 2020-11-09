#[derive(PartialEq, Eq)]
pub enum Comp {
    Zero,
    One,
    MinusOne,
    D,
    A,
    M,
    NotD,
    NotA,
    NotM,
    MinusD,
    MinusA,
    MinusM,
    DPlusOne,
    APlusOne,
    MPlusOne,
    DMinusOne,
    AMinusOne,
    MMinusOne,
    DPlusA,
    APlusD,
    DPlusM,
    MPlusD,
    DMinusA,
    AMinusD,
    DMinusM,
    MMinusD,
    DAndA,
    AAndD,
    DAndM,
    MAndD,
    DOrA,
    AOrD,
    DOrM,
    MOrD,
}

#[derive(PartialEq, Eq)]
pub enum Dest {
    M,
    D,
    DM,
    A,
    AM,
    AD,
    ADM,
}

#[derive(PartialEq, Eq)]
pub enum Jump {
    JGT,
    JEQ,
    JGE,
    JLT,
    JNE,
    JLE,
    JMP,
}

pub enum Token {
    AInst,
    Immediate(u16),
    Symbol { name: String },
    LInst { name: String },
    CInst,
    Comp(Comp),
    Dest(Dest),
    Jump(Jump),
}
