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
//! # use harm::instructions::ldst::{ldr, extended, shifted, shifted_by, LdrExtendOption32, LdrShift};
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
//!
//! TODO pre- and post-increment variants.
use aarchmrs_instructions::A64::ldst::{
    ldst_immpost::{LDR_32_ldst_immpost::LDR_32_ldst_immpost, LDR_64_ldst_immpost::LDR_64_ldst_immpost}, ldst_immpre::{LDR_32_ldst_immpre::LDR_32_ldst_immpre, LDR_64_ldst_immpre::LDR_64_ldst_immpre}, ldst_pos::{LDR_32_ldst_pos::LDR_32_ldst_pos, LDR_64_ldst_pos::LDR_64_ldst_pos}, ldst_regoff::{LDR_32_ldst_regoff::LDR_32_ldst_regoff, LDR_64_ldst_regoff::LDR_64_ldst_regoff}, loadlit::{LDR_32_loadlit::LDR_32_loadlit, LDR_64_loadlit::LDR_64_loadlit}
};

use super::{shift_extend::*, LdStIncOffset, LdStPcOffset, Pc, PostIncrement, PreIncrement};
use super::{Load, MakeLoad, ScaledOffset32, ScaledOffset64};
use crate::{
    bits::BitError,
    instructions::Instruction,
    register::{IntoCode, Reg32, Reg64, RegOrSp64, RegOrZero32, RegOrZero64},
};

/// `LDR` with 64-bit destination, base register with extended 64-bit register offset with scale.
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

/// `LDR` with 64-bit destination, base register with extended 32-bit register offset with scale.
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

/// `LDR` with 32-bit destination, base register with extended 64-bit register offset with scale.
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

/// `LDR` with 32-bit destination, base register with extended 32-bit register offset with scale.
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

/// `LDR` with fallible offset that delegates to non-faillible variants.
impl<DestInp, DestOut, Base, Ext, Err> MakeLoad<DestInp, (Base, Result<Ext, Err>)> for Load<DestOut, (Base, Ext)>
where
    Load<DestOut, (Base, Ext)>: MakeLoad<DestInp, (Base, Ext)>,
{
    type Output = Result<<Self as MakeLoad<DestInp, (Base, Ext)>>::Output, Err>;

    #[inline]
    fn new(dst: DestInp, (base, offset): (Base, Result<Ext, Err>)) -> Self::Output {
        offset.map(|offset| Load::new(dst, (base, offset)))
    }
}

/// `LDR` with fallible address that delegates to non-faillible variants.
impl<DestInp, DestOut, Addr, Err> MakeLoad<DestInp, Result<Addr, Err>> for Load<DestOut, Addr>
where
    Load<DestOut, Addr>: MakeLoad<DestInp, Addr>,
{
    type Output = Result<<Self as MakeLoad<DestInp, Addr>>::Output, Err>;

    #[inline]
    fn new(dst: DestInp, addr: Result<Addr, Err>) -> Self::Output {
        addr.map(|addr| Load::new(dst, addr))
    }
}
impl Instruction for Load<Reg32, (RegOrSp64, Extended<Reg32, Reg32>)> {
    #[inline]
    fn represent(self) -> impl Iterator<Item = aarchmrs_types::InstructionCode> + 'static {
        let (base, offset) = self.addr;
        let code = LDR_32_ldst_regoff(
            offset.shifted.offset.code(),
            (offset.extend as u8).into(),
            offset.shifted.shifted.into(),
            base.code(),
            self.dst.code(),
        );
        std::iter::once(code)
    }
}

/// `LDR` with 64-bit destination, bare base register.
impl<Base> MakeLoad<Reg64, Base> for Load<Reg64, (RegOrSp64, RegOrZero64)>
where
    Base: Into<RegOrSp64>,
{
    type Output = Self;

    #[inline]
    fn new(dst: Reg64, base: Base) -> Self {
        Self {
            dst,
            // TODO does the spec says something specific?
            addr: (base.into(), RegOrZero64::XZR),
        }
    }
}

/// `LDR` with 64-bit destination, bare base register as a tuple.
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

/// `LDR` with 64-bit destination, base register with 64-bit offset without scaling.
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

/// `LDR` with 32-bit destination, bare base register.
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

/// `LDR` with 32-bit destination, bare base register as a tuple.
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

/// `LDR` with 32-bit destination, base register with 64-bit offset without scaling.
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

/// `LDR` with 32-bit destination, base register with aligned immediate offset.
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

/// `LDR` with 64-bit destination, base register with aligned immediate offset.
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

/// `LDR` with 32-bit destination, base register with immediate offset. It is fallible, as the offset is has to be
/// properly aligned.
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

/// `LDR` with 64-bit destination, base register with immediate offset. It is fallible, as the offset is has to be
/// properly aligned.
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

impl<Dest> MakeLoad<Dest, (Pc, LdStPcOffset)> for Load<RegOrZero64, (Pc, LdStPcOffset)>
where
    Dest: Into<RegOrZero64>,
{
    type Output = Self;

    #[inline]
    fn new(dst: Dest, addr: (Pc, LdStPcOffset)) -> Self {
        Self {
            dst: dst.into(),
            addr,
        }
    }
}

impl Instruction for Load<RegOrZero64, (Pc, LdStPcOffset)> {
    #[inline]
    fn represent(self) -> impl Iterator<Item = aarchmrs_types::InstructionCode> + 'static {
        let (_pc, offset) = self.addr;
        let code = LDR_64_loadlit(offset.into(), self.dst.code());
        std::iter::once(code)
    }
}

impl<Dest> MakeLoad<Dest, (Pc, LdStPcOffset)> for Load<RegOrZero32, (Pc, LdStPcOffset)>
where
    Dest: Into<RegOrZero32>,
{
    type Output = Self;

    #[inline]
    fn new(dst: Dest, addr: (Pc, LdStPcOffset)) -> Self {
        Self {
            dst: dst.into(),
            addr,
        }
    }
}

impl Instruction for Load<RegOrZero32, (Pc, LdStPcOffset)> {
    #[inline]
    fn represent(self) -> impl Iterator<Item = aarchmrs_types::InstructionCode> + 'static {
        let (_pc, offset) = self.addr;
        let code = LDR_32_loadlit(offset.into(), self.dst.code());
        std::iter::once(code)
    }
}

impl<Dest: Into<RegOrZero64>> MakeLoad<Dest, PreIncrement<LdStIncOffset>>
    for Load<RegOrZero64, PreIncrement<LdStIncOffset>>
{
    type Output = Self;

    fn new(dst: Dest, addr: PreIncrement<LdStIncOffset>) -> Self {
        Self {
            dst: dst.into(),
            addr,
        }
    }
}

impl Instruction for Load<RegOrZero64, PreIncrement<LdStIncOffset>> {
    #[inline]
    fn represent(self) -> impl Iterator<Item = aarchmrs_types::InstructionCode> + 'static {
        let addr = self.addr;
        let code = LDR_64_ldst_immpre(
            addr.offset.into(),
            addr.base.code(),
            self.dst.code(),
        );
        std::iter::once(code)
    }
}

impl<Dest: Into<RegOrZero32>> MakeLoad<Dest, PreIncrement<LdStIncOffset>>
    for Load<RegOrZero32, PreIncrement<LdStIncOffset>>
{
    type Output = Self;

    fn new(dst: Dest, addr: PreIncrement<LdStIncOffset>) -> Self {
        Self {
            dst: dst.into(),
            addr,
        }
    }
}

impl Instruction for Load<RegOrZero32, PreIncrement<LdStIncOffset>> {
    #[inline]
    fn represent(self) -> impl Iterator<Item = aarchmrs_types::InstructionCode> + 'static {
        let addr = self.addr;
        let code = LDR_32_ldst_immpre(
            addr.offset.into(),
            addr.base.code(),
            self.dst.code(),
        );
        std::iter::once(code)
    }
}

impl<Dest: Into<RegOrZero64>> MakeLoad<Dest, PostIncrement<LdStIncOffset>>
    for Load<RegOrZero64, PostIncrement<LdStIncOffset>>
{
    type Output = Self;

    fn new(dst: Dest, addr: PostIncrement<LdStIncOffset>) -> Self {
        Self {
            dst: dst.into(),
            addr,
        }
    }
}

impl Instruction for Load<RegOrZero64, PostIncrement<LdStIncOffset>> {
    #[inline]
    fn represent(self) -> impl Iterator<Item = aarchmrs_types::InstructionCode> + 'static {
        let addr = self.addr;
        let code = LDR_64_ldst_immpost(
            addr.offset.into(),
            addr.base.code(),
            self.dst.code(),
        );
        std::iter::once(code)
    }
}

impl<Dest: Into<RegOrZero32>> MakeLoad<Dest, PostIncrement<LdStIncOffset>>
    for Load<RegOrZero32, PostIncrement<LdStIncOffset>>
{
    type Output = Self;

    fn new(dst: Dest, addr: PostIncrement<LdStIncOffset>) -> Self {
        Self {
            dst: dst.into(),
            addr,
        }
    }
}

impl Instruction for Load<RegOrZero32, PostIncrement<LdStIncOffset>> {
    #[inline]
    fn represent(self) -> impl Iterator<Item = aarchmrs_types::InstructionCode> + 'static {
        let addr = self.addr;
        let code = LDR_32_ldst_immpost(
            addr.offset.into(),
            addr.base.code(),
            self.dst.code(),
        );
        std::iter::once(code)
    }
}

/// ldr construction function.  See examples in the module documentation.
pub fn ldr<TargetInp, TargetOut, AddrInp, AddrOut>(
    dst: TargetInp,
    addr: AddrInp,
) -> <Load<TargetOut, AddrOut> as MakeLoad<TargetInp, AddrInp>>::Output
where
    Load<TargetOut, AddrOut>: MakeLoad<TargetInp, AddrInp>,
{
    Load::new(dst, addr)
}

#[cfg(test)]
mod tests {
    use harm_test_utils::test_cases;

    use super::*;
    use crate::{
        bits::UBitValue,
        instructions::ldst::{LdStPcOffset, Pc, postinc, preinc},
    };
    use Reg32::*;
    use Reg64::*;
    use RegOrSp64::SP;
    use RegOrZero64::XZR;

    const LDR_REG_EXT_DB: &str = "
b8634902	ldr w2, [x8, w3, uxtw #0]
b8634902	ldr w2, [x8, w3, uxtw]
b8635902	ldr w2, [x8, w3, uxtw #2]
b8636902	ldr w2, [x8, x3, lsl #0]
b8636902	ldr w2, [x8, x3]
b8636be2	ldr w2, [sp, x3]
b8637902	ldr w2, [x8, x3, lsl #2]
b8637902	ldr w2, [x8, x3, lsl #2]
b863c902	ldr w2, [x8, w3, sxtw #0]
b863c902	ldr w2, [x8, w3, sxtw]
b863d902	ldr w2, [x8, w3, sxtw #2]
b863e902	ldr w2, [x8, x3, sxtx #0]
b863e902	ldr w2, [x8, x3, sxtx]
b863f902	ldr w2, [x8, x3, sxtx #2]
b87f6902	ldr w2, [x8, xzr]
b87f6be2	ldr w2, [sp, xzr]
f8634902	ldr x2, [x8, w3, uxtw #0]
f8634902	ldr x2, [x8, w3, uxtw]
f8635902	ldr x2, [x8, w3, uxtw #3]
f8636902	ldr x2, [x8, x3, lsl #0]
f8636902	ldr x2, [x8, x3]
f8636be2	ldr x2, [sp, x3]
f8637902	ldr x2, [x8, x3, lsl #3]
f863c902	ldr x2, [x8, w3, sxtw #0]
f863c902	ldr x2, [x8, w3, sxtw]
f863d902	ldr x2, [x8, w3, sxtw #3]
f863e902	ldr x2, [x8, x3, sxtx #0]
f863e902	ldr x2, [x8, x3, sxtx]
f863f902	ldr x2, [x8, x3, sxtx #3]
f87f4902	ldr x2, [x8, wzr, uxtw]
f87f6902	ldr x2, [x8, xzr]
f87f6be2	ldr x2, [sp, xzr]
f87f7902	ldr x2, [x8, xzr, lsl #3]
f87fc902	ldr x2, [x8, wzr, sxtw]
f87fe902	ldr x2, [x8, xzr, sxtx]
";

    // 'ldr (w2|x2), [(x8|sp), #0x190]'
    const LDR_SCALED_IMM_DB: &str = "
b9419102	ldr w2, [x8, #0x190]
b94193e2	ldr w2, [sp, #0x190]
f940c902	ldr x2, [x8, #0x190]
f940cbe2	ldr x2, [sp, #0x190]
";

    // NB: not a real syntax.
    const LDR_PC_RELATIVE_DB: &str = "
18000162	ldr w2, [pc, #44]
58000162	ldr x2, [pc, #44]
18fffea2	ldr w2, [pc, #-44]
58fffea2	ldr x2, [pc, #-44]
";

    const LDR_PRE_POST_INC_DB: &str = "
b842a441	ldr w1, [x2], #0x2a
b842a7e1	ldr w1, [sp], #0x2a
b842ac41	ldr w1, [x2, #0x2a]!
b842afe1	ldr w1, [sp, #0x2a]!
b85d6441	ldr w1, [x2], #-0x2a
b85d67e1	ldr w1, [sp], #-0x2a
b85d6c41	ldr w1, [x2, #-0x2a]!
b85d6fe1	ldr w1, [sp, #-0x2a]!
f842a441	ldr x1, [x2], #0x2a
f842a7e1	ldr x1, [sp], #0x2a
f842ac41	ldr x1, [x2, #0x2a]!
f842afe1	ldr x1, [sp, #0x2a]!
f85d6441	ldr x1, [x2], #-0x2a
f85d67e1	ldr x1, [sp], #-0x2a
f85d6c41	ldr x1, [x2, #-0x2a]!
f85d6fe1	ldr x1, [sp, #-0x2a]!
";
    use LdrExtendOption32::*;

    test_cases! {
        LDR_REG_EXT_DB, untested_ldr_reg_ext_db;
        test_ldr_r64_r64_r32_sxtw, ldr(X2, (X8, extended(W3, SXTW))), "ldr x2, [x8, w3, sxtw]";
        test_ldr_r64_r64_r32_uxtw, ldr(X2, (X8, extended(W3, UXTW))), "ldr x2, [x8, w3, uxtw]";
        test_ldr_r32_r64_r32_sxtw, ldr(W2, (X8, extended(W3, SXTW))), "ldr w2, [x8, w3, sxtw]";
        test_ldr_r32_r64_r32_uxtw, ldr(W2, (X8, extended(W3, UXTW))), "ldr w2, [x8, w3, uxtw]";
        test_ldr_r32_r64_r64, ldr(W2, (X8, X3)), "ldr w2, [x8, x3]";
        test_ldr_r32_rsp_r64, ldr(W2, (SP, X3)), "ldr w2, [sp, x3]";
        test_ldr_r64_r64_r64, ldr(X2, (X8, X3)), "ldr x2, [x8, x3]";
        test_ldr_r64_rsp_r64, ldr(X2, (SP, X3)), "ldr x2, [sp, x3]";
        test_ldr_r32_r64_xzr, ldr(W2, (X8, XZR)), "ldr w2, [x8, xzr]";
        test_ldr_r32_rsp_xzr, ldr(W2, (SP, XZR)), "ldr w2, [sp, xzr]";
        test_ldr_r64_r64_xzr, ldr(X2, (X8, XZR)), "ldr x2, [x8, xzr]";
        test_ldr_r64_rsp_xzr, ldr(X2, (SP, XZR)), "ldr x2, [sp, xzr]";
    }

    test_cases! {
        LDR_SCALED_IMM_DB, untested_ldr_scaled_imm;
        test_ldr_r32_r64_scaled_imm, ldr(W2, (X8, UBitValue::<12, 2>::new(0x190).unwrap())), "ldr w2, [x8, #0x190]";
        test_ldr_r32_sp_scaled_imm, ldr(W2, (SP, UBitValue::<12, 2>::new(0x190).unwrap())), "ldr w2, [sp, #0x190]";
        test_ldr_r64_r64_scaled_imm, ldr(X2, (X8, UBitValue::<12, 3>::new(0x190).unwrap())), "ldr x2, [x8, #0x190]";
        test_ldr_r64_sp_scaled_imm, ldr(X2, (SP, UBitValue::<12, 3>::new(0x190).unwrap())), "ldr x2, [sp, #0x190]";
    }

    test_cases! {
        LDR_PC_RELATIVE_DB, untested_ldr_pc_relative;
            test_ldr_r32_pc_relative, ldr(W2, (Pc, LdStPcOffset::new(44).unwrap())), "ldr w2, [pc, #44]";
            test_ldr_r64_pc_relative, ldr(X2, (Pc, LdStPcOffset::new(44).unwrap())), "ldr x2, [pc, #44]";
            test_ldr_r32_pc_relative_neg, ldr(W2, (Pc, LdStPcOffset::new(-44).unwrap())), "ldr w2, [pc, #-44]";
            test_ldr_r64_pc_relative_neg, ldr(X2, (Pc, LdStPcOffset::new(-44).unwrap())), "ldr x2, [pc, #-44]";
    }

    test_cases! {
        LDR_PRE_POST_INC_DB, untested_ldr_pre_post_inc;
        test_ldr_r32_r64_pre_inc, ldr(W1, preinc(X2, 0x2a)).unwrap(), "ldr w1, [x2, #0x2a]!";
        test_ldr_r32_r64_post_inc, ldr(W1, postinc(X2, 0x2a)).unwrap(), "ldr w1, [x2], #0x2a";
        test_ldr_r64_r64_pre_inc, ldr(X1, preinc(X2, 0x2a)).unwrap(), "ldr x1, [x2, #0x2a]!";
        test_ldr_r64_r64_post_inc, ldr(X1, postinc(X2, 0x2a)).unwrap(), "ldr x1, [x2], #0x2a";
        test_ldr_r32_sp_pre_inc, ldr(W1, preinc(SP, 0x2a)).unwrap(), "ldr w1, [sp, #0x2a]!";
        test_ldr_r32_sp_post_inc, ldr(W1, postinc(SP, 0x2a)).unwrap(), "ldr w1, [sp], #0x2a";
        test_ldr_r64_sp_pre_inc, ldr(X1, preinc(SP, 0x2a)).unwrap(), "ldr x1, [sp, #0x2a]!";
        test_ldr_r64_sp_post_inc, ldr(X1, postinc(SP, 0x2a)).unwrap(), "ldr x1, [sp], #0x2a";
        test_ldr_r32_r64_pre_inc_neg, ldr(W1, preinc(X2, -0x2a)).unwrap(), "ldr w1, [x2, #-0x2a]!";
        test_ldr_r32_r64_post_inc_neg, ldr(W1, postinc(X2, -0x2a)).unwrap(), "ldr w1, [x2], #-0x2a";
        test_ldr_r64_r64_pre_inc_neg, ldr(X1, preinc(X2, -0x2a)).unwrap(), "ldr x1, [x2, #-0x2a]!";
        test_ldr_r64_r64_post_inc_neg, ldr(X1, postinc(X2, -0x2a)).unwrap(), "ldr x1, [x2], #-0x2a";
        test_ldr_r32_sp_pre_inc_neg, ldr(W1, preinc(SP, -0x2a)).unwrap(), "ldr w1, [sp, #-0x2a]!";
        test_ldr_r32_sp_post_inc_neg, ldr(W1, postinc(SP, -0x2a)).unwrap(), "ldr w1, [sp], #-0x2a";
        test_ldr_r64_sp_pre_inc_neg, ldr(X1, preinc(SP, -0x2a)).unwrap(), "ldr x1, [sp, #-0x2a]!";
        test_ldr_r64_sp_post_inc_neg, ldr(X1, postinc(SP, -0x2a)).unwrap(), "ldr x1, [sp], #-0x2a";
    }
}
