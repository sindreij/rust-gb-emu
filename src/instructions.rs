use std::fmt;

use crate::error::Error;
use crate::mem::Mmu;

#[derive(Debug, Eq, PartialEq)]
pub enum Instruction {
    Load8 { src: Loc8, dst: Loc8 },
    Load16 { src: Loc16, dst: Loc16 },
    XOR { src: Loc8, dst: Loc8 },
    Sub { src: Loc8 },
    AddA { src: Loc8 },
    CheckBit { bit: u8, loc: Loc8 },
    RotateLeftCarry { loc: Loc8 },
    RotateLeft { loc: Loc8 },
    JR { cond: Cond, offset: i8 },
    Inc8 { loc: Loc8 },
    Inc16 { loc: Loc16 },
    Dec8 { loc: Loc8 },
    Call { cond: Cond, addr: u16 },
    Return { cond: Cond },
    Push { loc: Loc16 },
    Pop { loc: Loc16 },
    // Other location is always A
    Compare { loc: Loc8 },
}

#[derive(Debug, Eq, PartialEq)]
pub enum Cond {
    Always,
    NotCarry,
    Carry,
    NotZero,
    Zero,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Loc8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    // (HL)
    IndHL,
    // (BC)
    IndBC,
    // (DE)
    IndDE,
    // (HL-)
    IndHLDec,
    // (HL+)
    IndHLInc,
    U8(u8),
    IOPlusC,
    IOPlus(u8),
    // (u16)
    IndU16(u16),
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Loc16 {
    BC,
    DE,
    HL,
    SP,
    U16(u16),
}

impl Instruction {
    // Returns the instruction and the number of bytes read
    pub fn parse(pc: u16, mmu: &Mmu) -> Result<(Instruction, u16), Error> {
        match mmu.read_u8(pc)? {
            0x01 => Ok((
                Instruction::Load16 {
                    src: Loc16::U16(mmu.read_u16(pc + 1)?),
                    dst: Loc16::BC,
                },
                3,
            )),
            0x02 => Ok((
                Instruction::Load8 {
                    src: Loc8::A,
                    dst: Loc8::IndBC,
                },
                1,
            )),
            0x03 => Ok((Instruction::Inc16 { loc: Loc16::BC }, 1)),
            0x04 => Ok((Instruction::Inc8 { loc: Loc8::B }, 1)),
            0x05 => Ok((Instruction::Dec8 { loc: Loc8::B }, 1)),
            0x06 => Ok((
                Instruction::Load8 {
                    src: Loc8::U8(mmu.read_u8(pc + 1)?),
                    dst: Loc8::B,
                },
                2,
            )),
            0x0a => Ok((
                Instruction::Load8 {
                    src: Loc8::IndBC,
                    dst: Loc8::A,
                },
                1,
            )),
            0x0c => Ok((Instruction::Inc8 { loc: Loc8::C }, 1)),
            0x0d => Ok((Instruction::Dec8 { loc: Loc8::C }, 1)),
            0x0e => Ok((
                Instruction::Load8 {
                    src: Loc8::U8(mmu.read_u8(pc + 1)?),
                    dst: Loc8::C,
                },
                2,
            )),
            0x11 => Ok((
                Instruction::Load16 {
                    src: Loc16::U16(mmu.read_u16(pc + 1)?),
                    dst: Loc16::DE,
                },
                3,
            )),
            0x12 => Ok((
                Instruction::Load8 {
                    src: Loc8::A,
                    dst: Loc8::IndDE,
                },
                1,
            )),
            0x13 => Ok((Instruction::Inc16 { loc: Loc16::DE }, 1)),
            0x14 => Ok((Instruction::Inc8 { loc: Loc8::D }, 1)),
            0x15 => Ok((Instruction::Dec8 { loc: Loc8::D }, 1)),
            0x16 => Ok((
                Instruction::Load8 {
                    src: Loc8::U8(mmu.read_u8(pc + 1)?),
                    dst: Loc8::D,
                },
                2,
            )),
            0x17 => Ok((Instruction::RotateLeft { loc: Loc8::A }, 1)),
            0x18 => Ok((
                Instruction::JR {
                    cond: Cond::Always,
                    offset: mmu.read_i8(pc + 1)?,
                },
                2,
            )),
            0x1a => Ok((
                Instruction::Load8 {
                    src: Loc8::IndDE,
                    dst: Loc8::A,
                },
                1,
            )),
            0x1c => Ok((Instruction::Inc8 { loc: Loc8::E }, 1)),
            0x1d => Ok((Instruction::Dec8 { loc: Loc8::E }, 1)),
            0x1e => Ok((
                Instruction::Load8 {
                    src: Loc8::U8(mmu.read_u8(pc + 1)?),
                    dst: Loc8::E,
                },
                2,
            )),
            0x20 => Ok((
                Instruction::JR {
                    cond: Cond::NotZero,
                    offset: mmu.read_i8(pc + 1)?,
                },
                2,
            )),
            0x21 => Ok((
                Instruction::Load16 {
                    src: Loc16::U16(mmu.read_u16(pc + 1)?),
                    dst: Loc16::HL,
                },
                3,
            )),
            0x22 => Ok((
                Instruction::Load8 {
                    src: Loc8::A,
                    dst: Loc8::IndHLInc,
                },
                1,
            )),
            0x23 => Ok((Instruction::Inc16 { loc: Loc16::HL }, 1)),
            0x24 => Ok((Instruction::Inc8 { loc: Loc8::H }, 1)),
            0x25 => Ok((Instruction::Dec8 { loc: Loc8::H }, 1)),
            0x26 => Ok((
                Instruction::Load8 {
                    src: Loc8::U8(mmu.read_u8(pc + 1)?),
                    dst: Loc8::H,
                },
                2,
            )),
            0x28 => Ok((
                Instruction::JR {
                    cond: Cond::Zero,
                    offset: mmu.read_i8(pc + 1)?,
                },
                2,
            )),
            0x2a => Ok((
                Instruction::Load8 {
                    src: Loc8::IndHLInc,
                    dst: Loc8::A,
                },
                1,
            )),
            0x2c => Ok((Instruction::Inc8 { loc: Loc8::L }, 1)),
            0x2d => Ok((Instruction::Dec8 { loc: Loc8::L }, 1)),
            0x2e => Ok((
                Instruction::Load8 {
                    src: Loc8::U8(mmu.read_u8(pc + 1)?),
                    dst: Loc8::L,
                },
                2,
            )),
            0x30 => Ok((
                Instruction::JR {
                    cond: Cond::NotCarry,
                    offset: mmu.read_i8(pc + 1)?,
                },
                2,
            )),
            0x31 => Ok((
                Instruction::Load16 {
                    src: Loc16::U16(mmu.read_u16(pc + 1)?),
                    dst: Loc16::SP,
                },
                3,
            )),
            0x32 => Ok((
                Instruction::Load8 {
                    src: Loc8::A,
                    dst: Loc8::IndHLDec,
                },
                1,
            )),
            0x33 => Ok((Instruction::Inc16 { loc: Loc16::SP }, 1)),
            0x34 => Ok((Instruction::Inc8 { loc: Loc8::IndHL }, 1)),
            0x35 => Ok((Instruction::Dec8 { loc: Loc8::IndHL }, 1)),
            0x36 => Ok((
                Instruction::Load8 {
                    src: Loc8::U8(mmu.read_u8(pc + 1)?),
                    dst: Loc8::IndHL,
                },
                2,
            )),
            0x38 => Ok((
                Instruction::JR {
                    cond: Cond::Carry,
                    offset: mmu.read_i8(pc + 1)?,
                },
                2,
            )),
            0x3a => Ok((
                Instruction::Load8 {
                    src: Loc8::IndHLDec,
                    dst: Loc8::A,
                },
                1,
            )),
            0x3c => Ok((Instruction::Inc8 { loc: Loc8::A }, 1)),
            0x3d => Ok((Instruction::Dec8 { loc: Loc8::A }, 1)),
            0x3e => Ok((
                Instruction::Load8 {
                    src: Loc8::U8(mmu.read_u8(pc + 1)?),
                    dst: Loc8::A,
                },
                2,
            )),
            // This needs to be above the below check since HALT is in the middle of the load stuff
            // between 40 and 7f
            0x76 => Err(Error::TODOHalt),
            inst @ 0x40...0x7f => {
                let high5 = inst & 0b11111000;
                let low3 = inst & 0x07;
                let src = match low3 {
                    0x0 => Loc8::B,
                    0x1 => Loc8::C,
                    0x2 => Loc8::D,
                    0x3 => Loc8::E,
                    0x4 => Loc8::H,
                    0x5 => Loc8::L,
                    0x6 => Loc8::IndHL,
                    0x7 => Loc8::A,
                    aaa => panic!("This should not be possible (low3, 76) ({:02x})", aaa),
                };
                let dst = match high5 {
                    0x40 => Loc8::B,
                    0x48 => Loc8::C,
                    0x50 => Loc8::D,
                    0x58 => Loc8::E,
                    0x60 => Loc8::H,
                    0x68 => Loc8::L,
                    0x70 => Loc8::IndHL,
                    0x78 => Loc8::A,
                    aaa => panic!("This should not be possible (high5, 76) ({:02x})", aaa),
                };

                Ok((Instruction::Load8 { src, dst }, 1))
            }
            inst @ 0x80...0xbf => {
                let high5 = inst & 0b11111000;
                let low3 = inst & 0x07;
                let src = match low3 {
                    0x0 => Loc8::B,
                    0x1 => Loc8::C,
                    0x2 => Loc8::D,
                    0x3 => Loc8::E,
                    0x4 => Loc8::H,
                    0x5 => Loc8::L,
                    0x6 => Loc8::IndHL,
                    0x7 => Loc8::A,
                    aaa => panic!("This should not be possible (low3, 76) ({:02x})", aaa),
                };
                let dst = Loc8::A;
                let inst = match high5 {
                    0x80 => Instruction::AddA { src },
                    0x90 => Instruction::Sub { src },
                    0xa8 => Instruction::XOR { src, dst },
                    0xb8 => Instruction::Compare { loc: src },
                    _ => return Err(Error::UnknownInstruction(inst)),
                };
                Ok((inst, 1))
            }
            0xc0 => Ok((
                Instruction::Return {
                    cond: Cond::NotZero,
                },
                1,
            )),
            0xc1 => Ok((Instruction::Pop { loc: Loc16::BC }, 1)),
            0xc5 => Ok((Instruction::Push { loc: Loc16::BC }, 1)),
            0xc8 => Ok((Instruction::Return { cond: Cond::Zero }, 1)),
            0xc9 => Ok((Instruction::Return { cond: Cond::Always }, 1)),
            0xcd => Ok((
                Instruction::Call {
                    cond: Cond::Always,
                    addr: mmu.read_u16(pc + 1)?,
                },
                3,
            )),
            0xd0 => Ok((
                Instruction::Return {
                    cond: Cond::NotCarry,
                },
                1,
            )),
            0xd1 => Ok((Instruction::Pop { loc: Loc16::DE }, 1)),
            0xd5 => Ok((Instruction::Push { loc: Loc16::DE }, 1)),
            0xd8 => Ok((Instruction::Return { cond: Cond::Carry }, 1)),
            0xe0 => Ok((
                Instruction::Load8 {
                    src: Loc8::A,
                    dst: Loc8::IOPlus(mmu.read_u8(pc + 1)?),
                },
                2,
            )),
            0xe2 => Ok((
                Instruction::Load8 {
                    src: Loc8::A,
                    dst: Loc8::IOPlusC,
                },
                1,
            )),
            0xe1 => Ok((Instruction::Pop { loc: Loc16::HL }, 1)),
            0xe5 => Ok((Instruction::Push { loc: Loc16::HL }, 1)),
            0xea => Ok((
                Instruction::Load8 {
                    src: Loc8::A,
                    dst: Loc8::IndU16(mmu.read_u16(pc + 1)?),
                },
                3,
            )),
            0xf0 => Ok((
                Instruction::Load8 {
                    src: Loc8::IOPlus(mmu.read_u8(pc + 1)?),
                    dst: Loc8::A,
                },
                2,
            )),
            0xfa => Ok((
                Instruction::Load8 {
                    src: Loc8::IndU16(mmu.read_u16(pc + 1)?),
                    dst: Loc8::A,
                },
                3,
            )),
            0xfe => Ok((
                Instruction::Compare {
                    loc: Loc8::U8(mmu.read_u8(pc + 1)?),
                },
                2,
            )),
            // 0xf5 => Ok((Instruction::Push { loc: Loc16::AF }, 1)),
            // CB Prefix
            0xcb => {
                let inst = mmu.read_u8(pc + 1)?;
                let high5 = inst & 0b11111000;
                let low3 = inst & 0x07;
                use Loc8::*;
                let loc = match low3 {
                    0x0 => B,
                    0x1 => C,
                    0x2 => D,
                    0x3 => E,
                    0x4 => H,
                    0x5 => L,
                    0x6 => IndHL,
                    _ => panic!("This should not be possible"),
                };

                let res = match high5 {
                    0x00 => Ok(Instruction::RotateLeftCarry { loc }),
                    0x10 => Ok(Instruction::RotateLeft { loc }),
                    0x40 => Ok(Instruction::CheckBit { bit: 0, loc }),
                    0x48 => Ok(Instruction::CheckBit { bit: 1, loc }),
                    0x50 => Ok(Instruction::CheckBit { bit: 2, loc }),
                    0x58 => Ok(Instruction::CheckBit { bit: 3, loc }),
                    0x60 => Ok(Instruction::CheckBit { bit: 4, loc }),
                    0x68 => Ok(Instruction::CheckBit { bit: 5, loc }),
                    0x70 => Ok(Instruction::CheckBit { bit: 6, loc }),
                    0x78 => Ok(Instruction::CheckBit { bit: 7, loc }),
                    _ => Err(Error::UnknownCbInstruction(inst)),
                }?;

                Ok((res, 2))
            }
            unknown => Err(Error::UnknownInstruction(unknown)),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Instruction::*;
        match self {
            Load8 { dst, src } => write!(f, "LD {},{}", dst, src),
            Load16 { dst, src } => write!(f, "LD {},{}", dst, src),
            XOR { dst, src } => write!(f, "XOR {},{}", dst, src),
            Sub { src } => write!(f, "SUB A,{}", src),
            AddA { src } => write!(f, "ADD A,{}", src),
            Inc8 { loc } => write!(f, "INC {}", loc),
            Inc16 { loc } => write!(f, "INC {}", loc),
            Dec8 { loc } => write!(f, "DEC {}", loc),
            Compare { loc } => write!(f, "CP A,{}", loc),
            CheckBit { bit, loc } => write!(f, "BIT {},{}", bit, loc),
            RotateLeftCarry { loc } => write!(f, "RLC {}", loc),
            RotateLeft { loc } => write!(f, "RL {}", loc),
            JR { cond, offset } => write!(f, "JR {}${:02x}", cond, offset),
            Call { cond, addr } => write!(f, "CALL {}${:04x}", cond, addr),
            Return { cond } => write!(f, "RET {}", cond),
            Push { loc } => write!(f, "PUSH {}", loc),
            Pop { loc } => write!(f, "POP {}", loc),
        }
    }
}

impl fmt::Display for Loc8 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Loc8::A => write!(f, "A"),
            Loc8::B => write!(f, "B"),
            Loc8::C => write!(f, "C"),
            Loc8::D => write!(f, "D"),
            Loc8::E => write!(f, "E"),
            Loc8::H => write!(f, "H"),
            Loc8::L => write!(f, "L"),
            Loc8::IndHL => write!(f, "(HL)"),
            Loc8::IndBC => write!(f, "(BC)"),
            Loc8::IndDE => write!(f, "(DE)"),
            Loc8::IndHLDec => write!(f, "(HL-)"),
            Loc8::IndHLInc => write!(f, "(HL+)"),
            Loc8::IOPlusC => write!(f, "(FF00+C)"),
            Loc8::IOPlus(val) => write!(f, "(FF00+${:02x})", val),
            Loc8::U8(val) => write!(f, "${:02x}", val),
            Loc8::IndU16(addr) => write!(f, "({:04x})", addr),
        }
    }
}

impl fmt::Display for Loc16 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Loc16::HL => write!(f, "HL"),
            Loc16::BC => write!(f, "BC"),
            Loc16::DE => write!(f, "DE"),
            Loc16::SP => write!(f, "SP"),
            Loc16::U16(val) => write!(f, "${:04x}", val),
        }
    }
}

impl fmt::Display for Cond {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cond::NotZero => write!(f, "NZ,"),
            Cond::Zero => write!(f, "Z,"),
            Cond::NotCarry => write!(f, "NC,"),
            Cond::Carry => write!(f, "C,"),
            Cond::Always => write!(f, ""),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::mem::Mmu;

    #[test]
    fn test_cb_c7() {
        let input = Mmu::with_mem(vec![0xcb, 0x7c]);
        let (inst, delta) = Instruction::parse(0, &input);
        assert_eq!(delta, 2);
        assert_eq!(
            inst,
            Instruction::CheckBit {
                bit: 7,
                loc: Loc8::H
            }
        );
    }
}
