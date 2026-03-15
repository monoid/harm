/* Copyright (C) 2026 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

//! AArch64 instruction relocations.
//!
//! This module defines relocation types used by the `harm` project, and code to apply them to instructions.
//!
//! There relocations follow the static AArch64 ELF relocation types.  It is important to note that while the spec
//! uses `S+A` for destination address, this module assumes that `A` is already added to the symbol address.
//!
//! The functions do not require the memory to be in place.  The functions' parameters are:
//!   - `base`: the base (starting) address of the memory.  This can be different from the real memory location for
//!     flexibility: the memory can be moved to final location later, even to another host.
//!   - `value`: the real target address (`S+A`).
//!   - `memory`: the memory mutable slice to apply the relocation to.
//!   - `offset`: the offset in the memory to apply the relocation at.
//!
//!  So, `P` in the spec is `base + offset`, and the memory to be modified starts from `&memory[offset]`.

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
            rel: Rel64Tag::LD_PREL_LO19,
            label,
        }
    }

    #[inline]
    pub const fn adr_prel_lo21(label: LabelRef) -> Self {
        Self::new(Rel64Tag::ADR_PREL_LO21, label)
    }

    #[inline]
    pub const fn adr_prel_pg_hi21(label: LabelRef) -> Self {
        Self::new(Rel64Tag::ADR_PREL_PG_HI21, label)
    }

    #[inline]
    pub const fn adr_prel_pg_hi21_nc(label: LabelRef) -> Self {
        Self::new(Rel64Tag::ADR_PREL_PG_HI21_NC, label)
    }

    #[inline]
    pub const fn add_abs_lo12_nc(label: LabelRef) -> Self {
        Self::new(Rel64Tag::ADD_ABS_LO12_NC, label)
    }

    #[inline]
    pub const fn ldst8_abs_lo12_nc(label: LabelRef) -> Self {
        Self::new(Rel64Tag::LDST8_ABS_LO12_NC, label)
    }

    #[inline]
    pub const fn ldst16_abs_lo12_nc(label: LabelRef) -> Self {
        Self::new(Rel64Tag::LDST16_ABS_LO12_NC, label)
    }

    #[inline]
    pub const fn ldst32_abs_lo12_nc(label: LabelRef) -> Self {
        Self::new(Rel64Tag::LDST32_ABS_LO12_NC, label)
    }

    #[inline]
    pub const fn ldst64_abs_lo12_nc(label: LabelRef) -> Self {
        Self::new(Rel64Tag::LDST64_ABS_LO12_NC, label)
    }

    #[inline]
    pub const fn ldst128_abs_lo12_nc(label: LabelRef) -> Self {
        Self::new(Rel64Tag::LDST128_ABS_LO12_NC, label)
    }

    #[inline]
    pub const fn cond_br19(label: LabelRef) -> Self {
        Self {
            rel: Rel64Tag::CONDBR19,
            label,
        }
    }

    #[inline]
    pub const fn tst_br14(label: LabelRef) -> Self {
        Self {
            rel: Rel64Tag::TSTBR14,
            label,
        }
    }

    #[inline]
    pub const fn jump26(label: LabelRef) -> Self {
        Self {
            rel: Rel64Tag::JUMP26,
            label,
        }
    }

    #[inline]
    pub const fn call26(label: LabelRef) -> Self {
        Self {
            rel: Rel64Tag::CALL26,
            label,
        }
    }

    #[inline]
    pub const fn movw_uabs_g0(label: LabelRef) -> Self {
        Self {
            rel: Rel64Tag::MOVW_UABS_G0,
            label,
        }
    }

    #[inline]
    pub const fn movw_uabs_g0_nc(label: LabelRef) -> Self {
        Self {
            rel: Rel64Tag::MOVW_UABS_G0_NC,
            label,
        }
    }

    #[inline]
    pub const fn movw_sabs_g0(label: LabelRef) -> Self {
        Self {
            rel: Rel64Tag::MOVW_SABS_G0,
            label,
        }
    }

    #[inline]
    pub const fn movw_uabs_g1(label: LabelRef) -> Self {
        Self {
            rel: Rel64Tag::MOVW_UABS_G1,
            label,
        }
    }

    #[inline]
    pub const fn movw_uabs_g1_nc(label: LabelRef) -> Self {
        Self {
            rel: Rel64Tag::MOVW_UABS_G1_NC,
            label,
        }
    }

    #[inline]
    pub const fn movw_sabs_g1(label: LabelRef) -> Self {
        Self {
            rel: Rel64Tag::MOVW_SABS_G1,
            label,
        }
    }

    #[inline]
    pub const fn movw_uabs_g2(label: LabelRef) -> Self {
        Self {
            rel: Rel64Tag::MOVW_UABS_G2,
            label,
        }
    }

    #[inline]
    pub const fn movw_uabs_g2_nc(label: LabelRef) -> Self {
        Self {
            rel: Rel64Tag::MOVW_UABS_G2_NC,
            label,
        }
    }

    #[inline]
    pub const fn movw_sabs_g2(label: LabelRef) -> Self {
        Self {
            rel: Rel64Tag::MOVW_SABS_G2,
            label,
        }
    }

    #[inline]
    pub const fn movw_uabs_g3(label: LabelRef) -> Self {
        Self {
            rel: Rel64Tag::MOVW_UABS_G3,
            label,
        }
    }

    #[inline]
    pub const fn movw_prel_g0(label: LabelRef) -> Self {
        Self {
            rel: Rel64Tag::MOVW_PREL_G0,
            label,
        }
    }

    #[inline]
    pub const fn movw_prel_g0_nc(label: LabelRef) -> Self {
        Self {
            rel: Rel64Tag::MOVW_PREL_G0_NC,
            label,
        }
    }

    #[inline]
    pub const fn movw_prel_g1(label: LabelRef) -> Self {
        Self {
            rel: Rel64Tag::MOVW_PREL_G1,
            label,
        }
    }

    #[inline]
    pub const fn movw_prel_g1_nc(label: LabelRef) -> Self {
        Self {
            rel: Rel64Tag::MOVW_PREL_G1_NC,
            label,
        }
    }

    #[inline]
    pub const fn movw_prel_g2(label: LabelRef) -> Self {
        Self {
            rel: Rel64Tag::MOVW_PREL_G2,
            label,
        }
    }

    #[inline]
    pub const fn movw_prel_g2_nc(label: LabelRef) -> Self {
        Self {
            rel: Rel64Tag::MOVW_PREL_G2_NC,
            label,
        }
    }

    #[inline]
    pub const fn movw_prel_g3(label: LabelRef) -> Self {
        Self {
            rel: Rel64Tag::MOVW_PREL_G3,
            label,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum Rel64Tag {
    NONE,
    // Static data relocations
    // ...
    ABS64,
    ABS32,
    ABS16,
    PREL64,
    PREL32,
    PREL16,

    // Static AArch64 address relocations
    LD_PREL_LO19,
    ADR_PREL_LO21,
    ADR_PREL_PG_HI21,
    ADR_PREL_PG_HI21_NC,
    ADD_ABS_LO12_NC,

    LDST8_ABS_LO12_NC,
    LDST16_ABS_LO12_NC,
    LDST32_ABS_LO12_NC,
    LDST64_ABS_LO12_NC,
    LDST128_ABS_LO12_NC,

    // Static control flow relocations
    TSTBR14,
    CONDBR19,
    JUMP26,
    CALL26, // same as JUMP26 actually

    MOVW_UABS_G0,
    MOVW_UABS_G0_NC,
    MOVW_SABS_G0,
    MOVW_UABS_G1,
    MOVW_UABS_G1_NC,
    MOVW_SABS_G1,
    MOVW_UABS_G2,
    MOVW_UABS_G2_NC,
    MOVW_SABS_G2,
    MOVW_UABS_G3,

    MOVW_PREL_G0,
    MOVW_PREL_G0_NC,
    MOVW_PREL_G1,
    MOVW_PREL_G1_NC,
    MOVW_PREL_G2,
    MOVW_PREL_G2_NC,
    MOVW_PREL_G3,
}

impl Rel64Tag {
    /// Applies the relocation in the `memory` at the `offset`, presuming the real target address ('S+A') is `value`,
    /// presuming that base (starting) address of the `memory` is `base` (the can be different from real `memory`
    /// location for flexibility: the memory can be moved to real destination later).
    pub fn apply(
        self,
        base: Addr,
        value: Addr,
        memory: &mut [u8],
        offset: usize,
    ) -> Result<(), Rel64Error> {
        match self {
            Rel64Tag::NONE => {
                // check parameters validity
                get_bytes_mut::<0>(memory, offset)?;
                Ok(())
            }
            Rel64Tag::ABS64 => abs64_reloc(value, memory, offset),
            Rel64Tag::ABS32 => abs32_reloc(value.cast_signed(), memory, offset),
            Rel64Tag::ABS16 => abs16_reloc(value.cast_signed(), memory, offset),
            Rel64Tag::PREL64 => prel64_reloc(base, value, memory, offset),
            Rel64Tag::PREL32 => prel32_reloc(base, value, memory, offset),
            Rel64Tag::PREL16 => prel16_reloc(base, value, memory, offset),

            Rel64Tag::LD_PREL_LO19 => ld_prel_lo19_reloc(base, value, memory, offset),
            Rel64Tag::ADR_PREL_LO21 => adr_prel_lo21_reloc(base, value, memory, offset),
            Rel64Tag::ADR_PREL_PG_HI21 => adrp_prel_pg_hi21_reloc(base, value, memory, offset),
            Rel64Tag::ADR_PREL_PG_HI21_NC => {
                adrp_prel_pg_hi21_nc_reloc(base, value, memory, offset)
            }
            Rel64Tag::ADD_ABS_LO12_NC => add_abs_lo12_nc_reloc(value, memory, offset),

            Rel64Tag::LDST8_ABS_LO12_NC => ldst8_abs_lo12_nc_reloc(value, memory, offset),
            Rel64Tag::LDST16_ABS_LO12_NC => ldst16_abs_lo12_nc_reloc(value, memory, offset),
            Rel64Tag::LDST32_ABS_LO12_NC => ldst32_abs_lo12_nc_reloc(value, memory, offset),
            Rel64Tag::LDST64_ABS_LO12_NC => ldst64_abs_lo12_nc_reloc(value, memory, offset),
            Rel64Tag::LDST128_ABS_LO12_NC => ldst128_abs_lo12_nc_reloc(value, memory, offset),

            Rel64Tag::TSTBR14 => tst_br14_reloc(base, value, memory, offset),
            Rel64Tag::CONDBR19 => cond_br19_reloc(base, value, memory, offset),
            Rel64Tag::JUMP26 | Rel64Tag::CALL26 => jump26_reloc(base, value, memory, offset),

            Rel64Tag::MOVW_UABS_G0 => movw_uabs_g0_reloc(value, memory, offset),
            Rel64Tag::MOVW_UABS_G0_NC => movw_uabs_g0_nc_reloc(value, memory, offset),
            Rel64Tag::MOVW_SABS_G0 => movw_sabs_g0_reloc(value.cast_signed(), memory, offset),
            Rel64Tag::MOVW_UABS_G1 => movw_uabs_g1_reloc(value, memory, offset),
            Rel64Tag::MOVW_UABS_G1_NC => movw_uabs_g1_nc_reloc(value, memory, offset),
            Rel64Tag::MOVW_SABS_G1 => movw_sabs_g1_reloc(value.cast_signed(), memory, offset),
            Rel64Tag::MOVW_UABS_G2 => movw_uabs_g2_reloc(value, memory, offset),
            Rel64Tag::MOVW_UABS_G2_NC => movw_uabs_g2_nc_reloc(value, memory, offset),
            Rel64Tag::MOVW_SABS_G2 => movw_sabs_g2_reloc(value.cast_signed(), memory, offset),
            Rel64Tag::MOVW_UABS_G3 => movw_uabs_g3_reloc(value, memory, offset),

            Rel64Tag::MOVW_PREL_G0 => movw_prel_g0_reloc(base, value, memory, offset),
            Rel64Tag::MOVW_PREL_G0_NC => movw_prel_g0_nc_reloc(base, value, memory, offset),
            Rel64Tag::MOVW_PREL_G1 => movw_prel_g1_reloc(base, value, memory, offset),
            Rel64Tag::MOVW_PREL_G1_NC => movw_prel_g1_nc_reloc(base, value, memory, offset),
            Rel64Tag::MOVW_PREL_G2 => movw_prel_g2_reloc(base, value, memory, offset),
            Rel64Tag::MOVW_PREL_G2_NC => movw_prel_g2_nc_reloc(base, value, memory, offset),
            Rel64Tag::MOVW_PREL_G3 => movw_prel_g3_reloc(base, value, memory, offset),
        }
    }
}

fn get_bytes_mut<const N: usize>(
    mem: &mut [u8],
    offset: usize,
) -> Result<&mut [u8; N], Rel64Error> {
    let mem_chunk = mem
        .get_mut(offset..)
        .ok_or(Rel64Error::InvalidOffset { offset })?;
    let bytes: &mut [u8; N] = mem_chunk
        .get_mut(..N)
        .and_then(|chunk| chunk.as_mut_array())
        .ok_or(Rel64Error::NotEnoughMemory { offset })?;
    Ok(bytes)
}

/// A function for calculating PC-relative relocation difference S - P, where P is `base + offset` and S is `value`.
pub fn calc_delta(base: u64, value: u64, offset: usize) -> Result<Offset, Rel64Error> {
    let offset64 = offset
        .try_into()
        .map_err(|_e| Rel64Error::InvalidOffset { offset })?;

    let instruction_addr = base
        .checked_add(offset64)
        .ok_or(Rel64Error::InvalidOffset { offset })?;

    Ok(value.wrapping_sub(instruction_addr).cast_signed())
}

/// A function for calculating PC-relative relocation difference Page(S) - Page(P), where P is `base + offset` and S is
/// `value`.
///
/// Please note that the difference is uses address offsets, i.e. the difference is not divided by page size (4096).
pub fn calc_page_offset(base: u64, value: u64, offset: usize) -> Result<Offset, Rel64Error> {
    const PAGE_MASK: u64 = !0xfff;
    let offset64 = offset
        .try_into()
        .map_err(|_e| Rel64Error::InvalidOffset { offset })?;

    let instruction_addr = base
        .checked_add(offset64)
        .ok_or(Rel64Error::InvalidOffset { offset })?;

    Ok((value & PAGE_MASK)
        .wrapping_sub(instruction_addr & PAGE_MASK)
        .cast_signed())
}

#[cfg(test)]
mod claude_tests;

#[cfg(test)]
mod tests {
    use crate::instructions::RawInstruction;

    use super::*;

    #[test]
    fn test_none() {
        const MEM: [u8; 8] = [0xfb, 0xa6, 0xd3, 0x67, 0x58, 0x50, 0x1d, 0x46];

        let mut mem = MEM;
        Rel64Tag::NONE.apply(0, 0, &mut mem, 0).unwrap();
        assert_eq!(mem, MEM);
    }

    #[test]
    fn test_abs64() {
        let mut mem = [0; 8];
        Rel64Tag::ABS64
            .apply(0, 0x123456789abcdef0, &mut mem, 0)
            .unwrap();
        assert_eq!(mem, 0x123456789abcdef0u64.to_le_bytes());
    }

    #[test]
    fn test_abs64_invalid_offset() {
        let mut mem = [0; 16];
        let err = Rel64Tag::ABS64.apply(0, 0, &mut mem, 17).unwrap_err();
        assert!(
            matches!(err, Rel64Error::InvalidOffset { offset: 17 }),
            "{err:?}"
        );
    }

    #[test]
    fn test_abs32() {
        let mut mem = [0; 4];
        Rel64Tag::ABS32.apply(0, 0x12345678, &mut mem, 0).unwrap();
        assert_eq!(mem, 0x12345678u32.to_le_bytes());
    }

    #[test]
    fn test_abs32_invalid_offset() {
        let mut mem = [0; 16];
        let err = Rel64Tag::ABS32.apply(0, 0, &mut mem, 17).unwrap_err();
        assert!(
            matches!(err, Rel64Error::InvalidOffset { offset: 17 }),
            "{err:?}"
        );
    }

    #[test]
    fn test_prel64() {
        let mut mem = [0; 8];
        Rel64Tag::PREL64
            .apply(0x1000, 0x123456789abcdef0, &mut mem, 0)
            .unwrap();
        assert_eq!(
            mem,
            0x123456789abcdef0u64.wrapping_sub(0x1000).to_le_bytes()
        );
    }

    #[test]
    fn test_prel64_invalid_offset() {
        let mut mem = [0; 16];
        let err = Rel64Tag::PREL64
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
        Rel64Tag::JUMP26.apply(0x1000, 0x2020, &mut mem, 0).unwrap();
        assert_eq!(mem, expected);
    }

    #[test]
    fn test_jump26_unaligned() {
        let mut mem = crate::instructions::control::b(40).unwrap().to_code().0;
        let err = Rel64Tag::JUMP26.apply(0x1000, 0x2022, &mut mem, 0);
        assert!(
            matches!(
                err,
                Err(Rel64Error::InvalidBits(BitError::Alignment { align: 2 }))
            ),
            "{err:?}"
        );
    }
}
