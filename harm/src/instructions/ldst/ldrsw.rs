/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

//! `LDRSW` and related commands.
//!
//! The `ldrsw` function returns an instance of `Instruction` for loading a sign-extended 32-bit
//! value into a 64-bit register from memory. While `LDRSW` has different variants with various
//! addressing modes, the `ldrsw` function takes two arguments: a destination register and an
//! "address" that encapsulates the rest: the base, offsets, extensions, etc. Tuples are often
//! used for the second argument, see the pattern in the examples below.
//!
//! The function is overloaded for various argument types. For some of them an `Instruction` trait
//! instance is returned; for others a `Result` if the arguments need validation. Such argument
//! combinations have `.unwrap()` in examples.
//!
//! # `LDRSW`: Register base with register offset
//!
//! # Examples:
//! ```
//! # use harm::instructions::ldst::{ldrsw, ext, LdStExtendOption32, LdStShift};
//! use harm::register::Reg32::*;
//! use harm::register::Reg64::*;
//! use LdStExtendOption32::*;
//!
//! ldrsw(X1, X2);        // LDRSW X1, [X2]
//! ldrsw(X1, (X2,));     // LDRSW X1, [X2]
//! ldrsw(X1, (X2, X3));  // LDRSW X1, [X2, X3]
//! ldrsw(X1, (X2, ext((W3, UXTW)))); // ldrsw x1, [x2, w3, uxtw]
//! ldrsw(X1, (X2, ext((W3, UXTW, LdStShift::Shifted)))); // ldrsw x1, [x2, w3, uxtw #2]
//! ```
//!
//! Please note that `uxtw` and `sxtw` can be used only with a 32-bit index register, and shift
//! can be absent (unshifted, encodes `#0`) or 2 (shifted, encodes `#2`). The `lsl` and `sxtx`
//! can be used only with 64-bit index registers; same shift rule applies.
//!
//! # `LDRSW`: Register base with immediate offset
//!
//! LDRSW with a register base and a 4-byte-aligned immediate unsigned offset. The offset has 12
//! significant bits available.
//!
//! You may also pass a `u32` or `i32` offset value; an error is returned if the value doesn't fit.
//!
//! Pre-increment and post-increment variants have the following syntax:
//! ```
//! # use harm::instructions::ldst::{ldrsw, inc, preinc, postinc, LdStIncOffset};
//! use harm::register::Reg64::*;
//! let offset = LdStIncOffset::new(4).unwrap();
//! ldrsw(X1, (inc(offset), X2));       // preincrement, LDRSW X1, [X2, #4]!
//! ldrsw(X1, (X2, inc(offset)));       // postincrement, LDRSW X1, [X2], #4
//! // Equivalent to the lines above:
//! ldrsw(X1, preinc(X2, offset));      // preincrement, LDRSW X1, [X2, #4]!
//! ldrsw(X1, postinc(X2, offset));     // postincrement, LDRSW X1, [X2], #4
//! // Fallible variants:
//! ldrsw(X1, (inc(4), X2)).unwrap();   // preincrement, LDRSW X1, [X2, #4]!
//! ldrsw(X1, postinc(X2, 4)).unwrap(); // postincrement, LDRSW X1, [X2], #4
//! ```
//!
//! # `LDRSW`: PC base with immediate offset
//!
//! A signed 19-bit offset (shifted left by 2) is added to `PC`.

use aarchmrs_instructions::A64::ldst::{
    ldst_immpost::LDRSW_64_ldst_immpost::LDRSW_64_ldst_immpost,
    ldst_immpre::LDRSW_64_ldst_immpre::LDRSW_64_ldst_immpre,
    ldst_pos::LDRSW_64_ldst_pos::LDRSW_64_ldst_pos,
    ldst_regoff::LDRSW_64_ldst_regoff::LDRSW_64_ldst_regoff,
    loadlit::LDRSW_64_loadlit::LDRSW_64_loadlit,
};

use super::args::LdStArgs;
use super::shift_extend::*;
use super::{Inc, LdStIncOffset, LdStPcOffset, Pc, ScaledOffset32};
use crate::{
    bits::BitError,
    instructions::{RawInstruction, RelocatableInstruction},
    register::{IntoReg, RegOrSp64, RegOrZero32, RegOrZero64, Register},
    sealed::Sealed,
};

/// A `ldrsw` instruction with a destination and an address.
#[derive(Debug, Copy, Clone)]
pub struct Ldrsw<Args>(pub Args);

impl<Args: Sealed> Sealed for Ldrsw<Args> {}

/// Defines possible ways to construct a `ldrsw` instruction.
pub trait MakeLdrsw<RtIn, AddrIn>: Sealed {
    type Output;
    fn new(rt: RtIn, addr: AddrIn) -> Self::Output;
}

// ── Register offset: extended 64-bit register ────────────────────────────────

impl<RtIn, Base, Ext> MakeLdrsw<RtIn, (Base, Ext)>
    for Ldrsw<LdStArgs<RegOrZero64, (RegOrSp64, Extended<RegOrZero32, RegOrZero64>)>>
where
    RtIn: IntoReg<RegOrZero64>,
    Base: IntoReg<RegOrSp64>,
    Ext: Into<Extended<RegOrZero32, RegOrZero64>>,
{
    type Output = Self;
    #[inline]
    fn new(rt: RtIn, (base, ext): (Base, Ext)) -> Self {
        Self(LdStArgs { rt: rt.into_reg(), addr: (base.into_reg(), ext.into()) })
    }
}

// ── Register offset: extended 32-bit register ────────────────────────────────

impl<RtIn, Base, Ext> MakeLdrsw<RtIn, (Base, Ext)>
    for Ldrsw<LdStArgs<RegOrZero64, (RegOrSp64, Extended<RegOrZero32, RegOrZero32>)>>
where
    RtIn: IntoReg<RegOrZero64>,
    Base: IntoReg<RegOrSp64>,
    Ext: Into<Extended<RegOrZero32, RegOrZero32>>,
{
    type Output = Self;
    #[inline]
    fn new(rt: RtIn, (base, ext): (Base, Ext)) -> Self {
        Self(LdStArgs { rt: rt.into_reg(), addr: (base.into_reg(), ext.into()) })
    }
}

// ── Register offset: bare 64-bit register ────────────────────────────────────

impl<RtIn, Base, OffsetReg> MakeLdrsw<RtIn, (Base, OffsetReg)>
    for Ldrsw<LdStArgs<RegOrZero64, (RegOrSp64, RegOrZero64)>>
where
    RtIn: IntoReg<RegOrZero64>,
    Base: IntoReg<RegOrSp64>,
    OffsetReg: IntoReg<RegOrZero64>,
{
    type Output = Self;
    #[inline]
    fn new(rt: RtIn, (base, offset): (Base, OffsetReg)) -> Self {
        Self(LdStArgs { rt: rt.into_reg(), addr: (base.into_reg(), offset.into_reg()) })
    }
}

// ── Scaled immediate offset: bare base ───────────────────────────────────────

impl<RtIn, Base> MakeLdrsw<RtIn, Base>
    for Ldrsw<LdStArgs<RegOrZero64, (RegOrSp64, ScaledOffset32)>>
where
    RtIn: IntoReg<RegOrZero64>,
    Base: IntoReg<RegOrSp64>,
{
    type Output = Self;
    #[inline]
    fn new(rt: RtIn, base: Base) -> Self {
        Self(LdStArgs { rt: rt.into_reg(), addr: (base.into_reg(), Default::default()) })
    }
}

// ── Scaled immediate offset: 1-tuple base ────────────────────────────────────

impl<RtIn, Base> MakeLdrsw<RtIn, (Base,)>
    for Ldrsw<LdStArgs<RegOrZero64, (RegOrSp64, ScaledOffset32)>>
where
    RtIn: IntoReg<RegOrZero64>,
    Base: IntoReg<RegOrSp64>,
{
    type Output = Self;
    #[inline]
    fn new(rt: RtIn, (base,): (Base,)) -> Self {
        Self(LdStArgs { rt: rt.into_reg(), addr: (base.into_reg(), Default::default()) })
    }
}

// ── Scaled immediate offset: typed ───────────────────────────────────────────

impl<RtIn, Base> MakeLdrsw<RtIn, (Base, ScaledOffset32)>
    for Ldrsw<LdStArgs<RegOrZero64, (RegOrSp64, ScaledOffset32)>>
where
    RtIn: IntoReg<RegOrZero64>,
    Base: IntoReg<RegOrSp64>,
{
    type Output = Self;
    #[inline]
    fn new(rt: RtIn, (base, offset): (Base, ScaledOffset32)) -> Self {
        Self(LdStArgs { rt: rt.into_reg(), addr: (base.into_reg(), offset) })
    }
}

// ── Scaled immediate offset: u32 (fallible) ──────────────────────────────────

impl<RtIn, Base> MakeLdrsw<RtIn, (Base, u32)>
    for Ldrsw<LdStArgs<RegOrZero64, (RegOrSp64, ScaledOffset32)>>
where
    RtIn: IntoReg<RegOrZero64>,
    Base: IntoReg<RegOrSp64>,
{
    type Output = Result<Self, BitError>;
    #[inline]
    fn new(rt: RtIn, (base, offset): (Base, u32)) -> Self::Output {
        ScaledOffset32::try_from(offset)
            .map(|offset| Self(LdStArgs { rt: rt.into_reg(), addr: (base.into_reg(), offset) }))
    }
}

// ── Scaled immediate offset: i32 (fallible) ──────────────────────────────────

impl<RtIn, Base> MakeLdrsw<RtIn, (Base, i32)>
    for Ldrsw<LdStArgs<RegOrZero64, (RegOrSp64, ScaledOffset32)>>
where
    RtIn: IntoReg<RegOrZero64>,
    Base: IntoReg<RegOrSp64>,
{
    type Output = Result<Self, BitError>;
    #[inline]
    fn new(rt: RtIn, (base, offset): (Base, i32)) -> Self::Output {
        ScaledOffset32::try_from(offset)
            .map(|offset| Self(LdStArgs { rt: rt.into_reg(), addr: (base.into_reg(), offset) }))
    }
}

// ── Pre-increment ─────────────────────────────────────────────────────────────

impl<RtIn, Base> MakeLdrsw<RtIn, (Inc<LdStIncOffset>, Base)>
    for Ldrsw<LdStArgs<RegOrZero64, (Inc<LdStIncOffset>, RegOrSp64)>>
where
    RtIn: IntoReg<RegOrZero64>,
    Base: IntoReg<RegOrSp64>,
{
    type Output = Self;
    #[inline]
    fn new(rt: RtIn, (inc, base): (Inc<LdStIncOffset>, Base)) -> Self {
        Self(LdStArgs { rt: rt.into_reg(), addr: (inc, base.into_reg()) })
    }
}

// ── Post-increment ────────────────────────────────────────────────────────────

impl<RtIn, Base> MakeLdrsw<RtIn, (Base, Inc<LdStIncOffset>)>
    for Ldrsw<LdStArgs<RegOrZero64, (RegOrSp64, Inc<LdStIncOffset>)>>
where
    RtIn: IntoReg<RegOrZero64>,
    Base: IntoReg<RegOrSp64>,
{
    type Output = Self;
    #[inline]
    fn new(rt: RtIn, (base, inc): (Base, Inc<LdStIncOffset>)) -> Self {
        Self(LdStArgs { rt: rt.into_reg(), addr: (base.into_reg(), inc) })
    }
}

// ── PC-relative: typed offset ─────────────────────────────────────────────────

impl<RtIn> MakeLdrsw<RtIn, (Pc, LdStPcOffset)>
    for Ldrsw<LdStArgs<RegOrZero64, (Pc, LdStPcOffset)>>
where
    RtIn: IntoReg<RegOrZero64>,
{
    type Output = Self;
    #[inline]
    fn new(rt: RtIn, addr: (Pc, LdStPcOffset)) -> Self {
        Self(LdStArgs { rt: rt.into_reg(), addr })
    }
}

// ── PC-relative: i32 (fallible) ───────────────────────────────────────────────

impl<RtIn> MakeLdrsw<RtIn, (Pc, i32)>
    for Ldrsw<LdStArgs<RegOrZero64, (Pc, LdStPcOffset)>>
where
    RtIn: IntoReg<RegOrZero64>,
{
    type Output = Result<Self, BitError>;
    #[inline]
    fn new(rt: RtIn, (pc, offset): (Pc, i32)) -> Self::Output {
        LdStPcOffset::new(offset)
            .map(|offset| Self(LdStArgs { rt: rt.into_reg(), addr: (pc, offset) }))
    }
}

// ── PC-relative: LabelRef ─────────────────────────────────────────────────────

impl<RtIn> MakeLdrsw<RtIn, crate::reloc::LabelRef>
    for Ldrsw<LdStArgs<RegOrZero64, crate::reloc::LabelRef>>
where
    RtIn: IntoReg<RegOrZero64>,
{
    type Output = Self;
    #[inline]
    fn new(rt: RtIn, addr: crate::reloc::LabelRef) -> Self {
        Self(LdStArgs { rt: rt.into_reg(), addr })
    }
}

// ── Fallible wrappers ─────────────────────────────────────────────────────────

impl<Rt, RtIn, BaseIn, Ext, Err> MakeLdrsw<RtIn, (BaseIn, Result<Ext, Err>)>
    for Ldrsw<LdStArgs<Rt, (RegOrSp64, Ext)>>
where
    Ldrsw<LdStArgs<Rt, (RegOrSp64, Ext)>>: MakeLdrsw<RtIn, (BaseIn, Ext)>,
    BaseIn: IntoReg<RegOrSp64>,
{
    type Output = Result<<Self as MakeLdrsw<RtIn, (BaseIn, Ext)>>::Output, Err>;
    #[inline]
    fn new(rt: RtIn, (base, ext_res): (BaseIn, Result<Ext, Err>)) -> Self::Output {
        ext_res.map(|ext| <Self as MakeLdrsw<RtIn, (BaseIn, Ext)>>::new(rt, (base, ext)))
    }
}

impl<Rt, RtIn, BaseIn, Ext, Err> MakeLdrsw<RtIn, (Result<Ext, Err>, BaseIn)>
    for Ldrsw<LdStArgs<Rt, (Ext, RegOrSp64)>>
where
    Ldrsw<LdStArgs<Rt, (Ext, RegOrSp64)>>: MakeLdrsw<RtIn, (Ext, BaseIn)>,
    BaseIn: IntoReg<RegOrSp64>,
{
    type Output = Result<<Self as MakeLdrsw<RtIn, (Ext, BaseIn)>>::Output, Err>;
    #[inline]
    fn new(rt: RtIn, (ext_res, base): (Result<Ext, Err>, BaseIn)) -> Self::Output {
        ext_res.map(|ext| <Self as MakeLdrsw<RtIn, (Ext, BaseIn)>>::new(rt, (ext, base)))
    }
}

impl<Rt, RtIn, Addr, Err> MakeLdrsw<RtIn, Result<Addr, Err>>
    for Ldrsw<LdStArgs<Rt, Addr>>
where
    Ldrsw<LdStArgs<Rt, Addr>>: MakeLdrsw<RtIn, Addr>,
{
    type Output = Result<<Self as MakeLdrsw<RtIn, Addr>>::Output, Err>;
    #[inline]
    fn new(rt: RtIn, addr_res: Result<Addr, Err>) -> Self::Output {
        addr_res.map(|addr| <Self as MakeLdrsw<RtIn, Addr>>::new(rt, addr))
    }
}

/// ldrsw construction function.  See examples in the module documentation.
pub fn ldrsw<RtIn, Rt, AddrIn, Addr>(
    dst: RtIn,
    addr: AddrIn,
) -> <Ldrsw<LdStArgs<Rt, Addr>> as MakeLdrsw<RtIn, AddrIn>>::Output
where
    Ldrsw<LdStArgs<Rt, Addr>>: MakeLdrsw<RtIn, AddrIn>,
{
    <Ldrsw<LdStArgs<Rt, Addr>> as MakeLdrsw<RtIn, AddrIn>>::new(dst, addr)
}

// ══ RawInstruction ════════════════════════════════════════════════════════════

impl RawInstruction
    for Ldrsw<LdStArgs<RegOrZero64, (RegOrSp64, Extended<RegOrZero32, RegOrZero64>)>>
{
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let (base, offset) = self.0.addr;
        LDRSW_64_ldst_regoff(
            offset.offset.index(),
            (offset.extend as u8).into(),
            offset.shifted.into(),
            base.index(),
            self.0.rt.index(),
        )
    }
}

impl RawInstruction
    for Ldrsw<LdStArgs<RegOrZero64, (RegOrSp64, Extended<RegOrZero32, RegOrZero32>)>>
{
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let (base, offset) = self.0.addr;
        LDRSW_64_ldst_regoff(
            offset.offset.index(),
            (offset.extend as u8).into(),
            offset.shifted.into(),
            base.index(),
            self.0.rt.index(),
        )
    }
}

impl RawInstruction for Ldrsw<LdStArgs<RegOrZero64, (RegOrSp64, RegOrZero64)>> {
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let (base, offset) = self.0.addr;
        LDRSW_64_ldst_regoff(
            offset.index(),
            (LdStExtendOption64::default() as u8).into(),
            0b0.into(),
            base.index(),
            self.0.rt.index(),
        )
    }
}

impl RawInstruction for Ldrsw<LdStArgs<RegOrZero64, (RegOrSp64, ScaledOffset32)>> {
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let (base, offset) = self.0.addr;
        LDRSW_64_ldst_pos(offset.into(), base.index(), self.0.rt.index())
    }
}

impl RawInstruction for Ldrsw<LdStArgs<RegOrZero64, (Inc<LdStIncOffset>, RegOrSp64)>> {
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let (inc, base) = self.0.addr;
        LDRSW_64_ldst_immpre(inc.offset.into(), base.index(), self.0.rt.index())
    }
}

impl RawInstruction for Ldrsw<LdStArgs<RegOrZero64, (RegOrSp64, Inc<LdStIncOffset>)>> {
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let (base, inc) = self.0.addr;
        LDRSW_64_ldst_immpost(inc.offset.into(), base.index(), self.0.rt.index())
    }
}

impl RawInstruction for Ldrsw<LdStArgs<RegOrZero64, (Pc, LdStPcOffset)>> {
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let (_pc, offset) = self.0.addr;
        LDRSW_64_loadlit(offset.into(), self.0.rt.index())
    }
}

impl RelocatableInstruction for Ldrsw<LdStArgs<RegOrZero64, crate::reloc::LabelRef>> {
    #[inline]
    fn to_code_with_reloc(
        &self,
    ) -> (aarchmrs_types::InstructionCode, Option<crate::reloc::Rel64>) {
        let code = LDRSW_64_loadlit(0.into(), self.0.rt.index());
        let rel = crate::reloc::Rel64::ld_prel_lo19(self.0.addr);
        (code, Some(rel))
    }
}

#[cfg(test)]
mod tests {
    use harm_test_utils::test_cases;

    use super::*;
    use crate::instructions::InstructionSeq;
    use crate::register::Reg32::*;
    use crate::register::Reg64::*;
    use crate::{
        bits::UBitValue,
        instructions::ldst::{LdStPcOffset, Pc, inc, postinc, preinc},
    };
    use LdStExtendOption32::*;
    use LdStExtendOption64::*;
    use RegOrSp64::SP;
    use RegOrZero32::WZR;
    use RegOrZero64::XZR;

    const LDRSW_REG_EXT_DB: &str = "
b8a34902	ldrsw x2, [x8, w3, uxtw #0]
b8a34902	ldrsw x2, [x8, w3, uxtw]
b8a35902	ldrsw x2, [x8, w3, uxtw #2]
b8a36902	ldrsw x2, [x8, x3, lsl #0]
b8a36902	ldrsw x2, [x8, x3]
b8a36be2	ldrsw x2, [sp, x3]
b8a37902	ldrsw x2, [x8, x3, lsl #2]
b8a3c902	ldrsw x2, [x8, w3, sxtw #0]
b8a3c902	ldrsw x2, [x8, w3, sxtw]
b8a3d902	ldrsw x2, [x8, w3, sxtw #2]
b8a3e902	ldrsw x2, [x8, x3, sxtx #0]
b8a3e902	ldrsw x2, [x8, x3, sxtx]
b8a3f902	ldrsw x2, [x8, x3, sxtx #2]
b8a9491f	ldrsw xzr, [x8, w9, uxtw]
b8a9691f	ldrsw xzr, [x8, x9]
b8a9691f	ldrsw xzr, [x8, x9]
b8a9c91f	ldrsw xzr, [x8, w9, sxtw]
b8bf4902	ldrsw x2, [x8, wzr, uxtw]
b8bf6902	ldrsw x2, [x8, xzr]
b8bf6be2	ldrsw x2, [sp, xzr]
b8bf7902	ldrsw x2, [x8, xzr, lsl #2]
b8bfc902	ldrsw x2, [x8, wzr, sxtw]
b8bfe902	ldrsw x2, [x8, xzr, sxtx]
";

    // 'ldrsw (w2|x2), [(x8|sp), #0x190]'
    const LDRSW_SCALED_IMM_DB: &str = "
b9819102	ldrsw x2, [x8, #0x190]
b98193e2	ldrsw x2, [sp, #0x190]
b98193ff	ldrsw xzr, [sp, #0x190]
b9800102	ldrsw x2, [x8]
";

    // NB: not a real syntax.
    const LDRSW_PC_RELATIVE_DB: &str = "
98000162	ldrsw x2, [pc, #44]
98fffea2	ldrsw x2, [pc, #-44]
";

    const LDRSW_PRE_POST_INC_DB: &str = "
b882a441	ldrsw x1, [x2], #0x2a
b882a45f	ldrsw xzr, [x2], #0x2a
b882a7e1	ldrsw x1, [sp], #0x2a
b882ac41	ldrsw x1, [x2, #0x2a]!
b882ac5f	ldrsw xzr, [x2, #0x2a]!
b882afe1	ldrsw x1, [sp, #0x2a]!
b89d6441	ldrsw x1, [x2], #-0x2a
b89d67e1	ldrsw x1, [sp], #-0x2a
b89d6c41	ldrsw x1, [x2, #-0x2a]!
b89d6fe1	ldrsw x1, [sp, #-0x2a]!
";
    test_cases! {
        LDRSW_REG_EXT_DB, untested_ldrsw_reg_ext_db;
        test_ldrsw_r64_r64_r32_sxtw, ldrsw(X2, (X8, ext((W3, SXTW)))), "ldrsw x2, [x8, w3, sxtw]";
        test_ldrsw_r64_r64_r32_uxtw, ldrsw(X2, (X8, ext((W3, UXTW)))), "ldrsw x2, [x8, w3, uxtw]";
        test_ldrsw_r64_r64_r64_sxtx, ldrsw(X2, (X8, ext((X3, SXTX)))), "ldrsw x2, [x8, x3, sxtx]";
        test_ldrsw_r64_r64_xzr_sxtx, ldrsw(X2, (X8, ext((XZR, SXTX)))), "ldrsw x2, [x8, xzr, sxtx]";
        test_ldrsw_r64_r64_xzr_lsl_2, ldrsw(X2, (X8, ext((XZR, LSL, 2)))).unwrap(), "ldrsw x2, [x8, xzr, lsl #2]";
        test_ldrsw_r64_r64_wzr_sxtw, ldrsw(X2, (X8, ext((WZR, SXTW)))), "ldrsw x2, [x8, wzr, sxtw]";
        test_ldrsw_r64_r64_wzr_uxtx, ldrsw(X2, (X8, ext((WZR, UXTW)))), "ldrsw x2, [x8, wzr, uxtw]";
        test_ldrsw_r64_r64_r32_sxtw_0, ldrsw(X2, (X8, ext((W3, SXTW, 0)))).unwrap(), "ldrsw x2, [x8, w3, sxtw #0]";
        test_ldrsw_r64_r64_r32_sxtw_2, ldrsw(X2, (X8, ext((W3, SXTW, 2)))).unwrap(), "ldrsw x2, [x8, w3, sxtw #2]";
        test_ldrsw_r64_r64_r32_uxtw_0, ldrsw(X2, (X8, ext((W3, UXTW, 0)))).unwrap(), "ldrsw x2, [x8, w3, uxtw #0]";
        test_ldrsw_r64_r64_r32_uxtw_2, ldrsw(X2, (X8, ext((W3, UXTW, 2)))).unwrap(), "ldrsw x2, [x8, w3, uxtw #2]";
        test_ldrsw_r64_r64_r64_lsl_0, ldrsw(X2, (X8, ext((X3, LSL, 0)))).unwrap(), "ldrsw x2, [x8, x3, lsl #0]";
        test_ldrsw_r64_r64_r64_lsl_2, ldrsw(X2, (X8, ext((X3, LSL, 2)))).unwrap(), "ldrsw x2, [x8, x3, lsl #2]";
        test_ldrsw_r64_r64_r32_sxtx_0, ldrsw(X2, (X8, ext((X3, SXTX, 0)))).unwrap(), "ldrsw x2, [x8, x3, sxtx #0]";
        test_ldrsw_r64_r64_r32_sxtx_2, ldrsw(X2, (X8, ext((X3, SXTX, 2)))).unwrap(), "ldrsw x2, [x8, x3, sxtx #2]";
        test_ldrsw_r64_r64_r64, ldrsw(X2, (X8, X3)), "ldrsw x2, [x8, x3]";
        test_ldrsw_r64_rsp_r64, ldrsw(X2, (SP, X3)), "ldrsw x2, [sp, x3]";
        test_ldrsw_r64_r64_xzr, ldrsw(X2, (X8, XZR)), "ldrsw x2, [x8, xzr]";
        test_ldrsw_r64_rsp_xzr, ldrsw(X2, (SP, XZR)), "ldrsw x2, [sp, xzr]";
        test_ldrsw_xzr_r64_r64, ldrsw(XZR, (X8, X9)), "ldrsw xzr, [x8, x9]";
        test_ldrsw_wzr_r64_r64, ldrsw(XZR, (X8, X9)), "ldrsw xzr, [x8, x9]";
        test_ldrsw_xzr_r64_r32_sxtw, ldrsw(XZR, (X8, ext((W9, SXTW)))), "ldrsw xzr, [x8, w9, sxtw]";
        test_ldrsw_wzr_r64_r32_uxtw, ldrsw(XZR, (X8, ext((W9, UXTW)))), "ldrsw xzr, [x8, w9, uxtw]";
    }

    test_cases! {
        LDRSW_SCALED_IMM_DB, untested_ldrsw_scaled_imm;
        test_ldrsw_r64_r64_scaled_imm, ldrsw(X2, (X8, UBitValue::<12, 2>::new(0x190).unwrap())), "ldrsw x2, [x8, #0x190]";
        test_ldrsw_r64_sp_scaled_imm, ldrsw(X2, (SP, UBitValue::<12, 2>::new(0x190).unwrap())), "ldrsw x2, [sp, #0x190]";
        test_ldrsw_r64_sp_scaled_imm2, ldrsw(X2, (SP, 0x190u32)).unwrap(), "ldrsw x2, [sp, #0x190]";
        test_ldrsw_xzr_sp_scaled_imm2, ldrsw(XZR, (SP, 0x190u32)).unwrap(), "ldrsw xzr, [sp, #0x190]";
        test_ldrsw_r64_sp_scaled_imm3, ldrsw(X2, (SP, 0x190i32)).unwrap(), "ldrsw x2, [sp, #0x190]";
        test_ldrsw_xzr_sp_scaled_imm3, ldrsw(XZR, (SP, 0x190i32)).unwrap(), "ldrsw xzr, [sp, #0x190]";
        test_ldrsw_r64_r64_simple, ldrsw(X2, (X8,)), "ldrsw x2, [x8]";
    }

    test_cases! {
        LDRSW_PC_RELATIVE_DB, untested_ldrsw_pc_relative;
        test_ldrsw_r64_pc_relative, ldrsw(X2, (Pc, LdStPcOffset::new(44).unwrap())), "ldrsw x2, [pc, #44]";
        test_ldrsw_r64_pc_relative_neg, ldrsw(X2, (Pc, LdStPcOffset::new(-44).unwrap())), "ldrsw x2, [pc, #-44]";
        test_ldrsw_r64_pc_relative2, ldrsw(X2, (Pc, 44)).unwrap(), "ldrsw x2, [pc, #44]";
    }

    test_cases! {
        LDRSW_PRE_POST_INC_DB, untested_ldrsw_pre_post_inc;
        test_ldrsw_r64_r64_preinc, ldrsw(X1, preinc(X2, 0x2a)).unwrap(), "ldrsw x1, [x2, #0x2a]!";
        test_ldrsw_r64_r64_postinc, ldrsw(X1, postinc(X2, 0x2a)).unwrap(), "ldrsw x1, [x2], #0x2a";
        test_ldrsw_r64_sp_preinc, ldrsw(X1, preinc(SP, 0x2a)).unwrap(), "ldrsw x1, [sp, #0x2a]!";
        test_ldrsw_r64_sp_postinc, ldrsw(X1, postinc(SP, 0x2a)).unwrap(), "ldrsw x1, [sp], #0x2a";
        test_ldrsw_r64_r64_preinc_neg, ldrsw(X1, preinc(X2, -0x2a)).unwrap(), "ldrsw x1, [x2, #-0x2a]!";
        test_ldrsw_r64_r64_postinc_neg, ldrsw(X1, postinc(X2, -0x2a)).unwrap(), "ldrsw x1, [x2], #-0x2a";
        test_ldrsw_r64_sp_preinc_neg, ldrsw(X1, preinc(SP, -0x2a)).unwrap(), "ldrsw x1, [sp, #-0x2a]!";
        test_ldrsw_r64_sp_postinc_neg, ldrsw(X1, postinc(SP, -0x2a)).unwrap(), "ldrsw x1, [sp], #-0x2a";
        test_ldrsw_r64_r64_preinc_neg2, ldrsw(X1, preinc(X2, LdStIncOffset::new(-0x2a).unwrap())), "ldrsw x1, [x2, #-0x2a]!";
        test_ldrsw_r64_r64_pre_inc, ldrsw(X1, (inc(0x2a), X2)).unwrap(), "ldrsw x1, [x2, #0x2a]!";
        test_ldrsw_r64_r64_post_inc, ldrsw(X1, (X2, inc(0x2a))).unwrap(), "ldrsw x1, [x2], #0x2a";
        test_ldrsw_r64_sp_pre_inc, ldrsw(X1, (inc(0x2a), SP)).unwrap(), "ldrsw x1, [sp, #0x2a]!";
        test_ldrsw_r64_sp_post_inc, ldrsw(X1, (SP, inc(0x2a))).unwrap(), "ldrsw x1, [sp], #0x2a";
        test_ldrsw_r64_r64_pre_inc_neg, ldrsw(X1, (inc(-0x2a), X2)).unwrap(), "ldrsw x1, [x2, #-0x2a]!";
        test_ldrsw_r64_r64_post_inc_neg, ldrsw(X1, (X2, inc(-0x2a))).unwrap(), "ldrsw x1, [x2], #-0x2a";
        test_ldrsw_r64_sp_pre_inc_neg, ldrsw(X1, (inc(-0x2a), SP)).unwrap(), "ldrsw x1, [sp, #-0x2a]!";
        test_ldrsw_r64_sp_post_inc_neg, ldrsw(X1, (SP, inc(-0x2a))).unwrap(), "ldrsw x1, [sp], #-0x2a";
        test_ldrsw_r64_r64_pre_inc_neg2, ldrsw(X1, (inc(LdStIncOffset::new(-0x2a).unwrap()), X2)), "ldrsw x1, [x2, #-0x2a]!";
        test_ldrsw_xzr_r64_pre_inc, ldrsw(XZR, (inc(0x2a), X2)).unwrap(), "ldrsw xzr, [x2, #0x2a]!";
        test_ldrsw_xzr_r64_post_inc, ldrsw(XZR, (X2, inc(0x2a))).unwrap(), "ldrsw xzr, [x2], #0x2a";
    }
}
