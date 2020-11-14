use crate::{chip::mem::Ram8k, screen::Screen, signal::Word};

pub struct WgpuScreen {
    ram: Ram8k<Word>,
    colors: Vec<u8>,
}

impl WgpuScreen {
    pub fn colors(&self) -> &[u8] {
        &self.colors
    }
}

impl Screen for WgpuScreen {
    fn new() -> Self {
        Self {
            ram: Ram8k::new(),
            colors: vec![0.0f32.to_ne_bytes().to_vec(); 512 * 256]
                .into_iter()
                .flatten()
                .collect(),
        }
    }

    fn get_output(&self) -> Word {
        self.ram.get_output()
    }

    fn tick(&mut self, address: &[bool; 13], load: bool, input: Word) {
        self.ram.tick(address, load, input);

        if load {
            let address = (address[0] as usize) << 12
                | (address[1] as usize) << 11
                | (address[2] as usize) << 10
                | (address[3] as usize) << 9
                | (address[4] as usize) << 8
                | (address[5] as usize) << 7
                | (address[6] as usize) << 6
                | (address[7] as usize) << 5
                | (address[8] as usize) << 4
                | (address[9] as usize) << 3
                | (address[10] as usize) << 2
                | (address[11] as usize) << 1
                | address[12] as usize;
            let input = input.split();
            for i in 0..16 {
                let value = if input[i] { 1.0f32 } else { 0.0 }.to_ne_bytes();
                self.colors[address * 16 * 4 + i * 4 + 0] = value[0];
                self.colors[address * 16 * 4 + i * 4 + 1] = value[1];
                self.colors[address * 16 * 4 + i * 4 + 2] = value[2];
                self.colors[address * 16 * 4 + i * 4 + 3] = value[3];
            }
        }
    }
}
