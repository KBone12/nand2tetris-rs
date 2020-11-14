use std::{
    convert::TryInto,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use crate::signal::{Signal, Word};

pub struct Rom {
    data: [Word; 32768],
    address: usize,
}

impl Rom {
    pub fn new() -> Self {
        Self {
            data: [Word::zero(); 32768],
            address: 0,
        }
    }

    pub fn from_binary<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let reader = BufReader::new(File::open(path)?);
        let mut bits = reader
            .lines()
            .filter_map(|line| line.ok())
            .map(|line| Word::from(u16::from_str_radix(line.trim(), 2).unwrap()))
            .collect::<Vec<_>>();
        bits.extend_from_slice(&vec![Word::zero(); 32768 - bits.len()]);
        let bits = bits.as_slice().try_into().unwrap();
        Ok(Self {
            data: bits,
            address: 0,
        })
    }

    pub fn set_address(&mut self, address: &[bool; 15]) {
        self.address = (address[0] as usize) << 14
            | (address[1] as usize) << 13
            | (address[2] as usize) << 12
            | (address[3] as usize) << 11
            | (address[4] as usize) << 10
            | (address[5] as usize) << 9
            | (address[6] as usize) << 8
            | (address[7] as usize) << 7
            | (address[8] as usize) << 6
            | (address[9] as usize) << 5
            | (address[10] as usize) << 4
            | (address[11] as usize) << 3
            | (address[12] as usize) << 2
            | (address[13] as usize) << 1
            | address[14] as usize;
    }

    pub fn get_output(&self) -> Word {
        self.data[self.address]
    }
}
