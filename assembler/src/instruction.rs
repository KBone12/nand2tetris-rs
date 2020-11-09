#[derive(Clone, Debug, PartialEq, Eq)]
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
    DPlusM,
    DMinusA,
    AMinusD,
    DMinusM,
    MMinusD,
    DAndA,
    DAndM,
    DOrA,
    DOrM,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Dest {
    M,
    D,
    DM,
    A,
    AM,
    AD,
    ADM,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Jump {
    JGT,
    JEQ,
    JGE,
    JLT,
    JNE,
    JLE,
    JMP,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    A {
        value: u16,
    },
    C {
        comp: Comp,
        dest: Option<Dest>,
        jump: Option<Jump>,
    },
}

impl Instruction {
    pub fn as_binary(&self) -> [bool; 16] {
        match self {
            Self::A { value } => {
                let mut binary = [false; 16];
                for i in 0..16 {
                    binary[15 - i] = (value >> i) & 1 == 1;
                }
                binary
            }
            Self::C { comp, dest, jump } => {
                let mut binary = [false; 16];
                binary[0] = true;
                binary[1] = true;
                binary[2] = true;
                match comp {
                    Comp::Zero => {
                        binary[3] = false;
                        binary[4] = true;
                        binary[5] = false;
                        binary[6] = true;
                        binary[7] = false;
                        binary[8] = true;
                        binary[9] = false;
                    }
                    Comp::One => {
                        binary[3] = false;
                        binary[4] = true;
                        binary[5] = true;
                        binary[6] = true;
                        binary[7] = true;
                        binary[8] = true;
                        binary[9] = true;
                    }
                    Comp::MinusOne => {
                        binary[3] = false;
                        binary[4] = true;
                        binary[5] = true;
                        binary[6] = true;
                        binary[7] = false;
                        binary[8] = true;
                        binary[9] = false;
                    }
                    Comp::D => {
                        binary[3] = false;
                        binary[4] = false;
                        binary[5] = false;
                        binary[6] = true;
                        binary[7] = true;
                        binary[8] = false;
                        binary[9] = false;
                    }
                    Comp::A | Comp::M => {
                        binary[3] = *comp == Comp::M;
                        binary[4] = true;
                        binary[5] = true;
                        binary[6] = false;
                        binary[7] = false;
                        binary[8] = false;
                        binary[9] = false;
                    }
                    Comp::NotD => {
                        binary[3] = false;
                        binary[4] = false;
                        binary[5] = false;
                        binary[6] = true;
                        binary[7] = true;
                        binary[8] = false;
                        binary[9] = true;
                    }
                    Comp::NotA | Comp::NotM => {
                        binary[3] = *comp == Comp::NotM;
                        binary[4] = true;
                        binary[5] = true;
                        binary[6] = false;
                        binary[7] = false;
                        binary[8] = false;
                        binary[9] = true;
                    }
                    Comp::MinusD => {
                        binary[3] = false;
                        binary[4] = false;
                        binary[5] = false;
                        binary[6] = true;
                        binary[7] = true;
                        binary[8] = true;
                        binary[9] = true;
                    }
                    Comp::MinusA | Comp::MinusM => {
                        binary[3] = *comp == Comp::MinusM;
                        binary[4] = true;
                        binary[5] = true;
                        binary[6] = false;
                        binary[7] = false;
                        binary[8] = true;
                        binary[9] = true;
                    }
                    Comp::DPlusOne => {
                        binary[3] = false;
                        binary[4] = false;
                        binary[5] = true;
                        binary[6] = true;
                        binary[7] = true;
                        binary[8] = true;
                        binary[9] = true;
                    }
                    Comp::APlusOne | Comp::MPlusOne => {
                        binary[3] = *comp == Comp::MPlusOne;
                        binary[4] = true;
                        binary[5] = true;
                        binary[6] = false;
                        binary[7] = true;
                        binary[8] = true;
                        binary[9] = true;
                    }
                    Comp::DMinusOne => {
                        binary[3] = false;
                        binary[4] = false;
                        binary[5] = false;
                        binary[6] = true;
                        binary[7] = true;
                        binary[8] = true;
                        binary[9] = false;
                    }
                    Comp::AMinusOne | Comp::MMinusOne => {
                        binary[3] = *comp == Comp::MMinusOne;
                        binary[4] = true;
                        binary[5] = true;
                        binary[6] = false;
                        binary[7] = false;
                        binary[8] = true;
                        binary[9] = false;
                    }
                    Comp::DPlusA | Comp::DPlusM => {
                        binary[3] = *comp == Comp::DPlusM;
                        binary[4] = false;
                        binary[5] = false;
                        binary[6] = false;
                        binary[7] = false;
                        binary[8] = true;
                        binary[9] = false;
                    }
                    Comp::DMinusA | Comp::DMinusM => {
                        binary[3] = *comp == Comp::DMinusM;
                        binary[4] = false;
                        binary[5] = true;
                        binary[6] = false;
                        binary[7] = false;
                        binary[8] = true;
                        binary[9] = true;
                    }
                    Comp::AMinusD | Comp::MMinusD => {
                        binary[3] = *comp == Comp::MMinusD;
                        binary[4] = false;
                        binary[5] = false;
                        binary[6] = false;
                        binary[7] = true;
                        binary[8] = true;
                        binary[9] = true;
                    }
                    Comp::DAndA | Comp::DAndM => {
                        binary[3] = *comp == Comp::DAndM;
                        binary[4] = false;
                        binary[5] = false;
                        binary[6] = false;
                        binary[7] = false;
                        binary[8] = false;
                        binary[9] = false;
                    }
                    Comp::DOrA | Comp::DOrM => {
                        binary[3] = *comp == Comp::DOrM;
                        binary[4] = false;
                        binary[5] = true;
                        binary[6] = false;
                        binary[7] = true;
                        binary[8] = false;
                        binary[9] = true;
                    }
                }
                if let Some(dest) = dest {
                    match dest {
                        Dest::M => {
                            binary[10] = false;
                            binary[11] = false;
                            binary[12] = true;
                        }
                        Dest::D => {
                            binary[10] = false;
                            binary[11] = true;
                            binary[12] = false;
                        }
                        Dest::DM => {
                            binary[10] = false;
                            binary[11] = true;
                            binary[12] = true;
                        }
                        Dest::A => {
                            binary[10] = true;
                            binary[11] = false;
                            binary[12] = false;
                        }
                        Dest::AM => {
                            binary[10] = true;
                            binary[11] = false;
                            binary[12] = true;
                        }
                        Dest::AD => {
                            binary[10] = true;
                            binary[11] = true;
                            binary[12] = false;
                        }
                        Dest::ADM => {
                            binary[10] = true;
                            binary[11] = true;
                            binary[12] = true;
                        }
                    }
                }
                if let Some(jump) = jump {
                    match jump {
                        Jump::JGT => {
                            binary[13] = false;
                            binary[14] = false;
                            binary[15] = true;
                        }
                        Jump::JEQ => {
                            binary[13] = false;
                            binary[14] = true;
                            binary[15] = false;
                        }
                        Jump::JGE => {
                            binary[13] = false;
                            binary[14] = true;
                            binary[15] = true;
                        }
                        Jump::JLT => {
                            binary[13] = true;
                            binary[14] = false;
                            binary[15] = false;
                        }
                        Jump::JNE => {
                            binary[13] = true;
                            binary[14] = false;
                            binary[15] = true;
                        }
                        Jump::JLE => {
                            binary[13] = true;
                            binary[14] = true;
                            binary[15] = false;
                        }
                        Jump::JMP => {
                            binary[13] = true;
                            binary[14] = true;
                            binary[15] = true;
                        }
                    }
                }
                binary
            }
        }
    }
}
