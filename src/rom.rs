pub struct Rom {
    data: [[bool; 16]; 32768],
    address: usize,
}

impl Rom {
    pub fn new() -> Self {
        Self {
            data: [[false; 16]; 32768],
            address: 0,
        }
    }

    pub fn set_address(&mut self, address: &[bool; 15]) {
        self.address = (address[0] as usize) << 14
            | (address[2] as usize) << 13
            | (address[3] as usize) << 12
            | (address[4] as usize) << 11
            | (address[5] as usize) << 10
            | (address[6] as usize) << 9
            | (address[7] as usize) << 8
            | (address[8] as usize) << 7
            | (address[9] as usize) << 6
            | (address[10] as usize) << 5
            | (address[11] as usize) << 4
            | (address[12] as usize) << 3
            | (address[13] as usize) << 2
            | (address[14] as usize) << 1
            | address[15] as usize;
    }

    pub fn get_output(&self) -> [bool; 16] {
        self.data[self.address]
    }
}
