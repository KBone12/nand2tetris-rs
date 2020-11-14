use crate::{
    chip::{
        arith::inc,
        basic::{mux, or},
    },
    signal::{Signal, Word},
};

pub struct Dff<S: Signal> {
    output: S,
}

impl<S: Signal> Dff<S> {
    pub fn new() -> Self {
        Self { output: S::zero() }
    }

    pub fn get_output(&self) -> S {
        self.output
    }

    pub fn tick(&mut self, input: S) {
        self.output = input;
    }
}

pub struct Register<S: Signal> {
    dff: Dff<S>,
}

impl<S: Signal> Register<S> {
    pub fn new() -> Self {
        Self { dff: Dff::new() }
    }

    pub fn get_output(&self) -> S {
        self.dff.get_output()
    }

    pub fn tick(&mut self, load: bool, input: S) {
        self.dff.tick(mux(self.get_output(), input, load));
    }
}

pub struct Ram8<S: Signal> {
    registers: [Register<S>; 8],
    address: usize,
}

impl<S: Signal> Ram8<S> {
    pub fn new() -> Self {
        Self {
            registers: [
                Register::new(),
                Register::new(),
                Register::new(),
                Register::new(),
                Register::new(),
                Register::new(),
                Register::new(),
                Register::new(),
            ],
            address: 0,
        }
    }

    pub fn get_output(&self) -> S {
        self.registers[self.address].get_output()
    }

    pub fn tick(&mut self, address: &[bool; 3], load: bool, input: S) {
        self.address =
            (address[0] as usize) << 2 | (address[1] as usize) << 1 | address[2] as usize;
        self.registers[self.address].tick(load, input);
    }
}

pub struct Ram64<S: Signal> {
    rams: [Ram8<S>; 8],
    address: usize,
}

impl<S: Signal> Ram64<S> {
    pub fn new() -> Self {
        Self {
            rams: [
                Ram8::new(),
                Ram8::new(),
                Ram8::new(),
                Ram8::new(),
                Ram8::new(),
                Ram8::new(),
                Ram8::new(),
                Ram8::new(),
            ],
            address: 0,
        }
    }

    pub fn get_output(&self) -> S {
        self.rams[self.address].get_output()
    }

    pub fn tick(&mut self, address: &[bool; 6], load: bool, input: S) {
        self.address =
            (address[0] as usize) << 2 | (address[1] as usize) << 1 | address[2] as usize;
        let ad = &[address[3], address[4], address[5]];
        self.rams[self.address].tick(ad, load, input);
    }
}

pub struct Ram512<S: Signal> {
    rams: [Ram64<S>; 8],
    address: usize,
}

impl<S: Signal> Ram512<S> {
    pub fn new() -> Self {
        Self {
            rams: [
                Ram64::new(),
                Ram64::new(),
                Ram64::new(),
                Ram64::new(),
                Ram64::new(),
                Ram64::new(),
                Ram64::new(),
                Ram64::new(),
            ],
            address: 0,
        }
    }

    pub fn get_output(&self) -> S {
        self.rams[self.address].get_output()
    }

    pub fn tick(&mut self, address: &[bool; 9], load: bool, input: S) {
        self.address =
            (address[0] as usize) << 2 | (address[1] as usize) << 1 | address[2] as usize;
        let ad = &[
            address[3], address[4], address[5], address[6], address[7], address[8],
        ];
        self.rams[self.address].tick(ad, load, input);
    }
}

pub struct Ram4k<S: Signal> {
    rams: [Ram512<S>; 8],
    address: usize,
}

impl<S: Signal> Ram4k<S> {
    pub fn new() -> Self {
        Self {
            rams: [
                Ram512::new(),
                Ram512::new(),
                Ram512::new(),
                Ram512::new(),
                Ram512::new(),
                Ram512::new(),
                Ram512::new(),
                Ram512::new(),
            ],
            address: 0,
        }
    }

    pub fn get_output(&self) -> S {
        self.rams[self.address].get_output()
    }

    pub fn tick(&mut self, address: &[bool; 12], load: bool, input: S) {
        self.address =
            (address[0] as usize) << 2 | (address[1] as usize) << 1 | address[2] as usize;
        let ad = &[
            address[3],
            address[4],
            address[5],
            address[6],
            address[7],
            address[8],
            address[9],
            address[10],
            address[11],
        ];
        self.rams[self.address].tick(ad, load, input);
    }
}

pub struct Ram8k<S: Signal> {
    rams: [Ram4k<S>; 2],
    address: usize,
}

impl<S: Signal> Ram8k<S> {
    pub fn new() -> Self {
        Self {
            rams: [Ram4k::new(), Ram4k::new()],
            address: 0,
        }
    }

    pub fn get_output(&self) -> S {
        self.rams[self.address].get_output()
    }

    pub fn tick(&mut self, address: &[bool; 13], load: bool, input: S) {
        self.address = address[0] as usize;
        let ad = &[
            address[1],
            address[2],
            address[3],
            address[4],
            address[5],
            address[6],
            address[7],
            address[8],
            address[9],
            address[10],
            address[11],
            address[12],
        ];
        self.rams[self.address].tick(ad, load, input);
    }
}

pub struct Ram16k<S: Signal> {
    rams: [Ram8k<S>; 2],
    address: usize,
}

impl<S: Signal> Ram16k<S> {
    pub fn new() -> Self {
        Self {
            rams: [Ram8k::new(), Ram8k::new()],
            address: 0,
        }
    }

    pub fn get_output(&self) -> S {
        self.rams[self.address].get_output()
    }

    pub fn tick(&mut self, address: &[bool; 14], load: bool, input: S) {
        self.address = address[0] as usize;
        let ad = &[
            address[1],
            address[2],
            address[3],
            address[4],
            address[5],
            address[6],
            address[7],
            address[8],
            address[9],
            address[10],
            address[11],
            address[12],
            address[13],
        ];
        self.rams[self.address].tick(ad, load, input);
    }
}

pub struct Pc {
    register: Register<Word>,
}

impl Pc {
    pub fn new() -> Self {
        Self {
            register: Register::new(),
        }
    }

    pub fn get_output(&self) -> Word {
        self.register.get_output()
    }

    pub fn tick(&mut self, reset: bool, load: bool, increment: bool, input: Word) {
        self.register.tick(
            or(or(reset, load), increment),
            mux(
                mux(
                    mux(self.get_output(), inc(self.get_output()), increment),
                    input,
                    load,
                ),
                Word::zero(),
                reset,
            ),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dff_outputs_the_previous_input() {
        let inputs = [false, false, true, true, false];
        let expected = [
            (false, false),
            (false, false),
            (false, true),
            (true, true),
            (true, false),
        ];
        let mut dff = Dff::new();

        inputs
            .iter()
            .zip(expected.iter())
            .for_each(|(&input, &(now, next))| {
                assert_eq!(dff.get_output(), now);
                dff.tick(input);
                assert_eq!(dff.get_output(), next);
            });
    }

    #[test]
    fn dff16_outputs_the_previous_input() {
        let inputs = [
            [false; 16],
            [true; 16],
            [true; 16],
            [
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, true,
            ],
            [false; 16],
        ];
        let expected = [
            ([false; 16], [false; 16]),
            ([false; 16], [true; 16]),
            ([true; 16], [true; 16]),
            (
                [true; 16],
                [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, true,
                ],
            ),
            (
                [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, true,
                ],
                [false; 16],
            ),
        ];
        let mut dff = Dff::new();

        inputs
            .iter()
            .zip(expected.iter())
            .for_each(|(&input, &(now, next))| {
                assert_eq!(dff.get_output(), Word::from(now));
                dff.tick(Word::from(input));
                assert_eq!(dff.get_output(), Word::from(next));
            });
    }

    #[test]
    fn register_changes_the_output_when_load_flag_is_on() {
        let inputs = [
            (false, [false; 16]),
            (true, [false; 16]),
            (false, [true; 16]),
            (true, [true; 16]),
            (false, [false; 16]),
            (true, [false; 16]),
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
            .for_each(|(&(load, input), &output)| {
                register.tick(load, Word::from(input));
                assert_eq!(register.get_output(), Word::from(output));
            });
    }

    #[test]
    fn ram8_has_8_registers() {
        let inputs = [
            ([false, false, false], false, [false; 16]),
            ([false, true, false], false, [false; 16]),
            ([true, false, true], false, [false; 16]),
            ([true, true, true], false, [false; 16]),
            ([false, false, false], true, [true; 16]),
            ([false, false, false], true, [false; 16]),
            ([false, true, false], true, [true; 16]),
            ([false, false, false], false, [true; 16]),
            ([true, true, true], true, [true; 16]),
        ];
        let expected = [
            [false; 16],
            [false; 16],
            [false; 16],
            [false; 16],
            [true; 16],
            [false; 16],
            [true; 16],
            [false; 16],
            [true; 16],
        ];
        let mut ram = Ram8::new();

        inputs
            .iter()
            .zip(expected.iter())
            .for_each(|(&(address, load, input), &output)| {
                ram.tick(&address, load, Word::from(input));
                assert_eq!(ram.get_output(), Word::from(output));
            });
    }

    #[test]
    fn ram64_has_64_registers() {
        let inputs = [
            ([false; 6], false, [false; 16]),
            ([true; 6], false, [false; 16]),
            ([false; 6], true, [true; 16]),
            ([false; 6], true, [false; 16]),
            ([true; 6], true, [true; 16]),
            ([false; 6], false, [true; 16]),
        ];
        let expected = [
            [false; 16],
            [false; 16],
            [true; 16],
            [false; 16],
            [true; 16],
            [false; 16],
        ];
        let mut ram = Ram64::new();

        inputs
            .iter()
            .zip(expected.iter())
            .for_each(|(&(address, load, input), &output)| {
                ram.tick(&address, load, Word::from(input));
                assert_eq!(ram.get_output(), Word::from(output));
            });
    }

    #[test]
    fn ram512_has_512_registers() {
        let inputs = [
            ([false; 9], false, [false; 16]),
            ([true; 9], false, [false; 16]),
            ([false; 9], true, [true; 16]),
            ([false; 9], true, [false; 16]),
            ([true; 9], true, [true; 16]),
            ([false; 9], false, [true; 16]),
        ];
        let expected = [
            [false; 16],
            [false; 16],
            [true; 16],
            [false; 16],
            [true; 16],
            [false; 16],
        ];
        let mut ram = Ram512::new();

        inputs
            .iter()
            .zip(expected.iter())
            .for_each(|(&(address, load, input), &output)| {
                ram.tick(&address, load, Word::from(input));
                assert_eq!(ram.get_output(), Word::from(output));
            });
    }

    #[test]
    fn ram4k_has_4096_registers() {
        let inputs = [
            ([false; 12], false, [false; 16]),
            ([true; 12], false, [false; 16]),
            ([false; 12], true, [true; 16]),
            ([false; 12], true, [false; 16]),
            ([true; 12], true, [true; 16]),
            ([false; 12], false, [true; 16]),
        ];
        let expected = [
            [false; 16],
            [false; 16],
            [true; 16],
            [false; 16],
            [true; 16],
            [false; 16],
        ];
        let mut ram = Ram4k::new();

        inputs
            .iter()
            .zip(expected.iter())
            .for_each(|(&(address, load, input), &output)| {
                ram.tick(&address, load, Word::from(input));
                assert_eq!(ram.get_output(), Word::from(output));
            });
    }

    #[test]
    fn ram8k_has_4096_registers() {
        let inputs = [
            ([false; 13], false, [false; 16]),
            ([true; 13], false, [false; 16]),
            ([false; 13], true, [true; 16]),
            ([false; 13], true, [false; 16]),
            ([true; 13], true, [true; 16]),
            ([false; 13], false, [true; 16]),
        ];
        let expected = [
            [false; 16],
            [false; 16],
            [true; 16],
            [false; 16],
            [true; 16],
            [false; 16],
        ];
        let mut ram = Ram8k::new();

        inputs
            .iter()
            .zip(expected.iter())
            .for_each(|(&(address, load, input), &output)| {
                ram.tick(&address, load, Word::from(input));
                assert_eq!(ram.get_output(), Word::from(output));
            });
    }

    #[test]
    fn ram16k_has_16384_registers() {
        let inputs = [
            ([false; 14], false, [false; 16]),
            ([true; 14], false, [false; 16]),
            ([false; 14], true, [true; 16]),
            ([false; 14], true, [false; 16]),
            ([true; 14], true, [true; 16]),
            ([false; 14], false, [true; 16]),
        ];
        let expected = [
            [false; 16],
            [false; 16],
            [true; 16],
            [false; 16],
            [true; 16],
            [false; 16],
        ];
        let mut ram = Ram16k::new();

        inputs
            .iter()
            .zip(expected.iter())
            .for_each(|(&(address, load, input), &output)| {
                ram.tick(&address, load, Word::from(input));
                assert_eq!(ram.get_output(), Word::from(output));
            });
    }

    #[test]
    fn pc_increments_the_output() {
        let inputs = [
            (true, false, false, [false; 16]),
            (false, false, true, [false; 16]),
            (false, false, true, [false; 16]),
            (false, false, true, [false; 16]),
        ];
        let expected = [
            [false; 16],
            [
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, true,
            ],
            [
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, true, false,
            ],
            [
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, true, true,
            ],
        ];
        let mut pc = Pc::new();

        inputs.iter().zip(expected.iter()).for_each(
            |(&(reset, load, increment, input), &output)| {
                pc.tick(reset, load, increment, Word::from(input));
                assert_eq!(pc.get_output(), Word::from(output));
            },
        );
    }

    #[test]
    fn pc_resets_the_output_with_reset_flag() {
        let inputs = [
            (true, false, false, [false; 16]),
            (false, false, true, [false; 16]),
            (true, false, true, [false; 16]),
        ];
        let expected = [
            [false; 16],
            [
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, true,
            ],
            [false; 16],
        ];
        let mut pc = Pc::new();

        inputs.iter().zip(expected.iter()).for_each(
            |(&(reset, load, increment, input), &output)| {
                pc.tick(reset, load, increment, Word::from(input));
                assert_eq!(pc.get_output(), Word::from(output));
            },
        );
    }

    #[test]
    fn pc_loads_the_input_with_load_flag() {
        let inputs = [
            (true, false, false, [false; 16]),
            (false, false, true, [false; 16]),
            (
                false,
                true,
                false,
                [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, true, false, false, false,
                ],
            ),
            (false, false, false, [false; 16]),
            (false, false, true, [false; 16]),
            (true, true, true, [true; 16]),
        ];
        let expected = [
            [false; 16],
            [
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, true,
            ],
            [
                false, false, false, false, false, false, false, false, false, false, false, false,
                true, false, false, false,
            ],
            [
                false, false, false, false, false, false, false, false, false, false, false, false,
                true, false, false, false,
            ],
            [
                false, false, false, false, false, false, false, false, false, false, false, false,
                true, false, false, true,
            ],
            [false; 16],
        ];
        let mut pc = Pc::new();

        inputs.iter().zip(expected.iter()).for_each(
            |(&(reset, load, increment, input), &output)| {
                pc.tick(reset, load, increment, Word::from(input));
                assert_eq!(pc.get_output(), Word::from(output));
            },
        );
    }
}
