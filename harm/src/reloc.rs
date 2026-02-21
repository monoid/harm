/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

mod addr;
mod control;
mod data;
mod movs;

use ::core::fmt;

pub use self::addr::*;
pub use self::control::*;
pub use self::data::*;
pub use self::movs::*;
use crate::bits::BitError;

// TODO These type definitions should probably be moved to `harm-types` crate.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct LabelId(pub usize);

// Every offset in an instruction does fit in i32.
// But "[relocation] is sign-extended to 64 bits".
pub type Offset = i64;

pub type Addr = u64;

// b_cond(Cond, LabelRef)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LabelRef {
    pub id: LabelId,
    pub addend: Offset,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rel64 {
    pub rel: Rel64Tag,
    pub label: LabelRef,
}

impl Rel64 {
    #[inline]
    pub const fn new(rel: Rel64Tag, label: LabelRef) -> Self {
        Self { rel, label }
    }

    #[inline]
    pub const fn ld_prel_lo19(label: LabelRef) -> Self {
        Self {
            rel: Rel64Tag::LdPrelLo19,
            label,
        }
    }

    #[inline]
    pub const fn cond_br19(label: LabelRef) -> Self {
        Self {
            rel: Rel64Tag::CondBr19,
            label,
        }
    }

    #[inline]
    pub const fn tst_br14(label: LabelRef) -> Self {
        Self {
            rel: Rel64Tag::TstBr14,
            label,
        }
    }

    #[inline]
    pub const fn jump26(label: LabelRef) -> Self {
        Self {
            rel: Rel64Tag::Jump26,
            label,
        }
    }

    #[inline]
    pub const fn call26(label: LabelRef) -> Self {
        Self {
            rel: Rel64Tag::Call26,
            label,
        }
    }

    #[inline]
    pub const fn mov_w_abs_g0(label: LabelRef) -> Self {
        Self {
            rel: Rel64Tag::MovWAbsG0,
            label,
        }
    }

    #[inline]
    pub const fn mov_w_abs_g0nc(label: LabelRef) -> Self {
        Self {
            rel: Rel64Tag::MovWAbsG0Nc,
            label,
        }
    }

    #[inline]
    pub const fn mov_w_abs_g0s(label: LabelRef) -> Self {
        Self {
            rel: Rel64Tag::MovWAbsG0S,
            label,
        }
    }

    #[inline]
    pub const fn mov_w_abs_g1(label: LabelRef) -> Self {
        Self {
            rel: Rel64Tag::MovWAbsG1,
            label,
        }
    }

    #[inline]
    pub const fn mov_w_abs_g1nc(label: LabelRef) -> Self {
        Self {
            rel: Rel64Tag::MovWAbsG1Nc,
            label,
        }
    }

    #[inline]
    pub const fn mov_w_abs_g1s(label: LabelRef) -> Self {
        Self {
            rel: Rel64Tag::MovWAbsG1S,
            label,
        }
    }

    #[inline]
    pub const fn mov_w_abs_g2(label: LabelRef) -> Self {
        Self {
            rel: Rel64Tag::MovWAbsG2,
            label,
        }
    }

    #[inline]
    pub const fn mov_w_abs_g2nc(label: LabelRef) -> Self {
        Self {
            rel: Rel64Tag::MovWAbsG2Nc,
            label,
        }
    }

    #[inline]
    pub const fn mov_w_abs_g2s(label: LabelRef) -> Self {
        Self {
            rel: Rel64Tag::MovWAbsG2S,
            label,
        }
    }

    #[inline]
    pub const fn mov_w_abs_g3(label: LabelRef) -> Self {
        Self {
            rel: Rel64Tag::MovWAbsG3,
            label,
        }
    }
}

#[derive(Debug)]
pub enum Rel64Error {
    NotEnoughMemory { offset: usize },
    InvalidOffset { offset: usize },
    InvalidValue(::core::num::TryFromIntError),
    InvalidBits(BitError),
}

impl fmt::Display for Rel64Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Rel64Error::NotEnoughMemory { offset } => {
                write!(f, "Not enough memory at 0x{:x}", offset)
            }
            Rel64Error::InvalidOffset { offset } => write!(f, "Invalid offset 0x{:x}", offset),
            Rel64Error::InvalidValue(try_from_int_error) => {
                write!(f, "Invalid value: {try_from_int_error}")
            }
            Rel64Error::InvalidBits(bit_error) => write!(f, "Invalid bit value: {bit_error}"),
        }
    }
}

impl ::core::error::Error for Rel64Error {}

impl From<::core::num::TryFromIntError> for Rel64Error {
    fn from(value: core::num::TryFromIntError) -> Self {
        Rel64Error::InvalidValue(value)
    }
}

impl From<BitError> for Rel64Error {
    fn from(value: BitError) -> Self {
        Rel64Error::InvalidBits(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rel64Tag {
    None,
    // Static data relocations
    // ...
    Abs64,
    Abs32,
    Abs16,
    PRel64,
    PRel32,
    PRel16,

    // Static AArch64 relocations
    LdPrelLo19,
    AdrPrelLo21,
    AdrPrelPgHi21,
    AdrPrelPgHi21Nc,
    AddAbsLo12Nc,
    // Static control flow relocations
    TstBr14,
    CondBr19,
    Jump26,
    Call26, // same as Jump26 actually?

    // TODO `MOVW` and some `add`/`ldst`-related relocations
    MovWAbsG0,
    MovWAbsG0Nc,
    MovWAbsG0S,
    MovWAbsG1,
    MovWAbsG1Nc,
    MovWAbsG1S,
    MovWAbsG2,
    MovWAbsG2Nc,
    MovWAbsG2S,
    MovWAbsG3,
}

impl Rel64Tag {
    /// Applies the relocation to the given `memory` at the given `offset`, presuming the real target address is
    /// `target` and the base (starting) address of the `memory` is `base` (the can be different from real `memory`
    /// location for flexibility).
    pub fn apply(
        self,
        base: Addr,
        target: Addr,
        memory: &mut [u8],
        offset: usize,
    ) -> Result<(), Rel64Error> {
        match self {
            Rel64Tag::None => {
                // check parameters validity
                get_bytes_mut::<0>(memory, offset)?;
                Ok(())
            }
            Rel64Tag::Abs64 => abs64_reloc(target, memory, offset),
            Rel64Tag::Abs32 => abs32_reloc(target.try_into()?, memory, offset),
            Rel64Tag::Abs16 => abs16_reloc(target.try_into()?, memory, offset),
            Rel64Tag::PRel64 => prel64_reloc(base, target, memory, offset),
            Rel64Tag::PRel32 => prel32_reloc(base, target, memory, offset),
            Rel64Tag::PRel16 => prel16_reloc(base, target, memory, offset),

            Rel64Tag::LdPrelLo19 => ld_prel_lo19_reloc(base, target, memory, offset),
            Rel64Tag::AdrPrelLo21 => adr_prel_lo21_reloc(base, target, memory, offset),
            Rel64Tag::AdrPrelPgHi21 => adr_prel_pg_hi21_reloc(base, target, memory, offset),
            Rel64Tag::AdrPrelPgHi21Nc => adr_prel_pg_hi21_nc_reloc(base, target, memory, offset),
            Rel64Tag::AddAbsLo12Nc => add_abs_lo_12_nc_reloc(base, target, memory, offset),

            Rel64Tag::TstBr14 => tst_br14_reloc(base, target, memory, offset),
            Rel64Tag::CondBr19 => cond_br19_reloc(base, target, memory, offset),
            Rel64Tag::Jump26 | Rel64Tag::Call26 => jump26_reloc(base, target, memory, offset),

            Rel64Tag::MovWAbsG0 => mov_w_abs_g0_reloc(base, target, memory, offset),
            Rel64Tag::MovWAbsG0Nc => mov_w_abs_g0nc_reloc(base, target, memory, offset),
            Rel64Tag::MovWAbsG0S => mov_w_abs_g0s_reloc(base, target, memory, offset),
            Rel64Tag::MovWAbsG1 => mov_w_abs_g1_reloc(base, target, memory, offset),
            Rel64Tag::MovWAbsG1Nc => mov_w_abs_g1nc_reloc(base, target, memory, offset),
            Rel64Tag::MovWAbsG1S => mov_w_abs_g1s_reloc(base, target, memory, offset),
            Rel64Tag::MovWAbsG2 => mov_w_abs_g2_reloc(base, target, memory, offset),
            Rel64Tag::MovWAbsG2Nc => mov_w_abs_g2nc_reloc(base, target, memory, offset),
            Rel64Tag::MovWAbsG2S => mov_w_abs_g2s_reloc(base, target, memory, offset),
            Rel64Tag::MovWAbsG3 => mov_w_abs_g3_reloc(base, target, memory, offset),
        }
    }
}

fn get_bytes_mut<const N: usize>(
    mem: &mut [u8],
    offset: usize,
) -> Result<&mut [u8; N], Rel64Error> {
    let mem_chunk = mem
        .get_mut(offset..(offset + N))
        .ok_or(Rel64Error::InvalidOffset { offset })?;
    let bytes: &mut [u8; N] = mem_chunk
        .as_mut_array()
        .ok_or(Rel64Error::NotEnoughMemory { offset })?;
    Ok(bytes)
}

// A function for P - S.
fn calc_offset(base: u64, target: u64, offset: usize) -> Result<Offset, Rel64Error> {
    let offset64 = offset
        .try_into()
        .map_err(|_e| Rel64Error::InvalidOffset { offset })?;

    let instruction_addr = base
        .checked_add(offset64)
        .ok_or_else(|| Rel64Error::InvalidOffset { offset })?;

    Ok(target.wrapping_sub(instruction_addr).cast_signed())
}

#[cfg(test)]
mod tests {
    use crate::instructions::RawInstruction;

    use super::*;

    #[test]
    fn test_none() {
        const MEM: [u8; 8] = [0xfb, 0xa6, 0xd3, 0x67, 0x58, 0x50, 0x1d, 0x46];

        let mut mem = MEM;
        Rel64Tag::None.apply(0, 0, &mut mem, 0).unwrap();
        assert_eq!(mem, MEM);
    }

    #[test]
    fn test_abs64() {
        let mut mem = [0; 8];
        Rel64Tag::Abs64
            .apply(0, 0x123456789abcdef0, &mut mem, 0)
            .unwrap();
        assert_eq!(mem, 0x123456789abcdef0u64.to_le_bytes());
    }

    #[test]
    fn test_abs64_invalid_offset() {
        let mut mem = [0; 16];
        let err = Rel64Tag::Abs64.apply(0, 0, &mut mem, 17).unwrap_err();
        assert!(
            matches!(err, Rel64Error::InvalidOffset { offset: 17 }),
            "{err:?}"
        );
    }

    #[test]
    fn test_abs32() {
        let mut mem = [0; 4];
        Rel64Tag::Abs32.apply(0, 0x12345678, &mut mem, 0).unwrap();
        assert_eq!(mem, 0x12345678u32.to_le_bytes());
    }

    #[test]
    fn test_abs32_invalid_offset() {
        let mut mem = [0; 16];
        let err = Rel64Tag::Abs32.apply(0, 0, &mut mem, 17).unwrap_err();
        assert!(
            matches!(err, Rel64Error::InvalidOffset { offset: 17 }),
            "{err:?}"
        );
    }

    #[test]
    fn test_prel64() {
        let mut mem = [0; 8];
        Rel64Tag::PRel64
            .apply(0x1000, 0x123456789abcdef0, &mut mem, 0)
            .unwrap();
        // TODO is it correct?
        assert_eq!(
            mem,
            0x123456789abcdef0u64.wrapping_sub(0x1000).to_le_bytes()
        );
    }

    #[test]
    fn test_prel64_invalid_offset() {
        let mut mem = [0; 16];
        let err = Rel64Tag::PRel64
            .apply(0x1000, 0x123456789abcdef0, &mut mem, 17)
            .unwrap_err();
        assert!(
            matches!(err, Rel64Error::InvalidOffset { offset: 17 }),
            "{err:?}"
        );
    }

    #[test]
    fn test_jump26() {
        let mut mem = crate::instructions::control::b(40).unwrap().to_code().0;
        let expected = crate::instructions::control::b(0x1020).unwrap().to_code().0;
        Rel64Tag::Jump26.apply(0x1000, 0x2020, &mut mem, 0).unwrap();
        assert_eq!(mem, expected);
    }

    #[test]
    fn test_jump26_unaligned() {
        let mut mem = crate::instructions::control::b(40).unwrap().to_code().0;
        let err = Rel64Tag::Jump26.apply(0x1000, 0x2022, &mut mem, 0);
        assert!(
            matches!(
                err,
                Err(Rel64Error::InvalidBits(BitError::Alignment { align: 2 }))
            ),
            "{err:?}"
        );
    }
}
