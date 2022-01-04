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

    pub fn exec_opcode(&mut self, opcode: u32) {
        self.registers.increment_program_counter(4);
        let bytes = opcode.to_be_bytes();
        let inst = bytes[0] >> 2;
        match inst {
            0b000000 => {
                let rd = (opcode >> 11) & 0b11111;
                let rs = (opcode >> 21) & 0b11111;
                let rt = (opcode >> 16) & 0b11111;
                self.add(rd as usize, rs as usize, rt as usize);
                if (opcode & 0b11111111111) == 0b00000100000 {
                    todo!("Throw exception for add overflow ADDU");
                }
            },
            0b001000 | 0b001001 => {
                let rt = (opcode >> 11) & 0b11111;
                let rs = (opcode >> 21) & 0b11111;
                let immediate = (opcode & 0xFFFF) as i16;
                self.addi(rt as usize, rs as usize, immediate);
                if inst == 0b001001 {
                    todo!("Throw exception for add overflow ADDIU");
                }
            },
            _ => unreachable!(),
        }
    }

    pub fn add(&mut self, rd: usize, rs: usize, rt: usize) {
        let s = self.registers.get_by_number(rs);
        let t = self.registers.get_by_number(rt);
        let result = s.wrapping_add(t);
        self.registers.set_by_number(rd, result);
    }

    pub fn addi(&mut self, rt: usize, rs: usize, immediate: i16) {
        let s = self.registers.get_by_number(rs);
        let result = s.wrapping_add(immediate as i64);
        self.registers.set_by_number(rt, result);
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

        cpu.registers.set_by_number(reg_s, 40);
        cpu.registers.set_by_number(reg_t, -80);
        cpu.add(reg_dest, reg_s, reg_t);
        assert_eq!(cpu.registers.get_by_number(reg_dest), -40);
    }

    #[test]
    fn test_addi() {
        let mut cpu = CPU::new();
        let reg_dest = 10;
        let reg_s = 15;
        cpu.registers.set_by_number(reg_s, 80);
        cpu.addi(reg_dest, reg_s, 80);
        assert_eq!(cpu.registers.get_by_number(reg_dest), 160);

        cpu.registers.set_by_number(reg_s, 80);
        cpu.addi(reg_dest, reg_s, -40);
        assert_eq!(cpu.registers.get_by_number(reg_dest), 40);
    }
}
