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

    pub fn tick(&mut self) {
        self.set_clock(true);
        self.set_clock(false);
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

    pub fn tick(&mut self) {
        self.set_clock(true);
        self.set_clock(false);
    }

    pub fn set_clock(&mut self, clock: bool) {
        self.dff.set_clock(clock);
    }
}

pub struct Register {
    bits: [Bit; 16],
}

impl Register {
    pub fn new() -> Self {
        Self {
            bits: [
                Bit::new(),
                Bit::new(),
                Bit::new(),
                Bit::new(),
                Bit::new(),
                Bit::new(),
                Bit::new(),
                Bit::new(),
                Bit::new(),
                Bit::new(),
                Bit::new(),
                Bit::new(),
                Bit::new(),
                Bit::new(),
                Bit::new(),
                Bit::new(),
            ],
        }
    }

    pub fn set_load(&mut self, load: bool) {
        for i in 0..16 {
            self.bits[i].set_load(load);
        }
    }

    pub fn set_input(&mut self, input: &[bool; 16]) {
        for i in 0..16 {
            self.bits[i].set_input(input[i]);
        }
    }

    pub fn get_output(&self) -> [bool; 16] {
        [
            self.bits[0].get_output(),
            self.bits[1].get_output(),
            self.bits[2].get_output(),
            self.bits[3].get_output(),
            self.bits[4].get_output(),
            self.bits[5].get_output(),
            self.bits[6].get_output(),
            self.bits[7].get_output(),
            self.bits[8].get_output(),
            self.bits[9].get_output(),
            self.bits[10].get_output(),
            self.bits[11].get_output(),
            self.bits[12].get_output(),
            self.bits[13].get_output(),
            self.bits[14].get_output(),
            self.bits[15].get_output(),
        ]
    }

    pub fn tick(&mut self) {
        self.set_clock(true);
        self.set_clock(false);
    }

    pub fn set_clock(&mut self, clock: bool) {
        for i in 0..16 {
            self.bits[i].set_clock(clock);
        }
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
                bit.tick();
                assert_eq!(bit.get_output(), output);
            });
    }

    #[test]
    fn register_changes_the_output_when_load_flag_is_on() {
        let inputs = [
            ([false; 16], false),
            ([false; 16], true),
            ([true; 16], false),
            ([true; 16], true),
            ([false; 16], false),
            ([false; 16], true),
        ];
        let expected = [
            [false; 16],
            [false; 16],
            [false; 16],
            [true; 16],
            [true; 16],
            [false; 16],
        ];
        let mut register = Register::new();

        inputs
            .iter()
            .zip(expected.iter())
            .for_each(|((input, load), &output)| {
                register.set_load(*load);
                register.set_input(input);
                register.tick();
                assert_eq!(register.get_output(), output);
            });
    }
}
