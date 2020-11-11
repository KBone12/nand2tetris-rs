#![allow(dead_code)]

use crate::chip::{
    arith::inc16,
    basic::{mux, mux16, or},
};

pub struct Dff {
    output: bool,
}

impl Dff {
    pub fn new() -> Self {
        Self { output: false }
    }

    pub fn get_output(&self) -> bool {
        self.output
    }

    pub fn tick(&mut self, input: bool) {
        self.output = input;
    }
}

pub struct Dff16 {
    output: [bool; 16],
}

impl Dff16 {
    pub fn new() -> Self {
        Self {
            output: [false; 16],
        }
    }

    pub fn get_output(&self) -> [bool; 16] {
        self.output
    }

    pub fn tick(&mut self, input: &[bool; 16]) {
        self.output = *input;
    }
}

pub struct Bit {
    dff: Dff,
}

impl Bit {
    pub fn new() -> Self {
        Self { dff: Dff::new() }
    }

    pub fn get_output(&self) -> bool {
        self.dff.get_output()
    }

    pub fn tick(&mut self, load: bool, input: bool) {
        self.dff.tick(mux(self.get_output(), input, load));
    }
}

pub struct Register {
    dff: Dff16,
}

impl Register {
    pub fn new() -> Self {
        Self { dff: Dff16::new() }
    }

    pub fn get_output(&self) -> [bool; 16] {
        self.dff.get_output()
    }

    pub fn tick(&mut self, load: bool, input: &[bool; 16]) {
        self.dff.tick(&mux16(&self.get_output(), input, load));
    }
}

pub struct Ram8 {
    registers: [Register; 8],
    address: usize,
}

impl Ram8 {
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

    pub fn get_output(&self) -> [bool; 16] {
        self.registers[self.address].get_output()
    }

    pub fn tick(&mut self, address: &[bool; 3], load: bool, input: &[bool; 16]) {
        self.address =
            (address[0] as usize) << 2 | (address[1] as usize) << 1 | address[2] as usize;
        self.registers[self.address].tick(load, input);
    }
}

pub struct Ram64 {
    rams: [Ram8; 8],
    address: usize,
}

impl Ram64 {
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

    pub fn get_output(&self) -> [bool; 16] {
        self.rams[self.address].get_output()
    }

    pub fn tick(&mut self, address: &[bool; 6], load: bool, input: &[bool; 16]) {
        self.address =
            (address[0] as usize) << 2 | (address[1] as usize) << 1 | address[2] as usize;
        let ad = &[address[3], address[4], address[5]];
        self.rams[self.address].tick(ad, load, input);
    }
}

pub struct Ram512 {
    rams: [Ram64; 8],
    address: usize,
}

impl Ram512 {
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

    pub fn get_output(&self) -> [bool; 16] {
        self.rams[self.address].get_output()
    }

    pub fn tick(&mut self, address: &[bool; 9], load: bool, input: &[bool; 16]) {
        self.address =
            (address[0] as usize) << 2 | (address[1] as usize) << 1 | address[2] as usize;
        let ad = &[
            address[3], address[4], address[5], address[6], address[7], address[8],
        ];
        self.rams[self.address].tick(ad, load, input);
    }
}

pub struct Ram4k {
    rams: [Ram512; 8],
    address: usize,
}

impl Ram4k {
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

    pub fn get_output(&self) -> [bool; 16] {
        self.rams[self.address].get_output()
    }

    pub fn tick(&mut self, address: &[bool; 12], load: bool, input: &[bool; 16]) {
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

pub struct Ram8k {
    rams: [Ram4k; 2],
    address: usize,
}

impl Ram8k {
    pub fn new() -> Self {
        Self {
            rams: [Ram4k::new(), Ram4k::new()],
            address: 0,
        }
    }

    pub fn get_output(&self) -> [bool; 16] {
        self.rams[self.address].get_output()
    }

    pub fn tick(&mut self, address: &[bool; 13], load: bool, input: &[bool; 16]) {
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

pub struct Ram16k {
    rams: [Ram8k; 2],
    address: usize,
}

impl Ram16k {
    pub fn new() -> Self {
        Self {
            rams: [Ram8k::new(), Ram8k::new()],
            address: 0,
        }
    }

    pub fn get_output(&self) -> [bool; 16] {
        self.rams[self.address].get_output()
    }

    pub fn tick(&mut self, address: &[bool; 14], load: bool, input: &[bool; 16]) {
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
    register: Register,
}

impl Pc {
    pub fn new() -> Self {
        Self {
            register: Register::new(),
        }
    }

    pub fn get_output(&self) -> [bool; 16] {
        self.register.get_output()
    }

    pub fn tick(&mut self, reset: bool, load: bool, increment: bool, input: &[bool; 16]) {
        self.register.tick(
            or(or(reset, load), increment),
            &mux16(
                &mux16(
                    &mux16(&self.get_output(), &inc16(&self.get_output()), increment),
                    input,
                    load,
                ),
                &[false; 16],
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
        let mut dff = Dff16::new();

        inputs
            .iter()
            .zip(expected.iter())
            .for_each(|(input, &(now, next))| {
                assert_eq!(dff.get_output(), now);
                dff.tick(input);
                assert_eq!(dff.get_output(), next);
            });
    }

    #[test]
    fn bit_changes_the_output_when_load_flag_is_on() {
        let inputs = [
            (false, false),
            (true, false),
            (false, true),
            (true, true),
            (false, false),
            (true, false),
        ];
        let expected = [false, false, false, true, true, false];
        let mut bit = Bit::new();

        inputs
            .iter()
            .zip(expected.iter())
            .for_each(|(&(load, input), &output)| {
                bit.tick(load, input);
                assert_eq!(bit.get_output(), output);
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
            .for_each(|((load, input), &output)| {
                register.tick(*load, input);
                assert_eq!(register.get_output(), output);
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
            .for_each(|((address, load, input), &output)| {
                ram.tick(address, *load, input);
                assert_eq!(ram.get_output(), output);
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
            .for_each(|((address, load, input), &output)| {
                ram.tick(address, *load, input);
                assert_eq!(ram.get_output(), output);
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
            .for_each(|((address, load, input), &output)| {
                ram.tick(address, *load, input);
                assert_eq!(ram.get_output(), output);
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
            .for_each(|((address, load, input), &output)| {
                ram.tick(address, *load, input);
                assert_eq!(ram.get_output(), output);
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
            .for_each(|((address, load, input), &output)| {
                ram.tick(address, *load, input);
                assert_eq!(ram.get_output(), output);
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
            .for_each(|((address, load, input), &output)| {
                ram.tick(address, *load, input);
                assert_eq!(ram.get_output(), output);
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
            |((reset, load, increment, input), &output)| {
                pc.tick(*reset, *load, *increment, input);
                assert_eq!(pc.get_output(), output);
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
            |((reset, load, increment, input), &output)| {
                pc.tick(*reset, *load, *increment, input);
                assert_eq!(pc.get_output(), output);
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
            |((reset, load, increment, input), &output)| {
                pc.tick(*reset, *load, *increment, input);
                assert_eq!(pc.get_output(), output);
            },
        );
    }
}
