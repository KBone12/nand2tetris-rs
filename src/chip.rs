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

pub fn xor(a: bool, b: bool) -> bool {
    nand(nand(a, not(b)), nand(not(a), b))
}

pub fn mux(a: bool, b: bool, selector: bool) -> bool {
    or(and(not(selector), a), and(selector, b))
}

pub fn dmux(input: bool, selector: bool) -> (bool, bool) {
    (and(input, not(selector)), and(input, selector))
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
}
