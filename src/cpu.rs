use std::fmt;

use crate::{
    error::Error,
    instructions::{Cond, Instruction, Loc16, Loc8},
    mem::Mmu,
};

#[derive(Default, Debug)]
pub struct Cpu {
    sp: u16,
    pc: u16,
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    flags: Flags,
}

#[derive(Default, Debug)]
pub struct Flags {
    // Z
    zero: bool,
    // N
    subtract: bool,
    // H
    half_carry: bool,
    // C
    carry: bool,
}

impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "sp: {:04x}", self.sp)?;
        writeln!(f, "pc: {:04x}", self.pc)?;
        writeln!(f, "a: {:02x}", self.a)?;
        writeln!(f, "b: {:02x}", self.b)?;
        writeln!(f, "c: {:02x}", self.c)?;
        writeln!(f, "d: {:02x}", self.d)?;
        writeln!(f, "e: {:02x}", self.e)?;
        writeln!(f, "h: {:02x}", self.h)?;
        writeln!(f, "l: {:02x}", self.l)?;
        writeln!(f, "hl: {:04x}", self.get_hl())?;
        writeln!(f, "bc: {:04x}", self.get_bc())?;
        writeln!(f, "de: {:04x}", self.get_de())?;
        writeln!(f, "flags: {:?}", self.flags)?;

        Ok(())
    }
}

impl Cpu {
    fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) + (self.l as u16)
    }

    fn set_hl(&mut self, val: u16) {
        self.h = (val >> 8) as u8;
        self.l = (val & 0xff) as u8;
    }

    fn get_bc(&self) -> u16 {
        ((self.b as u16) << 8) + (self.c as u16)
    }

    fn set_bc(&mut self, val: u16) {
        self.b = (val >> 8) as u8;
        self.c = (val & 0xff) as u8;
    }

    fn get_de(&self) -> u16 {
        ((self.d as u16) << 8) + (self.e as u16)
    }

    fn set_de(&mut self, val: u16) {
        self.d = (val >> 8) as u8;
        self.e = (val & 0xff) as u8;
    }

    fn get_loc8(&mut self, loc: Loc8, mmu: &Mmu) -> Result<u8, Error> {
        use Loc8::*;
        let res = match loc {
            A => self.a,
            B => self.b,
            C => self.c,
            D => self.d,
            E => self.e,
            H => self.h,
            L => self.l,
            // (HL)
            IndHL => mmu.read_u8(self.get_hl())?,
            IndBC => mmu.read_u8(self.get_bc())?,
            IndDE => mmu.read_u8(self.get_de())?,
            // (HL-)
            IndHLDec => {
                let hl = self.get_hl();
                self.set_hl(hl - 1);
                mmu.read_u8(hl)?
            }
            IndHLInc => {
                let hl = self.get_hl();
                self.set_hl(hl + 1);
                mmu.read_u8(hl)?
            }
            IOPlusC => mmu.read_u8(0xff00 + (self.c as u16))?,
            IOPlus(pos) => mmu.read_u8(0xff00 + (pos as u16))?,
            U8(val) => val,
        };

        Ok(res)
    }

    fn set_loc8(&mut self, loc: Loc8, mmu: &mut Mmu, val: u8) -> Result<(), Error> {
        match loc {
            Loc8::A => self.a = val,
            Loc8::B => self.b = val,
            Loc8::C => self.c = val,
            Loc8::D => self.d = val,
            Loc8::E => self.e = val,
            Loc8::H => self.h = val,
            Loc8::L => self.l = val,
            // (HL)
            Loc8::IndHL => mmu.write_u8(self.get_hl(), val)?,
            Loc8::IndBC => mmu.write_u8(self.get_bc(), val)?,
            Loc8::IndDE => mmu.write_u8(self.get_de(), val)?,
            // (HL-)
            Loc8::IndHLDec => {
                let hl = self.get_hl();
                self.set_hl(hl - 1);
                mmu.write_u8(hl, val)?;
            }
            Loc8::IndHLInc => {
                let hl = self.get_hl();
                self.set_hl(hl + 1);
                mmu.write_u8(hl, val)?;
            }
            Loc8::IOPlusC => mmu.write_u8(0xff00 + (self.c as u16), val)?,
            Loc8::IOPlus(pos) => mmu.write_u8(0xff00 + (pos as u16), val)?,
            Loc8::U8(_) => panic!("Invalid write to a const u8 value"),
        }

        Ok(())
    }

    fn get_loc16(&self, loc: Loc16) -> u16 {
        match loc {
            Loc16::HL => self.get_hl(),
            Loc16::BC => self.get_bc(),
            Loc16::DE => self.get_de(),
            Loc16::SP => self.sp,
            Loc16::U16(val) => val,
        }
    }

    fn set_loc16(&mut self, loc: Loc16, val: u16) {
        match loc {
            Loc16::HL => self.set_hl(val),
            Loc16::BC => self.set_bc(val),
            Loc16::DE => self.set_de(val),
            Loc16::SP => self.sp = val,
            Loc16::U16(_) => panic!("Invalid write to a const u16 value"),
        }
    }

    fn check_cond(&self, cond: Cond) -> bool {
        match cond {
            Cond::NotZero => !self.flags.zero,
            Cond::Zero => self.flags.zero,
            Cond::NotCarry => !self.flags.carry,
            Cond::Carry => self.flags.carry,
            Cond::Always => true,
        }
    }

    pub fn step(&mut self, mmu: &mut Mmu) -> Result<(), Error> {
        let (inst, delta) = Instruction::parse(self.pc, &mmu)?;
        println!("{:04x}    {}", self.pc, inst);
        self.pc += delta;

        use Instruction::*;
        match inst {
            Load8 { src, dst } => {
                let val = self.get_loc8(src, mmu)?;
                self.set_loc8(dst, mmu, val)?;
            }
            Load16 { src, dst } => {
                let val = self.get_loc16(src);
                self.set_loc16(dst, val);
            }
            Inc8 { loc } => {
                let val = self.get_loc8(loc, mmu)?;
                self.set_loc8(loc, mmu, val.wrapping_add(1))?;
            }
            Inc16 { loc } => {
                let val = self.get_loc16(loc);
                self.set_loc16(loc, val.wrapping_add(1));
            }
            Dec8 { loc } => {
                let val = self.get_loc8(loc, mmu)?;
                self.set_loc8(loc, mmu, val.wrapping_sub(1))?;
                self.flags.zero = val == 0;
                self.flags.subtract = true;
                self.flags.half_carry = val & 0xff == 0xff;
            }
            XOR { src, dst } => {
                let srcval = self.get_loc8(src, mmu)?;
                let dstval = self.get_loc8(dst, mmu)?;
                let res = srcval ^ dstval;
                self.set_loc8(dst, mmu, res)?;
            }
            CheckBit { bit, loc } => {
                let mask = 1 << bit;
                self.flags.zero = self.get_loc8(loc, mmu)? & mask == 0;
                self.flags.subtract = false;
                self.flags.half_carry = true;
            }
            RotateLeftCarry { loc } => {
                let val = self.get_loc8(loc, mmu)?;
                let val = val.rotate_left(1);
                self.set_loc8(loc, mmu, val)?;
                self.flags.zero = val == 0;
                self.flags.carry = val & 1 == 1;
                self.flags.subtract = false;
                self.flags.half_carry = false;
            }
            RotateLeft { loc } => {
                let val = self.get_loc8(loc, mmu)?;
                let val = val.rotate_left(1);
                // carry is set to the bit that was moved to the start
                let new_carry = val & 1 == 1;
                // the rightmost bit is set to the old carry
                let val = val | (self.flags.carry as u8);
                self.set_loc8(loc, mmu, val)?;
                self.flags.zero = val == 0;
                self.flags.carry = new_carry;
                self.flags.subtract = false;
                self.flags.half_carry = false;
            }
            JR { cond, offset } => {
                if self.check_cond(cond) {
                    self.pc = ((self.pc as i16) + (offset as i16)) as u16;
                }
            }
            Call { cond, addr } => {
                if self.check_cond(cond) {
                    mmu.write_u16(self.sp - 1, self.pc)?;
                    self.sp -= 2;
                    self.pc = addr;
                }
            }
            Push { loc } => {
                let value = self.get_loc16(loc);
                mmu.write_u16(self.sp - 1, value)?;
                self.sp -= 2;
            }
            Pop { loc } => {
                self.sp += 2;
                let value = mmu.read_u16(self.sp - 1)?;
                self.set_loc16(loc, value);
            }
        }

        Ok(())
    }
}
