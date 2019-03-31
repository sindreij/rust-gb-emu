use std::collections::BTreeSet;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use rustyline::{error::ReadlineError, Editor};
use structopt::StructOpt;

use crate::{cpu::Cpu, error::Error, mem::Mmu};

pub struct Debugger {
    mmu: Mmu,
    cpu: Cpu,
    interrupt: Arc<AtomicBool>,
    breakpoints: BTreeSet<u16>,
}

fn parse_hex_16(src: &str) -> Result<u16, std::num::ParseIntError> {
    u16::from_str_radix(src, 16)
}

// Parse readline commands
#[derive(StructOpt, Debug)]
#[structopt(raw(setting = "structopt::clap::AppSettings::NoBinaryName"))]
enum Opt {
    #[structopt(name = "run")]
    Run,
    #[structopt(name = "step", alias = "s")]
    Step,
    #[structopt(name = "reg")]
    PrintRegister { register: String },
    #[structopt(name = "regs")]
    PrintRegisters,
    #[structopt(name = "mem8", alias = "mem")]
    PrintMem8 {
        #[structopt(parse(try_from_str = "parse_hex_16"))]
        addr: u16,
    },
    #[structopt(name = "break")]
    Break {
        #[structopt(parse(try_from_str = "parse_hex_16"))]
        addr: u16,
    },
    #[structopt(name = "break_clear")]
    ClearBreakepoints,
    #[structopt(name = "inst")]
    PrintNextInstruction,
    #[structopt(name = "dumpmem")]
    DumpMemory,
}

impl Debugger {
    pub fn new(mmu: Mmu, cpu: Cpu) -> Debugger {
        Debugger {
            mmu,
            cpu,
            interrupt: Arc::new(AtomicBool::new(false)),
            breakpoints: BTreeSet::new(),
        }
    }

    pub fn run(mut self) -> Result<(), Error> {
        self.interrupt.store(false, Ordering::SeqCst);
        ctrlc::set_handler({
            let interrupt = self.interrupt.clone();
            move || {
                interrupt.store(true, Ordering::SeqCst);
            }
        })?;

        let mut rl = Editor::<()>::new();
        if rl.load_history("history.txt").is_err() {
            println!("No previous history.");
        }

        let mut last = String::new();

        loop {
            let readline = rl.readline(&format!("{:04x} >> ", self.cpu.pc));
            match readline {
                Ok(line) => {
                    let line = if line == "" { last } else { line };

                    rl.add_history_entry(line.as_ref());

                    let args: Vec<_> = line.split(" ").collect();
                    if let Err(err) = self.run_command(&args) {
                        println!("Running command failed: {}", err);
                    }

                    last = line;
                }
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-C");
                }
                Err(ReadlineError::Eof) => {
                    println!("CTRL-D");
                    break;
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            }
        }

        rl.save_history("history.txt").unwrap();

        Ok(())
    }

    fn run_command(&mut self, args: &[&str]) -> Result<(), Error> {
        let opt = Opt::from_iter_safe(args)?;

        use Opt::*;
        match opt {
            Run => self.game_loop(),
            Step => self.step()?,
            PrintRegister { register } => self.print_register(&register),
            PrintRegisters => println!("{}", self.cpu),
            PrintMem8 { addr } => println!("(${:04x}) = {:04x}", addr, self.mmu.read_u8(addr)?),
            PrintNextInstruction => self.cpu.print_next(&self.mmu)?,
            Break { addr } => {
                self.breakpoints.insert(addr);
                println!("Added breakpoint @ {:04x}", addr);
            }
            ClearBreakepoints => self.breakpoints.clear(),
            DumpMemory => {
                self.mmu.dump_to_file("dbgdump.hex")?;
                println!("Memory dumped to dbgdump.hex");
            }
        };

        Ok(())
    }

    fn print_register(&self, register: &str) {
        match register.to_lowercase().as_ref() {
            "a" => println!("a = {:02x}", self.cpu.a),
            "b" => println!("b = {:02x}", self.cpu.b),
            "c" => println!("c = {:02x}", self.cpu.c),
            "d" => println!("d = {:02x}", self.cpu.d),
            "e" => println!("e = {:02x}", self.cpu.e),
            "h" => println!("h = {:02x}", self.cpu.h),
            "l" => println!("l = {:02x}", self.cpu.l),
            "pc" => println!("pc = {:04x}", self.cpu.pc),
            "sp" => println!("sp = {:04x}", self.cpu.sp),
            "hl" => println!("hl = {:04x}", self.cpu.get_hl()),
            "bc" => println!("bc = {:04x}", self.cpu.get_bc()),
            "de" => println!("de = {:04x}", self.cpu.get_de()),
            unknown => println!("Unknown register: {}", unknown),
        }
    }

    fn step(&mut self) -> Result<(), Error> {
        self.cpu.print_next(&self.mmu)?;
        self.cpu.step(&mut self.mmu)
    }

    fn game_loop(&mut self) {
        self.interrupt.store(false, Ordering::SeqCst);

        let res: Result<(), Error> = (|| loop {
            if self.interrupt.load(Ordering::SeqCst) {
                return Ok(());
            }
            if self.breakpoints.contains(&self.cpu.pc) {
                println!("Breakpoint @ {:04x}", self.cpu.pc);
                return Ok(());
            }
            self.cpu.step(&mut self.mmu)?;
        })();

        if let Err(err) = res {
            println!("Execution stopped: {}", err);
        }
    }
}
