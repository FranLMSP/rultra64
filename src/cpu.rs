use crate::registers::{CPURegisters};

pub fn params_rd_rs_rt(opcode: u32) -> (usize, usize, usize) {
    let rd = (opcode >> 11) & 0b11111;
    let rs = (opcode >> 21) & 0b11111;
    let rt = (opcode >> 16) & 0b11111;
    (rd as usize, rs as usize, rt as usize)
}

pub fn params_rt_rs_immediate(opcode: u32) -> (usize, usize, i16) {
    let rt = (opcode >> 11) & 0b11111;
    let rs = (opcode >> 21) & 0b11111;
    let immediate = ((opcode & 0xFFFF) as u16) as i16;
    (rt as usize, rs as usize, immediate)
}

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
            0b0000_00 => {
                match opcode & 0b111_1111_1111 {
                    // ADD
                    0b000_0010_0000 => {
                        let (rd, rs, rt) = params_rd_rs_rt(opcode);
                        let res = self.add(rd, rs, rt);
                        if let Err(_) = res {
                            todo!("Throw exception for add overflow ADD");
                        }
                    },
                    // DADD
                    0b000_0010_1100 => {
                        let (rd, rs, rt) = params_rd_rs_rt(opcode);
                        let res = self.dadd(rd, rs, rt);
                        if let Err(_) = res {
                            todo!("Throw exception for add overflow DADD");
                        }
                    },
                    // DADDU
                    0b000_0010_1101 => {
                        let (rd, rs, rt) = params_rd_rs_rt(opcode);
                        let _ = self.dadd(rd, rs, rt);
                    },
                    // ADDU
                    0b000_0010_0001 => {
                        let (rd, rs, rt) = params_rd_rs_rt(opcode);
                        let _ = self.add(rd, rs, rt);
                    },
                    // SUB
                    0b000_0010_0010 => {
                        let (rd, rs, rt) = params_rd_rs_rt(opcode);
                        let res = self.sub(rd, rs, rt);
                        if let Err(_) = res {
                            todo!("Throw exception for sub overflow SUB");
                        }
                    },
                    // SUBU
                    0b000_0010_0011 => {
                        let (rd, rs, rt) = params_rd_rs_rt(opcode);
                        let _ = self.sub(rd, rs, rt);
                    },
                    // DSUB
                    0b000_0010_1110 => {
                        let (rd, rs, rt) = params_rd_rs_rt(opcode);
                        let res = self.dsub(rd, rs, rt);
                        if let Err(_) = res {
                            todo!("Throw exception for sub overflow DSUB");
                        }
                    },
                    // DSUBU
                    0b000_0010_1111 => {
                        let (rd, rs, rt) = params_rd_rs_rt(opcode);
                        let _ = self.dsub(rd, rs, rt);
                    },
                    // AND
                    0b000_0010_0100 => {
                        let (rd, rs, rt) = params_rd_rs_rt(opcode);
                        self.and(rd, rs, rt);
                    },
                    // OR
                    0b000_0010_0101 => {
                        let (rd, rs, rt) = params_rd_rs_rt(opcode);
                        self.or(rd, rs, rt);
                    },
                    // NOR
                    0b000_0010_0111 => {
                        let (rd, rs, rt) = params_rd_rs_rt(opcode);
                        self.nor(rd, rs, rt);
                    },
                    _ => unimplemented!(),
                };
            },
            // DADDI | DADDIU
            0b0110_00 | 0b0110_01 => {
                let (rt, rs, immediate) = params_rt_rs_immediate(opcode);
                let res = self.daddi(rt, rs, immediate);
                if inst == 0b0110_00 {
                    if let Err(_) = res {
                        todo!("Throw exception for add overflow DADDI");
                    }
                }
            },
            // ADDI | ADDIU
            0b0010_00 | 0b0010_01 => {
                let (rt, rs, immediate) = params_rt_rs_immediate(opcode);
                let res = self.addi(rt, rs, immediate);
                if inst == 0b0010_00 {
                    if let Err(_) = res {
                        todo!("Throw exception for add overflow ADDI");
                    }
                }
            },
            // ANDI
            0b0011_00 => {
                let (rt, rs, immediate) = params_rt_rs_immediate(opcode);
                self.andi(rt, rs, immediate);
            },
            // ORI
            0b0011_01 => {
                let (rt, rs, immediate) = params_rt_rs_immediate(opcode);
                self.ori(rt, rs, immediate);
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

    pub fn dadd(&mut self, rd: usize, rs: usize, rt: usize) -> Result<i64, i64> {
        let s = self.registers.get_by_number(rs);
        let t = self.registers.get_by_number(rt);
        let result = s.wrapping_add(t);
        let will_overflow = s.checked_add(t);
        self.registers.set_by_number(rd, result);
        match will_overflow {
            Some(_) => Ok(result),
            None => Err(result),
        }
    }

    pub fn daddi(&mut self, rt: usize, rs: usize, immediate: i16) -> Result<i64, i64> {
        let s = self.registers.get_by_number(rs);
        let immediate = immediate as i64;
        let result = s.wrapping_add(immediate);
        let will_overflow = s.checked_add(immediate);
        self.registers.set_by_number(rt, result);
        match will_overflow {
            Some(_) => Ok(result),
            None => Err(result),
        }
    }

    pub fn sub(&mut self, rd: usize, rs: usize, rt: usize) -> Result<i64, i64> {
        let s = self.registers.get_by_number(rs) as i32;
        let t = self.registers.get_by_number(rt) as i32;
        let result = s.wrapping_sub(t) as i64;
        let will_overflow = s.checked_sub(t);
        self.registers.set_by_number(rd, result);
        match will_overflow {
            Some(_) => Ok(result),
            None => Err(result),
        }
    }

    pub fn dsub(&mut self, rd: usize, rs: usize, rt: usize) -> Result<i64, i64> {
        let s = self.registers.get_by_number(rs);
        let t = self.registers.get_by_number(rt);
        let result = s.wrapping_sub(t);
        let will_overflow = s.checked_sub(t);
        self.registers.set_by_number(rd, result);
        match will_overflow {
            Some(_) => Ok(result),
            None => Err(result),
        }
    }

    pub fn and(&mut self, rd: usize, rs: usize, rt: usize) {
        let result = self.registers.get_by_number(rs) & self.registers.get_by_number(rt);
        self.registers.set_by_number(rd, result);
    }

    pub fn andi(&mut self, rt: usize, rs: usize, immediate: i16) {
        let s = self.registers.get_by_number(rs);
        let immediate = immediate as i64;
        let result = s & immediate;
        self.registers.set_by_number(rt, result);
    }

    pub fn or(&mut self, rd: usize, rs: usize, rt: usize) {
        let result = self.registers.get_by_number(rs) | self.registers.get_by_number(rt);
        self.registers.set_by_number(rd, result);
    }

    pub fn ori(&mut self, rt: usize, rs: usize, immediate: i16) {
        let s = self.registers.get_by_number(rs);
        let immediate = immediate as i64;
        let result = s | immediate;
        self.registers.set_by_number(rt, result);
    }

    pub fn xor(&mut self, rd: usize, rs: usize, rt: usize) {
        let result = self.registers.get_by_number(rs) ^ self.registers.get_by_number(rt);
        self.registers.set_by_number(rd, result);
    }

    pub fn xori(&mut self, rt: usize, rs: usize, immediate: i16) {
        let s = self.registers.get_by_number(rs);
        let immediate = immediate as i64;
        let result = s ^ immediate;
        self.registers.set_by_number(rt, result);
    }

    pub fn nor(&mut self, rd: usize, rs: usize, rt: usize) {
        let result = !(self.registers.get_by_number(rs) | self.registers.get_by_number(rt));
        self.registers.set_by_number(rd, result);
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
    fn test_dadd() {
        let mut cpu = CPU::new();
        let reg_dest = 10;
        let reg_s = 15;
        let reg_t = 20;
        cpu.registers.set_by_number(reg_s, 80);
        cpu.registers.set_by_number(reg_t, 80);
        let _ = cpu.dadd(reg_dest, reg_s, reg_t);
        assert_eq!(cpu.registers.get_by_number(reg_dest), 160);

        cpu.registers.set_by_number(reg_s, 40);
        cpu.registers.set_by_number(reg_t, -80);
        let _ = cpu.dadd(reg_dest, reg_s, reg_t);
        assert_eq!(cpu.registers.get_by_number(reg_dest), -40);

        cpu.registers.set_by_number(reg_s, i64::MAX);
        cpu.registers.set_by_number(reg_t, 1);
        let res = cpu.dadd(reg_dest, reg_s, reg_t);
        assert!(res.is_err());
        assert_eq!(cpu.registers.get_by_number(reg_dest), i64::MIN);
    }

    #[test]
    fn test_daddi() {
        let mut cpu = CPU::new();
        let reg_dest = 10;
        let reg_s = 15;
        cpu.registers.set_by_number(reg_s, 80);
        let _ = cpu.daddi(reg_dest, reg_s, 80);
        assert_eq!(cpu.registers.get_by_number(reg_dest), 160);

        cpu.registers.set_by_number(reg_s, 80);
        let _ = cpu.daddi(reg_dest, reg_s, -40);
        assert_eq!(cpu.registers.get_by_number(reg_dest), 40);

        cpu.registers.set_by_number(reg_s, i64::MAX);
        let res = cpu.daddi(reg_dest, reg_s, 1);
        assert!(res.is_err());
        assert_eq!(cpu.registers.get_by_number(reg_dest), i64::MIN);
    }

    #[test]
    fn test_sub() {
        let mut cpu = CPU::new();
        let reg_dest = 10;
        let reg_s = 15;
        let reg_t = 20;
        cpu.registers.set_by_number(reg_s, 80);
        cpu.registers.set_by_number(reg_t, 80);
        let _ = cpu.sub(reg_dest, reg_s, reg_t);
        assert_eq!(cpu.registers.get_by_number(reg_dest), 0);

        cpu.registers.set_by_number(reg_s, 40);
        cpu.registers.set_by_number(reg_t, -80);
        let _ = cpu.sub(reg_dest, reg_s, reg_t);
        assert_eq!(cpu.registers.get_by_number(reg_dest), 120);

        cpu.registers.set_by_number(reg_s, i32::MIN as i64);
        cpu.registers.set_by_number(reg_t, 1);
        let res = cpu.sub(reg_dest, reg_s, reg_t);
        assert!(res.is_err());
        assert_eq!(cpu.registers.get_by_number(reg_dest) as i32, i32::MAX);
    }

    #[test]
    fn test_dsub() {
        let mut cpu = CPU::new();
        let reg_dest = 10;
        let reg_s = 15;
        let reg_t = 20;
        cpu.registers.set_by_number(reg_s, 80);
        cpu.registers.set_by_number(reg_t, 80);
        let _ = cpu.dsub(reg_dest, reg_s, reg_t);
        assert_eq!(cpu.registers.get_by_number(reg_dest), 0);

        cpu.registers.set_by_number(reg_s, 40);
        cpu.registers.set_by_number(reg_t, -80);
        let _ = cpu.dsub(reg_dest, reg_s, reg_t);
        assert_eq!(cpu.registers.get_by_number(reg_dest), 120);

        cpu.registers.set_by_number(reg_s, i64::MIN);
        cpu.registers.set_by_number(reg_t, 1);
        let res = cpu.dsub(reg_dest, reg_s, reg_t);
        assert!(res.is_err());
        assert_eq!(cpu.registers.get_by_number(reg_dest), i64::MAX);
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

    #[test]
    fn test_or() {
        let mut cpu = CPU::new();
        let reg_dest = 10;
        let reg_s = 15;
        let reg_t = 20;
        cpu.registers.set_by_number(reg_s, 123);
        cpu.registers.set_by_number(reg_t, 123);
        cpu.or(reg_dest, reg_s, reg_t);
        assert_eq!(cpu.registers.get_by_number(reg_dest), 123);

        cpu.registers.set_by_number(reg_s, 123);
        cpu.registers.set_by_number(reg_t, 321);
        cpu.or(reg_dest, reg_s, reg_t);
        assert_eq!(cpu.registers.get_by_number(reg_dest), 379);
    }

    #[test]
    fn test_ori() {
        let mut cpu = CPU::new();
        let reg_dest = 10;
        let reg_s = 15;
        cpu.registers.set_by_number(reg_s, 80);
        cpu.ori(reg_dest, reg_s, 80);
        assert_eq!(cpu.registers.get_by_number(reg_dest), 80);

        cpu.registers.set_by_number(reg_s, 123);
        cpu.ori(reg_dest, reg_s, 321);
        assert_eq!(cpu.registers.get_by_number(reg_dest), 379);
    }

    #[test]
    fn test_xor() {
        let mut cpu = CPU::new();
        let reg_dest = 10;
        let reg_s = 15;
        let reg_t = 20;
        cpu.registers.set_by_number(reg_s, 123);
        cpu.registers.set_by_number(reg_t, 123);
        cpu.xor(reg_dest, reg_s, reg_t);
        assert_eq!(cpu.registers.get_by_number(reg_dest), 0);

        cpu.registers.set_by_number(reg_s, 123);
        cpu.registers.set_by_number(reg_t, 321);
        cpu.xor(reg_dest, reg_s, reg_t);
        assert_eq!(cpu.registers.get_by_number(reg_dest), 314);
    }

    #[test]
    fn test_xori() {
        let mut cpu = CPU::new();
        let reg_dest = 10;
        let reg_s = 15;
        cpu.registers.set_by_number(reg_s, 80);
        cpu.xori(reg_dest, reg_s, 80);
        assert_eq!(cpu.registers.get_by_number(reg_dest), 0);

        cpu.registers.set_by_number(reg_s, 123);
        cpu.xori(reg_dest, reg_s, 321);
        assert_eq!(cpu.registers.get_by_number(reg_dest), 314);
    }

    #[test]
    fn test_nor() {
        let mut cpu = CPU::new();
        let reg_dest = 10;
        let reg_s = 15;
        let reg_t = 20;
        cpu.registers.set_by_number(reg_s, 123);
        cpu.registers.set_by_number(reg_t, 123);
        cpu.nor(reg_dest, reg_s, reg_t);
        assert_eq!(cpu.registers.get_by_number(reg_dest), -124);

        cpu.registers.set_by_number(reg_s, 123);
        cpu.registers.set_by_number(reg_t, 321);
        cpu.nor(reg_dest, reg_s, reg_t);
        assert_eq!(cpu.registers.get_by_number(reg_dest), -380);
    }
}
