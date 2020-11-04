use crate::chip::basic::{nand, not, or};

pub fn half_adder(a: bool, b: bool) -> (bool, bool) {
    // This is the optimal.
    let tmp = nand(a, b);
    (not(tmp), nand(nand(a, tmp), nand(tmp, b)))
    // This is readable.
    /*
    (and(a, b), xor(a, b))
    */
}

pub fn full_adder(a: bool, b: bool, c: bool) -> (bool, bool) {
    // This implementation is not optimal but readable.
    let (c0, sum) = half_adder(a, b);
    let (c1, sum) = half_adder(sum, c);
    (or(c1, c0), sum)
}

pub fn add16(a: &[bool; 16], b: &[bool; 16]) -> [bool; 16] {
    let mut output = [false; 16];
    let (c, s) = half_adder(a[15], b[15]);
    let mut carry = c;
    output[15] = s;
    for i in 0..15 {
        let (c, s) = full_adder(a[14 - i], b[14 - i], carry);
        carry = c;
        output[14 - i] = s;
    }
    output
}

pub fn inc16(input: &[bool; 16]) -> [bool; 16] {
    add16(
        input,
        &[
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, true,
        ],
    )
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

    #[test]
    fn add16_adds_two_16bit_integers() {
        let inputs = [
            ([false; 16], [false; 16]),
            ([false; 16], [true; 16]),
            ([true; 16], [false; 16]),
            ([true; 16], [true; 16]),
            (
                [
                    false, true, false, true, false, true, false, true, false, true, false, true,
                    false, true, false, true,
                ],
                [
                    true, false, true, false, true, false, true, false, true, false, true, false,
                    true, false, true, false,
                ],
            ),
            (
                [
                    false, false, false, true, false, false, true, false, false, false, true, true,
                    false, true, false, false,
                ],
                [
                    true, false, false, true, true, false, false, false, false, true, true, true,
                    false, true, true, false,
                ],
            ),
        ];
        let expected = [
            [false; 16],
            [true; 16],
            [true; 16],
            [
                true, true, true, true, true, true, true, true, true, true, true, true, true, true,
                true, false,
            ],
            [true; 16],
            [
                true, false, true, false, true, false, true, false, true, false, true, false, true,
                false, true, false,
            ],
        ];

        inputs
            .iter()
            .zip(expected.iter())
            .for_each(|((a, b), &out)| assert_eq!(add16(a, b), out));
    }

    #[test]
    fn inc16_increments_an_input() {
        let inputs = [
            [false; 16],
            [true; 16],
            [
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, true,
            ],
            [
                false, false, false, false, false, false, false, false, false, false, true, false,
                false, false, false, false,
            ],
        ];
        let expected = [
            [
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, true,
            ],
            [false; 16],
            [
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, true, false,
            ],
            [
                false, false, false, false, false, false, false, false, false, false, true, false,
                false, false, false, true,
            ],
        ];

        inputs
            .iter()
            .zip(expected.iter())
            .for_each(|(input, &out)| assert_eq!(inc16(input), out));
    }
}
