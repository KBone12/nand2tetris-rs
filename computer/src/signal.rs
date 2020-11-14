pub trait Signal: Clone + Copy + From<bool> {
    fn zero() -> Self;
    fn nand(&self, other: Self) -> Self;
}

impl Signal for bool {
    fn zero() -> Self {
        false
    }

    fn nand(&self, other: Self) -> Self {
        !(*self && other)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Word(u16);

impl Word {
    pub fn split(self) -> [bool; 16] {
        [
            (self.0 >> 15 & 1) == 1,
            (self.0 >> 14 & 1) == 1,
            (self.0 >> 13 & 1) == 1,
            (self.0 >> 12 & 1) == 1,
            (self.0 >> 11 & 1) == 1,
            (self.0 >> 10 & 1) == 1,
            (self.0 >> 9 & 1) == 1,
            (self.0 >> 8 & 1) == 1,
            (self.0 >> 7 & 1) == 1,
            (self.0 >> 6 & 1) == 1,
            (self.0 >> 5 & 1) == 1,
            (self.0 >> 4 & 1) == 1,
            (self.0 >> 3 & 1) == 1,
            (self.0 >> 2 & 1) == 1,
            (self.0 >> 1 & 1) == 1,
            (self.0 >> 0 & 1) == 1,
        ]
    }

    pub fn as_raw(&self) -> u16 {
        self.0
    }
}

impl Signal for Word {
    fn zero() -> Self {
        Self(0)
    }

    fn nand(&self, other: Self) -> Self {
        Self::from(!(self.0 & other.0))
    }
}

impl From<u16> for Word {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<[bool; 16]> for Word {
    fn from(value: [bool; 16]) -> Self {
        Self(value.iter().fold(0, |acc, x| (acc << 1) | (*x as u16)))
    }
}

impl From<bool> for Word {
    fn from(value: bool) -> Self {
        Self::from([value; 16])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_true_is_two_to_the_power_of_sixteen_minus_one() {
        assert_eq!(Word::from([true; 16]), Word::from(0b1111111111111111));
    }

    #[test]
    fn split_from_is_identity_function() {
        let inputs = [0, 1, 2, 3, 8, 32, (1 << 15), 0b1111111111111111];
        inputs.iter().for_each(|&input| {
            assert_eq!(Word::from(Word::from(input).split()), Word::from(input));
        });
    }
}
