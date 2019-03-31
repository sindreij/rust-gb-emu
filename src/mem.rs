use std::fs::File;
use std::io::prelude::*;

use crate::error::Error;

pub struct Mmu {
    mem: Vec<u8>,
}

impl Mmu {
    pub fn empty() -> Mmu {
        let mut mem = vec![0; 65535];
        // Simulate always beeing in vblank :)
        mem[0xff44] = 0x90;
        Mmu { mem }
    }

    pub fn load_game_rom(&mut self, rom_file: &str) -> Result<(), Error> {
        let mut game_rom_file = File::open(rom_file)?;
        let mut game_rom = Vec::new();
        game_rom_file.read_to_end(&mut game_rom)?;
        assert_eq!(game_rom.len(), 0x8000);
        // TODO: We need to save the first 256 bytes and put that in when the booting is finished
        // The first 256 bytes will be overwritten when the boot rom is loaded
        let (left, _) = self.mem.split_at_mut(0x8000);
        left.copy_from_slice(&game_rom);

        Ok(())
    }

    pub fn load_boot_rom(&mut self) -> Result<(), Error> {
        let mut boot_rom_file = File::open("roms/DMG_ROM.bin")?;
        let mut boot_rom = Vec::new();
        boot_rom_file.read_to_end(&mut boot_rom)?;
        assert_eq!(boot_rom.len(), 256);

        let (left, _) = self.mem.split_at_mut(256);
        left.copy_from_slice(&boot_rom);

        Ok(())
    }

    pub fn dump_to_file(&self, filename: &str) -> Result<(), Error> {
        // TODO: Use read_u8 to read IO registers correctly
        let mut file = File::create(filename)?;
        file.write_all(&self.mem)?;
        Ok(())
    }

    #[cfg(test)]
    pub fn with_mem(mem: Vec<u8>) -> Mmu {
        Mmu { mem }
    }

    fn read_ram(&self, addr: u16) -> Result<u8, Error> {
        self.mem
            .get(addr as usize)
            .cloned()
            .ok_or(Error::InvalidReadFromMemoryLocation(addr))
    }

    fn write_ram(&mut self, addr: u16, val: u8) -> Result<(), Error> {
        let location = self
            .mem
            .get_mut(addr as usize)
            .ok_or(Error::InvalidWriteToMemoryLocation(addr))?;

        *location = val;

        Ok(())
    }

    fn write_io_register(&mut self, addr: u16, val: u8) -> Result<(), Error> {
        if addr == 0xff50 {
            // TODO: Turn off bootrom
            return Err(Error::Abort("Bootrom finished, aborting for now"));
        }
        // Also write to ram for easier debugging and to make read and write work for now
        self.write_ram(addr, val)?;
        Ok(())
    }

    pub fn read_u8(&self, addr: u16) -> Result<u8, Error> {
        self.read_ram(addr)
    }

    pub fn write_u8(&mut self, addr: u16, val: u8) -> Result<(), Error> {
        match addr {
            // IO
            0x0000...0xfeff => self.write_ram(addr, val),
            0xff00...0xffff => self.write_io_register(addr, val),
        }
    }

    pub fn write_u16(&mut self, addr: u16, val: u16) -> Result<(), Error> {
        let high = (val >> 8) as u8;
        let low = (val & 0xff) as u8;
        self.write_u8(addr + 1, high)?;
        self.write_u8(addr, low)?;

        Ok(())
    }

    pub fn read_i8(&self, addr: u16) -> Result<i8, Error> {
        Ok(self.read_u8(addr)? as i8)
    }

    pub fn read_u16(&self, addr: u16) -> Result<u16, Error> {
        let first = self.read_u8(addr)?;
        let second = self.read_u8(addr + 1)?;

        Ok((first as u16) + ((second as u16) << 8))
    }
}
