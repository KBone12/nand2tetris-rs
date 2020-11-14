use crate::{
    chip::basic::{and, mux, nand, not, or},
    signal::Word,
};

pub fn half_adder(a: bool, b: bool) -> (bool, bool) {
    // This is readable.
    /*
    (and(a, b), xor(a, b))
    */
    // This is the optimal.
    let tmp = nand(a, b);
    (not(tmp), nand(nand(a, tmp), nand(tmp, b)))
}

pub fn full_adder(a: bool, b: bool, c: bool) -> (bool, bool) {
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

pub fn add(a: Word, b: Word) -> Word {
    let a = a.split();
    let b = b.split();
    let mut output = [false; 16];
    let (c, s) = half_adder(a[15], b[15]);
    let mut carry = c;
    output[15] = s;
    for i in 1..16 {
        let (c, s) = full_adder(a[15 - i], b[15 - i], carry);
        carry = c;
        output[15 - i] = s;
    }
    Word::from(output)
}

pub fn inc(input: Word) -> Word {
    add(input, Word::from(1))
}

pub fn alu(
    a: Word,
    b: Word,
    zero_a: bool,
    negate_a: bool,
    zero_b: bool,
    negate_b: bool,
    f: bool,
    negate_output: bool,
) -> (Word, bool, bool) {
    let a = and(Word::from(not(zero_a)), a);
    let a = mux(a, not(a), negate_a);
    let b = and(Word::from(not(zero_b)), b);
    let b = mux(b, not(b), negate_b);
    let and_ab = and(a, b);
    let add_ab = add(a, b);
    let output = mux(and_ab, add_ab, f);
    let output = mux(output, not(output), negate_output);
    let bits = output.split();
    let zero = not(or(
        or(
            or(or(bits[0], bits[1]), or(bits[2], bits[3])),
            or(or(bits[4], bits[5]), or(bits[6], bits[7])),
        ),
        or(
            or(or(bits[8], bits[9]), or(bits[10], bits[11])),
            or(or(bits[12], bits[13]), or(bits[14], bits[15])),
        ),
    ));
    let negate = bits[0];
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
            .for_each(|(&(a, b), &out)| {
                assert_eq!(
                    add(Word::from(a), Word::from(b)),
                    Word::from(out),
                    "{:?} + {:?} = {:?}",
                    Word::from(a),
                    Word::from(b),
                    Word::from(out)
                )
            });
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
            .for_each(|(&input, &out)| assert_eq!(inc(Word::from(input)), Word::from(out)));
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
            (Word::from([false; 16]), true, false),
            (Word::from([false; 16]), true, false),
            (Word::from([false; 16]), true, false),
            (Word::from([true; 16]), false, true),
            (Word::from([false; 16]), true, false),
            (Word::from([false; 16]), true, false),
            (
                Word::from([
                    true, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false,
                ]),
                false,
                true,
            ),
        ];

        inputs
            .iter()
            .zip(expected.iter())
            .for_each(|(&(a, b), &out)| {
                assert_eq!(
                    alu(
                        Word::from(a),
                        Word::from(b),
                        false,
                        false,
                        false,
                        false,
                        false,
                        false
                    ),
                    out
                )
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
            (Word::from([false; 16]), true, false),
            (Word::from([true; 16]), false, true),
            (Word::from([true; 16]), false, true),
            (
                Word::from([
                    true, true, true, true, true, true, true, true, true, true, true, true, true,
                    true, true, false,
                ]),
                false,
                true,
            ),
            (Word::from([true; 16]), false, true),
            (
                Word::from([
                    true, false, true, false, true, false, true, false, true, false, true, false,
                    true, false, true, false,
                ]),
                false,
                true,
            ),
        ];

        inputs
            .iter()
            .zip(expected.iter())
            .for_each(|(&(a, b), &out)| {
                assert_eq!(
                    alu(
                        Word::from(a),
                        Word::from(b),
                        false,
                        false,
                        false,
                        false,
                        true,
                        false
                    ),
                    out
                )
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
            (Word::from([true; 16]), false, true),
            (Word::from([true; 16]), false, true),
            (Word::from([true; 16]), false, true),
            (Word::from([false; 16]), true, false),
            (Word::from([true; 16]), false, true),
            (Word::from([false; 16]), true, false),
            (Word::from([false; 16]), true, false),
            (
                Word::from([
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, true,
                ]),
                false,
                false,
            ),
        ];

        inputs
            .iter()
            .zip(expected.iter())
            .for_each(|(&(a, b, za, na, zb, nb, f, no), &out)| {
                assert_eq!(
                    alu(Word::from(a), Word::from(b), za, na, zb, nb, f, no),
                    out
                )
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
            (Word::from([false; 16]), true, false),
            (Word::from([false; 16]), true, false),
            (Word::from([false; 16]), true, false),
            (Word::from([true; 16]), false, true),
            (Word::from([true; 16]), false, true),
            (Word::from([false; 16]), true, false),
            (Word::from([false; 16]), true, false),
        ];

        inputs
            .iter()
            .zip(expected.iter())
            .for_each(|(&(a, b, za, zb), &out)| {
                assert_eq!(
                    alu(
                        Word::from(a),
                        Word::from(b),
                        za,
                        false,
                        zb,
                        false,
                        true,
                        false
                    ),
                    out
                )
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
            (Word::from([false; 16]), true, false),
            (
                Word::from([
                    true, true, true, true, true, true, true, true, true, true, true, true, true,
                    true, true, false,
                ]),
                false,
                true,
            ),
            (Word::from([false; 16]), true, false),
            (
                Word::from([
                    true, true, true, true, true, true, true, true, true, true, true, true, true,
                    true, true, false,
                ]),
                false,
                true,
            ),
            (
                Word::from([
                    true, true, true, true, true, true, true, true, true, true, true, true, true,
                    true, true, false,
                ]),
                false,
                true,
            ),
            (Word::from([false; 16]), true, false),
            (
                Word::from([
                    true, true, true, true, true, true, true, true, true, true, true, true, true,
                    true, true, false,
                ]),
                false,
                true,
            ),
            (Word::from([false; 16]), true, false),
        ];

        inputs
            .iter()
            .zip(expected.iter())
            .for_each(|(&(a, b, na, nb), &out)| {
                assert_eq!(
                    alu(
                        Word::from(a),
                        Word::from(b),
                        false,
                        na,
                        false,
                        nb,
                        true,
                        false
                    ),
                    out
                )
            });
    }
}
