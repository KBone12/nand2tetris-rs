pub fn nand(a: bool, b: bool) -> bool {
    !(a && b)
}

pub fn not(input: bool) -> bool {
    nand(input, input)
}

pub fn and(a: bool, b: bool) -> bool {
    not(nand(a, b))
}

pub fn or(a: bool, b: bool) -> bool {
    nand(not(a), not(b))
}

pub fn or8way(input: &[bool; 8]) -> bool {
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

pub fn xor(a: bool, b: bool) -> bool {
    nand(nand(a, not(b)), nand(not(a), b))
}

pub fn mux(a: bool, b: bool, selector: bool) -> bool {
    or(and(not(selector), a), and(selector, b))
}

pub fn dmux(input: bool, selector: bool) -> (bool, bool) {
    (and(input, not(selector)), and(input, selector))
}

pub fn dmux4way(input: bool, s1: bool, s0: bool) -> (bool, bool, bool, bool) {
    (
        and(input, and(not(s1), not(s0))),
        and(input, and(not(s1), s0)),
        and(input, and(s1, not(s0))),
        and(input, and(s1, s0)),
    )
}

pub fn dmux8way(input: bool, s2: bool, s1: bool, s0: bool) -> [bool; 8] {
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
    fn not_inverts_a_input() {
        let inputs = [false, true];
        let expected = [true, false];

        inputs
            .iter()
            .zip(expected.iter())
            .for_each(|(&input, &output)| assert_eq!(not(input), output));
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
    fn or_returns_true_except_both_inputs_are_false() {
        let inputs = [(false, false), (false, true), (true, false), (true, true)];
        let expected = [false, true, true, true];

        inputs
            .iter()
            .zip(expected.iter())
            .for_each(|(&(a, b), &out)| assert_eq!(or(a, b), out));
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
