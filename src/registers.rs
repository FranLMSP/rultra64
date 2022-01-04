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
    "zero", "at", "v0", "v1", "a0", "a1", "a2", "a3", "t0", "t1", "t2",
    "t3",   "t4", "t5", "t6", "t7", "s0", "s1", "s2", "s3", "s4", "s5",
    "s6",   "s7", "t8", "t9", "k0", "k1", "gp", "sp", "s8", "ra"
];

pub struct CPURegisters {
    registers: [Box<dyn Register<i64>>; 32],
    program_counter: Generic<i64>,
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
        }
    }

    pub fn get_by_number(&self, index: usize) -> i64 {
        if index > 31 {
            unreachable!("Register number {} not valid", index);
        }
        self.registers[index].get()
    }

    pub fn get_by_name(&self, name: &'static str) -> i64 {
        let index = CPU_REGISTER_NAMES.iter().position(|v| *v == name).unwrap();
        self.registers[index].get()
    }

    pub fn get_program_counter(&self) -> i64 {
        self.program_counter.get()
    }

    pub fn set_by_number(&mut self, index: usize, val: i64) {
        if index > 31 {
            unreachable!("Register number {} not valid", index);
        }
        self.registers[index].set(val);
    }

    pub fn set_by_name(&mut self, name: &'static str, val: i64) {
        let index = CPU_REGISTER_NAMES.iter().position(|v| *v == name).unwrap();
        self.registers[index].set(val);
    }

    pub fn set_program_counter(&mut self, val: i64) {
        self.program_counter.set(val);
    }

    pub fn increment_program_counter(&mut self, val: i64) {
        self.program_counter.set(self.program_counter.get().wrapping_add(val));
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
