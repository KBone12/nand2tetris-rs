use crate::signal::Signal;

#[inline]
pub fn nand<S: Signal>(a: S, b: S) -> S {
    a.nand(b)
}

#[inline]
pub fn not<S: Signal>(input: S) -> S {
    nand(input, input)
}

#[inline]
pub fn and<S: Signal>(a: S, b: S) -> S {
    not(nand(a, b))
}

#[inline]
pub fn or<S: Signal>(a: S, b: S) -> S {
    nand(not(a), not(b))
}

#[inline]
pub fn xor<S: Signal>(a: S, b: S) -> S {
    let tmp = nand(a, b);
    nand(nand(a, tmp), nand(tmp, b))
}

#[inline]
pub fn mux<S: Signal>(a: S, b: S, selector: bool) -> S {
    // Readable
    // or(and(not(selector), a), and(selector, b))

    // Optimal
    nand(nand(S::from(not(selector)), a), nand(S::from(selector), b))
}

#[inline]
pub fn mux4way<S: Signal>(a: S, b: S, c: S, d: S, s1: bool, s0: bool) -> S {
    // Readable
    /*
    or16(
        &or16(
            &and16(&[and(not(s1), not(s0)); 16], a),
            &and16(&[and(not(s1), s0); 16], b),
        ),
        &or16(
            &and16(&[and(s1, not(s0)); 16], c),
            &and16(&[and(s1, s0); 16], d),
        ),
    )
    */

    // Optimal
    or(
        nand(
            nand(S::from(and(not(s1), not(s0))), a),
            nand(S::from(and(not(s1), s0)), b),
        ),
        nand(
            nand(S::from(and(s1, not(s0))), c),
            nand(S::from(and(s1, s0)), d),
        ),
    )
}

#[inline]
pub fn mux8way<S: Signal>(
    a: S,
    b: S,
    c: S,
    d: S,
    e: S,
    f: S,
    g: S,
    h: S,
    s2: bool,
    s1: bool,
    s0: bool,
) -> S {
    // Readable
    /*
    or16(
        &or16(
            &or16(
                &and16(&[and(not(s2), and(not(s1), not(s0))); 16], a),
                &and16(&[and(not(s2), and(not(s1), s0)); 16], b),
            ),
            &or16(
                &and16(&[and(not(s2), and(s1, not(s0))); 16], c),
                &and16(&[and(not(s2), and(s1, s0)); 16], d),
            ),
        ),
        &or16(
            &or16(
                &and16(&[and(s2, and(not(s1), not(s0))); 16], e),
                &and16(&[and(s2, and(not(s1), s0)); 16], f),
            ),
            &or16(
                &and16(&[and(s2, and(s1, not(s0))); 16], g),
                &and16(&[and(s2, and(s1, s0)); 16], h),
            ),
        ),
    )
    */

    // Optimal
    or(
        or(
            nand(
                nand(S::from(and(not(s2), and(not(s1), not(s0)))), a),
                nand(S::from(and(not(s2), and(not(s1), s0))), b),
            ),
            nand(
                nand(S::from(and(not(s2), and(s1, not(s0)))), c),
                nand(S::from(and(not(s2), and(s1, s0))), d),
            ),
        ),
        or(
            nand(
                nand(S::from(and(s2, and(not(s1), not(s0)))), e),
                nand(S::from(and(s2, and(not(s1), s0))), f),
            ),
            nand(
                nand(S::from(and(s2, and(s1, not(s0)))), g),
                nand(S::from(and(s2, and(s1, s0))), h),
            ),
        ),
    )
}

#[inline]
pub fn dmux<S: Signal>(input: S, selector: bool) -> (S, S) {
    (
        and(input, S::from(not(selector))),
        and(input, S::from(selector)),
    )
}

#[inline]
pub fn dmux4way<S: Signal>(input: S, s1: bool, s0: bool) -> (S, S, S, S) {
    (
        and(input, S::from(and(not(s1), not(s0)))),
        and(input, S::from(and(not(s1), s0))),
        and(input, S::from(and(s1, not(s0)))),
        and(input, S::from(and(s1, s0))),
    )
}

#[inline]
pub fn dmux8way<S: Signal>(input: S, s2: bool, s1: bool, s0: bool) -> (S, S, S, S, S, S, S, S) {
    (
        and(input, S::from(and(not(s2), and(not(s1), not(s0))))),
        and(input, S::from(and(not(s2), and(not(s1), s0)))),
        and(input, S::from(and(not(s2), and(s1, not(s0))))),
        and(input, S::from(and(not(s2), and(s1, s0)))),
        and(input, S::from(and(s2, and(not(s1), not(s0))))),
        and(input, S::from(and(s2, and(not(s1), s0)))),
        and(input, S::from(and(s2, and(s1, not(s0))))),
        and(input, S::from(and(s2, and(s1, s0)))),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::signal::Word;

    #[test]
    fn nand_returns_true_except_both_inputs_are_true() {
        let inputs = [(false, false), (false, true), (true, false), (true, true)];
        let expected = [true, true, true, false];

        inputs
            .iter()
            .zip(expected.iter())
            .for_each(|(&(a, b), &out)| assert_eq!(nand(a, b), out));
    }

    #[test]
    fn nand16_returns_true_except_both_inputs_are_true() {
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
            [true; 16],
            [true; 16],
            [true; 16],
            [false; 16],
            [true; 16],
            [true; 16],
            [
                false, true, true, true, true, true, true, true, true, true, true, true, true,
                true, true, true,
            ],
        ];

        inputs
            .iter()
            .zip(expected.iter())
            .for_each(|(&(a, b), &out)| {
                assert_eq!(nand(Word::from(a), Word::from(b)), Word::from(out))
            });
    }

    #[test]
    fn not_inverts_a_input() {
        let inputs = [false, true];
        let expected = [true, false];

        inputs
            .iter()
            .zip(expected.iter())
            .for_each(|(&input, &output)| assert_eq!(not(input), output));
    }

    #[test]
    fn not16_inverts_a_input() {
        let inputs = [
            [false; 16],
            [true; 16],
            [
                true, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false,
            ],
        ];
        let expected = [
            [true; 16],
            [false; 16],
            [
                false, true, true, true, true, true, true, true, true, true, true, true, true,
                true, true, true,
            ],
        ];

        inputs
            .iter()
            .zip(expected.iter())
            .for_each(|(&input, &output)| assert_eq!(not(Word::from(input)), Word::from(output)));
    }

    #[test]
    fn and_returns_false_except_both_inputs_are_true() {
        let inputs = [(false, false), (false, true), (true, false), (true, true)];
        let expected = [false, false, false, true];

        inputs
            .iter()
            .zip(expected.iter())
            .for_each(|(&(a, b), &out)| assert_eq!(and(a, b), out));
    }

    #[test]
    fn and16_returns_false_except_both_inputs_are_true() {
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
            [false; 16],
            [false; 16],
            [false; 16],
            [true; 16],
            [false; 16],
            [false; 16],
            [
                true, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false,
            ],
        ];

        inputs
            .iter()
            .zip(expected.iter())
            .for_each(|(&(a, b), &out)| {
                assert_eq!(and(Word::from(a), Word::from(b)), Word::from(out))
            });
    }

    #[test]
    fn or_returns_true_except_both_inputs_are_false() {
        let inputs = [(false, false), (false, true), (true, false), (true, true)];
        let expected = [false, true, true, true];

        inputs
            .iter()
            .zip(expected.iter())
            .for_each(|(&(a, b), &out)| assert_eq!(or(a, b), out));
    }

    #[test]
    fn or16_returns_true_except_both_inputs_are_false() {
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
            [false; 16],
            [true; 16],
            [true; 16],
            [true; 16],
            [
                true, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false,
            ],
            [
                true, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false,
            ],
            [
                true, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false,
            ],
        ];

        inputs
            .iter()
            .zip(expected.iter())
            .for_each(|(&(a, b), &out)| {
                assert_eq!(or(Word::from(a), Word::from(b)), Word::from(out))
            });
    }

    #[test]
    fn xor_returns_false_if_both_inputs_are_the_same() {
        let inputs = [(false, false), (false, true), (true, false), (true, true)];
        let expected = [false, true, true, false];

        inputs
            .iter()
            .zip(expected.iter())
            .for_each(|(&(a, b), &out)| assert_eq!(xor(a, b), out));
    }

    #[test]
    fn mux_selects_the_input() {
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
        let expected = [false, false, true, true, false, true, false, true];

        inputs
            .iter()
            .zip(expected.iter())
            .for_each(|(&(a, b, s), &out)| assert_eq!(mux(a, b, s), out));
    }

    #[test]
    fn mux16_selects_the_input() {
        let inputs = [
            ([false; 16], [false; 16], false),
            ([false; 16], [true; 16], false),
            ([true; 16], [false; 16], false),
            ([false; 16], [true; 16], true),
            ([true; 16], [false; 16], true),
            ([true; 16], [true; 16], true),
        ];
        let expected = [
            [false; 16],
            [false; 16],
            [true; 16],
            [true; 16],
            [false; 16],
            [true; 16],
        ];

        inputs
            .iter()
            .zip(expected.iter())
            .for_each(|(&(a, b, s), &out)| {
                assert_eq!(mux(Word::from(a), Word::from(b), s), Word::from(out))
            });
    }

    #[test]
    fn mux4way16_selects_the_input() {
        let inputs = [
            (
                [false; 16],
                [false; 16],
                [false; 16],
                [false; 16],
                (false, false),
            ),
            (
                [true; 16],
                [false; 16],
                [false; 16],
                [false; 16],
                (false, false),
            ),
            (
                [false; 16],
                [true; 16],
                [false; 16],
                [false; 16],
                (false, true),
            ),
            (
                [false; 16],
                [false; 16],
                [true; 16],
                [false; 16],
                (true, false),
            ),
            (
                [false; 16],
                [false; 16],
                [false; 16],
                [true; 16],
                (true, true),
            ),
        ];
        let expected = [[false; 16], [true; 16], [true; 16], [true; 16], [true; 16]];

        inputs
            .iter()
            .zip(expected.iter())
            .for_each(|(&(a, b, c, d, (s1, s0)), &out)| {
                assert_eq!(
                    mux4way(
                        Word::from(a),
                        Word::from(b),
                        Word::from(c),
                        Word::from(d),
                        s1,
                        s0
                    ),
                    Word::from(out)
                )
            });
    }

    #[test]
    fn mux8way16_selects_the_input() {
        let inputs = [
            (
                [false; 16],
                [false; 16],
                [false; 16],
                [false; 16],
                [false; 16],
                [false; 16],
                [false; 16],
                [false; 16],
                (false, false, false),
            ),
            (
                [true; 16],
                [false; 16],
                [false; 16],
                [false; 16],
                [false; 16],
                [false; 16],
                [false; 16],
                [false; 16],
                (false, false, false),
            ),
            (
                [false; 16],
                [true; 16],
                [false; 16],
                [false; 16],
                [false; 16],
                [false; 16],
                [false; 16],
                [false; 16],
                (false, false, true),
            ),
            (
                [false; 16],
                [false; 16],
                [true; 16],
                [false; 16],
                [false; 16],
                [false; 16],
                [false; 16],
                [false; 16],
                (false, true, false),
            ),
            (
                [false; 16],
                [false; 16],
                [false; 16],
                [true; 16],
                [false; 16],
                [false; 16],
                [false; 16],
                [false; 16],
                (false, true, true),
            ),
            (
                [false; 16],
                [false; 16],
                [false; 16],
                [false; 16],
                [true; 16],
                [false; 16],
                [false; 16],
                [false; 16],
                (true, false, false),
            ),
            (
                [false; 16],
                [false; 16],
                [false; 16],
                [false; 16],
                [false; 16],
                [true; 16],
                [false; 16],
                [false; 16],
                (true, false, true),
            ),
            (
                [false; 16],
                [false; 16],
                [false; 16],
                [false; 16],
                [false; 16],
                [false; 16],
                [true; 16],
                [false; 16],
                (true, true, false),
            ),
            (
                [false; 16],
                [false; 16],
                [false; 16],
                [false; 16],
                [false; 16],
                [false; 16],
                [false; 16],
                [true; 16],
                (true, true, true),
            ),
        ];
        let expected = [
            [false; 16],
            [true; 16],
            [true; 16],
            [true; 16],
            [true; 16],
            [true; 16],
            [true; 16],
            [true; 16],
            [true; 16],
        ];

        inputs.iter().zip(expected.iter()).for_each(
            |(&(a, b, c, d, e, f, g, h, (s2, s1, s0)), &out)| {
                assert_eq!(
                    mux8way(
                        Word::from(a),
                        Word::from(b),
                        Word::from(c),
                        Word::from(d),
                        Word::from(e),
                        Word::from(f),
                        Word::from(g),
                        Word::from(h),
                        s2,
                        s1,
                        s0
                    ),
                    Word::from(out)
                )
            },
        );
    }

    #[test]
    fn dmux_divides_the_input() {
        let inputs = [(false, false), (false, true), (true, false), (true, true)];
        let expected = [(false, false), (false, false), (true, false), (false, true)];

        inputs
            .iter()
            .zip(expected.iter())
            .for_each(|(&(input, s), &(x, y))| {
                let (xx, yy) = dmux(input, s);
                assert_eq!(xx, x);
                assert_eq!(yy, y);
            });
    }

    #[test]
    fn dmux4way_divides_the_input() {
        let inputs = [
            (false, false, false),
            (true, false, false),
            (true, false, true),
            (true, true, false),
            (true, true, true),
        ];
        let expected = [
            (false, false, false, false),
            (true, false, false, false),
            (false, true, false, false),
            (false, false, true, false),
            (false, false, false, true),
        ];

        inputs
            .iter()
            .zip(expected.iter())
            .for_each(|(&(input, s1, s0), &(w, x, y, z))| {
                let (ww, xx, yy, zz) = dmux4way(input, s1, s0);
                assert_eq!(ww, w);
                assert_eq!(xx, x);
                assert_eq!(yy, y);
                assert_eq!(zz, z);
            });
    }

    #[test]
    fn dmux8way_divides_the_input() {
        let inputs = [
            (false, false, false, false),
            (true, false, false, false),
            (true, false, false, true),
            (true, false, true, false),
            (true, false, true, true),
            (true, true, false, false),
            (true, true, false, true),
            (true, true, true, false),
            (true, true, true, true),
        ];
        let expected = [
            (false, false, false, false, false, false, false, false),
            (true, false, false, false, false, false, false, false),
            (false, true, false, false, false, false, false, false),
            (false, false, true, false, false, false, false, false),
            (false, false, false, true, false, false, false, false),
            (false, false, false, false, true, false, false, false),
            (false, false, false, false, false, true, false, false),
            (false, false, false, false, false, false, true, false),
            (false, false, false, false, false, false, false, true),
        ];

        inputs.iter().zip(expected.iter()).for_each(
            |(&(input, s2, s1, s0), &(a, b, c, d, e, f, g, h))| {
                let (aa, bb, cc, dd, ee, ff, gg, hh) = dmux8way(input, s2, s1, s0);
                assert_eq!(aa, a);
                assert_eq!(bb, b);
                assert_eq!(cc, c);
                assert_eq!(dd, d);
                assert_eq!(ee, e);
                assert_eq!(ff, f);
                assert_eq!(gg, g);
                assert_eq!(hh, h);
            },
        );
    }
}
