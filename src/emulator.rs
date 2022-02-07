use crate::mmu::MMU;
use crate::cpu::CPU;

pub struct Emulator {
    cpu: CPU,
    mmu: MMU,
}

impl Emulator {
    pub fn new() -> Self {
        Self {
            cpu: CPU::new(),
            mmu: MMU::new(),
        }
    }

    pub fn new_hle() -> Self {
        Self {
            cpu: CPU::new_hle(),
            mmu: MMU::new_hle(),
        }
    }

    pub fn tick(&mut self) {
        self.cpu.fetch_and_exec_opcode(&mut self.mmu);
    }

    pub fn cpu(&self) -> &CPU {
        &self.cpu
    }
}