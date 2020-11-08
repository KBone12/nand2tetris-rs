use crate::chip::{
    basic::{and, mux16, not, or},
    mem::{Ram16k, Ram4k, Register},
};

pub struct Memory {
    address: [bool; 15],
    load: bool,
    input: [bool; 16],
    ram: Ram16k,
    screen: [Ram4k; 2],
    keyboard: Register,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            address: [false; 15],
            load: false,
            input: [false; 16],
            ram: Ram16k::new(),
            screen: [Ram4k::new(), Ram4k::new()],
            keyboard: Register::new(),
        }
    }

    pub fn get_output(&self) -> [bool; 16] {
        mux16(
            &self.ram.get_output(),
            &mux16(
                &mux16(
                    &self.screen[0].get_output(),
                    &self.screen[1].get_output(),
                    self.address[2],
                ),
                &self.keyboard.get_output(),
                self.address[1],
            ),
            self.address[0],
        )
    }

    pub fn tick(&mut self, address: &[bool; 15], load: bool, input: &[bool; 16]) {
        assert!(not(and(
            and(address[0], address[1]),
            or(
                or(
                    or(or(address[2], address[3]), or(address[4], address[5])),
                    or(or(address[6], address[7]), or(address[8], address[9]))
                ),
                or(
                    or(or(address[10], address[11]), or(address[12], address[13])),
                    address[14]
                )
            )
        )));
        self.ram.tick(
            &[
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
                address[14],
            ],
            and(not(address[0]), load),
            input,
        );
        self.screen[0].tick(
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
                address[12],
                address[13],
                address[14],
            ],
            and(and(and(address[0], not(address[1])), not(address[2])), load),
            input,
        );
        self.screen[1].tick(
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
                address[12],
                address[13],
                address[14],
            ],
            and(and(and(address[0], not(address[1])), address[2]), load),
            input,
        );
        self.keyboard
            .tick(and(and(address[0], address[1]), load), input);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memorys_can_read_and_write_data_with_clock() {
        let inputs = [
            ([false; 16], [false; 15]),
            (
                [false; 16],
                [
                    false, true, false, false, false, false, false, false, false, false, false,
                    false, false, false, false,
                ],
            ),
            (
                [false; 16],
                [
                    true, true, false, false, false, false, false, false, false, false, false,
                    false, false, false, false,
                ],
            ),
            ([true; 16], [false; 15]),
            (
                [true; 16],
                [
                    false, true, false, false, false, false, false, false, false, false, false,
                    false, false, false, false,
                ],
            ),
            (
                [true; 16],
                [
                    true, true, false, false, false, false, false, false, false, false, false,
                    false, false, false, false,
                ],
            ),
            ([false; 16], [false; 15]),
            (
                [false; 16],
                [
                    false, true, false, false, false, false, false, false, false, false, false,
                    false, false, false, false,
                ],
            ),
            (
                [false; 16],
                [
                    true, true, false, false, false, false, false, false, false, false, false,
                    false, false, false, false,
                ],
            ),
        ];
        let expected = [
            [false; 16],
            [false; 16],
            [false; 16],
            [true; 16],
            [true; 16],
            [true; 16],
            [false; 16],
            [false; 16],
            [false; 16],
        ];
        let mut mem = Memory::new();

        inputs
            .iter()
            .zip(expected.iter())
            .for_each(|((input, address), &output)| {
                mem.tick(address, true, input);
                assert_eq!(mem.get_output(), output);
            });
    }

    #[test]
    #[should_panic]
    fn memory_fail_to_access_0x6001() {
        let address = [
            true, true, false, false, false, false, false, false, false, false, false, false,
            false, false, true,
        ];
        let mut mem = Memory::new();

        mem.tick(&address, false, &[false; 16]);
    }
}