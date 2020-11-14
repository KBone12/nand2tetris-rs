use crate::{chip::mem::Ram8k, signal::Word};

pub mod wgpu;

pub trait Screen {
    fn new() -> Self;
    fn get_output(&self) -> Word;
    fn tick(&mut self, address: &[bool; 13], load: bool, input: Word);
}

pub struct DummyScreen {
    ram: Ram8k<Word>,
}

impl Screen for DummyScreen {
    fn new() -> Self {
        Self { ram: Ram8k::new() }
    }

    fn get_output(&self) -> Word {
        self.ram.get_output()
    }

    fn tick(&mut self, address: &[bool; 13], load: bool, input: Word) {
        self.ram.tick(address, load, input);
    }
}
