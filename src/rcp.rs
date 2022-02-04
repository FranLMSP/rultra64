pub struct VideoInterface {
    registers: [u8; 0x100000],
}

impl VideoInterface {
    pub fn new() -> Self {
        let mut registers = [0; 0x100000];
        // Initialize VI_V_INTR 0x0440 000C: https://n64brew.dev/wiki/Video_Interface#0x0440_000C_-_VI_V_INTR
        registers[0x0440000C] = 0xFF;
        registers[0x0440000B] = 0x03;
        // Initialize VI_BURST 0x0440 0014: https://n64brew.dev/wiki/Video_Interface#0x0440_0014_-_VI_BURST
        registers[0x04400014] = 0x01;
        // Initialize VI_H_SYNC 0x0440 001C: https://n64brew.dev/wiki/Video_Interface#0x0440_001C_-_VI_H_SYNC
        registers[0x0440001C] = 0xFF;
        registers[0x0440001B] = 0x07;
        Self {
            registers,
        }
    }

    pub fn get_register(&self, address: i64) -> u8 {
        self.registers[(address - 0x04400000) as usize]
    }

    pub fn set_register(&mut self, address: i64, data: u8) {
        self.registers[(address - 0x04400000) as usize] = data;
    }
}

pub struct RCP {
    pub video_interface: VideoInterface,
}

impl RCP {
    pub fn new() -> Self {
        Self {
            video_interface: VideoInterface::new(),
        }
    }
}