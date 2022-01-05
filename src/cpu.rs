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
                match opcode & 0b111_1111_1111 {
                    // ADD
                    0b000_0010_0000 => {
                        let rd = (opcode >> 11) & 0b11111;
                        let rs = (opcode >> 21) & 0b11111;
                        let rt = (opcode >> 16) & 0b11111;
                        let res = self.add(rd as usize, rs as usize, rt as usize);
                        if let Err(_) = res {
                            todo!("Throw exception for add overflow ADDU");
                        }
                    },
                    // ADDU
                    0b000_0010_0001 => {
                        let rd = (opcode >> 11) & 0b11111;
                        let rs = (opcode >> 21) & 0b11111;
                        let rt = (opcode >> 16) & 0b11111;
                        let _ = self.add(rd as usize, rs as usize, rt as usize);
                    },
                    // AND
                    0b000_0010_0100 => {
                        let rd = (opcode >> 11) & 0b11111;
                        let rs = (opcode >> 21) & 0b11111;
                        let rt = (opcode >> 16) & 0b11111;
                        self.and(rd as usize, rs as usize, rt as usize);
                    },
                    _ => unimplemented!(),
                };
            },
            // ADDI | ADDIU
            0b001000 | 0b001001 => {
                let rt = (opcode >> 11) & 0b11111;
                let rs = (opcode >> 21) & 0b11111;
                let immediate = (opcode & 0xFFFF) as i16;
                let res = self.addi(rt as usize, rs as usize, immediate);
                if inst == 0b001000 {
                    if let Err(_) = res {
                        todo!("Throw exception for add overflow ADDIU");
                    }
                }
            },
        // ANDI
        0b001100 => {
            let rt = (opcode >> 11) & 0b11111;
            let rs = (opcode >> 21) & 0b11111;
            let immediate = ((opcode & 0xFFFF) as u16) as i16;
            self.andi(rt as usize, rs as usize, immediate);
        },
            _ => unimplemented!(),
        }
    }

    pub fn add(&mut self, rd: usize, rs: usize, rt: usize) -> Result<i64, i64> {
        let s = self.registers.get_by_number(rs) as i32;
        let t = self.registers.get_by_number(rt) as i32;
        let result = s.wrapping_add(t) as i64;
        let will_overflow = s.checked_add(t);
        self.registers.set_by_number(rd, result);
        match will_overflow {
            Some(_) => Ok(result),
            None => Err(result),
        }
    }

    pub fn addi(&mut self, rt: usize, rs: usize, immediate: i16) -> Result<i64, i64> {
        let s = self.registers.get_by_number(rs) as i32;
        let immediate = immediate as i32;
        let result = s.wrapping_add(immediate) as i64;
        let will_overflow = s.checked_add(immediate);
        self.registers.set_by_number(rt, result);
        match will_overflow {
            Some(_) => Ok(result),
            None => Err(result),
        }
    }

    pub fn and(&mut self, rd: usize, rs: usize, rt: usize) {
        let s = self.registers.get_by_number(rs);
        let t = self.registers.get_by_number(rt);
        let result = s & t;
        self.registers.set_by_number(rd, result);
    }

    pub fn andi(&mut self, rt: usize, rs: usize, immediate: i16) {
        let s = self.registers.get_by_number(rs);
        let immediate = immediate as i64;
        let result = s & immediate;
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
        let _ = cpu.add(reg_dest, reg_s, reg_t);
        assert_eq!(cpu.registers.get_by_number(reg_dest), 160);

        cpu.registers.set_by_number(reg_s, 40);
        cpu.registers.set_by_number(reg_t, -80);
        let _ = cpu.add(reg_dest, reg_s, reg_t);
        assert_eq!(cpu.registers.get_by_number(reg_dest), -40);

        cpu.registers.set_by_number(reg_s, i32::MAX as i64);
        cpu.registers.set_by_number(reg_t, 1);
        let res = cpu.add(reg_dest, reg_s, reg_t);
        assert!(res.is_err());
        assert_eq!(cpu.registers.get_by_number(reg_dest) as i32, i32::MIN);
    }

    #[test]
    fn test_addi() {
        let mut cpu = CPU::new();
        let reg_dest = 10;
        let reg_s = 15;
        cpu.registers.set_by_number(reg_s, 80);
        let _ = cpu.addi(reg_dest, reg_s, 80);
        assert_eq!(cpu.registers.get_by_number(reg_dest), 160);

        cpu.registers.set_by_number(reg_s, 80);
        let _ = cpu.addi(reg_dest, reg_s, -40);
        assert_eq!(cpu.registers.get_by_number(reg_dest), 40);

        cpu.registers.set_by_number(reg_s, i32::MAX as i64);
        let res = cpu.addi(reg_dest, reg_s, 1);
        assert!(res.is_err());
        assert_eq!(cpu.registers.get_by_number(reg_dest) as i32, i32::MIN);
    }

    #[test]
    fn test_and() {
        let mut cpu = CPU::new();
        let reg_dest = 10;
        let reg_s = 15;
        let reg_t = 20;
        cpu.registers.set_by_number(reg_s, 123);
        cpu.registers.set_by_number(reg_t, 123);
        cpu.and(reg_dest, reg_s, reg_t);
        assert_eq!(cpu.registers.get_by_number(reg_dest), 123);

        cpu.registers.set_by_number(reg_s, 123);
        cpu.registers.set_by_number(reg_t, 321);
        cpu.and(reg_dest, reg_s, reg_t);
        assert_eq!(cpu.registers.get_by_number(reg_dest), 65);
    }

    #[test]
    fn test_andi() {
        let mut cpu = CPU::new();
        let reg_dest = 10;
        let reg_s = 15;
        cpu.registers.set_by_number(reg_s, 80);
        cpu.andi(reg_dest, reg_s, 80);
        assert_eq!(cpu.registers.get_by_number(reg_dest), 80);

        cpu.registers.set_by_number(reg_s, 123);
        cpu.andi(reg_dest, reg_s, 321);
        assert_eq!(cpu.registers.get_by_number(reg_dest), 65);
    }
}
