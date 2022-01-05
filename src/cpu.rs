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

pub fn params_rs_rt(opcode: u32) -> (usize, usize) {
    let rs = (opcode >> 21) & 0b11111;
    let rt = (opcode >> 11) & 0b11111;
    (rt as usize, rs as usize)
}

pub fn params_rt_immediate(opcode: u32) -> (usize, i16) {
    let rt = (opcode >> 16) & 0b11111;
    let immediate = (opcode & 0xFFFF) as i16;
    (rt as usize, immediate)
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
                        self.daddu(rd, rs, rt);
                    },
                    // ADDU
                    0b000_0010_0001 => {
                        let (rd, rs, rt) = params_rd_rs_rt(opcode);
                        self.addu(rd, rs, rt);
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
                        self.subu(rd, rs, rt);
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
                        self.dsubu(rd, rs, rt);
                    },
                    // DIV
                    0b000_0001_1010 => {
                        let (rs, rt) = params_rs_rt(opcode);
                        self.div(rs, rt);
                    },
                    // DIVU
                    0b000_0001_1011 => {
                        let (rs, rt) = params_rs_rt(opcode);
                        self.divu(rs, rt);
                    },
                    // DDIV
                    0b000_0001_1110 => {
                        let (rs, rt) = params_rs_rt(opcode);
                        self.ddiv(rs, rt);
                    },
                    // DDIVU
                    0b000_0001_1111 => {
                        let (rs, rt) = params_rs_rt(opcode);
                        self.ddivu(rs, rt);
                    },
                    // MULT
                    0b000_0001_1000 => {
                        let (rs, rt) = params_rs_rt(opcode);
                        self.mult(rs, rt);
                    },
                    // MULTU
                    0b000_0001_1001 => {
                        let (rs, rt) = params_rs_rt(opcode);
                        self.multu(rs, rt);
                    },
                    // DMULT
                    0b000_0001_1100 => {
                        let (rs, rt) = params_rs_rt(opcode);
                        self.dmult(rs, rt);
                    },
                    // DMULTU
                    0b000_0001_1101 => {
                        let (rs, rt) = params_rs_rt(opcode);
                        self.dmultu(rs, rt);
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
                    // SLT
                    0b000_0010_1010 => {
                        let (rd, rs, rt) = params_rd_rs_rt(opcode);
                        self.slt(rd, rs, rt);
                    },
                    // SLTU
                    0b000_0010_1011 => {
                        let (rd, rs, rt) = params_rd_rs_rt(opcode);
                        self.slt(rd, rs, rt);
                    },
                    _ => unimplemented!(),
                };
            },
            // DADDI
            0b0110_00 => {
                let (rt, rs, immediate) = params_rt_rs_immediate(opcode);
                let res = self.daddi(rt, rs, immediate);
                if inst == 0b0110_00 {
                    if let Err(_) = res {
                        todo!("Throw exception for add overflow DADDI");
                    }
                }
            },
            // DADDIU
            0b0110_01 => {
                let (rt, rs, immediate) = params_rt_rs_immediate(opcode);
                self.daddiu(rt, rs, immediate);
            },
            // ADDI
            0b0010_00 => {
                let (rt, rs, immediate) = params_rt_rs_immediate(opcode);
                let res = self.addi(rt, rs, immediate);
                if inst == 0b0010_00 {
                    if let Err(_) = res {
                        todo!("Throw exception for add overflow ADDI");
                    }
                }
            },
            // ADDIU
            0b0010_01 => {
                let (rt, rs, immediate) = params_rt_rs_immediate(opcode);
                self.addiu(rt, rs, immediate);
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
            // SLTI
            0b0010_10 => {
                let (rt, rs, immediate) = params_rt_rs_immediate(opcode);
                self.slti(rt, rs, immediate);
            },
            // SLTIU
            0b0010_11 => {
                let (rt, rs, immediate) = params_rt_rs_immediate(opcode);
                self.sltiu(rt, rs, immediate);
            },
            // LUI
            0b0011_11 => {
                let (rt, immediate) = params_rt_immediate(opcode);
                self.lui(rt, immediate);
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

    pub fn addu(&mut self, rd: usize, rs: usize, rt: usize) {
        let s = (self.registers.get_by_number(rs) as i32) as u32;
        let t = (self.registers.get_by_number(rt) as i32) as u32;
        let result = s.wrapping_add(t) as u64;
        self.registers.set_by_number(rd, result as i64);
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

    pub fn addiu(&mut self, rt: usize, rs: usize, immediate: i16) {
        let s = (self.registers.get_by_number(rs) as i32) as u32;
        let immediate = (immediate as i32) as u32;
        let result = s.wrapping_add(immediate) as u64;
        self.registers.set_by_number(rt, result as i64);
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

    pub fn daddu(&mut self, rd: usize, rs: usize, rt: usize) {
        let s = self.registers.get_by_number(rs) as u64;
        let t = self.registers.get_by_number(rt) as u64;
        let result = s.wrapping_add(t);
        self.registers.set_by_number(rd, result as i64);
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

    pub fn daddiu(&mut self, rt: usize, rs: usize, immediate: i16) {
        let s = self.registers.get_by_number(rs) as u64;
        let immediate = (immediate as u16) as u64;
        let result = s.wrapping_add(immediate);
        self.registers.set_by_number(rt, result as i64);
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

    pub fn subu(&mut self, rd: usize, rs: usize, rt: usize) {
        let s = (self.registers.get_by_number(rs) as i32) as u32;
        let t = (self.registers.get_by_number(rt) as i32) as u32;
        let result = s.wrapping_sub(t) as u64;
        self.registers.set_by_number(rd, result as i64);
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

    pub fn dsubu(&mut self, rd: usize, rs: usize, rt: usize) {
        let s = self.registers.get_by_number(rs) as u64;
        let t = self.registers.get_by_number(rt) as u64;
        let result = s.wrapping_sub(t);
        self.registers.set_by_number(rd, result as i64);
    }

    pub fn div(&mut self, rs: usize, rt: usize) {
        let s = self.registers.get_by_number(rs) as i32;
        let t = self.registers.get_by_number(rt) as i32;
        let quotient = s.wrapping_div(t);
        let remainder = s.wrapping_rem_euclid(t);
        self.registers.set_lo(quotient as i64);
        self.registers.set_hi(remainder as i64);
    }

    pub fn ddiv(&mut self, rs: usize, rt: usize) {
        let s = self.registers.get_by_number(rs);
        let t = self.registers.get_by_number(rt);
        let quotient = s.wrapping_div(t);
        let remainder = s.wrapping_rem_euclid(t);
        self.registers.set_lo(quotient);
        self.registers.set_hi(remainder);
    }

    pub fn divu(&mut self, rs: usize, rt: usize) {
        let s = self.registers.get_by_number(rs) as u32;
        let t = self.registers.get_by_number(rt) as u32;
        let quotient = s.wrapping_div(t);
        let remainder = s.wrapping_rem_euclid(t);
        self.registers.set_lo((quotient as i32) as i64);
        self.registers.set_hi((remainder as i32) as i64);
    }

    pub fn ddivu(&mut self, rs: usize, rt: usize) {
        let s = self.registers.get_by_number(rs) as u64;
        let t = self.registers.get_by_number(rt) as u64;
        let quotient = s.wrapping_div(t);
        let remainder = s.wrapping_rem_euclid(t);
        self.registers.set_lo(quotient as i64);
        self.registers.set_hi(remainder as i64);
    }

    pub fn mult(&mut self, rs: usize, rt: usize) {
        let s = (self.registers.get_by_number(rs) as i32) as i64;
        let t = (self.registers.get_by_number(rt) as i32) as i64;
        let result = s * t;
        self.registers.set_lo(result & 0x000000FFFFFF);
        self.registers.set_hi(result >> 32);
    }

    pub fn dmult(&mut self, rs: usize, rt: usize) {
        let s = self.registers.get_by_number(rs) as i128;
        let t = self.registers.get_by_number(rt) as i128;
        let result = s * t;
        self.registers.set_lo((result & 0xFFFFFFFFFFFF) as i64);
        self.registers.set_hi((result >> 64) as i64);
    }

    pub fn multu(&mut self, rs: usize, rt: usize) {
        let s = self.registers.get_by_number(rs) as u64;
        let t = self.registers.get_by_number(rt) as u64;
        let result = s * t;
        self.registers.set_lo((result & 0x000000FFFFFF) as i64);
        self.registers.set_hi((result >> 32) as i64);
    }

    pub fn dmultu(&mut self, rs: usize, rt: usize) {
        let s = self.registers.get_by_number(rs) as u128;
        let t = self.registers.get_by_number(rt) as u128;
        let result = s * t;
        self.registers.set_lo((result & 0xFFFFFFFFFFFF) as i64);
        self.registers.set_hi((result >> 64) as i64);
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

    pub fn slt(&mut self, rd: usize, rs: usize, rt: usize) {
        let result = self.registers.get_by_number(rs) < self.registers.get_by_number(rt);
        self.registers.set_by_number(rd, if result {1} else {0});
    }

    pub fn sltu(&mut self, rd: usize, rs: usize, rt: usize) {
        let result = (self.registers.get_by_number(rs) as u64) < (self.registers.get_by_number(rt) as u64);
        self.registers.set_by_number(rd, if result {1} else {0});
    }

    pub fn slti(&mut self, rt: usize, rs: usize, immediate: i16) {
        let s = self.registers.get_by_number(rs);
        let immediate = immediate as i64;
        let result = s < immediate;
        self.registers.set_by_number(rt, if result {1} else {0});
    }

    pub fn sltiu(&mut self, rt: usize, rs: usize, immediate: i16) {
        let s = self.registers.get_by_number(rs) as u64;
        let immediate = (immediate as u16) as u64;
        let result = s < immediate;
        self.registers.set_by_number(rt, if result {1} else {0});
    }

    pub fn lui(&mut self, rt: usize, immediate: i16) {
        let shift = ((immediate as u16) as u32) << 16;
        let result = (shift as i32) as i64;
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
    fn test_div() {
        let mut cpu = CPU::new();
        let reg_s = 15;
        let reg_t = 20;
        cpu.registers.set_by_number(reg_s, 80);
        cpu.registers.set_by_number(reg_t, 80);
        cpu.div(reg_s, reg_t);
        assert_eq!(cpu.registers.get_lo(), 1);
        assert_eq!(cpu.registers.get_hi(), 0);

        cpu.registers.set_by_number(reg_s, 3);
        cpu.registers.set_by_number(reg_t, 2);
        cpu.div(reg_s, reg_t);
        assert_eq!(cpu.registers.get_lo(), 1);
        assert_eq!(cpu.registers.get_hi(), 1);
    }

    #[test]
    fn test_ddiv() {
        let mut cpu = CPU::new();
        let reg_s = 15;
        let reg_t = 20;
        cpu.registers.set_by_number(reg_s, 80);
        cpu.registers.set_by_number(reg_t, 80);
        cpu.ddiv(reg_s, reg_t);
        assert_eq!(cpu.registers.get_lo(), 1);
        assert_eq!(cpu.registers.get_hi(), 0);

        cpu.registers.set_by_number(reg_s, 3);
        cpu.registers.set_by_number(reg_t, 2);
        cpu.ddiv(reg_s, reg_t);
        assert_eq!(cpu.registers.get_lo(), 1);
        assert_eq!(cpu.registers.get_hi(), 1);
    }

    #[test]
    fn test_mult() {
        let mut cpu = CPU::new();
        let reg_s = 15;
        let reg_t = 20;
        cpu.registers.set_by_number(reg_s, 20);
        cpu.registers.set_by_number(reg_t, 20);
        cpu.mult(reg_s, reg_t);
        assert_eq!(cpu.registers.get_lo(), 400);
        assert_eq!(cpu.registers.get_hi(), 0);
    }

    #[test]
    fn test_dmult() {
        let mut cpu = CPU::new();
        let reg_s = 15;
        let reg_t = 20;
        cpu.registers.set_by_number(reg_s, 20);
        cpu.registers.set_by_number(reg_t, 20);
        cpu.dmult(reg_s, reg_t);
        assert_eq!(cpu.registers.get_lo(), 400);
        assert_eq!(cpu.registers.get_hi(), 0);
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

    #[test]
    fn test_slt() {
        let mut cpu = CPU::new();
        let reg_dest = 10;
        let reg_s = 15;
        let reg_t = 20;
        cpu.registers.set_by_number(reg_s, 123);
        cpu.registers.set_by_number(reg_t, 123);
        cpu.slt(reg_dest, reg_s, reg_t);
        assert_eq!(cpu.registers.get_by_number(reg_dest), 0);

        cpu.registers.set_by_number(reg_s, 123);
        cpu.registers.set_by_number(reg_t, 321);
        cpu.slt(reg_dest, reg_s, reg_t);
        assert_eq!(cpu.registers.get_by_number(reg_dest), 1);

        cpu.registers.set_by_number(reg_s, 321);
        cpu.registers.set_by_number(reg_t, 123);
        cpu.slt(reg_dest, reg_s, reg_t);
        assert_eq!(cpu.registers.get_by_number(reg_dest), 0);
    }

    #[test]
    fn test_slti() {
        let mut cpu = CPU::new();
        let reg_t = 20;
        let reg_s = 15;
        cpu.registers.set_by_number(reg_s, 123);
        cpu.slti(reg_t, reg_s, 123);
        assert_eq!(cpu.registers.get_by_number(reg_t), 0);

        cpu.registers.set_by_number(reg_s, 123);
        cpu.slti(reg_t, reg_s, -123);
        assert_eq!(cpu.registers.get_by_number(reg_t), 0);

        cpu.registers.set_by_number(reg_s, -123);
        cpu.slti(reg_t, reg_s, 123);
        assert_eq!(cpu.registers.get_by_number(reg_t), 1);
    }

    #[test]
    fn test_sltiu() {
        let mut cpu = CPU::new();
        let reg_t = 20;
        let reg_s = 15;
        cpu.registers.set_by_number(reg_s, 123);
        cpu.sltiu(reg_t, reg_s, 123);
        assert_eq!(cpu.registers.get_by_number(reg_t), 0);

        cpu.registers.set_by_number(reg_s, 123);
        cpu.sltiu(reg_t, reg_s, 321);
        assert_eq!(cpu.registers.get_by_number(reg_t), 1);
    }

    #[test]
    fn test_lui() {
        let mut cpu = CPU::new();
        let reg_t = 20;
        cpu.lui(reg_t, -10);
        assert_eq!(cpu.registers.get_by_number(reg_t), -655360);
    }
}
