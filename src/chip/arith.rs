use crate::chip::basic::{and, or, xor};

pub fn half_adder(a: bool, b: bool) -> (bool, bool) {
    // This implementation is not optimal but readable.
    (and(a, b), xor(a, b))
}

pub fn full_adder(a: bool, b: bool, c: bool) -> (bool, bool) {
    // This implementation is not optimal but readable.
    let (c0, sum) = half_adder(a, b);
    let (c1, sum) = half_adder(sum, c);
    (or(c1, c0), sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn half_adder_simply_adds_two_bits() {
        let inputs = [(false, false), (false, true), (true, false), (true, true)];
        let expected = [(false, false), (false, true), (false, true), (true, false)];

        inputs
            .iter()
            .zip(expected.iter())
            .for_each(|(&(a, b), &expected)| assert_eq!(half_adder(a, b), expected));
    }

    #[test]
    fn full_adder_adds_two_bits_and_the_carried_bit() {
        let inputs = [
            (false, false, false),
            (false, true, false),
            (true, false, false),
            (true, true, false),
            (false, false, true),
            (false, true, true),
            (true, false, true),
            (true, true, true),
        ];
        let expected = [
            (false, false),
            (false, true),
            (false, true),
            (true, false),
            (false, true),
            (true, false),
            (true, false),
            (true, true),
        ];

        inputs
            .iter()
            .zip(expected.iter())
            .for_each(|(&(a, b, c), &expected)| assert_eq!(full_adder(a, b, c), expected));
    }
}
