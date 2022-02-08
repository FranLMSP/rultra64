use crate::rdram::RDRAM;
use crate::utils::box_array;

pub struct VideoInterface {
    registers: Box<[u8; 0x100000]>,
}

impl VideoInterface {
    pub fn new() -> Self {
        let mut registers = box_array![0; 0x100000];
        // Initialize VI_V_INTR 0x0440 000C: https://n64brew.dev/wiki/Video_Interface#0x0440_000C_-_VI_V_INTR
        registers[0x0440000C - 0x04400000] = 0xFF;
        registers[0x0440000B - 0x04400000] = 0x03;
        // Initialize VI_BURST 0x0440 0014: https://n64brew.dev/wiki/Video_Interface#0x0440_0014_-_VI_BURST
        registers[0x04400014 - 0x04400000] = 0x01;
        // Initialize VI_H_SYNC 0x0440 001C: https://n64brew.dev/wiki/Video_Interface#0x0440_001C_-_VI_H_SYNC
        registers[0x0440001C - 0x04400000] = 0xFF;
        registers[0x0440001B - 0x04400000] = 0x07;
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

    /*
        RDRAM base address of the video output Frame Buffer. This can be changed as needed to implement double or triple buffering. 
        https://n64brew.dev/wiki/Video_Interface#0x0440_0004_-_VI_ORIGIN
    */
    pub fn get_vi_origin(&self) -> u32 {
        ((self.get_register(0x04400005) as u32) << 16) | ((self.get_register(0x04400006) as u32) << 8) | (self.get_register(0x04400007) as u32)
    }

    /*
        This is the width in pixels of the frame buffer if you draw to the frame buffer based on a different width than what
        is given here the image will drift with each line to the left or right. The common values are 320 and 640,
        the maximum value is 640. The minimum value depends on the TV set, 160 would probably be a safe minimum but no guarantee.
        The same value would also be used on drawing commands for clipping or scissors.
        This can also be used with High Res interlacing modes to change the odd and even lines of the frame buffer to be drawn
        to screen by doubling the width of this value and changing the VI_ORIGIN register to the odd or even field being displayed.
        RDRAM base address of the video output Frame Buffer. This can be changed as needed to implement double or triple buffering. 
        https://n64brew.dev/wiki/Video_Interface#0x0440_0008_-_VI_WIDTH
    */
    pub fn get_vi_width(&self) -> u16 {
        (((self.get_register(0x04400010) as u16) << 8) & 0b1111) | (self.get_register(0x04400011) as u16)
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

    pub fn copy_framebuffer(&self, rdram: &RDRAM, dest: &mut [u8]) {
        let mut addr = self.video_interface.get_vi_origin() as i64;
        for elem in dest {
            *elem = rdram.read8(addr);
            addr += 1;
        }
    }
}