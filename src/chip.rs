pub fn nand(a: bool, b: bool) -> bool {
    !(a && b)
}

pub fn not(input: bool) -> bool {
    nand(input, input)
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
}
