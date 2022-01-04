use crate::registers::{CPURegisters};

pub struct CPU {
    registers: CPURegisters,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            registers: CPURegisters::new(),
        }
    }

    pub fn add(&mut self, destination: usize, reg_s: usize, reg_t: usize) {
        let s = self.registers.get_by_number(reg_s);
        let t = self.registers.get_by_number(reg_t);
        let result = s.wrapping_add(t);
        self.registers.set_by_number(destination, result);
    }
}

#[cfg(test)]
mod cpu_instructions_tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut cpu = CPU::new();
        let reg_dest = 10;
        let reg_s = 15;
        let reg_t = 20;
        cpu.registers.set_by_number(reg_s, 80);
        cpu.registers.set_by_number(reg_t, 80);
        cpu.add(reg_dest, reg_s, reg_t);
        assert_eq!(cpu.registers.get_by_number(reg_dest), 160);
    }

}
