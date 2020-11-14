use crate::{chip::mem::Register, signal::Word};

pub mod winit;

pub trait Keyboard {
    type State;

    fn new() -> Self;
    fn get_output(&self) -> Word;
    fn set_state(&mut self, state: Self::State);
}

pub struct DummyKeyboard {
    register: Register<Word>,
}

impl Keyboard for DummyKeyboard {
    type State = ();

    fn new() -> Self {
        Self {
            register: Register::new(),
        }
    }

    fn get_output(&self) -> Word {
        self.register.get_output()
    }

    fn set_state(&mut self, _state: Self::State) {}
}
