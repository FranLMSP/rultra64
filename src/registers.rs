pub trait Register<T: PartialOrd + Copy> {
    fn get(&self) -> T;
    fn set(&mut self, val: T);
}

#[derive(Copy, Clone)]
pub struct Fixed<T>(T);
impl<T: PartialOrd + Copy> Register<T> for Fixed<T> {
    fn get(&self) -> T {self.0}
    fn set(&mut self, _: T) {}
}

#[derive(Copy, Clone)]
pub struct Generic<T>(T);
impl<T: PartialOrd + Copy> Register<T> for Generic<T> {
    fn get(&self) -> T {self.0}
    fn set(&mut self, val: T) {self.0 = val}
}


pub const CPU_REGISTER_NAMES: [&'static str; 32] = [
    "zero", "at", "v0", "v1", "a0", "a1", "a2", "a3",
    "t0",   "t1", "t2", "t3", "t4", "t5", "t6", "t7",
    "s0",   "s1", "s2", "s3", "s4", "s5", "s6", "s7",
    "t8",   "t9", "k0", "k1", "gp", "sp", "s8", "ra"
];

pub struct CPURegisters {
    registers: [Box<dyn Register<i64>>; 32],
    program_counter: Generic<i64>,
    next_program_counter: Generic<i64>,
    hi: Generic<i64>,
    lo: Generic<i64>,
    load_link: bool,
}

impl CPURegisters {
    pub fn new() -> Self {
        Self {
            registers: [
                Box::new(Fixed(0_i64)),
                Box::new(Generic(0_i64)),
                Box::new(Generic(0_i64)),
                Box::new(Generic(0_i64)),
                Box::new(Generic(0_i64)),
                Box::new(Generic(0_i64)),
                Box::new(Generic(0_i64)),
                Box::new(Generic(0_i64)),
                Box::new(Generic(0_i64)),
                Box::new(Generic(0_i64)),
                Box::new(Generic(0_i64)),
                Box::new(Generic(0_i64)),
                Box::new(Generic(0_i64)),
                Box::new(Generic(0_i64)),
                Box::new(Generic(0_i64)),
                Box::new(Generic(0_i64)),
                Box::new(Generic(0_i64)),
                Box::new(Generic(0_i64)),
                Box::new(Generic(0_i64)),
                Box::new(Generic(0_i64)),
                Box::new(Generic(0_i64)),
                Box::new(Generic(0_i64)),
                Box::new(Generic(0_i64)),
                Box::new(Generic(0_i64)),
                Box::new(Generic(0_i64)),
                Box::new(Generic(0_i64)),
                Box::new(Generic(0_i64)),
                Box::new(Generic(0_i64)),
                Box::new(Generic(0_i64)),
                Box::new(Generic(0_i64)),
                Box::new(Generic(0_i64)),
                Box::new(Generic(0_i64)),
            ],
            program_counter: Generic(0xBFC00000),
            next_program_counter: Generic(0xBFC00004),
            hi: Generic(0_i64),
            lo: Generic(0_i64),
            load_link: false,
        }
    }

    pub fn new_hle() -> Self {
        let mut registers = Self::new();
        registers.set_by_name("t3", 0xFFFFFFFFA4000040_u64 as i64);
        registers.set_by_name("s4", 0x0000000000000001);
        registers.set_by_name("s6", 0x000000000000003F);
        registers.set_by_name("sp", 0xFFFFFFFFA4001FF0_u64 as i64);

        registers.set_program_counter(0x80001000);
        registers.set_next_program_counter(0x80001000 + 4);
        /* registers.set_program_counter(0xA4000040);
        registers.set_next_program_counter(0xA4000040 + 4); */

        registers
    }

    pub fn set_load_link(&mut self, val: bool) {
        self.load_link = val;
    }

    pub fn get_load_link(&self) -> bool {
        self.load_link
    }

    fn find_index(name: &'static str) -> usize {
        CPU_REGISTER_NAMES.iter().position(|v| *v == name).unwrap()
    }

    pub fn get_by_number(&self, index: usize) -> i64 {
        if index > 31 {
            unreachable!("Register number {} not valid", index);
        }
        self.registers[index].get()
    }

    pub fn get_by_name(&self, name: &'static str) -> i64 {
        let index = CPURegisters::find_index(name);
        self.registers[index].get()
    }

    pub fn set_by_number(&mut self, index: usize, val: i64) {
        if index > 31 {
            unreachable!("Register number {} not valid", index);
        }
        self.registers[index].set(val);
    }

    pub fn set_by_name(&mut self, name: &'static str, val: i64) {
        let index = CPURegisters::find_index(name);
        self.registers[index].set(val);
    }

    pub fn get_program_counter(&self) -> i64 {
        self.program_counter.get()
    }

    pub fn set_program_counter(&mut self, val: i64) {
        self.program_counter.set(val);
    }

    pub fn increment_program_counter(&mut self, val: i64) {
        let pc: i64 = self.program_counter.get();
        self.program_counter.set(pc.wrapping_add(val));
    }

    pub fn get_next_program_counter(&self) -> i64 {
        self.next_program_counter.get()
    }

    pub fn set_next_program_counter(&mut self, val: i64) {
        self.next_program_counter.set(val);
    }

    pub fn increment_next_program_counter(&mut self, val: i64) {
        let pc: i64 = self.next_program_counter.get();
        self.next_program_counter.set(pc.wrapping_add(val));
    }

    pub fn set_hi(&mut self, val: i64) {
        self.hi.set(val);
    }

    pub fn set_lo(&mut self, val: i64) {
        self.lo.set(val);
    }

    pub fn get_hi(&self) -> i64 {
        self.hi.get()
    }

    pub fn get_lo(&self) -> i64 {
        self.lo.get()
    }
}

pub const CP0_REGISTER_NAMES: [&'static str; 32] = [
    "index", "random", "EntryLo0", "EntryLo1", "context", "PageMask", "wired", "7",
    "BadVAddr", "count", "EntryHi", "compare", "status", "cause", "epc", "PRId",
    "config", "LLAddr", "WatchLo", "WatchHi", "XContext", "21", "22", "23",
    "24", "25", "ParityError", "CacheError", "TagLo", "TagHi", "ErrorEPC", "31"
];

pub struct CP0Registers {
    index: Generic<i32>,
    random: Generic<i32>,
    entry_lo_0: Generic<i64>,
    entry_lo_1: Generic<i64>,
    context: Generic<i64>,
    page_mask: Generic<i32>,
    wired: Generic<i32>,
    r7: Generic<i64>,
    bad_v_addr: Generic<i64>,
    count: Generic<i32>,
    entry_hi: Generic<i64>,
    compare: Generic<i32>,
    status: Generic<i32>,
    cause: Generic<i32>,
    epc: Generic<i64>,
    prid: Generic<i32>,
    config: Generic<i32>,
    lladdr: Generic<i32>,
    watch_lo: Generic<i32>,
    watch_hi: Generic<i32>,
    xcontext: Generic<i64>,
    r21: Generic<i64>,
    r22: Generic<i64>,
    r23: Generic<i64>,
    r24: Generic<i64>,
    r25: Generic<i64>,
    parity_error: Generic<i32>,
    cache_error: Generic<i32>,
    tag_lo: Generic<i32>,
    tag_hi: Generic<i32>,
    error_epc: Generic<i64>,
    r31: Generic<i64>,
}

impl CP0Registers {
    pub fn new() -> Self {
        Self {
            index: Generic(0),
            random: Generic(0),
            entry_lo_0: Generic(0),
            entry_lo_1: Generic(0),
            context: Generic(0),
            page_mask: Generic(0),
            wired: Generic(0),
            r7: Generic(0),
            bad_v_addr: Generic(0),
            count: Generic(0),
            entry_hi: Generic(0),
            compare: Generic(0),
            status: Generic(0),
            cause: Generic(0),
            epc: Generic(0),
            prid: Generic(0),
            config: Generic(0),
            lladdr: Generic(0),
            watch_lo: Generic(0),
            watch_hi: Generic(0),
            xcontext: Generic(0),
            r21: Generic(0),
            r22: Generic(0),
            r23: Generic(0),
            r24: Generic(0),
            r25: Generic(0),
            parity_error: Generic(0),
            cache_error: Generic(0),
            tag_lo: Generic(0),
            tag_hi: Generic(0),
            error_epc: Generic(0),
            r31: Generic(0),
        }
    }

    pub fn new_hle() -> Self {
        let mut cp0 = Self::new();
        cp0.set_by_name_32("random", 0x0000001F);
        cp0.set_by_name_32("status", 0x70400004);
        cp0.set_by_name_32("PRId", 0x00000B00);
        cp0.set_by_name_32("config", 0x0006E463);

        cp0
    }

    fn find_index(name: &'static str) -> usize {
        CP0_REGISTER_NAMES.iter().position(|v| *v == name).unwrap()
    }

    pub fn is_32bits(index: usize) -> bool {
        match index {
            0 | 1 | 5 | 6 | 9 | 11 | 12 | 13 | 15 | 16 | 17 | 18 | 19 | 26 | 27 | 28 | 29 => true,
            2 | 3 | 4 | 8 | 10 | 14 | 20 | 21 | 22 | 23 | 24 | 25 | 30 => false,
            _ => unreachable!(),
        }
    }

    pub fn is_64bits(index: usize) -> bool {
        !CP0Registers::is_32bits(index)
    }

    pub fn get_by_number_32(&self, index: usize) -> i32 {
        if index > 31 {
            unreachable!("Register number {} not valid", index);
        }
        match index {
            0  => self.index.get(),
            1  => self.random.get(),
            5  => self.page_mask.get(),
            6  => self.wired.get(),
            9  => self.count.get(),
            11 => self.compare.get(),
            12 => self.status.get(),
            13 => self.cause.get(),
            15 => self.prid.get(),
            16 => self.config.get(),
            17 => self.lladdr.get(),
            18 => self.watch_lo.get(),
            19 => self.watch_hi.get(),
            26 => self.parity_error.get(),
            27 => self.cache_error.get(),
            28 => self.tag_lo.get(),
            29 => self.tag_hi.get(),
            _ => unreachable!("Register {} is not 32bit", index),
        }
    }

    pub fn set_by_number_32(&mut self, index: usize, val: i32) {
        if index > 31 {
            unreachable!("Register number {} not valid", index);
        }
        match index {
            0  => self.index.set(val),
            1  => self.random.set(val),
            5  => self.page_mask.set(val),
            6  => self.wired.set(val),
            9  => self.count.set(val),
            11 => self.compare.set(val),
            12 => self.status.set(val),
            13 => self.cause.set(val),
            15 => self.prid.set(val),
            16 => self.config.set(val),
            17 => self.lladdr.set(val),
            18 => self.watch_lo.set(val),
            19 => self.watch_hi.set(val),
            26 => self.parity_error.set(val),
            27 => self.cache_error.set(val),
            28 => self.tag_lo.set(val),
            29 => self.tag_hi.set(val),
            _ => unreachable!("Register {} is not 32bit", index),
        };
    }

    pub fn get_by_number_64(&self, index: usize) -> i64 {
        if index > 31 {
            unreachable!("Register number {} not valid", index);
        }
        match index {
            2  => self.entry_lo_0.get(),
            3  => self.entry_lo_1.get(),
            4  => self.context.get(),
            7  => self.r7.get(),
            8  => self.bad_v_addr.get(),
            10 => self.entry_hi.get(),
            14 => self.epc.get(),
            20 => self.xcontext.get(),
            21 => self.r21.get(),
            22 => self.r22.get(),
            23 => self.r23.get(),
            24 => self.r24.get(),
            25 => self.r25.get(),
            30 => self.error_epc.get(),
            31 => self.r31.get(),
            _ => unreachable!("Register {} is not 64bit", index),
        }
    }

    pub fn set_by_number_64(&mut self, index: usize, val: i64) {
        if index > 31 {
            unreachable!("Register number {} not valid", index);
        }
        match index {
            2  => self.entry_lo_0.set(val),
            3  => self.entry_lo_1.set(val),
            4  => self.context.set(val),
            7  => self.r7.set(val),
            8  => self.bad_v_addr.set(val),
            10 => self.entry_hi.set(val),
            14 => self.epc.set(val),
            20 => self.xcontext.set(val),
            21 => self.r21.set(val),
            22 => self.r22.set(val),
            23 => self.r23.set(val),
            24 => self.r24.set(val),
            25 => self.r25.set(val),
            30 => self.error_epc.set(val),
            31 => self.r31.set(val),
            _ => unreachable!("Register {} is not 64bit", index),
        };
    }

    pub fn get_by_name_32(&self, name: &'static str) -> i32 {
        let index = CP0Registers::find_index(name);
        self.get_by_number_32(index)
    }

    pub fn set_by_name_32(&mut self, name: &'static str, val: i32) {
        let index = CP0Registers::find_index(name);
        self.set_by_number_32(index, val);
    }

    pub fn get_by_name_64(&self, name: &'static str) -> i64 {
        let index = CP0Registers::find_index(name);
        self.get_by_number_64(index)
    }

    pub fn set_by_name_64(&mut self, name: &'static str, val: i64) {
        let index = CP0Registers::find_index(name);
        self.set_by_number_64(index, val);
    }
}

#[cfg(test)]
mod cpu_registers_tests {
    use super::*;

    #[test]
    fn test_set_by_number() {
        let mut registers = CPURegisters::new();
        registers.set_by_number(0, 20);
        assert_eq!(registers.get_by_number(0), 0);
        registers.set_by_number(5, 20);
        assert_eq!(registers.get_by_number(5), 20);
    }

    #[test]
    fn test_set_by_name() {
        let mut registers = CPURegisters::new();
        registers.set_by_name("zero", 20);
        assert_eq!(registers.get_by_name("zero"), 0);
        registers.set_by_name("a0", 20);
        assert_eq!(registers.get_by_name("a0"), 20);
        assert_eq!(registers.get_by_number(4), 20);
    }
}

#[cfg(test)]
mod cp0_registers_tests {
    use super::*;

    #[test]
    fn test_set_by_number() {
        let mut registers = CP0Registers::new();
        registers.set_by_number_32(0, 20);
        assert_eq!(registers.get_by_number_32(0), 20);
        registers.set_by_number_64(4, 20);
        assert_eq!(registers.get_by_number_64(4), 20);
    }

    #[test]
    fn test_set_by_name() {
        let mut registers = CP0Registers::new();
        registers.set_by_name_32("index", 0);
        assert_eq!(registers.get_by_name_32("index"), 0);
        registers.set_by_name_64("context", 20);
        assert_eq!(registers.get_by_name_64("context"), 20);
        assert_eq!(registers.get_by_number_64(4), 20);
    }
}
