mod cpu;
mod error;
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
    Run { rom_file: String },
}

fn main() {
    if let Err(err) = main_() {
        println!("----\nExecution stopped with error:\n{}", err);
    }
}

fn main_() -> Result<(), Box<Error>> {
    let matches = Opt::from_args();

    match matches {
        Opt::DisassembleBootrom => disassemble_bootrom(),
        Opt::Run { rom_file } => run(&rom_file),
    }
}

fn run(rom_file: &str) -> Result<(), Box<Error>> {
    let mut mmu = Mmu::empty();
    mmu.load_game_rom(rom_file)?;
    mmu.load_boot_rom()?;
    let mut cpu = Cpu::default();

    if let Err(err) = game_loop(&mut cpu, &mut mmu) {
        println!(
            "----\nExecution stopped while running simulation:\n{}\n\nDumping memory to memdump.hex",
            err
        );
        println!("Registers:\n{}", cpu);
        mmu.dump_to_file("memdump.hex")?;
    }

    Ok(())
}

fn game_loop(cpu: &mut Cpu, mmu: &mut Mmu) -> Result<(), Box<Error>> {
    loop {
        cpu.step(mmu)?;
    }
}

fn disassemble_bootrom() -> Result<(), Box<Error>> {
    let mut mmu = Mmu::empty();
    mmu.load_boot_rom()?;
    let mmu = mmu;

    let mut pc = 0;

    loop {
        let (inst, delta) = Instruction::parse(pc, &mmu)?;
        println!("{:04x}    {}", pc, inst);
        pc += delta;
    }
}
