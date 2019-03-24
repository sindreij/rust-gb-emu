use crate::{
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

impl Cpu {
    fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) + (self.l as u16)
    }

    fn set_hl(&mut self, val: u16) {
        self.l = (val & 0xff) as u8;
        self.h = (val >> 8) as u8;
    }

    fn get_loc8(&mut self, loc: Loc8, mmu: &Mmu) -> u8 {
        use Loc8::*;
        match loc {
            A => self.a,
            B => self.b,
            C => self.c,
            D => self.d,
            E => self.e,
            H => self.h,
            L => self.l,
            // (HL)
            IndHL => mmu.read_u8(self.get_hl()),
            // (HL-)
            IndHLDec => {
                let hl = self.get_hl();
                self.set_hl(hl - 1);
                mmu.read_u8(hl)
            }
        }
    }

    fn set_loc8(&mut self, loc: Loc8, mmu: &mut Mmu, val: u8) {
        match loc {
            Loc8::A => self.a = val,
            Loc8::B => self.b = val,
            Loc8::C => self.c = val,
            Loc8::D => self.d = val,
            Loc8::E => self.e = val,
            Loc8::H => self.h = val,
            Loc8::L => self.l = val,
            // (HL)
            Loc8::IndHL => mmu.write_u8(self.get_hl(), val),
            // (HL-)
            Loc8::IndHLDec => {
                let hl = self.get_hl();
                self.set_hl(hl - 1);
                mmu.write_u8(hl, val);
            }
        }
    }

    fn get_loc16(&self, loc: Loc16) -> u16 {
        match loc {
            Loc16::HL => self.get_hl(),
            Loc16::SP => self.sp,
            Loc16::U16(val) => val,
        }
    }

    fn set_loc16(&mut self, loc: Loc16, val: u16) {
        match loc {
            Loc16::HL => self.set_hl(val),
            Loc16::SP => self.sp = val,
            Loc16::U16(_) => panic!("Can't set a u16 location"),
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

    pub fn step(&mut self, mmu: &mut Mmu) {
        let (inst, delta) = Instruction::parse(self.pc, &mmu);
        println!("{:04x}    {}", self.pc, inst);
        self.pc += delta;

        use Instruction::*;
        match inst {
            Load8 { src, dst } => {
                let val = self.get_loc8(src, mmu);
                self.set_loc8(dst, mmu, val);
            }
            Load16 { src, dst } => {
                let val = self.get_loc16(src);
                self.set_loc16(dst, val);
            }
            XOR { src, dst } => {
                let srcval = self.get_loc8(src, mmu);
                let dstval = self.get_loc8(dst, mmu);
                let res = srcval ^ dstval;
                self.set_loc8(dst, mmu, res);
            }
            CheckBit { bit, loc } => {
                let mask = 1 << bit;
                self.flags.zero = self.get_loc8(loc, mmu) & mask == 0;
            }
            JR { cond, offset } => {
                if self.check_cond(cond) {
                    self.pc = ((self.pc as i16) + (offset as i16)) as u16;
                }
            }
        }
    }
}
