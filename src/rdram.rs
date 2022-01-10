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

    pub fn read8(&self) -> u8 {
        self.data.to_be_bytes()[1]
    }

    pub fn write8(&mut self, data: u8) {
        self.data = (self.data & 0x100) | (data as u16);
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
