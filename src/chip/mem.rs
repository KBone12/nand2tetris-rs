pub struct Dff {
    input: bool,
    output: bool,
    clock: bool,
}

impl Dff {
    pub fn new() -> Self {
        Self {
            input: false,
            output: false,
            clock: false,
        }
    }

    pub fn set_input(&mut self, input: bool) {
        self.input = input;
    }

    pub fn get_output(&self) -> bool {
        self.output
    }

    pub fn set_clock(&mut self, clock: bool) {
        if !self.clock && clock {
            self.output = self.input;
        }
        self.clock = clock;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dff_outputs_the_previous_input() {
        let inputs = [
            (false, false),
            (false, true),
            (false, false),
            (true, false),
            (true, true),
            (false, true),
            (false, false),
            (false, true),
        ];
        let expected = [false, false, false, false, true, true, true, false];
        let mut dff = Dff::new();

        inputs
            .iter()
            .zip(expected.iter())
            .for_each(|(&(input, clock), &output)| {
                dff.set_input(input);
                dff.set_clock(clock);
                assert_eq!(dff.get_output(), output);
            });
    }
}
