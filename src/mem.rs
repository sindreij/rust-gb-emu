use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Mmu {
    mem: Vec<u8>,
}

impl Mmu {
    pub fn init() -> Result<Mmu, Box<Error>> {
        let mut boot_rom_file = File::open("roms/DMG_ROM.bin")?;
        let mut mem = Vec::new();
        boot_rom_file.read_to_end(&mut mem)?;

        Ok(Mmu { mem })
    }

    pub fn with_mem(mem: Vec<u8>) -> Mmu {
        Mmu { mem }
    }

    pub fn read_u8(&self, addr: u16) -> u8 {
        self.mem[addr as usize]
    }

    pub fn read_u16(&self, addr: u16) -> u16 {
        let first = self.read_u8(addr);
        let second = self.read_u8(addr + 1);

        (first as u16) + ((second as u16) << 8)
    }
}
