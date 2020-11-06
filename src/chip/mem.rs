use crate::chip::{
    arith::inc16,
    basic::{mux, mux16},
};

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

pub struct Dff16 {
    input: [bool; 16],
    output: [bool; 16],
    clock: bool,
}

impl Dff16 {
    pub fn new() -> Self {
        Self {
            input: [false; 16],
            output: [false; 16],
            clock: false,
        }
    }

    pub fn set_input(&mut self, input: &[bool; 16]) {
        self.input = input.clone();
    }

    pub fn get_output(&self) -> [bool; 16] {
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
    load: bool,
    dff: Dff16,
}

impl Register {
    pub fn new() -> Self {
        Self {
            load: false,
            dff: Dff16::new(),
        }
    }

    pub fn set_load(&mut self, load: bool) {
        self.load = load;
    }

    pub fn set_input(&mut self, input: &[bool; 16]) {
        self.dff
            .set_input(&mux16(&self.get_output(), input, self.load));
    }

    pub fn get_output(&self) -> [bool; 16] {
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

    pub fn set_load(&mut self, load: bool) {
        for i in 0..8 {
            self.registers[i].set_load(load);
        }
    }

    pub fn set_input(&mut self, input: &[bool; 16], address: &[bool; 3]) {
        self.address =
            (address[0] as usize) << 2 | (address[1] as usize) << 1 | address[2] as usize;
        self.registers[self.address].set_input(input);
    }

    pub fn get_output(&self) -> [bool; 16] {
        self.registers[self.address].get_output()
    }

    pub fn tick(&mut self) {
        self.set_clock(true);
        self.set_clock(false);
    }

    pub fn set_clock(&mut self, clock: bool) {
        for i in 0..8 {
            self.registers[i].set_clock(clock);
        }
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

    pub fn set_load(&mut self, load: bool) {
        for i in 0..8 {
            self.rams[i].set_load(load);
        }
    }

    pub fn set_input(&mut self, input: &[bool; 16], address: &[bool; 6]) {
        self.address = (address[0] as usize) << 5
            | (address[1] as usize) << 4
            | (address[2] as usize) << 3
            | (address[3] as usize) << 2
            | (address[4] as usize) << 1
            | (address[5] as usize);
        self.rams[self.address >> 3].set_input(input, &[address[3], address[4], address[5]]);
    }

    pub fn get_output(&self) -> [bool; 16] {
        self.rams[self.address >> 3].get_output()
    }

    pub fn tick(&mut self) {
        self.set_clock(true);
        self.set_clock(false);
    }

    pub fn set_clock(&mut self, clock: bool) {
        for i in 0..8 {
            self.rams[i].set_clock(clock);
        }
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

    pub fn set_load(&mut self, load: bool) {
        for i in 0..8 {
            self.rams[i].set_load(load);
        }
    }

    pub fn set_input(&mut self, input: &[bool; 16], address: &[bool; 9]) {
        self.address = (address[0] as usize) << 8
            | (address[1] as usize) << 7
            | (address[2] as usize) << 6
            | (address[3] as usize) << 5
            | (address[4] as usize) << 4
            | (address[5] as usize) << 3
            | (address[6] as usize) << 2
            | (address[7] as usize) << 1
            | (address[8] as usize);
        self.rams[self.address >> 6].set_input(
            input,
            &[
                address[3], address[4], address[5], address[6], address[7], address[8],
            ],
        );
    }

    pub fn get_output(&self) -> [bool; 16] {
        self.rams[self.address >> 6].get_output()
    }

    pub fn tick(&mut self) {
        self.set_clock(true);
        self.set_clock(false);
    }

    pub fn set_clock(&mut self, clock: bool) {
        for i in 0..8 {
            self.rams[i].set_clock(clock);
        }
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

    pub fn set_load(&mut self, load: bool) {
        for i in 0..8 {
            self.rams[i].set_load(load);
        }
    }

    pub fn set_input(&mut self, input: &[bool; 16], address: &[bool; 12]) {
        self.address = (address[0] as usize) << 11
            | (address[1] as usize) << 10
            | (address[2] as usize) << 9
            | (address[3] as usize) << 8
            | (address[4] as usize) << 7
            | (address[5] as usize) << 6
            | (address[6] as usize) << 5
            | (address[7] as usize) << 4
            | (address[8] as usize) << 3
            | (address[9] as usize) << 2
            | (address[10] as usize) << 1
            | (address[11] as usize);
        self.rams[self.address >> 9].set_input(
            input,
            &[
                address[3],
                address[4],
                address[5],
                address[6],
                address[7],
                address[8],
                address[9],
                address[10],
                address[11],
            ],
        );
    }

    pub fn get_output(&self) -> [bool; 16] {
        self.rams[self.address >> 9].get_output()
    }

    pub fn tick(&mut self) {
        self.set_clock(true);
        self.set_clock(false);
    }

    pub fn set_clock(&mut self, clock: bool) {
        for i in 0..8 {
            self.rams[i].set_clock(clock);
        }
    }
}

pub struct Ram16k {
    rams: [Ram4k; 4],
    address: usize,
}

impl Ram16k {
    pub fn new() -> Self {
        Self {
            rams: [Ram4k::new(), Ram4k::new(), Ram4k::new(), Ram4k::new()],
            address: 0,
        }
    }

    pub fn set_load(&mut self, load: bool) {
        for i in 0..4 {
            self.rams[i].set_load(load);
        }
    }

    pub fn set_input(&mut self, input: &[bool; 16], address: &[bool; 14]) {
        self.address = (address[0] as usize) << 13
            | (address[1] as usize) << 12
            | (address[2] as usize) << 11
            | (address[3] as usize) << 10
            | (address[4] as usize) << 9
            | (address[5] as usize) << 8
            | (address[6] as usize) << 7
            | (address[7] as usize) << 6
            | (address[8] as usize) << 5
            | (address[9] as usize) << 4
            | (address[10] as usize) << 3
            | (address[11] as usize) << 2
            | (address[12] as usize) << 1
            | (address[13] as usize);
        self.rams[self.address >> 12].set_input(
            input,
            &[
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
            ],
        );
    }

    pub fn get_output(&self) -> [bool; 16] {
        self.rams[self.address >> 12].get_output()
    }

    pub fn tick(&mut self) {
        self.set_clock(true);
        self.set_clock(false);
    }

    pub fn set_clock(&mut self, clock: bool) {
        for i in 0..4 {
            self.rams[i].set_clock(clock);
        }
    }
}

pub struct Pc {
    register: Register,
    increment: bool,
    load: bool,
    reset: bool,
}

impl Pc {
    pub fn new() -> Self {
        let mut register = Register::new();
        register.set_load(true);
        Self {
            register,
            increment: false,
            load: false,
            reset: false,
        }
    }

    pub fn set_increment(&mut self, increment: bool) {
        self.increment = increment;
    }

    pub fn set_load(&mut self, load: bool) {
        self.load = load;
    }

    pub fn set_reset(&mut self, reset: bool) {
        self.reset = reset;
    }

    pub fn set_input(&mut self, input: &[bool; 16]) {
        self.register.set_input(&mux16(
            &mux16(
                &mux16(
                    &self.get_output(),
                    &inc16(&self.get_output()),
                    self.increment,
                ),
                input,
                self.load,
            ),
            &[false; 16],
            self.reset,
        ));
    }

    pub fn get_output(&self) -> [bool; 16] {
        self.register.get_output()
    }

    pub fn tick(&mut self) {
        self.set_clock(true);
        self.set_clock(false);
    }

    pub fn set_clock(&mut self, clock: bool) {
        self.register.set_clock(clock);
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
    fn dff16_outputs_the_previous_input() {
        let inputs = [
            ([false; 16], false),
            ([false; 16], true),
            ([false; 16], false),
            ([true; 16], false),
            ([true; 16], true),
            ([false; 16], true),
            ([false; 16], false),
            ([false; 16], true),
        ];
        let expected = [
            [false; 16],
            [false; 16],
            [false; 16],
            [false; 16],
            [true; 16],
            [true; 16],
            [true; 16],
            [false; 16],
        ];
        let mut dff = Dff16::new();

        inputs
            .iter()
            .zip(expected.iter())
            .for_each(|((input, clock), &output)| {
                dff.set_input(input);
                dff.set_clock(*clock);
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

    #[test]
    fn ram8_has_8_registers() {
        let inputs = [
            ([false; 16], [false; 3], false),
            ([true; 16], [false; 3], false),
            ([true; 16], [false; 3], true),
            ([true; 16], [true; 3], false),
            ([true; 16], [false; 3], false),
            ([true; 16], [true; 3], true),
            ([true; 16], [true; 3], false),
            ([false; 16], [false; 3], true),
            ([false; 16], [true; 3], false),
        ];
        let expected = [
            [false; 16],
            [false; 16],
            [true; 16],
            [false; 16],
            [true; 16],
            [true; 16],
            [true; 16],
            [false; 16],
            [true; 16],
        ];
        let mut ram = Ram8::new();

        inputs
            .iter()
            .zip(expected.iter())
            .for_each(|((input, address, load), &output)| {
                ram.set_load(*load);
                ram.set_input(input, address);
                ram.tick();
                assert_eq!(ram.get_output(), output);
            });
    }

    #[test]
    fn ram64_has_64_registers() {
        let inputs = [
            ([false; 16], [false; 6], false),
            ([true; 16], [false; 6], false),
            ([true; 16], [false; 6], true),
            ([true; 16], [true; 6], false),
            ([true; 16], [false; 6], false),
            ([true; 16], [true; 6], true),
            ([true; 16], [true; 6], false),
            ([false; 16], [false; 6], true),
            ([false; 16], [true; 6], false),
        ];
        let expected = [
            [false; 16],
            [false; 16],
            [true; 16],
            [false; 16],
            [true; 16],
            [true; 16],
            [true; 16],
            [false; 16],
            [true; 16],
        ];
        let mut ram = Ram64::new();

        inputs
            .iter()
            .zip(expected.iter())
            .for_each(|((input, address, load), &output)| {
                ram.set_load(*load);
                ram.set_input(input, address);
                ram.tick();
                assert_eq!(ram.get_output(), output);
            });
    }

    #[test]
    fn ram512_has_512_registers() {
        let inputs = [
            ([false; 16], [false; 9], false),
            ([true; 16], [false; 9], false),
            ([true; 16], [false; 9], true),
            ([true; 16], [true; 9], false),
            ([true; 16], [false; 9], false),
            ([true; 16], [true; 9], true),
            ([true; 16], [true; 9], false),
            ([false; 16], [false; 9], true),
            ([false; 16], [true; 9], false),
        ];
        let expected = [
            [false; 16],
            [false; 16],
            [true; 16],
            [false; 16],
            [true; 16],
            [true; 16],
            [true; 16],
            [false; 16],
            [true; 16],
        ];
        let mut ram = Ram512::new();

        inputs
            .iter()
            .zip(expected.iter())
            .for_each(|((input, address, load), &output)| {
                ram.set_load(*load);
                ram.set_input(input, address);
                ram.tick();
                assert_eq!(ram.get_output(), output);
            });
    }

    #[test]
    fn ram4k_has_4096_registers() {
        let inputs = [
            ([false; 16], [false; 12], false),
            ([true; 16], [false; 12], false),
            ([true; 16], [false; 12], true),
            ([true; 16], [true; 12], false),
            ([true; 16], [false; 12], false),
            ([true; 16], [true; 12], true),
            ([true; 16], [true; 12], false),
            ([false; 16], [false; 12], true),
            ([false; 16], [true; 12], false),
        ];
        let expected = [
            [false; 16],
            [false; 16],
            [true; 16],
            [false; 16],
            [true; 16],
            [true; 16],
            [true; 16],
            [false; 16],
            [true; 16],
        ];
        let mut ram = Ram4k::new();

        inputs
            .iter()
            .zip(expected.iter())
            .for_each(|((input, address, load), &output)| {
                ram.set_load(*load);
                ram.set_input(input, address);
                ram.tick();
                assert_eq!(ram.get_output(), output);
            });
    }

    #[test]
    fn ram16k_has_16384_registers() {
        let inputs = [
            ([false; 16], [false; 14], false),
            ([true; 16], [false; 14], false),
            ([true; 16], [false; 14], true),
            ([true; 16], [true; 14], false),
            ([true; 16], [false; 14], false),
            ([true; 16], [true; 14], true),
            ([true; 16], [true; 14], false),
            ([false; 16], [false; 14], true),
            ([false; 16], [true; 14], false),
        ];
        let expected = [
            [false; 16],
            [false; 16],
            [true; 16],
            [false; 16],
            [true; 16],
            [true; 16],
            [true; 16],
            [false; 16],
            [true; 16],
        ];
        let mut ram = Ram16k::new();

        inputs
            .iter()
            .zip(expected.iter())
            .for_each(|((input, address, load), &output)| {
                ram.set_load(*load);
                ram.set_input(input, address);
                ram.tick();
                assert_eq!(ram.get_output(), output);
            });
    }

    #[test]
    fn pc_increments_the_output() {
        let inputs = [
            ([false; 16], false, false, true),
            ([false; 16], true, false, false),
            ([false; 16], true, false, false),
            ([false; 16], true, false, false),
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

        inputs
            .iter()
            .zip(expected.iter())
            .for_each(|((input, inc, load, reset), &output)| {
                pc.set_reset(*reset);
                pc.set_load(*load);
                pc.set_increment(*inc);
                pc.set_input(input);
                pc.tick();
                assert_eq!(pc.get_output(), output);
            });
    }

    #[test]
    fn pc_resets_the_output_with_reset_flag() {
        let inputs = [
            ([false; 16], false, false, true),
            ([false; 16], true, false, false),
            ([false; 16], true, false, true),
            ([false; 16], true, false, false),
        ];
        let expected = [
            [false; 16],
            [
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, true,
            ],
            [false; 16],
            [
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, true,
            ],
        ];
        let mut pc = Pc::new();

        inputs
            .iter()
            .zip(expected.iter())
            .for_each(|((input, inc, load, reset), &output)| {
                pc.set_reset(*reset);
                pc.set_load(*load);
                pc.set_increment(*inc);
                pc.set_input(input);
                pc.tick();
                assert_eq!(pc.get_output(), output);
            });
    }

    #[test]
    fn pc_loads_the_input_with_load_flag() {
        let inputs = [
            ([false; 16], false, false, true),
            ([false; 16], true, false, false),
            (
                [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, true, false, false, false,
                ],
                false,
                true,
                false,
            ),
            ([false; 16], false, false, false),
            ([false; 16], true, false, false),
            ([true; 16], true, true, true),
            (
                [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, true, false, false, false,
                ],
                false,
                true,
                false,
            ),
            ([true; 16], true, true, false),
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
            [
                false, false, false, false, false, false, false, false, false, false, false, false,
                true, false, false, false,
            ],
            [true; 16],
        ];
        let mut pc = Pc::new();

        inputs
            .iter()
            .zip(expected.iter())
            .for_each(|((input, inc, load, reset), &output)| {
                pc.set_reset(*reset);
                pc.set_load(*load);
                pc.set_increment(*inc);
                pc.set_input(input);
                pc.tick();
                assert_eq!(pc.get_output(), output);
            });
    }
}
