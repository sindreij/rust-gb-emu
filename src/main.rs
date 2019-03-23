mod cpu;
mod instructions;
mod mem;

use std::error::Error;

use structopt::StructOpt;

use instructions::Instruction;
use mem::Mmu;

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "gbemu")]
enum Opt {
    #[structopt(name = "disassemble_bootrom")]
    DisassembleBootrom,
}

fn main() -> Result<(), Box<Error>> {
    let matches = Opt::from_args();

    match matches {
        Opt::DisassembleBootrom => disassemble_bootrom(),
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
