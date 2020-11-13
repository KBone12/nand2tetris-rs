use crate::chip::mem::Register;

pub mod winit;

pub trait Keyboard {
    type State;

    fn new() -> Self;
    fn get_output(&self) -> [bool; 16];
    fn set_state(&mut self, state: Self::State);
}

pub struct DummyKeyboard {
    register: Register,
}

impl Keyboard for DummyKeyboard {
    type State = ();

    fn new() -> Self {
        Self {
            register: Register::new(),
        }
    }

    fn get_output(&self) -> [bool; 16] {
        self.register.get_output()
    }

    fn set_state(&mut self, _state: Self::State) {}
}
