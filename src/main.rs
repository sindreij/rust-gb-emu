mod cpu;
mod instructions;
mod mem;

use std::error::Error;

use structopt::StructOpt;

use cpu::Cpu;
use instructions::Instruction;
use mem::Mmu;

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "gbemu")]
enum Opt {
    #[structopt(name = "disassemble_bootrom")]
    DisassembleBootrom,
    #[structopt(name = "run")]
    Run,
}

fn main() -> Result<(), Box<Error>> {
    let matches = Opt::from_args();

    match matches {
        Opt::DisassembleBootrom => disassemble_bootrom(),
        Opt::Run => run(),
    }
}

fn run() -> Result<(), Box<Error>> {
    let mut mmu = Mmu::init()?;
    let mut cpu = Cpu::default();

    loop {
        cpu.step(&mut mmu);
    }
}

fn disassemble_bootrom() -> Result<(), Box<Error>> {
    let mmu = Mmu::init()?;

    let mut pc = 0;

    loop {
        let (inst, delta) = Instruction::parse(pc, &mmu);
        println!("{:04x}    {}", pc, inst);
        pc += delta;
    }
}
