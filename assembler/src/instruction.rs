use std::convert::TryInto;

pub enum Instruction {
    A { value: [bool; 15] },
}

impl Instruction {
    pub fn as_binary(&self) -> [bool; 16] {
        match self {
            Self::A { value } => (0..16)
                .map(|i| if i == 0 { false } else { value[i - 1] })
                .collect::<Vec<_>>()
                .as_slice()
                .try_into()
                .unwrap(),
        }
    }
}
