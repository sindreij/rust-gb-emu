use std::fmt;

use crate::mem::Mmu;

#[derive(Debug)]
pub enum Instruction {
    Load { src: Loc, dst: Loc },
    XOR { src: Loc, dst: Loc },
}

#[derive(Debug)]
pub enum Loc {
    A,
    SP,
    HL,
    // (HL-)
    IndHLDec,
    U16(u16),
}

impl Instruction {
    // Returns the instruction and the number of bytes read
    pub fn parse(pc: u16, mmu: &Mmu) -> (Instruction, u16) {
        match mmu.read_u8(pc) {
            0x21 => (
                Instruction::Load {
                    src: Loc::U16(mmu.read_u16(pc + 1)),
                    dst: Loc::HL,
                },
                3,
            ),
            0x31 => (
                Instruction::Load {
                    src: Loc::U16(mmu.read_u16(pc + 1)),
                    dst: Loc::SP,
                },
                3,
            ),
            0x32 => (
                Instruction::Load {
                    src: Loc::A,
                    dst: Loc::IndHLDec,
                },
                1,
            ),
            0xaf => (
                Instruction::XOR {
                    src: Loc::A,
                    dst: Loc::A,
                },
                1,
            ),
            // CB Prefix
            0xcb => match mmu.read_u8(pc + 1) {
                unknown => panic!("Unknown instruction `CB {:x}`", unknown),
            },
            unknown => panic!("Unknown instruction `{:x}`", unknown),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Instruction::*;
        match self {
            Load { dst, src } => write!(f, "LD {},{}", dst, src),
            XOR { dst, src } => write!(f, "XOR {},{}", dst, src),
        }
    }
}

impl fmt::Display for Loc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Loc::A => write!(f, "A"),
            Loc::SP => write!(f, "SP"),
            Loc::HL => write!(f, "HL"),
            Loc::IndHLDec => write!(f, "(HL-)"),
            Loc::U16(val) => write!(f, "${:x}", val),
        }
    }
}
