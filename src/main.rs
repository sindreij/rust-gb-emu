mod cpu;
mod debugger;
mod error;
mod instructions;
mod mem;

use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use structopt::StructOpt;

use cpu::Cpu;
use debugger::Debugger;
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
    #[structopt(name = "debug")]
    Debug { rom_file: String },
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
        Opt::Debug { rom_file } => debug(&rom_file),
    }
}

fn debug(rom_file: &str) -> Result<(), Box<Error>> {
    let mut mmu = Mmu::empty();
    mmu.load_game_rom(rom_file)?;
    mmu.load_boot_rom()?;
    let cpu = Cpu::default();

    Debugger::new(mmu, cpu).run()?;

    Ok(())
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
    let interrupt = Arc::new(AtomicBool::new(false));
    ctrlc::set_handler({
        let interrupt = interrupt.clone();
        move || {
            interrupt.store(true, Ordering::Relaxed);
        }
    })?;

    loop {
        cpu.print_next(mmu)?;
        cpu.step(mmu)?;

        if interrupt.load(Ordering::Relaxed) {
            return Err(crate::error::Error::Abort("Interrupt").into());
        }
    }
}

fn disassemble_bootrom() -> Result<(), Box<Error>> {
    let mut mmu = Mmu::empty();
    mmu.load_boot_rom()?;
    let mmu = mmu;

    let mut pc = 0;

    while pc < 0x100 {
        if pc > 0xa7 && pc < 0xe0 {
            // DATA
            print!("{:02x} ", mmu.read_u8(pc)?);
            if pc == 0xdf {
                println!("");
            }
            pc += 1;
            continue;
        }

        let (inst, delta) = Instruction::parse(pc, &mmu)?;
        println!("{:04x}    {}", pc, inst);
        pc += delta;
    }

    Ok(())
}
