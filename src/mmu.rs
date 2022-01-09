use std::ops::RangeInclusive;

pub const KUSEG: RangeInclusive<i64> = 0x00000000..=0x7FFFFFFF;
pub const KSEG0: RangeInclusive<i64> = 0x80000000..=0x9FFFFFFF;
pub const KSEG1: RangeInclusive<i64> = 0xA0000000..=0xBFFFFFFF;
pub const KSSEG: RangeInclusive<i64> = 0xC0000000..=0xDFFFFFFF;
pub const KSEG3: RangeInclusive<i64> = 0xE0000000..=0xFFFFFFFF;

pub const RDRAM1: RangeInclusive<i64>                       = 0x00000000..=0x003FFFFF;
pub const RDRAM2: RangeInclusive<i64>                       = 0x00400000..=0x007FFFFF;
pub const RESERVED1: RangeInclusive<i64>                    = 0x00800000..=0x03EFFFFF;
pub const RDRAM_REGISTERS: RangeInclusive<i64>              = 0x03F00000..=0x03FFFFFF;
pub const RSP_DMEM: RangeInclusive<i64>                     = 0x04000000..=0x04000FFF;
pub const RSP_IMEM: RangeInclusive<i64>                     = 0x04001000..=0x04001FFF;
pub const UNKNOWN: RangeInclusive<i64>                      = 0x04002000..=0x0403FFFF;
pub const RSP_REGISTERS: RangeInclusive<i64>                = 0x04040000..=0x040FFFFF;
pub const RDP_COMMAND_REGISTERS: RangeInclusive<i64>        = 0x04100000..=0x041FFFFF;
pub const RDP_SPAN_REGISTERS: RangeInclusive<i64>           = 0x04200000..=0x042FFFFF;
pub const MIPS_INTERFACE: RangeInclusive<i64>               = 0x04300000..=0x043FFFFF;
pub const VIDEO_INTERFACE: RangeInclusive<i64>              = 0x04400000..=0x044FFFFF;
pub const AUDIO_INTERFACE: RangeInclusive<i64>              = 0x04500000..=0x045FFFFF;
pub const PERIPHERAL_INTERFACE: RangeInclusive<i64>         = 0x04600000..=0x046FFFFF;
pub const RDRAM_INTERFACE: RangeInclusive<i64>              = 0x04700000..=0x047FFFFF;
pub const SERIAL_INTERFACE: RangeInclusive<i64>             = 0x04800000..=0x048FFFFF;
pub const UNUSED: RangeInclusive<i64>                       = 0x04900000..=0x04FFFFFF;
pub const CARTRIDGE_DOMAIN_2_ADDRESS_1: RangeInclusive<i64> = 0x05000000..=0x05FFFFFF;
pub const CARTRIDGE_DOMAIN_1_ADDRESS_1: RangeInclusive<i64> = 0x06000000..=0x07FFFFFF;
pub const CARTRIDGE_DOMAIN_2_ADDRESS_2: RangeInclusive<i64> = 0x08000000..=0x0FFFFFFF;
pub const CARTRIDGE_DOMAIN_1_ADDRESS_2: RangeInclusive<i64> = 0x10000000..=0x1FBFFFFF;
pub const PIF_ROM: RangeInclusive<i64>                      = 0x1FC00000..=0x1FC007BF;
pub const PIF_RAM: RangeInclusive<i64>                      = 0x1FC007C0..=0x1FC007FF;
pub const RESERVED2: RangeInclusive<i64>                    = 0x1FC00800..=0x1FCFFFFF;
pub const CARTRIDGE_DOMAIN_1_ADDRESS_3: RangeInclusive<i64> = 0x1FD00000..=0x7FFFFFFF;
pub const EXTERNAL_SYSAD_DEVICE_BUS: RangeInclusive<i64>    = 0x80000000..=0xFFFFFFFF;

pub struct MMU {
}

impl MMU {
    pub fn convert(address: i64) -> i64 {
        if KUSEG.contains(&address) {
            return address - KUSEG.min().unwrap();
        } else if KSEG0.contains(&address) {
            return address - KSEG0.min().unwrap();
        } else if KSEG1.contains(&address) {
            return address - KSEG1.min().unwrap();
        } else if KSSEG.contains(&address) {
            return address - KSSEG.min().unwrap();
        } else if KSEG3.contains(&address) {
            return address - KSEG3.min().unwrap();
        }
        unreachable!("Invalid virtual memory address {:08X}", address);
    }
}