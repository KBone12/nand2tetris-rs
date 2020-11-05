use crate::chip::basic::mux;

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

pub struct Bit {
    load: bool,
    dff: Dff,
}

impl Bit {
    pub fn new() -> Self {
        Self {
            load: false,
            dff: Dff::new(),
        }
    }

    pub fn set_load(&mut self, load: bool) {
        self.load = load;
    }

    pub fn set_input(&mut self, input: bool) {
        self.dff.set_input(mux(self.get_output(), input, self.load));
    }

    pub fn get_output(&self) -> bool {
        self.dff.get_output()
    }

    pub fn set_clock(&mut self, clock: bool) {
        self.dff.set_clock(clock);
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

    #[test]
    fn bit_changes_the_output_when_load_flag_is_on() {
        let inputs = [
            (false, false),
            (false, true),
            (true, false),
            (true, true),
            (false, false),
            (false, true),
        ];
        let expected = [false, false, false, true, true, false];
        let mut bit = Bit::new();

        inputs
            .iter()
            .zip(expected.iter())
            .for_each(|(&(input, load), &output)| {
                bit.set_load(load);
                bit.set_input(input);
                bit.set_clock(true);
                bit.set_clock(false);
                assert_eq!(bit.get_output(), output);
            });
    }
}
