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
//! ```
//! # use harm::instructions::load::{ldr, extended, shifted, shifted_by, LdrExtendOption32, LdrShift};
//! use harm::register::Reg32::*;
//! use harm::register::Reg64::*;
//! use LdrExtendOption32::*;
//!
//! ldr(W1, X2);        // LDR W1, [X2]
//! ldr(W1, (X2,));     // LDR W1, [X2]
//! ldr(W1, (X2, X3));  // LDR W1, [X2, X3] ; n.b. a 32-bit register offset requires an extend speicifer (sxtw or uxtw):
//! ldr(W1, (X2, extended(W3, UXTW))); // ldr w1, [x2, w3, uxtw]
//! ldr(W1, (X2, extended(shifted(W3, LdrShift::Shifted), UXTW))); // ldr w1, [x2, w3, uxtw #2]
//! ldr(W1, (X2, extended(W3, UXTW))); // ldr w1, [x2, w3, uxtw]
//! ldr(X1, (X2, extended(W3, UXTW))); // ldr x1, [x2, w3, uxtw]
//! ldr(X1, (X2, extended(shifted(W3, LdrShift::Shifted), UXTW))); // ldr x1, [x2, w3, uxtw #3]
//! # // ldr(X1, (X2, shifted_by(W3, 3).unwrap())); // ldr x1, [x2, w3, uxtw 3]
//! # // TODO ldr(W1, (X2, (W3, UXTW, 3))) // !!!
//! # // TODO ldr(X1, (X2, uxtw(W3, 2))).unwrap();  // LDR X1, [X2, W3 uxtw 3]
//! # // TODO ldr(X1, (X2, sxtw(W3, 2))).unwrap();  // LDR X1, [X2, W3 sxtw 3]
//! # // TODO ldr(X1, (X2, uxtw(W3, 2).unwrap()));  // LDR X1, [X2, W3 uxtw 3]
//! # // TODO ldr(X1, (X2, sxtw(W3, 2).unwrap()));  // LDR X1, [X2, W3 sxtw 3]
//! # // TODO uxtw/sxtw functions
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
use aarchmrs_types::BitValue;

use super::Instruction;
use crate::{
    bits::{BitError, SBitValue, UBitValue},
    register::{IntoCode, Reg32, Reg64, RegOrSp64, RegOrZero32, RegOrZero64},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
// It seems that these variants are actually noop: you cannot actually extend r64 to r64.
// It is only useful if you want to produce a specific bit pattern.
pub enum LdrExtendOption64 {
    #[default]
    LSL = 0b011,
    SXTX = 0b111,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum LdrExtendOption32 {
    #[default]
    UXTW = 0b010,
    SXTW = 0b110,
}

pub trait LdrDestShiftOption {
    const SHIFT_SIZE: u32;
}
pub trait LdrOffsetExtendOption {
    type ExtendOption;
}

impl LdrDestShiftOption for Reg64 {
    const SHIFT_SIZE: u32 = 3;
}

impl LdrOffsetExtendOption for Reg64 {
    type ExtendOption = LdrExtendOption64;
}

impl LdrDestShiftOption for RegOrZero64 {
    const SHIFT_SIZE: u32 = 3;
}

impl LdrOffsetExtendOption for RegOrZero64 {
    type ExtendOption = LdrExtendOption64;
}

impl LdrDestShiftOption for Reg32 {
    const SHIFT_SIZE: u32 = 2;
}

impl LdrOffsetExtendOption for Reg32 {
    type ExtendOption = LdrExtendOption32;
}

impl LdrDestShiftOption for RegOrZero32 {
    const SHIFT_SIZE: u32 = 2;
}

impl LdrOffsetExtendOption for RegOrZero32 {
    type ExtendOption = LdrExtendOption32;
}

pub struct Extended<Dest: LdrDestShiftOption, Offset: LdrOffsetExtendOption> {
    pub shifted: Shifted<Dest, Offset>,
    pub extend: <Offset as LdrOffsetExtendOption>::ExtendOption,
}

pub fn shifted<Dest: LdrDestShiftOption, Offset: LdrOffsetExtendOption>(
    offset: Offset,
    shifted: LdrShift,
) -> Shifted<Dest, Offset> {
    Shifted {
        offset,
        shifted,
        phantom: std::marker::PhantomData,
    }
}

impl<Dest, Offset: LdrOffsetExtendOption> From<Offset> for Shifted<Dest, Offset> {
    fn from(reg: Offset) -> Self {
        Shifted {
            offset: reg,
            shifted: LdrShift::Unshifted,
            phantom: std::marker::PhantomData,
        }
    }
}

// TODO
// ext((W1, UXTW))
// ext((W1, UXTW, 3))
// ldr(W1, (X2, (W3, UXTW, 3))) // !!!
pub fn extended<Dest, Offset>(
    shifted: impl Into<Shifted<Dest, Offset>>,
    extend: <Offset as LdrOffsetExtendOption>::ExtendOption,
) -> Extended<Dest, Offset>
where
    Dest: LdrDestShiftOption,
    Offset: LdrOffsetExtendOption,
{
    Extended {
        shifted: shifted.into(),
        extend,
    }
}

pub fn shifted_by<Dest: LdrDestShiftOption, Offset: LdrOffsetExtendOption>(
    reg: Offset,
    shift: u32,
) -> Result<Shifted<Dest, Offset>, ShiftedError> {
    let accepted = Dest::SHIFT_SIZE;
    let shifted = if shift == 0 {
        LdrShift::Unshifted
    } else if shift == accepted {
        LdrShift::Shifted
    } else {
        return Err(ShiftedError::InvalidShiftSize { shift, accepted });
    };
    Ok(Shifted {
        offset: reg,
        shifted,
        phantom: std::marker::PhantomData,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShiftedError {
    InvalidShiftSize { shift: u32, accepted: u32 },
}

use std::fmt;
impl fmt::Display for ShiftedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidShiftSize { shift, accepted } => {
                write!(
                    f,
                    "Invalid shift {shift} for a register of shift size {accepted}"
                )
            }
        }
    }
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
pub enum LdrShift {
    Unshifted = 0,
    Shifted = 1,
}

impl From<LdrShift> for BitValue<1> {
    fn from(value: LdrShift) -> Self {
        (value as u8).into()
    }
}

pub struct Shifted<Dest, Reg> {
    pub offset: Reg,
    pub shifted: LdrShift,
    phantom: std::marker::PhantomData<Dest>,
}

pub struct Load<Dst, Addr> {
    pub dst: Dst,
    pub addr: Addr,
}

pub trait MakeLoad<Dst, Addr> {
    type Output;
    fn new(dst: Dst, addr: Addr) -> Self::Output;
}

impl<Base, Ext> MakeLoad<Reg64, (Base, Ext)> for Load<Reg64, (RegOrSp64, Extended<Reg64, Reg64>)>
where
    Base: Into<RegOrSp64>,
    Ext: Into<Extended<Reg64, Reg64>>,
{
    type Output = Self;

    #[inline]
    fn new(dst: Reg64, (base, offset): (Base, Ext)) -> Self {
        Self {
            dst,
            addr: (base.into(), offset.into()),
        }
    }
}

impl Instruction for Load<Reg64, (RegOrSp64, Extended<Reg64, Reg64>)> {
    #[inline]
    fn represent(self) -> impl Iterator<Item = aarchmrs_types::InstructionCode> + 'static {
        let (base, offset) = self.addr;
        let code = LDR_64_ldst_regoff(
            offset.shifted.offset.code(),
            (offset.extend as u8).into(),
            offset.shifted.shifted.into(),
            base.code(),
            self.dst.code(),
        );
        std::iter::once(code)
    }
}

impl<Base, Ext> MakeLoad<Reg64, (Base, Ext)> for Load<Reg64, (RegOrSp64, Extended<Reg64, Reg32>)>
where
    Base: Into<RegOrSp64>,
    Ext: Into<Extended<Reg64, Reg32>>,
{
    type Output = Self;

    #[inline]
    fn new(dst: Reg64, (base, offset): (Base, Ext)) -> Self {
        Self {
            dst,
            addr: (base.into(), offset.into()),
        }
    }
}

impl Instruction for Load<Reg64, (RegOrSp64, Extended<Reg64, Reg32>)> {
    #[inline]
    fn represent(self) -> impl Iterator<Item = aarchmrs_types::InstructionCode> + 'static {
        let (base, offset) = self.addr;
        let code = LDR_64_ldst_regoff(
            offset.shifted.offset.code(),
            (offset.extend as u8).into(),
            offset.shifted.shifted.into(),
            base.code(),
            self.dst.code(),
        );
        std::iter::once(code)
    }
}

impl<Base, Ext> MakeLoad<Reg32, (Base, Ext)> for Load<Reg32, (RegOrSp64, Extended<Reg32, Reg64>)>
where
    Base: Into<RegOrSp64>,
    Ext: Into<Extended<Reg32, Reg64>>,
{
    type Output = Self;

    #[inline]
    fn new(dst: Reg32, (base, offset): (Base, Ext)) -> Self {
        Self {
            dst,
            addr: (base.into(), offset.into()),
        }
    }
}

impl Instruction for Load<Reg32, (RegOrSp64, Extended<Reg32, Reg64>)> {
    #[inline]
    fn represent(self) -> impl Iterator<Item = aarchmrs_types::InstructionCode> + 'static {
        let (base, offset) = self.addr;
        let code = LDR_64_ldst_regoff(
            offset.shifted.offset.code(),
            (offset.extend as u8).into(),
            offset.shifted.shifted.into(),
            base.code(),
            self.dst.code(),
        );
        std::iter::once(code)
    }
}

impl<Base, Ext> MakeLoad<Reg32, (Base, Ext)> for Load<Reg32, (RegOrSp64, Extended<Reg32, Reg32>)>
where
    Base: Into<RegOrSp64>,
    Ext: Into<Extended<Reg32, Reg32>>,
{
    type Output = Self;

    #[inline]
    fn new(dst: Reg32, (base, offset): (Base, Ext)) -> Self {
        Self {
            dst,
            addr: (base.into(), offset.into()),
        }
    }
}

impl<Dest, Base, Ext, Err> MakeLoad<Dest, (Base, Result<Ext, Err>)> for Load<Dest, (Base, Ext)>
where
    Load<Dest, (Base, Ext)>: MakeLoad<Dest, (Base, Ext)>,
{
    type Output = Result<<Self as MakeLoad<Dest, (Base, Ext)>>::Output, Err>;

    #[inline]
    fn new(dst: Dest, (base, offset): (Base, Result<Ext, Err>)) -> Self::Output {
        offset.map(|offset| Load::new(dst, (base, offset)))
    }
}

impl Instruction for Load<Reg32, (RegOrSp64, Extended<Reg32, Reg32>)> {
    #[inline]
    fn represent(self) -> impl Iterator<Item = aarchmrs_types::InstructionCode> + 'static {
        let (base, offset) = self.addr;
        let code = LDR_64_ldst_regoff(
            offset.shifted.offset.code(),
            (offset.extend as u8).into(),
            offset.shifted.shifted.into(),
            base.code(),
            self.dst.code(),
        );
        std::iter::once(code)
    }
}

impl<Base> MakeLoad<Reg64, Base> for Load<Reg64, (RegOrSp64, RegOrZero64)>
where
    Base: Into<RegOrSp64>,
{
    type Output = Self;

    #[inline]
    fn new(dst: Reg64, base: Base) -> Self {
        Self {
            dst,
            addr: (base.into(), RegOrZero64::XZR),
        }
    }
}

impl<Base> MakeLoad<Reg64, (Base,)> for Load<Reg64, (RegOrSp64, RegOrZero64)>
where
    Base: Into<RegOrSp64>,
{
    type Output = Self;

    #[inline]
    fn new(dst: Reg64, (base,): (Base,)) -> Self {
        Self {
            dst,
            addr: (base.into(), RegOrZero64::XZR),
        }
    }
}

impl<Base, OffsetReg> MakeLoad<Reg64, (Base, OffsetReg)> for Load<Reg64, (RegOrSp64, RegOrZero64)>
where
    Base: Into<RegOrSp64>,
    OffsetReg: Into<RegOrZero64>,
{
    type Output = Self;

    #[inline]
    fn new(dst: Reg64, (base, offset): (Base, OffsetReg)) -> Self {
        Self {
            dst,
            addr: (base.into(), offset.into()),
        }
    }
}

impl<Base> MakeLoad<Reg32, Base> for Load<Reg32, (RegOrSp64, RegOrZero64)>
where
    Base: Into<RegOrSp64>,
{
    type Output = Self;

    #[inline]
    fn new(dst: Reg32, base: Base) -> Self {
        Self {
            dst,
            addr: (base.into(), RegOrZero64::XZR),
        }
    }
}

impl<Base> MakeLoad<Reg32, (Base,)> for Load<Reg32, (RegOrSp64, RegOrZero64)>
where
    Base: Into<RegOrSp64>,
{
    type Output = Self;

    #[inline]
    fn new(dst: Reg32, (base,): (Base,)) -> Self {
        Self {
            dst,
            addr: (base.into(), RegOrZero64::XZR),
        }
    }
}

impl Instruction for Load<Reg64, (RegOrSp64, RegOrZero64)> {
    #[inline]
    fn represent(self) -> impl Iterator<Item = aarchmrs_types::InstructionCode> + 'static {
        let (base, offset) = self.addr;
        let code = LDR_64_ldst_regoff(
            offset.code(),
            (LdrExtendOption64::default() as u8).into(),
            0b0.into(),
            base.code(),
            self.dst.code(),
        );
        std::iter::once(code)
    }
}

impl<B, O> MakeLoad<Reg32, (B, O)> for Load<Reg32, (RegOrSp64, RegOrZero64)>
where
    B: Into<RegOrSp64>,
    O: Into<RegOrZero64>,
{
    type Output = Self;

    #[inline]
    fn new(dst: Reg32, (base, offset): (B, O)) -> Self {
        Self {
            dst,
            addr: (base.into(), offset.into()),
        }
    }
}

impl Instruction for Load<Reg32, (RegOrSp64, RegOrZero64)> {
    #[inline]
    fn represent(self) -> impl Iterator<Item = aarchmrs_types::InstructionCode> + 'static {
        let (base, offset) = self.addr;
        let code = LDR_32_ldst_regoff(
            offset.code(),
            (LdrExtendOption64::default() as u8).into(),
            0b0.into(),
            base.code(),
            self.dst.code(),
        );
        std::iter::once(code)
    }
}

impl<B> MakeLoad<Reg32, (B, ScaledOffset32)> for Load<Reg32, (RegOrSp64, ScaledOffset32)>
where
    B: Into<RegOrSp64>,
{
    type Output = Self;

    #[inline]
    fn new(dst: Reg32, (base, offset): (B, ScaledOffset32)) -> Self {
        Self {
            dst,
            addr: (base.into(), offset),
        }
    }
}

impl Instruction for Load<Reg32, (RegOrSp64, ScaledOffset32)> {
    #[inline]
    fn represent(self) -> impl Iterator<Item = aarchmrs_types::InstructionCode> + 'static {
        let (base, offset) = self.addr;
        let code = LDR_32_ldst_pos(offset.into(), base.code(), self.dst.code());
        std::iter::once(code)
    }
}

impl<B> MakeLoad<Reg64, (B, ScaledOffset64)> for Load<Reg64, (RegOrSp64, ScaledOffset64)>
where
    B: Into<RegOrSp64>,
{
    type Output = Self;

    #[inline]
    fn new(dst: Reg64, (base, offset): (B, ScaledOffset64)) -> Self {
        Self {
            dst,
            addr: (base.into(), offset),
        }
    }
}

impl Instruction for Load<Reg64, (RegOrSp64, ScaledOffset64)> {
    #[inline]
    fn represent(self) -> impl Iterator<Item = aarchmrs_types::InstructionCode> + 'static {
        let (base, offset) = self.addr;
        let code = LDR_64_ldst_pos(offset.into(), base.code(), self.dst.code());
        std::iter::once(code)
    }
}

impl<B> MakeLoad<Reg32, (B, u32)> for Load<Reg32, (RegOrSp64, ScaledOffset32)>
where
    B: Into<RegOrSp64>,
{
    type Output = Result<Self, BitError>;

    #[inline]
    fn new(dst: Reg32, (base, offset): (B, u32)) -> Self::Output {
        let offset = ScaledOffset32::new(offset)?;
        Ok(Self {
            dst,
            addr: (base.into(), offset),
        })
    }
}

impl<B> MakeLoad<Reg64, (B, u32)> for Load<Reg64, (RegOrSp64, ScaledOffset64)>
where
    B: Into<RegOrSp64>,
{
    type Output = Result<Self, BitError>;

    #[inline]
    fn new(dst: Reg64, (base, offset): (B, u32)) -> Self::Output {
        let offset = ScaledOffset64::new(offset)?;
        Ok(Self {
            dst,
            addr: (base.into(), offset),
        })
    }
}

pub fn ldr<Dst, Addr, AddrReal>(
    dst: Dst,
    addr: Addr,
) -> <Load<Dst, AddrReal> as MakeLoad<Dst, Addr>>::Output
where
    Load<Dst, AddrReal>: MakeLoad<Dst, Addr>,
{
    Load::new(dst, addr)
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

    const SIMPLE_LDR_REG_EXT_DB: &str = "
f8634902	ldr x2, [x8, w3, uxtw]
f8634902	ldr x2, [x8, w3, uxtw #0]
f8635902	ldr x2, [x8, w3, uxtw #3]
f863c902	ldr x2, [x8, w3, sxtw]
f863c902	ldr x2, [x8, w3, sxtw #0]
f863d902	ldr x2, [x8, w3, sxtw #3]
f863e902	ldr x2, [x8, x3, sxtx]
f863e902	ldr x2, [x8, x3, sxtx #0]
f863f902	ldr x2, [x8, x3, sxtx #3]
f8636902	ldr x2, [x8, x3]
f8636902	ldr x2, [x8, x3, lsl #0]
f8637902	ldr x2, [x8, x3, lsl #3]
b8634902	ldr w2, [x8, w3, uxtw]
b8634902	ldr w2, [x8, w3, uxtw #0]
b8635902	ldr w2, [x8, w3, uxtw #2]
b863c902	ldr w2, [x8, w3, sxtw]
b863c902	ldr w2, [x8, w3, sxtw #0]
b863d902	ldr w2, [x8, w3, sxtw #2]
b863e902	ldr w2, [x8, x3, sxtx]
b863e902	ldr w2, [x8, x3, sxtx #0]
b863f902	ldr w2, [x8, x3, sxtx #2]
b8636902	ldr w2, [x8, x3]
b8636902	ldr w2, [x8, x3, lsl #0]
b8637902	ldr w2, [x8, x3, lsl #2]";

    // 'ldr (w2|x2), [(x8|sp), #0x190]'
    const SIMPLE_LDR_SCALED_IMM: &str = "
b9419102	ldr w2, [x8, #0x190]
b94193e2	ldr w2, [sp, #0x190]
f940c902	ldr x2, [x8, #0x190]
f940cbe2	ldr x2, [sp, #0x190]
";

    #[test]
    fn test_ldr_r64_r64_r32_sxtw() {
        use LdrExtendOption32::*;
        let inst: Vec<_> = ldr(X2, (X8, extended(W3, SXTW))).represent().collect();
        assert_eq!(
            inst,
            vec![db(SIMPLE_LDR_REG_EXT_DB, "ldr x2, [x8, w3, sxtw]")]
        );
    }

    #[test]
    fn test_ldr_r64_r64_r32_uxtw() {
        use LdrExtendOption32::*;
        let inst: Vec<_> = ldr(X2, (X8, extended(W3, UXTW))).represent().collect();
        assert_eq!(
            inst,
            vec![db(SIMPLE_LDR_REG_EXT_DB, "ldr x2, [x8, w3, uxtw]")]
        );
    }

    #[test]
    fn test_ldr_r32_r64_r32_sxtw() {
        use LdrExtendOption32::*;
        let inst: Vec<_> = ldr(W2, (X8, extended(W3, SXTW))).represent().collect();
        assert_eq!(
            inst,
            vec![db(SIMPLE_LDR_REG_EXT_DB, "ldr x2, [x8, w3, sxtw]")]
        );
    }

    #[test]
    fn test_ldr_r32_r64_r32_uxtw() {
        use LdrExtendOption32::*;
        let inst: Vec<_> = ldr(W2, (X8, extended(W3, UXTW))).represent().collect();
        assert_eq!(
            inst,
            vec![db(SIMPLE_LDR_REG_EXT_DB, "ldr x2, [x8, w3, uxtw]")]
        );
    }

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
