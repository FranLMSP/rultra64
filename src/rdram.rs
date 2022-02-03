#[derive(Copy, Clone)]
pub struct Byte {
    data: u16,
}

impl Byte {
    pub fn new() -> Self {
        Self {
            data: 0,
        }
    }

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
    pub fn new() -> Self {
        Self {
            data: [Byte::new(); 0x400000],
        }
    }

    pub fn read(&self, address: i64) -> u16 {
        self.data[address as usize].read()
    }

    pub fn write(&mut self, address: i64, data: u16) {
        self.data[address as usize].write(data);
    }

    pub fn read8(&self, address: i64) -> u8 {
        self.data[address as usize].read8()
    }

    pub fn write8(&mut self, address: i64, data: u8) {
        self.data[address as usize].write8(data);
    }
}
