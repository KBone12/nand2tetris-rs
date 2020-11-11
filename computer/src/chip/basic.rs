pub const fn nand(a: bool, b: bool) -> bool {
    !(a && b)
}

pub const fn nand16(a: &[bool; 16], b: &[bool; 16]) -> [bool; 16] {
    [
        nand(a[0], b[0]),
        nand(a[1], b[1]),
        nand(a[2], b[2]),
        nand(a[3], b[3]),
        nand(a[4], b[4]),
        nand(a[5], b[5]),
        nand(a[6], b[6]),
        nand(a[7], b[7]),
        nand(a[8], b[8]),
        nand(a[9], b[9]),
        nand(a[10], b[10]),
        nand(a[11], b[11]),
        nand(a[12], b[12]),
        nand(a[13], b[13]),
        nand(a[14], b[14]),
        nand(a[15], b[15]),
    ]
}

pub const fn not(input: bool) -> bool {
    nand(input, input)
}

pub const fn not16(input: &[bool; 16]) -> [bool; 16] {
    nand16(input, input)
}

pub const fn and(a: bool, b: bool) -> bool {
    not(nand(a, b))
}

pub const fn and16(a: &[bool; 16], b: &[bool; 16]) -> [bool; 16] {
    not16(&nand16(a, b))
}

pub const fn or(a: bool, b: bool) -> bool {
    nand(not(a), not(b))
}

pub const fn or16(a: &[bool; 16], b: &[bool; 16]) -> [bool; 16] {
    nand16(&not16(a), &not16(b))
}

pub const fn or8way(input: &[bool; 8]) -> bool {
    or(
        or(
            or(
                or(or(or(or(input[0], input[1]), input[2]), input[3]), input[4]),
                input[5],
            ),
            input[6],
        ),
        input[7],
    )
}

pub const fn xor(a: bool, b: bool) -> bool {
    let tmp = nand(a, b);
    nand(nand(a, tmp), nand(tmp, b))
}

pub const fn mux(a: bool, b: bool, selector: bool) -> bool {
    // Readable
    // or(and(not(selector), a), and(selector, b))

    // Optimal
    nand(nand(not(selector), a), nand(selector, b))
}

pub const fn mux16(a: &[bool; 16], b: &[bool; 16], selector: bool) -> [bool; 16] {
    // Readable
    // or16(&and16(&[not(selector); 16], a), &and16(&[selector; 16], b))

    // Optimal
    nand16(
        &nand16(&[not(selector); 16], a),
        &nand16(&[selector; 16], b),
    )
}

pub const fn mux4way16(
    a: &[bool; 16],
    b: &[bool; 16],
    c: &[bool; 16],
    d: &[bool; 16],
    s1: bool,
    s0: bool,
) -> [bool; 16] {
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
    or16(
        &nand16(
            &nand16(&[and(not(s1), not(s0)); 16], a),
            &nand16(&[and(not(s1), s0); 16], b),
        ),
        &nand16(
            &nand16(&[and(s1, not(s0)); 16], c),
            &nand16(&[and(s1, s0); 16], d),
        ),
    )
}

pub const fn mux8way16(
    a: &[bool; 16],
    b: &[bool; 16],
    c: &[bool; 16],
    d: &[bool; 16],
    e: &[bool; 16],
    f: &[bool; 16],
    g: &[bool; 16],
    h: &[bool; 16],
    s2: bool,
    s1: bool,
    s0: bool,
) -> [bool; 16] {
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
    or16(
        &or16(
            &nand16(
                &nand16(&[and(not(s2), and(not(s1), not(s0))); 16], a),
                &nand16(&[and(not(s2), and(not(s1), s0)); 16], b),
            ),
            &nand16(
                &nand16(&[and(not(s2), and(s1, not(s0))); 16], c),
                &nand16(&[and(not(s2), and(s1, s0)); 16], d),
            ),
        ),
        &or16(
            &nand16(
                &nand16(&[and(s2, and(not(s1), not(s0))); 16], e),
                &nand16(&[and(s2, and(not(s1), s0)); 16], f),
            ),
            &nand16(
                &nand16(&[and(s2, and(s1, not(s0))); 16], g),
                &nand16(&[and(s2, and(s1, s0)); 16], h),
            ),
        ),
    )
}

pub const fn dmux(input: bool, selector: bool) -> (bool, bool) {
    (and(input, not(selector)), and(input, selector))
}

pub const fn dmux4way(input: bool, s1: bool, s0: bool) -> (bool, bool, bool, bool) {
    (
        and(input, and(not(s1), not(s0))),
        and(input, and(not(s1), s0)),
        and(input, and(s1, not(s0))),
        and(input, and(s1, s0)),
    )
}

pub const fn dmux8way(input: bool, s2: bool, s1: bool, s0: bool) -> [bool; 8] {
    [
        and(input, and(not(s2), and(not(s1), not(s0)))),
        and(input, and(not(s2), and(not(s1), s0))),
        and(input, and(not(s2), and(s1, not(s0)))),
        and(input, and(not(s2), and(s1, s0))),
        and(input, and(s2, and(not(s1), not(s0)))),
        and(input, and(s2, and(not(s1), s0))),
        and(input, and(s2, and(s1, not(s0)))),
        and(input, and(s2, and(s1, s0))),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

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
            .for_each(|((a, b), &out)| assert_eq!(nand16(a, b), out));
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
            .for_each(|(input, &output)| assert_eq!(not16(input), output));
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
            .for_each(|((a, b), &out)| assert_eq!(and16(a, b), out));
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
            .for_each(|((a, b), &out)| assert_eq!(or16(a, b), out));
    }

    #[test]
    fn or8way_returns_true_except_all_inputs_are_false() {
        let inputs = [
            [false; 8],
            [false, false, false, false, false, false, false, true],
            [false, false, false, false, false, false, true, false],
            [false, false, false, false, false, true, false, false],
            [false, false, false, false, true, false, false, false],
            [false, false, false, true, false, false, false, false],
            [false, false, true, false, false, false, false, false],
            [false, true, false, false, false, false, false, false],
            [true, false, false, false, false, false, false, false],
        ];
        let expected = [false, true, true, true, true, true, true, true, true];

        inputs
            .iter()
            .zip(expected.iter())
            .for_each(|(input, &output)| assert_eq!(or8way(input), output));
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
            .for_each(|((a, b, s), &out)| assert_eq!(mux16(a, b, *s), out));
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
            .for_each(|((a, b, c, d, (s1, s0)), &out)| {
                assert_eq!(mux4way16(a, b, c, d, *s1, *s0), out)
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
            |((a, b, c, d, e, f, g, h, (s2, s1, s0)), &out)| {
                assert_eq!(mux8way16(a, b, c, d, e, f, g, h, *s2, *s1, *s0), out)
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
            [false; 8],
            [true, false, false, false, false, false, false, false],
            [false, true, false, false, false, false, false, false],
            [false, false, true, false, false, false, false, false],
            [false, false, false, true, false, false, false, false],
            [false, false, false, false, true, false, false, false],
            [false, false, false, false, false, true, false, false],
            [false, false, false, false, false, false, true, false],
            [false, false, false, false, false, false, false, true],
        ];

        inputs
            .iter()
            .zip(expected.iter())
            .for_each(|(&(input, s2, s1, s0), &expected)| {
                let output = dmux8way(input, s2, s1, s0);
                assert_eq!(output, expected);
            });
    }
}
