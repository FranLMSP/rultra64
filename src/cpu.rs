use crate::registers::{CPURegisters, CP0Registers};
use crate::mmu::{MMU};

pub fn params_rd_rs_rt(opcode: u32) -> (usize, usize, usize) {
    let rd = (opcode >> 11) & 0b11111;
    let rs = (opcode >> 21) & 0b11111;
    let rt = (opcode >> 16) & 0b11111;
    (rd as usize, rs as usize, rt as usize)
}

pub fn params_rd_rt_rs(opcode: u32) -> (usize, usize, usize) {
    let rd = (opcode >> 11) & 0b11111;
    let rt = (opcode >> 16) & 0b11111;
    let rs = (opcode >> 21) & 0b11111;
    (rd as usize, rt as usize, rs as usize)
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

pub fn params_rt_offset_base(opcode: u32) -> (usize, i16, usize) {
    let rt = (opcode >> 16) & 0b11111;
    let offset = (opcode & 0xFFFF) as i16;
    let base = (opcode >> 21) & 0b11111;
    (rt as usize, offset, base as usize)
}

pub fn params_rd_rt_sa(opcode: u32) -> (usize, usize, usize) {
    let rd = (opcode >> 11) & 0b11111;
    let rt = (opcode >> 16) & 0b11111;
    let sa = (opcode >> 6)  & 0b11111;
    (rd as usize, rt as usize, sa as usize)
}

pub fn params_rt_rd(opcode: u32) -> (usize, usize) {
    let rd = (opcode >> 11) & 0b11111;
    let rt = (opcode >> 16) & 0b11111;
    (rt as usize, rd as usize)
}

pub fn params_rd(opcode: u32) -> usize {
    return ((opcode >> 11) & 0b11111) as usize;
}

pub fn params_rs(opcode: u32) -> usize {
    return ((opcode >> 21) & 0b11111) as usize;
}

pub struct CPU {
    registers: CPURegisters,
    cp0: CP0Registers,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            registers: CPURegisters::new(),
            cp0: CP0Registers::new(),
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
                    // SLL
                    0b000_0000_0000 => {
                        let (rd, rt, sa) = params_rd_rt_sa(opcode);
                        self.sll(rd, rt, sa);
                    },
                    // SRL
                    0b000_0000_0010 => {
                        let (rd, rt, sa) = params_rd_rt_sa(opcode);
                        self.srl(rd, rt, sa);
                    },
                    // SRA
                    0b000_0000_0011 => {
                        let (rd, rt, sa) = params_rd_rt_sa(opcode);
                        self.sra(rd, rt, sa);
                    },
                    // SLLV
                    0b000_0000_0100 => {
                        let (rd, rt, rs) = params_rd_rt_rs(opcode);
                        self.sllv(rd, rt, rs);
                    },
                    // SRL
                    0b000_0000_0110 => {
                        let (rd, rt, rs) = params_rd_rt_rs(opcode);
                        self.sllv(rd, rt, rs);
                    },
                    // SRAV
                    0b000_0000_0111 => {
                        let (rd, rt, rs) = params_rd_rt_rs(opcode);
                        self.sllv(rd, rt, rs);
                    },
                    // DSLL
                    0b000_0011_1000 => {
                        let (rd, rt, sa) = params_rd_rt_sa(opcode);
                        self.dsll(rd, rt, sa);
                    },
                    // DSRL
                    0b000_0011_1010 => {
                        let (rd, rt, sa) = params_rd_rt_sa(opcode);
                        self.dsrl(rd, rt, sa);
                    },
                    // DSRA
                    0b000_0011_1011 => {
                        let (rd, rt, rs) = params_rd_rt_rs(opcode);
                        self.dsra(rd, rt, rs);
                    },
                    // DSLLV
                    0b000_0001_0100 => {
                        let (rd, rt, rs) = params_rd_rt_rs(opcode);
                        self.dsllv(rd, rt, rs);
                    },
                    // DSRLV
                    0b000_0001_0110 => {
                        let (rd, rt, rs) = params_rd_rt_rs(opcode);
                        self.dsrlv(rd, rt, rs);
                    },
                    // DSRAV
                    0b000_0001_0111 => {
                        let (rd, rt, rs) = params_rd_rt_rs(opcode);
                        self.dsrav(rd, rt, rs);
                    },
                    // DSLL32
                    0b000_0011_1100 => {
                        let (rd, rt, sa) = params_rd_rt_sa(opcode);
                        self.dsll32(rd, rt, sa);
                    },
                    // DSRL32
                    0b000_0011_1110 => {
                        let (rd, rt, sa) = params_rd_rt_sa(opcode);
                        self.dsrl32(rd, rt, sa);
                    },
                    // DSRA32
                    0b000_0011_1111 => {
                        let (rd, rt, sa) = params_rd_rt_sa(opcode);
                        self.dsra32(rd, rt, sa);
                    },
                    // MFHI
                    0b000_0001_0000 => self.mfhi(params_rd(opcode)),
                    // MFLO
                    0b000_0001_0010 => self.mflo(params_rd(opcode)),
                    // MTHI
                    0b000_0001_0001 => self.mthi(params_rs(opcode)),
                    // MTLO
                    0b000_0001_0011 => self.mtlo(params_rs(opcode)),
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
            0b0100_00 => {
                let instr = (opcode >> 21) & 11;
                match instr {
                    // MTC0
                    0b0100_0000_100 => {
                        let (rt, rd) = params_rt_rd(opcode);
                        self.mtc0(rt, rd);
                    },
                    // MFC0
                    0b0100_0000_000 => {
                        let (rt, rd) = params_rt_rd(opcode);
                        self.mfc0(rt, rd);
                    },
                    // DMTC0
                    0b0100_0000_101 => {
                        let (rt, rd) = params_rt_rd(opcode);
                        self.dmtc0(rt, rd);
                    },
                    // DMFC0
                    0b0100_0000_001 => {
                        let (rt, rd) = params_rt_rd(opcode);
                        self.dmfc0(rt, rd);
                    },
                    _ => unimplemented!(),
                };
            },
            // LB
            0b1000_00 => {
                // let (rt, offset, base) = params_rt_offset_base(opcode);
                // self.lb(rt, offset, base, mmu);
                todo!("Receive MMU parameter");
            }
            // LBU
            0b1001_00 => {
                // let (rt, offset, base) = params_rt_offset_base(opcode);
                // self.lbu(rt, offset, base, mmu);
                todo!("Receive MMU parameter");
            }
            // LH
            0b1000_01 => {
                // let (rt, offset, base) = params_rt_offset_base(opcode);
                // self.lh(rt, offset, base, mmu);
                todo!("Receive MMU parameter");
            }
            // LHU
            0b1001_01 => {
                // let (rt, offset, base) = params_rt_offset_base(opcode);
                // self.lhu(rt, offset, base, mmu);
                todo!("Receive MMU parameter");
            }
            // LW
            0b1000_11 => {
                // let (rt, offset, base) = params_rt_offset_base(opcode);
                // self.lw(rt, offset, base, mmu);
                todo!("Receive MMU parameter");
            }
            // LWL
            0b1000_10 => {
                // let (rt, offset, base) = params_rt_offset_base(opcode);
                // self.lwl(rt, offset, base, mmu);
                todo!("Receive MMU parameter");
            }
            // LWR
            0b1001_10 => {
                // let (rt, offset, base) = params_rt_offset_base(opcode);
                // self.lwr(rt, offset, base, mmu);
                todo!("Receive MMU parameter");
            }
            // SB
            0b1010_00 => {
                // let (rt, offset, base) = params_rt_offset_base(opcode);
                // self.sb(rt, offset, base, mmu);
                todo!("Receive MMU parameter");
            }
            // SH
            0b1010_00 => {
                // let (rt, offset, base) = params_rt_offset_base(opcode);
                // self.sh(rt, offset, base, mmu);
                todo!("Receive MMU parameter");
            }
            // SW
            0b1010_00 => {
                // let (rt, offset, base) = params_rt_offset_base(opcode);
                // self.sw(rt, offset, base, mmu);
                todo!("Receive MMU parameter");
            }
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

    pub fn sll(&mut self, rd: usize, rt: usize, sa: usize) {
        let t = self.registers.get_by_number(rt) as i32;
        let result = t << sa;
        self.registers.set_by_number(rd, result as i64);
    }

    pub fn srl(&mut self, rd: usize, rt: usize, sa: usize) {
        let t = self.registers.get_by_number(rt) as i32;
        let result = t >> sa;
        self.registers.set_by_number(rd, result as i64);
    }

    pub fn sra(&mut self, rd: usize, rt: usize, sa: usize) {
        let t = self.registers.get_by_number(rt) as i32;
        let sign = (t as u32) & 0x80000000;
        let result = ((t >> sa) as u32) & 0xEFFFFFFF;
        self.registers.set_by_number(rd, ((result | sign) as i32) as i64);
    }

    pub fn sllv(&mut self, rd: usize, rt: usize, rs: usize) {
        let t = self.registers.get_by_number(rt);
        let s = (self.registers.get_by_number(rs) & 0b11111) as usize;
        let result = t << s;
        self.registers.set_by_number(rd, result as i64);
    }

    pub fn srlv(&mut self, rd: usize, rt: usize, rs: usize) {
        let t = self.registers.get_by_number(rt);
        let s = (self.registers.get_by_number(rs) & 0b11111) as usize;
        let result = t >> s;
        self.registers.set_by_number(rd, result as i64);
    }

    pub fn srav(&mut self, rd: usize, rt: usize, rs: usize) {
        let t = self.registers.get_by_number(rt);
        let s = (self.registers.get_by_number(rs) & 0b11111) as usize;
        let result = t >> s;
        self.registers.set_by_number(rd, result as i64);
    }

    pub fn dsll(&mut self, rd: usize, rt: usize, sa: usize) {
        let t = self.registers.get_by_number(rt);
        let result = t << sa;
        self.registers.set_by_number(rd, result);
    }

    pub fn dsrl(&mut self, rd: usize, rt: usize, sa: usize) {
        let t = self.registers.get_by_number(rt);
        let result = t >> sa;
        self.registers.set_by_number(rd, result);
    }

    pub fn dsra(&mut self, rd: usize, rt: usize, sa: usize) {
        let t = self.registers.get_by_number(rt);
        let result = t >> sa;
        self.registers.set_by_number(rd, result);
    }

    pub fn dsllv(&mut self, rd: usize, rt: usize, rs: usize) {
        let t = self.registers.get_by_number(rt);
        let s = (self.registers.get_by_number(rs) & 0b111111) as usize;
        let result = t << s;
        self.registers.set_by_number(rd, result);
    }

    pub fn dsrlv(&mut self, rd: usize, rt: usize, rs: usize) {
        let t = self.registers.get_by_number(rt);
        let s = (self.registers.get_by_number(rs) & 0b111111) as usize;
        let result = t >> s;
        self.registers.set_by_number(rd, result);
    }

    pub fn dsrav(&mut self, rd: usize, rt: usize, rs: usize) {
        let t = self.registers.get_by_number(rt);
        let s = (self.registers.get_by_number(rs) & 0b111111) as usize;
        let result = t >> s;
        self.registers.set_by_number(rd, result);
    }

    pub fn dsll32(&mut self, rd: usize, rt: usize, sa: usize) {
        let t = self.registers.get_by_number(rt);
        let result = t << (32 + sa);
        self.registers.set_by_number(rd, result);
    }

    pub fn dsrl32(&mut self, rd: usize, rt: usize, sa: usize) {
        let t = self.registers.get_by_number(rt);
        let result = t >> (32 + sa);
        self.registers.set_by_number(rd, result);
    }

    pub fn dsra32(&mut self, rd: usize, rt: usize, sa: usize) {
        let t = self.registers.get_by_number(rt);
        let result = t >> (32 + sa);
        self.registers.set_by_number(rd, result);
    }

    pub fn mfhi(&mut self, rd: usize) {
        self.registers.set_by_number(rd, self.registers.get_hi());
    }

    pub fn mflo(&mut self, rd: usize) {
        self.registers.set_by_number(rd, self.registers.get_lo());
    }

    pub fn mthi(&mut self, rs: usize) {
        self.registers.set_hi(self.registers.get_by_number(rs));
    }

    pub fn mtlo(&mut self, rs: usize) {
        self.registers.set_lo(self.registers.get_by_number(rs));
    }

    pub fn mtc0(&mut self, rt: usize, rd: usize) {
        match CP0Registers::is_32bits(rd) {
            true => self.cp0.set_by_number_32(rd, self.registers.get_by_number(rt) as i32),
            false => self.cp0.set_by_number_64(rd, self.registers.get_by_number(rt)),
        };
    }

    pub fn mfc0(&mut self, rt: usize, rd: usize) {
        match CP0Registers::is_32bits(rd) {
            true => self.registers.set_by_number(rt, self.cp0.get_by_number_32(rd) as i64),
            false => self.registers.set_by_number(rt, self.cp0.get_by_number_64(rd))
        };
    }

    pub fn dmtc0(&mut self, rt: usize, rd: usize) {
        match CP0Registers::is_32bits(rd) {
            true => self.cp0.set_by_number_32(rd, self.registers.get_by_number(rt) as i32),
            false => self.cp0.set_by_number_64(rd, self.registers.get_by_number(rt)),
        };
    }

    pub fn dmfc0(&mut self, rt: usize, rd: usize) {
        match CP0Registers::is_32bits(rd) {
            true => self.registers.set_by_number(rt, self.cp0.get_by_number_32(rd) as i64),
            false => self.registers.set_by_number(rt, self.cp0.get_by_number_64(rd))
        };
    }

    pub fn lb(&mut self, rt: usize, offset: i16, base: usize, mmu: &MMU) {
        let address = self.registers.get_by_number(base) + (offset as i64);
        let data = mmu.read_virtual(address, 1);
        self.registers.set_by_number(rt, (data[0] as i8) as i64)
    }

    pub fn lbu(&mut self, rt: usize, offset: i16, base: usize, mmu: &MMU) {
        let address = self.registers.get_by_number(base) + (offset as i64);
        let data = mmu.read_virtual(address, 1);
        self.registers.set_by_number(rt, (data[0] as u64) as i64)
    }

    pub fn lh(&mut self, rt: usize, offset: i16, base: usize, mmu: &MMU) {
        let address = self.registers.get_by_number(base) + (offset as i64);
        let data = mmu.read_virtual(address, 2);
        let data = ((data[0] as u16) << 8) | (data[1] as u16);
        self.registers.set_by_number(rt, (data as i16) as i64)
    }

    pub fn lhu(&mut self, rt: usize, offset: i16, base: usize, mmu: &MMU) {
        let address = self.registers.get_by_number(base) + (offset as i64);
        let data = mmu.read_virtual(address, 2);
        let data = ((data[0] as u16) << 8) | (data[1] as u16);
        self.registers.set_by_number(rt, (data as u64) as i64)
    }

    pub fn lw(&mut self, rt: usize, offset: i16, base: usize, mmu: &MMU) {
        let address = self.registers.get_by_number(base) + (offset as i64);
        let data = mmu.read_virtual(address, 4);
        let data = ((data[0] as u32) << 24) | ((data[1] as u32) << 16) | ((data[2] as u32) << 8) | ((data[3] as u32) << 8);
        self.registers.set_by_number(rt, (data as i32) as i64)
    }

    pub fn lwl(&mut self, rt: usize, offset: i16, base: usize, mmu: &MMU) {
        let address = self.registers.get_by_number(base) + (offset as i64);
        let bytes_to_read = (4 - (address % 4)) as usize;
        let t = self.registers.get_by_number(rt) as u32;
        let data = mmu.read_virtual(address, bytes_to_read);
        let mut result: u32 = 0;
        let mut bitmask: u32 = 0xFFFFFFFF;
        for byte in data {
            result = (result << 8) | (byte as u32);
            bitmask = bitmask >> 8;
        }
        let left = 4 - bytes_to_read;
        let result = ((t & bitmask) | (result << left)) as i32;
        self.registers.set_by_number(rt, result as i64)
    }

    pub fn lwr(&mut self, rt: usize, offset: i16, base: usize, mmu: &MMU) {
        let address = self.registers.get_by_number(base) + (offset as i64);
        let bytes_to_read = (4 - (address % 4)) as usize;
        let t = self.registers.get_by_number(rt) as u32;
        let data = mmu.read_virtual(address, bytes_to_read);
        let mut result: u32 = 0;
        let mut bitmask: u32 = 0xFFFFFFFF;
        for byte in data {
            result = (result << 8) | (byte as u32);
            bitmask = bitmask << 8;
        }
        let result = ((t & bitmask) | result) as i32;
        self.registers.set_by_number(rt, result as i64)
    }

    pub fn sb(&mut self, rt: usize, offset: i16, base: usize, mmu: &mut MMU) {
        let address = self.registers.get_by_number(base) + (offset as i64);
        let byte = self.registers.get_by_number(rt).to_le_bytes()[0] as u8;
        mmu.write_virtual(address, (self.registers.get_by_number(rt) as i8).to_be_bytes());
    }

    pub fn sh(&mut self, rt: usize, offset: i16, base: usize, mmu: &mut MMU) {
        let address = self.registers.get_by_number(base) + (offset as i64);
        mmu.write_virtual(address, (self.registers.get_by_number(rt) as i16).to_be_bytes());
    }

    pub fn sw(&mut self, rt: usize, offset: i16, base: usize, mmu: &mut MMU) {
        let address = self.registers.get_by_number(base) + (offset as i64);
        mmu.write_virtual(address, (self.registers.get_by_number(rt) as i32).to_be_bytes());
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

        let mut cpu = CPU::new();
        let rt = 15;
        cpu.lui(rt, 0x3400);
        assert_eq!(cpu.registers.get_by_number(rt) as i32, 0x34000000);
    }

    #[test]
    fn test_sll() {
        let mut cpu = CPU::new();
        let rd = 15;
        let rt = 20;
        cpu.registers.set_by_number(rt, 0b111);
        cpu.sll(rd, rt, 3);
        assert_eq!(cpu.registers.get_by_number(rd), 0b111000);
    }

    #[test]
    fn test_srl() {
        let mut cpu = CPU::new();
        let rd = 15;
        let rt = 20;
        cpu.registers.set_by_number(rt, 0b111000);
        cpu.srl(rd, rt, 3);
        assert_eq!(cpu.registers.get_by_number(rd), 0b111);
    }

    #[test]
    fn test_sra() {
        let mut cpu = CPU::new();
        let rd = 15;
        let rt = 20;
        cpu.registers.set_by_number(rt, 0b111000);
        cpu.sra(rd, rt, 3);
        assert_eq!(cpu.registers.get_by_number(rd), 0b111);
    }

    #[test]
    fn test_sllv() {
        let mut cpu = CPU::new();
        let rd = 15;
        let rt = 20;
        let rs = 25;
        cpu.registers.set_by_number(rt, 0b111);
        cpu.registers.set_by_number(rs, 0b11);
        cpu.sllv(rd, rt, rs);
        assert_eq!(cpu.registers.get_by_number(rd), 0b111000);
    }

    #[test]
    fn test_srlv() {
        let mut cpu = CPU::new();
        let rd = 15;
        let rt = 20;
        let rs = 25;
        cpu.registers.set_by_number(rt, 0b111000);
        cpu.registers.set_by_number(rs, 0b11);
        cpu.srlv(rd, rt, rs);
        assert_eq!(cpu.registers.get_by_number(rd), 0b111);
    }

    #[test]
    fn test_srav() {
        let mut cpu = CPU::new();
        let rd = 15;
        let rt = 20;
        let rs = 25;
        cpu.registers.set_by_number(rt, 0b111000);
        cpu.registers.set_by_number(rs, 0b11);
        cpu.srav(rd, rt, rs);
        assert_eq!(cpu.registers.get_by_number(rd), 0b111);
    }

    #[test]
    fn test_dsll() {
        let mut cpu = CPU::new();
        let rd = 15;
        let rt = 20;
        cpu.registers.set_by_number(rt, 0b111);
        cpu.dsll(rd, rt, 3);
        assert_eq!(cpu.registers.get_by_number(rd), 0b111000);
    }

    #[test]
    fn test_dsrl() {
        let mut cpu = CPU::new();
        let rd = 15;
        let rt = 20;
        cpu.registers.set_by_number(rt, 0b111000);
        cpu.dsrl(rd, rt, 3);
        assert_eq!(cpu.registers.get_by_number(rd), 0b111);
    }

    #[test]
    fn test_dsra() {
        let mut cpu = CPU::new();
        let rd = 15;
        let rt = 20;
        cpu.registers.set_by_number(rt, 0b111000);
        cpu.dsra(rd, rt, 3);
        assert_eq!(cpu.registers.get_by_number(rd), 0b111);
    }

    #[test]
    fn test_dsllv() {
        let mut cpu = CPU::new();
        let rd = 15;
        let rt = 20;
        let rs = 25;
        cpu.registers.set_by_number(rt, 0b111);
        cpu.registers.set_by_number(rs, 0b11);
        cpu.dsllv(rd, rt, rs);
        assert_eq!(cpu.registers.get_by_number(rd), 0b111000);
    }

    #[test]
    fn test_dsrlv() {
        let mut cpu = CPU::new();
        let rd = 15;
        let rt = 20;
        let rs = 25;
        cpu.registers.set_by_number(rt, 0b111000);
        cpu.registers.set_by_number(rs, 0b11);
        cpu.dsrlv(rd, rt, rs);
        assert_eq!(cpu.registers.get_by_number(rd), 0b111);
    }

    #[test]
    fn test_dsrav() {
        let mut cpu = CPU::new();
        let rd = 15;
        let rt = 20;
        let rs = 25;
        cpu.registers.set_by_number(rt, 0b111000);
        cpu.registers.set_by_number(rs, 0b11);
        cpu.dsrav(rd, rt, rs);
        assert_eq!(cpu.registers.get_by_number(rd), 0b111);
    }

    #[test]
    fn test_dsll32() {
        let mut cpu = CPU::new();
        let rd = 15;
        let rt = 20;
        cpu.registers.set_by_number(rt, 0b1);
        cpu.dsll32(rd, rt, 2);
        assert_eq!(cpu.registers.get_by_number(rd), 0x400000000);
    }

    #[test]
    fn test_dsrl32() {
        let mut cpu = CPU::new();
        let rd = 15;
        let rt = 20;
        cpu.registers.set_by_number(rt, 0x400000000);
        cpu.dsrl32(rd, rt, 2);
        assert_eq!(cpu.registers.get_by_number(rd), 0b1);
    }

    #[test]
    fn test_dsra32() {
        let mut cpu = CPU::new();
        let rd = 15;
        let rt = 20;
        cpu.registers.set_by_number(rt, 0x400000000);
        cpu.dsra32(rd, rt, 2);
        assert_eq!(cpu.registers.get_by_number(rd), 0b1);
    }

    #[test]
    fn test_mfhi() {
        let mut cpu = CPU::new();
        let rd = 15;
        cpu.registers.set_hi(65535);
        cpu.mfhi(rd);
        assert_eq!(cpu.registers.get_by_number(rd), 65535);
    }

    #[test]
    fn test_mflo() {
        let mut cpu = CPU::new();
        let rd = 15;
        cpu.registers.set_lo(65535);
        cpu.mflo(rd);
        assert_eq!(cpu.registers.get_by_number(rd), 65535);
    }

    #[test]
    fn test_mthi() {
        let mut cpu = CPU::new();
        let rs = 15;
        cpu.registers.set_by_number(rs, 65535);
        cpu.mthi(rs);
        assert_eq!(cpu.registers.get_hi(), 65535);
    }

    #[test]
    fn test_mtlo() {
        let mut cpu = CPU::new();
        let rs = 15;
        cpu.registers.set_by_number(rs, 65535);
        cpu.mtlo(rs);
        assert_eq!(cpu.registers.get_lo(), 65535);
    }

    #[test]
    fn test_mtc0() {
        let mut cpu = CPU::new();
        let rt = 15;
        let rd = 12;
        cpu.registers.set_by_number(rt, 65535);
        cpu.mtc0(rt, rd);
        assert_eq!(cpu.cp0.get_by_number_32(rd), 65535);
    }

    #[test]
    fn test_mfc0() {
        let mut cpu = CPU::new();
        let rt = 15;
        let rd = 21;
        cpu.cp0.set_by_number_64(rd, 65535);
        cpu.mfc0(rt, rd);
        assert_eq!(cpu.registers.get_by_number(rt), 65535);
    }

    #[test]
    fn test_dmtc0() {
        let mut cpu = CPU::new();
        let rt = 15;
        let rd = 12;
        cpu.registers.set_by_number(rt, 65535);
        cpu.dmtc0(rt, rd);
        assert_eq!(cpu.cp0.get_by_number_32(rd), 65535);
    }

    #[test]
    fn test_dmfc0() {
        let mut cpu = CPU::new();
        let rt = 15;
        let rd = 21;
        cpu.cp0.set_by_number_64(rd, 65535);
        cpu.dmfc0(rt, rd);
        assert_eq!(cpu.registers.get_by_number(rt), 65535);
    }

    #[test]
    fn test_lb() {
        todo!("test LB");
    }

    #[test]
    fn test_lbu() {
        todo!("test LBU");
    }

    #[test]
    fn test_lh() {
        todo!("test LH");
    }

    #[test]
    fn test_lhu() {
        todo!("test LHU");
    }

    #[test]
    fn test_lw() {
        todo!("test LW");
    }

    #[test]
    fn test_lwl() {
        todo!("test LWL");
    }

    #[test]
    fn test_lwr() {
        todo!("test LWR");
    }

    #[test]
    fn test_sb() {
        todo!("test SB");
    }

    #[test]
    fn test_sh() {
        todo!("test SH");
    }

    #[test]
    fn test_sw() {
        todo!("test sw");
    }

    #[test]
    fn test_swl() {
        todo!("test swl");
    }

    #[test]
    fn test_swr() {
        todo!("test swr");
    }
}
