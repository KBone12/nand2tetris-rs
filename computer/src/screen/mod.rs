use crate::chip::mem::Ram8k;

pub mod wgpu;

pub trait Screen {
    fn new() -> Self;
    fn get_output(&self) -> [bool; 16];
    fn tick(&mut self, address: &[bool; 13], load: bool, input: &[bool; 16]);
}

pub struct DummyScreen {
    ram: Ram8k,
}

impl Screen for DummyScreen {
    fn new() -> Self {
        Self { ram: Ram8k::new() }
    }

    fn get_output(&self) -> [bool; 16] {
        self.ram.get_output()
    }

    fn tick(&mut self, address: &[bool; 13], load: bool, input: &[bool; 16]) {
        self.ram.tick(address, load, input);
    }
}
