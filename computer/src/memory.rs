use crate::{
    chip::{
        basic::{and, mux16, not, or},
        mem::Ram16k,
    },
    keyboard::Keyboard,
    screen::Screen,
};

pub struct Memory<S: Screen, K: Keyboard> {
    address: [bool; 15],
    ram: Ram16k,
    screen: S,
    keyboard: K,
}

impl<S: Screen, K: Keyboard> Memory<S, K> {
    pub fn new() -> Self {
        Self {
            address: [false; 15],
            ram: Ram16k::new(),
            screen: S::new(),
            keyboard: K::new(),
        }
    }

    pub fn get_output(&self) -> [bool; 16] {
        mux16(
            &self.ram.get_output(),
            &mux16(
                &self.screen.get_output(),
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
        self.address = *address;
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
        self.screen.tick(
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
                address[14],
            ],
            and(and(address[0], not(address[1])), load),
            input,
        );
    }

    pub fn set_keystate(&mut self, state: K::State) {
        self.keyboard.set_state(state);
    }

    pub fn screen(&self) -> &S {
        &self.screen
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{keyboard::DummyKeyboard, screen::DummyScreen};

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
                    true, false, true, false, false, false, false, false, false, false, false,
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
        let mut mem = Memory::<DummyScreen, DummyKeyboard>::new();

        inputs
            .iter()
            .zip(expected.iter())
            .for_each(|((input, address), &output)| {
                mem.tick(address, true, input);
                assert_eq!(
                    mem.get_output(),
                    output,
                    "input: {:?}, address: {:?}",
                    input,
                    address
                );
            });
    }

    #[test]
    #[should_panic]
    fn memory_fail_to_access_0x6001() {
        let address = [
            true, true, false, false, false, false, false, false, false, false, false, false,
            false, false, true,
        ];
        let mut mem = Memory::<DummyScreen, DummyKeyboard>::new();

        mem.tick(&address, false, &[false; 16]);
    }
}
