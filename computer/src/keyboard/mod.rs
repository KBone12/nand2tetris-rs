pub mod winit;

pub trait Keyboard {
    type State;

    fn new() -> Self;
    fn get_output(&self) -> [bool; 16];
    fn set_state(&mut self, state: Self::State);
}
