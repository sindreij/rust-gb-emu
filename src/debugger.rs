use crate::{cpu::Cpu, error::Error, mem::Mmu};

pub struct Debugger {
    mmu: Mmu,
    cpu: Cpu,
}

impl Debugger {
    pub fn new(mmu: Mmu, cpu: Cpu) -> Debugger {
        Debugger { mmu, cpu }
    }

    pub fn run(self) -> Result<(), Error> {
        Ok(())
    }
}
