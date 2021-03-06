use crate::{
    chip::{
        basic::{and, mux, not},
        mem::Ram16k,
    },
    keyboard::Keyboard,
    screen::Screen,
    signal::Word,
};

pub struct Memory<S: Screen, K: Keyboard> {
    address: [bool; 15],
    ram: Ram16k<Word>,
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

    pub fn get_output(&self) -> Word {
        mux(
            self.ram.get_output(),
            mux(
                self.screen.get_output(),
                self.keyboard.get_output(),
                self.address[1],
            ),
            self.address[0],
        )
    }

    pub fn tick(&mut self, address: &[bool; 15], load: bool, input: Word) {
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
            .for_each(|(&(input, address), &output)| {
                mem.tick(&address, true, Word::from(input));
                assert_eq!(mem.get_output(), Word::from(output));
            });
    }
}
