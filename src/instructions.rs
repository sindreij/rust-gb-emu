use std::fmt;

use crate::mem::Mmu;

#[derive(Debug, Eq, PartialEq)]
pub enum Instruction {
    Load8 { src: Loc8, dst: Loc8 },
    Load16 { src: Loc16, dst: Loc16 },
    XOR { src: Loc8, dst: Loc8 },
    CheckBit { bit: u8, loc: Loc8 },
    JR { cond: Cond, offset: i8 },
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
    // (HL-)
    IndHLDec,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Loc16 {
    HL,
    SP,
    U16(u16),
}

impl Instruction {
    // Returns the instruction and the number of bytes read
    pub fn parse(pc: u16, mmu: &Mmu) -> (Instruction, u16) {
        match mmu.read_u8(pc) {
            0x18 => (
                Instruction::JR {
                    cond: Cond::Always,
                    offset: mmu.read_i8(pc + 1),
                },
                2,
            ),
            0x20 => (
                Instruction::JR {
                    cond: Cond::NotZero,
                    offset: mmu.read_i8(pc + 1),
                },
                2,
            ),
            0x21 => (
                Instruction::Load16 {
                    src: Loc16::U16(mmu.read_u16(pc + 1)),
                    dst: Loc16::HL,
                },
                3,
            ),
            0x28 => (
                Instruction::JR {
                    cond: Cond::Zero,
                    offset: mmu.read_i8(pc + 1),
                },
                2,
            ),
            0x30 => (
                Instruction::JR {
                    cond: Cond::NotCarry,
                    offset: mmu.read_i8(pc + 1),
                },
                2,
            ),
            0x38 => (
                Instruction::JR {
                    cond: Cond::Carry,
                    offset: mmu.read_i8(pc + 1),
                },
                2,
            ),
            0x31 => (
                Instruction::Load16 {
                    src: Loc16::U16(mmu.read_u16(pc + 1)),
                    dst: Loc16::SP,
                },
                3,
            ),
            0x32 => (
                Instruction::Load8 {
                    src: Loc8::A,
                    dst: Loc8::IndHLDec,
                },
                1,
            ),
            0xaf => (
                Instruction::XOR {
                    src: Loc8::A,
                    dst: Loc8::A,
                },
                1,
            ),
            // CB Prefix
            0xcb => {
                let inst = mmu.read_u8(pc + 1);
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
                    0x40 => Instruction::CheckBit { bit: 0, loc },
                    0x48 => Instruction::CheckBit { bit: 1, loc },
                    0x50 => Instruction::CheckBit { bit: 2, loc },
                    0x58 => Instruction::CheckBit { bit: 3, loc },
                    0x60 => Instruction::CheckBit { bit: 4, loc },
                    0x68 => Instruction::CheckBit { bit: 5, loc },
                    0x70 => Instruction::CheckBit { bit: 6, loc },
                    0x78 => Instruction::CheckBit { bit: 7, loc },
                    _ => panic!("Unknow instruction `cb {:02x}`", inst),
                };

                (res, 2)
            }
            unknown => panic!("Unknown instruction `{:02x}`", unknown),
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
            CheckBit { bit, loc } => write!(f, "BIT {},{}", bit, loc),
            JR { cond, offset } => write!(f, "JR {}${:x}", cond, offset),
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
            Loc8::IndHLDec => write!(f, "(HL-)"),
        }
    }
}

impl fmt::Display for Loc16 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Loc16::HL => write!(f, "HL"),
            Loc16::SP => write!(f, "SP"),
            Loc16::U16(val) => write!(f, "${:x}", val),
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
