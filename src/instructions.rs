use std::fmt;

use crate::mem::Mmu;

#[derive(Debug, Eq, PartialEq)]
pub enum Instruction {
    Load { src: Loc, dst: Loc },
    XOR { src: Loc, dst: Loc },
    SetBit { bit: u8, loc: Loc },
}

#[derive(Debug, Eq, PartialEq)]
pub enum Loc {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
    // (HL)
    IndHL,
    // (HL-)
    IndHLDec,
    SP,
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
            0xcb => {
                let inst = mmu.read_u8(pc + 1);
                let high5 = inst & 0b11111000;
                let low3 = inst & 0x07;
                use Loc::*;
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
                    0x40 => Instruction::SetBit { bit: 0, loc },
                    0x48 => Instruction::SetBit { bit: 1, loc },
                    0x50 => Instruction::SetBit { bit: 2, loc },
                    0x58 => Instruction::SetBit { bit: 3, loc },
                    0x60 => Instruction::SetBit { bit: 4, loc },
                    0x68 => Instruction::SetBit { bit: 5, loc },
                    0x70 => Instruction::SetBit { bit: 6, loc },
                    0x78 => Instruction::SetBit { bit: 7, loc },
                    _ => panic!("Unknow instruction `cb {:x}`", inst),
                };

                (res, 2)
            }
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
            SetBit { bit, loc } => write!(f, "BIT {},{}", bit, loc),
        }
    }
}

impl fmt::Display for Loc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Loc::A => write!(f, "A"),
            Loc::B => write!(f, "B"),
            Loc::C => write!(f, "C"),
            Loc::D => write!(f, "D"),
            Loc::E => write!(f, "E"),
            Loc::H => write!(f, "H"),
            Loc::L => write!(f, "L"),
            Loc::HL => write!(f, "HL"),
            Loc::IndHL => write!(f, "(HL)"),
            Loc::IndHLDec => write!(f, "(HL-)"),
            Loc::SP => write!(f, "SP"),
            Loc::U16(val) => write!(f, "${:x}", val),
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
            Instruction::SetBit {
                bit: 7,
                loc: Loc::H
            }
        );
    }
}
