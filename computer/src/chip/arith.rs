use crate::chip::basic::{and16, mux16, nand, not, not16, or};

pub const fn half_adder(a: bool, b: bool) -> (bool, bool) {
    // This is readable.
    /*
    (and(a, b), xor(a, b))
    */
    // This is the optimal.
    let tmp = nand(a, b);
    (not(tmp), nand(nand(a, tmp), nand(tmp, b)))
}

pub const fn full_adder(a: bool, b: bool, c: bool) -> (bool, bool) {
    // This is readable.
    /*
    let (c0, sum) = half_adder(a, b);
    let (c1, sum) = half_adder(sum, c);
    (or(c1, c0), sum)
    */
    // This is the optimal.
    let nab = nand(a, b);
    let xor_ab = nand(nand(a, nab), nand(nab, b));
    let tmp = nand(xor_ab, c);
    (nand(nab, tmp), nand(nand(xor_ab, tmp), nand(tmp, c)))
}

pub const fn add16(a: &[bool; 16], b: &[bool; 16]) -> [bool; 16] {
    let mut output = [false; 16];
    let (c, s) = half_adder(a[15], b[15]);
    let mut carry = c;
    output[15] = s;
    // Expand for-loop for const fn
    let (c, s) = full_adder(a[14 - 0], b[14 - 0], carry);
    carry = c;
    output[14 - 0] = s;
    let (c, s) = full_adder(a[14 - 1], b[14 - 1], carry);
    carry = c;
    output[14 - 1] = s;
    let (c, s) = full_adder(a[14 - 2], b[14 - 2], carry);
    carry = c;
    output[14 - 2] = s;
    let (c, s) = full_adder(a[14 - 3], b[14 - 3], carry);
    carry = c;
    output[14 - 3] = s;
    let (c, s) = full_adder(a[14 - 4], b[14 - 4], carry);
    carry = c;
    output[14 - 4] = s;
    let (c, s) = full_adder(a[14 - 5], b[14 - 5], carry);
    carry = c;
    output[14 - 5] = s;
    let (c, s) = full_adder(a[14 - 6], b[14 - 6], carry);
    carry = c;
    output[14 - 6] = s;
    let (c, s) = full_adder(a[14 - 7], b[14 - 7], carry);
    carry = c;
    output[14 - 7] = s;
    let (c, s) = full_adder(a[14 - 8], b[14 - 8], carry);
    carry = c;
    output[14 - 8] = s;
    let (c, s) = full_adder(a[14 - 9], b[14 - 9], carry);
    carry = c;
    output[14 - 9] = s;
    let (c, s) = full_adder(a[14 - 10], b[14 - 10], carry);
    carry = c;
    output[14 - 10] = s;
    let (c, s) = full_adder(a[14 - 11], b[14 - 11], carry);
    carry = c;
    output[14 - 11] = s;
    let (c, s) = full_adder(a[14 - 12], b[14 - 12], carry);
    carry = c;
    output[14 - 12] = s;
    let (c, s) = full_adder(a[14 - 13], b[14 - 13], carry);
    carry = c;
    output[14 - 13] = s;
    let (_c, s) = full_adder(a[14 - 14], b[14 - 14], carry);
    output[14 - 14] = s;
    output
}

pub const fn inc16(input: &[bool; 16]) -> [bool; 16] {
    add16(
        input,
        &[
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, true,
        ],
    )
}

pub const fn alu(
    a: &[bool; 16],
    b: &[bool; 16],
    zero_a: bool,
    negate_a: bool,
    zero_b: bool,
    negate_b: bool,
    f: bool,
    negate_output: bool,
) -> ([bool; 16], bool, bool) {
    let a = &and16(&[not(zero_a); 16], a);
    let a = &mux16(a, &not16(a), negate_a);
    let b = &and16(&[not(zero_b); 16], b);
    let b = &mux16(b, &not16(b), negate_b);
    let and_ab = &and16(a, b);
    let add_ab = &add16(a, b);
    let output = &mux16(and_ab, add_ab, f);
    let output = mux16(output, &not16(output), negate_output);
    let zero = not(or(
        or(
            or(or(output[0], output[1]), or(output[2], output[3])),
            or(or(output[4], output[5]), or(output[6], output[7])),
        ),
        or(
            or(or(output[8], output[9]), or(output[10], output[11])),
            or(or(output[12], output[13]), or(output[14], output[15])),
        ),
    ));
    let negate = output[0];
    (output, zero, negate)
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

    #[test]
    fn alu_without_any_flags_outputs_and_of_both_inputs() {
        let inputs = [
            ([false; 16], [false; 16]),
            ([false; 16], [true; 16]),
            ([true; 16], [false; 16]),
            ([true; 16], [true; 16]),
            (
                [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false,
                ],
                [
                    true, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false,
                ],
            ),
            (
                [
                    true, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false,
                ],
                [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false,
                ],
            ),
            (
                [
                    true, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false,
                ],
                [
                    true, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false,
                ],
            ),
        ];
        let expected = [
            ([false; 16], true, false),
            ([false; 16], true, false),
            ([false; 16], true, false),
            ([true; 16], false, true),
            ([false; 16], true, false),
            ([false; 16], true, false),
            (
                [
                    true, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false,
                ],
                false,
                true,
            ),
        ];

        inputs
            .iter()
            .zip(expected.iter())
            .for_each(|((a, b), &out)| {
                assert_eq!(alu(a, b, false, false, false, false, false, false), out)
            });
    }

    #[test]
    fn alu_with_f_outputs_sum_of_both_inputs() {
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
            ([false; 16], true, false),
            ([true; 16], false, true),
            ([true; 16], false, true),
            (
                [
                    true, true, true, true, true, true, true, true, true, true, true, true, true,
                    true, true, false,
                ],
                false,
                true,
            ),
            ([true; 16], false, true),
            (
                [
                    true, false, true, false, true, false, true, false, true, false, true, false,
                    true, false, true, false,
                ],
                false,
                true,
            ),
        ];

        inputs
            .iter()
            .zip(expected.iter())
            .for_each(|((a, b), &out)| {
                assert_eq!(alu(a, b, false, false, false, false, true, false), out)
            });
    }

    #[test]
    fn alu_with_negate_output_inverts_the_output() {
        let inputs = [
            (
                [false; 16],
                [false; 16],
                false,
                false,
                false,
                false,
                false,
                true,
            ),
            (
                [false; 16],
                [true; 16],
                false,
                false,
                false,
                false,
                false,
                true,
            ),
            (
                [true; 16],
                [false; 16],
                false,
                false,
                false,
                false,
                false,
                true,
            ),
            (
                [true; 16], [true; 16], false, false, false, false, false, true,
            ),
            (
                [false; 16],
                [false; 16],
                false,
                false,
                false,
                false,
                true,
                true,
            ),
            (
                [false; 16],
                [true; 16],
                false,
                false,
                false,
                false,
                true,
                true,
            ),
            (
                [true; 16],
                [false; 16],
                false,
                false,
                false,
                false,
                true,
                true,
            ),
            (
                [true; 16], [true; 16], false, false, false, false, true, true,
            ),
        ];
        let expected = [
            ([true; 16], false, true),
            ([true; 16], false, true),
            ([true; 16], false, true),
            ([false; 16], true, false),
            ([true; 16], false, true),
            ([false; 16], true, false),
            ([false; 16], true, false),
            (
                [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, true,
                ],
                false,
                false,
            ),
        ];

        inputs
            .iter()
            .zip(expected.iter())
            .for_each(|((a, b, za, na, zb, nb, f, no), &out)| {
                assert_eq!(alu(a, b, *za, *na, *zb, *nb, *f, *no), out)
            });
    }

    #[test]
    fn alu_with_zero_flag_forces_the_input_zero() {
        let inputs = [
            ([false; 16], [false; 16], false, false),
            ([false; 16], [false; 16], true, true),
            ([false; 16], [true; 16], false, true),
            ([false; 16], [true; 16], true, false),
            ([true; 16], [false; 16], false, true),
            ([true; 16], [false; 16], true, false),
            ([true; 16], [true; 16], true, true),
        ];
        let expected = [
            ([false; 16], true, false),
            ([false; 16], true, false),
            ([false; 16], true, false),
            ([true; 16], false, true),
            ([true; 16], false, true),
            ([false; 16], true, false),
            ([false; 16], true, false),
        ];

        inputs
            .iter()
            .zip(expected.iter())
            .for_each(|((a, b, za, zb), &out)| {
                assert_eq!(alu(a, b, *za, false, *zb, false, true, false), out)
            });
    }

    #[test]
    fn alu_with_negate_flag_inverts_the_input() {
        let inputs = [
            ([false; 16], [false; 16], false, false),
            ([false; 16], [false; 16], true, true),
            ([false; 16], [true; 16], false, true),
            ([false; 16], [true; 16], true, false),
            ([true; 16], [false; 16], false, true),
            ([true; 16], [false; 16], true, false),
            ([true; 16], [true; 16], false, false),
            ([true; 16], [true; 16], true, true),
        ];
        let expected = [
            ([false; 16], true, false),
            (
                [
                    true, true, true, true, true, true, true, true, true, true, true, true, true,
                    true, true, false,
                ],
                false,
                true,
            ),
            ([false; 16], true, false),
            (
                [
                    true, true, true, true, true, true, true, true, true, true, true, true, true,
                    true, true, false,
                ],
                false,
                true,
            ),
            (
                [
                    true, true, true, true, true, true, true, true, true, true, true, true, true,
                    true, true, false,
                ],
                false,
                true,
            ),
            ([false; 16], true, false),
            (
                [
                    true, true, true, true, true, true, true, true, true, true, true, true, true,
                    true, true, false,
                ],
                false,
                true,
            ),
            ([false; 16], true, false),
        ];

        inputs
            .iter()
            .zip(expected.iter())
            .for_each(|((a, b, na, nb), &out)| {
                assert_eq!(alu(a, b, false, *na, false, *nb, true, false), out)
            });
    }
}
