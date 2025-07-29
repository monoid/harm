/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

//! `LDR` and related commands.
//!
//! The `ldr` function returns an instance of `Instruction` for loading a 32-bit or 64-bit register from memory. While
//! `LDR` instruction has different variants with various number of arguments, the `ldr` function has two arguments: a
//! destination register and an "address" that encapsulates the rest of arguments: the base, offsets, extensions, etc.
//! Tuples are often used for the second argument, see the pattern in the examples below.
//!
//! The funciton is overloaded for various argument types. For some of them, and `Instruction` trait instance is
//! returned, for others, a `Result` if the aguments need validation. Such arugment combinations have `.unwrap()` in
//! examples.
//!
//! # `LDR`: Register base with register offset
//!
//! # Examples:
//! ```ignore
//! ldr(W1, X2);        // LDR W1, [X2]
//! ldr(W1, (X2,));     // LDR W1, [X2]
//! ldr(W1, (X2, X3));  // LDR W1, [X2, X3] ; n.b. a 32-bit register offset requires extend speicifer (sxtw or uxtw):
//! ldr(X1, (X2, uxtw(W3, 2))).unwrap();  // LDR X1, [X2, W3 uxtw 3]
//! ldr(X1, (X2, sxtw(W3, 2))).unwrap();  // LDR X1, [X2, W3 sxtw 3]
//! ldr(X1, (X2, uxtw(W3, 2).unwrap()));  // LDR X1, [X2, W3 uxtw 3]
//! ldr(X1, (X2, sxtw(W3, 2).unwrap()));  // LDR X1, [X2, W3 sxtw 3]
//! ```
//!
//! Please note, that `uxtw` and `sxtw` can be used only with 32-bit register, and shift can be only either 0 or 2. The
//! `lsl` and `sxtx` can be used only with 64-bit registers, and while they produce different bit patterns, they are
//! equivalent; shift can be only 0 or 3.
//!
//! # `LDR`: Register base with immediate offset
//!
//! LDR with register base with immediate offset has an unsigned offset aligned by destination register size.
//! For example, if the desination register is `W1`, the offset has be aligned by 4 bytes (two lower bits are clear),
//! and if it is `X1`, the offset has to be aligned by 8 bytes (three lower bits are clear).  The offset has 12
//! significan bytes available.
//!
//! You may also a `u32` offset value, and a error is returned if the value doesn't fit the offset pattern.
//!
//! Examples:
//! ```ignore
//! let word_aligned_offset: UBitValue<12, 2> = ...;
//! let dword_aligned_offset: UBitValue<12, 3> = ...;
//!
//! ldr(W1, (X2, offset as u32)).unwrap(),
//! ldr(X1, (X2, offset as u32)).unwrap(),
//! ldr(W1, (X2, word_aligned_offset)),
//! ldr(X1, (X2, dword_aligned_offset)),
//! ```
//!
//! # `LDR`: PC base with immediate offset
//!
//! An immediate signed offset of `SBitValue<12, 2>` is added to `PC`, i.e. the offset is relative to the instruction's
//! address.  The 2-bit alignment is the same for both 32-bit and 64-bit variants.
//!
//! ```ignore
//! let bit_offset: PcOffset = ...;
//! let raw_offset: i32 = ...;
//!
//! ldr(W1, pc(bit_offset)),
//! ldr(W1, (Pc, bit_offset)),
//! ldr(W1, (Pc, raw_offset)).unwrap(),
//! ```
//!
//! # `LDUR`: Register base with unaligned immediate offset
//!
//! `LDUR` is similar to `LDR` with register base with immediate offset, but its offset is **signed** 9 bit wide. For
//! convenience, a `SBitValue<9>` value can be used with `ldr` as well, and both signed and unsigned raw values can be
//! encoded as LDUR if they fit into the range and cannot be encoded with `LDR` with immediate offset (TODO it makes the
//! code little bit more complex, unless we use an enum).
//!
//! ```ignore
//! let bit_offset: SBitOffset<9> = ...;
//! let raw_offset: i32 = ...;
//!
//! ldr(W1, (X2, bit_offset)),
//! ldur(W1, (X2, bit_offset)),
//! ldr(X1, (X2, bit_offset)),
//! ldur(X1, (X2, bit_offset)),
//! ldr(W1, (X2, raw_offset)).unwap(),
//! ldur(W1, (X2, raw_offset)).unwap(),
//! ldr(X1, (X2, raw_offset)).unwap(),
//! ldur(X1, (X2, raw_offset)).unwap(),
//! ```

use aarchmrs_instructions::A64::ldst::{
    ldst_pos::{LDR_32_ldst_pos::LDR_32_ldst_pos, LDR_64_ldst_pos::LDR_64_ldst_pos},
    ldst_regoff::{LDR_32_ldst_regoff::LDR_32_ldst_regoff, LDR_64_ldst_regoff::LDR_64_ldst_regoff},
};

use super::Instruction;
use crate::{
    bits::{BitError, SBitValue, UBitValue},
    register::{IntoCode, Reg32, Reg64, RegOrSp64, RegOrZero64},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum LdrExtendOption64 {
    #[default]
    LSL = 0b011,
    // It seems that these variants are noop: you cannot actually extend r64 to r64.
    // It is only useful if you want to produce specific bit pattern.
    SXTX = 0b111,
}

pub type PcOffset = SBitValue<19>;
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Pc(PcOffset);

pub type ScaledOffset32 = UBitValue<12, 2>;
pub type ScaledOffset64 = UBitValue<12, 3>;

pub type UnscaledOffset = SBitValue<9>;
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Unscaled(pub RegOrSp64, pub UnscaledOffset);

pub const fn unscaled(base: RegOrSp64, offset: UnscaledOffset) -> Unscaled {
    Unscaled(base, offset)
}

#[repr(u8)]
pub enum RegShift {
    Unshifted = 0,
    Shifted = 1,
}

pub struct Shifted<Reg> {
    pub reg: Reg,
    pub shifted: RegShift,
}

pub struct Load<Dst, Base, Offset> {
    pub dst: Dst,
    pub base: Base,
    pub offset: Offset,
}

pub trait MakeLoad<Dst, Base, Offset> {
    type Output;
    fn new(dst: Dst, base: Base, offset: Offset) -> Self::Output;
}

impl<B, O> MakeLoad<Reg64, B, O> for Load<Reg64, RegOrSp64, RegOrZero64>
where
    B: Into<RegOrSp64>,
    O: Into<RegOrZero64>,
{
    type Output = Self;

    #[inline]
    fn new(dst: Reg64, base: B, offset: O) -> Self {
        Self {
            dst,
            base: base.into(),
            offset: offset.into(),
        }
    }
}

impl Instruction for Load<Reg64, RegOrSp64, RegOrZero64> {
    #[inline]
    fn represent(self) -> impl Iterator<Item = aarchmrs_types::InstructionCode> + 'static {
        let code = LDR_64_ldst_regoff(
            self.offset.code(),
            (LdrExtendOption64::default() as u8).into(),
            0b0.into(),
            self.base.code(),
            self.dst.code(),
        );
        std::iter::once(code)
    }
}

impl<B, O> MakeLoad<Reg32, B, O> for Load<Reg32, RegOrSp64, RegOrZero64>
where
    B: Into<RegOrSp64>,
    O: Into<RegOrZero64>,
{
    type Output = Self;

    #[inline]
    fn new(dst: Reg32, base: B, offset: O) -> Self {
        Self {
            dst,
            base: base.into(),
            offset: offset.into(),
        }
    }
}

impl Instruction for Load<Reg32, RegOrSp64, RegOrZero64> {
    #[inline]
    fn represent(self) -> impl Iterator<Item = aarchmrs_types::InstructionCode> + 'static {
        let code = LDR_32_ldst_regoff(
            self.offset.code(),
            (LdrExtendOption64::default() as u8).into(),
            0b0.into(),
            self.base.code(),
            self.dst.code(),
        );
        std::iter::once(code)
    }
}

impl<B> MakeLoad<Reg32, B, ScaledOffset32> for Load<Reg32, RegOrSp64, ScaledOffset32>
where
    B: Into<RegOrSp64>,
{
    type Output = Self;

    #[inline]
    fn new(dst: Reg32, base: B, offset: ScaledOffset32) -> Self {
        Self {
            dst,
            base: base.into(),
            offset,
        }
    }
}

impl Instruction for Load<Reg32, RegOrSp64, ScaledOffset32> {
    #[inline]
    fn represent(self) -> impl Iterator<Item = aarchmrs_types::InstructionCode> + 'static {
        let code = LDR_32_ldst_pos(self.offset.into(), self.base.code(), self.dst.code());
        std::iter::once(code)
    }
}

impl<B> MakeLoad<Reg64, B, ScaledOffset64> for Load<Reg64, RegOrSp64, ScaledOffset64>
where
    B: Into<RegOrSp64>,
{
    type Output = Self;

    #[inline]
    fn new(dst: Reg64, base: B, offset: ScaledOffset64) -> Self {
        Self {
            dst,
            base: base.into(),
            offset,
        }
    }
}

impl Instruction for Load<Reg64, RegOrSp64, ScaledOffset64> {
    #[inline]
    fn represent(self) -> impl Iterator<Item = aarchmrs_types::InstructionCode> + 'static {
        let code = LDR_64_ldst_pos(self.offset.into(), self.base.code(), self.dst.code());
        std::iter::once(code)
    }
}

impl<B> MakeLoad<Reg32, B, u32> for Load<Reg32, RegOrSp64, ScaledOffset32>
where
    B: Into<RegOrSp64>,
{
    type Output = Result<Self, BitError>;

    #[inline]
    fn new(dst: Reg32, base: B, offset: u32) -> Self::Output {
        let offset = ScaledOffset32::new(offset)?;
        Ok(Self {
            dst,
            base: base.into(),
            offset,
        })
    }
}

impl<B> MakeLoad<Reg64, B, u32> for Load<Reg64, RegOrSp64, ScaledOffset64>
where
    B: Into<RegOrSp64>,
{
    type Output = Result<Self, BitError>;

    #[inline]
    fn new(dst: Reg64, base: B, offset: u32) -> Self::Output {
        let offset = ScaledOffset64::new(offset)?;
        Ok(Self {
            dst,
            base: base.into(),
            offset,
        })
    }
}

pub fn ldr<Dst, Base, BaseReal, Offset, OffsetReal>(
    dst: Dst,
    (base, offset): (Base, Offset),
) -> <Load<Dst, BaseReal, OffsetReal> as MakeLoad<Dst, Base, Offset>>::Output
where
    Load<Dst, BaseReal, OffsetReal>: MakeLoad<Dst, Base, Offset>,
{
    Load::new(dst, base, offset)
}

#[cfg(test)]
mod tests {
    use super::*;
    use Reg32::*;
    use Reg64::*;
    use RegOrSp64::SP;
    use RegOrZero64::XZR;
    use harm_test_utils::db;

    // 'ldr (w2|x2), [(x8|sp), (x3|xzr)]'
    const SIMPLE_LDR_REG_DB: &str = "
b8636902	ldr w2, [x8, x3]
b87f6902	ldr w2, [x8, xzr]
b8636be2	ldr w2, [sp, x3]
b87f6be2	ldr w2, [sp, xzr]
f8636902	ldr x2, [x8, x3]
f87f6902	ldr x2, [x8, xzr]
f8636be2	ldr x2, [sp, x3]
f87f6be2	ldr x2, [sp, xzr]
";

    // 'ldr (w2|x2), [(x8|sp), #0x190]'
    const SIMPLE_LDR_SCALED_IMM: &str = "
b9419102	ldr w2, [x8, #0x190]
b94193e2	ldr w2, [sp, #0x190]
f940c902	ldr x2, [x8, #0x190]
f940cbe2	ldr x2, [sp, #0x190]
";

    #[test]
    fn test_ldr_r32_r64_r64() {
        let inst: Vec<_> = ldr(W2, (X8, X3)).represent().collect();
        assert_eq!(inst, vec![db(SIMPLE_LDR_REG_DB, "ldr w2, [x8, x3]")]);
    }

    #[test]
    fn test_ldr_r32_rsp_r64() {
        let inst: Vec<_> = ldr(W2, (SP, X3)).represent().collect();
        assert_eq!(inst, vec![db(SIMPLE_LDR_REG_DB, "ldr w2, [sp, x3]")]);
    }

    #[test]
    fn test_ldr_r64_r64_r64() {
        let inst: Vec<_> = ldr(X2, (X8, X3)).represent().collect();
        assert_eq!(inst, vec![db(SIMPLE_LDR_REG_DB, "ldr x2, [x8, x3]")]);
    }

    #[test]
    fn test_ldr_r64_rsp_r64() {
        let inst: Vec<_> = ldr(X2, (SP, X3)).represent().collect();
        assert_eq!(inst, vec![db(SIMPLE_LDR_REG_DB, "ldr x2, [sp, x3]")]);
    }

    #[test]
    fn test_ldr_r32_r64_xzr() {
        let inst: Vec<_> = ldr(W2, (X8, XZR)).represent().collect();
        assert_eq!(inst, vec![db(SIMPLE_LDR_REG_DB, "ldr w2, [x8, xzr]")]);
    }

    #[test]
    fn test_ldr_r32_rsp_xzr() {
        let inst: Vec<_> = ldr(W2, (SP, XZR)).represent().collect();
        assert_eq!(inst, vec![db(SIMPLE_LDR_REG_DB, "ldr w2, [sp, xzr]")]);
    }

    #[test]
    fn test_ldr_r64_r64_xzr() {
        let inst: Vec<_> = ldr(X2, (X8, XZR)).represent().collect();
        assert_eq!(inst, vec![db(SIMPLE_LDR_REG_DB, "ldr x2, [x8, xzr]")]);
    }

    #[test]
    fn test_ldr_r64_rsp_xzr() {
        let inst: Vec<_> = ldr(X2, (SP, XZR)).represent().collect();
        assert_eq!(inst, vec![db(SIMPLE_LDR_REG_DB, "ldr x2, [sp, xzr]")]);
    }

    #[test]
    fn test_ldr_r32_r64_scaled_imm() {
        let offset = UBitValue::<12, 2>::new(0x190).unwrap();
        let inst: Vec<_> = ldr(W2, (X8, offset)).represent().collect();
        assert_eq!(
            inst,
            vec![db(SIMPLE_LDR_SCALED_IMM, "ldr w2, [x8, #0x190]")]
        );
    }

    #[test]
    fn test_ldr_r32_sp_scaled_imm() {
        let offset = UBitValue::<12, 2>::new(0x190).unwrap();
        let inst: Vec<_> = ldr(W2, (SP, offset)).represent().collect();
        assert_eq!(
            inst,
            vec![db(SIMPLE_LDR_SCALED_IMM, "ldr w2, [sp, #0x190]")]
        );
    }

    #[test]
    fn test_ldr_r64_r64_scaled_imm() {
        let offset = UBitValue::<12, 3>::new(0x190).unwrap();
        let inst: Vec<_> = ldr(X2, (X8, offset)).represent().collect();
        assert_eq!(
            inst,
            vec![db(SIMPLE_LDR_SCALED_IMM, "ldr x2, [x8, #0x190]")]
        );
    }

    #[test]
    fn test_ldr_r64_sp_scaled_imm() {
        let offset = UBitValue::<12, 3>::new(0x190).unwrap();
        let inst: Vec<_> = ldr(X2, (SP, offset)).represent().collect();
        assert_eq!(
            inst,
            vec![db(SIMPLE_LDR_SCALED_IMM, "ldr x2, [sp, #0x190]")]
        );
    }
}
