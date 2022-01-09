#[derive(Copy, Clone)]
pub struct Byte {
    data: u16,
}

impl Byte {
    pub fn read(&self) -> u16 {
        self.data & 0x1FF
    }

    pub fn write(&mut self, data: u16) {
        self.data = data & 0x1FF;
    }
}

pub struct RDRAM {
    data: [Byte; 0x400000],
}

impl RDRAM {
    pub fn read(&self, address: i64) -> u16 {
        self.data[address as usize].read()
    }

    pub fn write(&mut self, address: i64, data: u16) {
        self.data[address as usize].write(data);
    }
}
