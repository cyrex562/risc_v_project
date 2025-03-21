pub struct Cpu {
    pub regs: [u32; 32],
    pub pc: u32,
    pub csr: [u32; 4096],
}

pub enum Instruction {
    
}