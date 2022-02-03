use std::fs::File;
use std::io::Read;

use crate::mmu::CARTRIDGE_DOMAIN_2_ADDRESS_2;
use crate::mmu::CARTRIDGE_DOMAIN_1_ADDRESS_2;

pub struct ROM {
    data: Vec<u8>,
    ram: Vec<u8>,
}

impl ROM {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            ram: Vec::new(),
        }
    }

    pub fn load_file(filename: &str) -> std::io::Result<Self> {
        let mut file = File::open(filename)?; 
        let mut data = vec![];
        file.read_to_end(&mut data)?;
        Ok(Self {
            data,
            ram: vec![0; 0xFC00000],
        })
    }

    pub fn read(&self, address: i64) -> u8 {
        if CARTRIDGE_DOMAIN_2_ADDRESS_2.contains(&address) {
            return match self.ram.get((address - CARTRIDGE_DOMAIN_2_ADDRESS_2.min().unwrap()) as usize) {
                Some(byte) => *byte,
                None => 0xFF,
            };
        } else if CARTRIDGE_DOMAIN_1_ADDRESS_2.contains(&address) {
            return match self.data.get((address - CARTRIDGE_DOMAIN_1_ADDRESS_2.min().unwrap()) as usize) {
                Some(byte) => *byte,
                None => 0xFF,
            };
        }
        unreachable!("Invalid ROM access");
    }

    pub fn write(&mut self, address: i64, data: u8) {
        if let Some(elem) = self.ram.get_mut((address - CARTRIDGE_DOMAIN_2_ADDRESS_2.min().unwrap()) as usize) {
            *elem = data;
        }
    }
}