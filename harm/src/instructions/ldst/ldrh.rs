/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

//! `LDRH` and related commands.
//!
//! The `ldrh` function returns an instance of `Instruction` for loading a halfword into a 32-bit
//! register from memory. While `LDRH` has different variants with various addressing modes, the
//! `ldrh` function takes two arguments: a destination register and an "address" that encapsulates
//! the rest: the base, offsets, extensions, etc. Tuples are often used for the second argument,
//! see the pattern in the examples below.
//!
//! The function is overloaded for various argument types. For some of them an `Instruction` trait
//! instance is returned; for others a `Result` if the arguments need validation. Such argument
//! combinations have `.unwrap()` in examples.
//!
//! # `LDRH`: Register base with register offset
//!
//! # Examples:
//! ```
//! # use harm::instructions::ldst::{ldrh, ext, LdStExtendOption32, LdStShift};
//! use harm::register::Reg32::*;
//! use harm::register::Reg64::*;
//! use LdStExtendOption32::*;
//!
//! ldrh(W1, X2);        // LDRH W1, [X2]
//! ldrh(W1, (X2,));     // LDRH W1, [X2]
//! ldrh(W1, (X2, X3));  // LDRH W1, [X2, X3]
//! ldrh(W1, (X2, ext((W3, UXTW)))); // ldrh w1, [x2, w3, uxtw]
//! ldrh(W1, (X2, ext((W3, UXTW, LdStShift::Unshifted)))); // ldrh w1, [x2, w3, uxtw #0]
//! ldrh(W1, (X2, ext((W3, UXTW, LdStShift::Shifted)))); // ldrh w1, [x2, w3, uxtw #1]
//! ```
//!
//! Please note that `uxtw` and `sxtw` can be used only with a 32-bit index register, and shift
//! can be absent, 0, or 1.  The `lsl` and `sxtx` can be used only with 64-bit index registers;
//! shift can be absent, 0, or 1.
//!
//! # `LDRH`: Register base with immediate offset
//!
//! LDRH with a register base and a 2-byte-aligned immediate offset. The offset has 12 significant
//! bits available.
//!
//! You may also pass a `u32` offset value; an error is returned if the value doesn't fit.
//!
//! Examples:
//! ```ignore
//! let halfword_offset: ScaledOffset16 = ...;
//!
//! ldrh(W1, (X2, offset as u32)).unwrap(),
//! ldrh(W1, (X2, halfword_offset)),
//! ```
//!
//! Pre-increment and post-increment variants have the following syntax:
//! ```
//! # use harm::instructions::ldst::{ldrh, inc, preinc, postinc, LdStIncOffset};
//! use harm::register::Reg32::*;
//! use harm::register::Reg64::*;
//! let offset = LdStIncOffset::new(4).unwrap();
//! ldrh(W1, (inc(offset), X2));       // preincrement, LDRH W1, [X2, #4]!
//! ldrh(W1, (X2, inc(offset)));       // postincrement, LDRH W1, [X2], #4
//! // Equivalent to the lines above:
//! ldrh(W1, preinc(X2, offset));      // preincrement, LDRH W1, [X2, #4]!
//! ldrh(W1, postinc(X2, offset));     // postincrement, LDRH W1, [X2], #4
//! // Fallible variants:
//! ldrh(W1, (inc(4), X2)).unwrap();   // preincrement, LDRH W1, [X2, #4]!
//! ldrh(W1, postinc(X2, 4)).unwrap(); // postincrement, LDRH W1, [X2], #4
//! ```

use aarchmrs_instructions::A64::ldst::{
    ldst_immpost::LDRH_32_ldst_immpost::LDRH_32_ldst_immpost,
    ldst_immpre::LDRH_32_ldst_immpre::LDRH_32_ldst_immpre,
    ldst_pos::LDRH_32_ldst_pos::LDRH_32_ldst_pos,
    ldst_regoff::LDRH_32_ldst_regoff::LDRH_32_ldst_regoff,
};

use super::args::LdStArgs;
use super::shift_extend::*;
use super::{HalfShift, Inc, LdStIncOffset, ScaledOffset16};
use crate::{
    bits::BitError,
    instructions::RawInstruction,
    register::{IntoReg, RegOrSp64, RegOrZero32, RegOrZero64, Register},
    sealed::Sealed,
};

/// A `ldrh` instruction with a destination and an address.
#[derive(Debug, Copy, Clone)]
pub struct Ldrh<Args>(pub Args);

impl<Args: Sealed> Sealed for Ldrh<Args> {}

/// Defines possible ways to construct a `ldrh` instruction.
pub trait MakeLdrh<RtIn, AddrIn>: Sealed {
    type Output;
    fn new(rt: RtIn, addr: AddrIn) -> Self::Output;
}

// ── Register offset: extended 64-bit register ────────────────────────────────

impl<RtIn, Base, Ext> MakeLdrh<RtIn, (Base, Ext)>
    for Ldrh<LdStArgs<RegOrZero32, (RegOrSp64, Extended<HalfShift, RegOrZero64>)>>
where
    RtIn: IntoReg<RegOrZero32>,
    Base: IntoReg<RegOrSp64>,
    Ext: Into<Extended<HalfShift, RegOrZero64>>,
{
    type Output = Self;
    #[inline]
    fn new(rt: RtIn, (base, ext): (Base, Ext)) -> Self {
        Self(LdStArgs { rt: rt.into_reg(), addr: (base.into_reg(), ext.into()) })
    }
}

// ── Register offset: extended 32-bit register ────────────────────────────────

impl<RtIn, Base, Ext> MakeLdrh<RtIn, (Base, Ext)>
    for Ldrh<LdStArgs<RegOrZero32, (RegOrSp64, Extended<HalfShift, RegOrZero32>)>>
where
    RtIn: IntoReg<RegOrZero32>,
    Base: IntoReg<RegOrSp64>,
    Ext: Into<Extended<HalfShift, RegOrZero32>>,
{
    type Output = Self;
    #[inline]
    fn new(rt: RtIn, (base, ext): (Base, Ext)) -> Self {
        Self(LdStArgs { rt: rt.into_reg(), addr: (base.into_reg(), ext.into()) })
    }
}

// ── Register offset: bare 64-bit register ────────────────────────────────────

impl<RtIn, Base, OffsetReg> MakeLdrh<RtIn, (Base, OffsetReg)>
    for Ldrh<LdStArgs<RegOrZero32, (RegOrSp64, RegOrZero64)>>
where
    RtIn: IntoReg<RegOrZero32>,
    Base: IntoReg<RegOrSp64>,
    OffsetReg: IntoReg<RegOrZero64>,
{
    type Output = Self;
    #[inline]
    fn new(rt: RtIn, (base, offset): (Base, OffsetReg)) -> Self {
        Self(LdStArgs { rt: rt.into_reg(), addr: (base.into_reg(), offset.into_reg()) })
    }
}

// ── Scaled immediate offset: bare base (zero offset) ─────────────────────────

impl<RtIn, Base> MakeLdrh<RtIn, Base>
    for Ldrh<LdStArgs<RegOrZero32, (RegOrSp64, ScaledOffset16)>>
where
    RtIn: IntoReg<RegOrZero32>,
    Base: IntoReg<RegOrSp64>,
{
    type Output = Self;
    #[inline]
    fn new(rt: RtIn, base: Base) -> Self {
        Self(LdStArgs { rt: rt.into_reg(), addr: (base.into_reg(), Default::default()) })
    }
}

// ── Scaled immediate offset: 1-tuple base ────────────────────────────────────

impl<RtIn, Base> MakeLdrh<RtIn, (Base,)>
    for Ldrh<LdStArgs<RegOrZero32, (RegOrSp64, ScaledOffset16)>>
where
    RtIn: IntoReg<RegOrZero32>,
    Base: IntoReg<RegOrSp64>,
{
    type Output = Self;
    #[inline]
    fn new(rt: RtIn, (base,): (Base,)) -> Self {
        Self(LdStArgs { rt: rt.into_reg(), addr: (base.into_reg(), Default::default()) })
    }
}

// ── Scaled immediate offset: typed ───────────────────────────────────────────

impl<RtIn, Base> MakeLdrh<RtIn, (Base, ScaledOffset16)>
    for Ldrh<LdStArgs<RegOrZero32, (RegOrSp64, ScaledOffset16)>>
where
    RtIn: IntoReg<RegOrZero32>,
    Base: IntoReg<RegOrSp64>,
{
    type Output = Self;
    #[inline]
    fn new(rt: RtIn, (base, offset): (Base, ScaledOffset16)) -> Self {
        Self(LdStArgs { rt: rt.into_reg(), addr: (base.into_reg(), offset) })
    }
}

// ── Scaled immediate offset: u32 (fallible) ──────────────────────────────────

impl<RtIn, Base> MakeLdrh<RtIn, (Base, u32)>
    for Ldrh<LdStArgs<RegOrZero32, (RegOrSp64, ScaledOffset16)>>
where
    RtIn: IntoReg<RegOrZero32>,
    Base: IntoReg<RegOrSp64>,
{
    type Output = Result<Self, BitError>;
    #[inline]
    fn new(rt: RtIn, (base, offset): (Base, u32)) -> Self::Output {
        ScaledOffset16::try_from(offset)
            .map(|offset| Self(LdStArgs { rt: rt.into_reg(), addr: (base.into_reg(), offset) }))
    }
}

// ── Scaled immediate offset: i32 (fallible) ──────────────────────────────────

impl<RtIn, Base> MakeLdrh<RtIn, (Base, i32)>
    for Ldrh<LdStArgs<RegOrZero32, (RegOrSp64, ScaledOffset16)>>
where
    RtIn: IntoReg<RegOrZero32>,
    Base: IntoReg<RegOrSp64>,
{
    type Output = Result<Self, BitError>;
    #[inline]
    fn new(rt: RtIn, (base, offset): (Base, i32)) -> Self::Output {
        ScaledOffset16::try_from(offset)
            .map(|offset| Self(LdStArgs { rt: rt.into_reg(), addr: (base.into_reg(), offset) }))
    }
}

// ── Pre-increment ─────────────────────────────────────────────────────────────

impl<RtIn, Base> MakeLdrh<RtIn, (Inc<LdStIncOffset>, Base)>
    for Ldrh<LdStArgs<RegOrZero32, (Inc<LdStIncOffset>, RegOrSp64)>>
where
    RtIn: IntoReg<RegOrZero32>,
    Base: IntoReg<RegOrSp64>,
{
    type Output = Self;
    #[inline]
    fn new(rt: RtIn, (inc, base): (Inc<LdStIncOffset>, Base)) -> Self {
        Self(LdStArgs { rt: rt.into_reg(), addr: (inc, base.into_reg()) })
    }
}

// ── Post-increment ────────────────────────────────────────────────────────────

impl<RtIn, Base> MakeLdrh<RtIn, (Base, Inc<LdStIncOffset>)>
    for Ldrh<LdStArgs<RegOrZero32, (RegOrSp64, Inc<LdStIncOffset>)>>
where
    RtIn: IntoReg<RegOrZero32>,
    Base: IntoReg<RegOrSp64>,
{
    type Output = Self;
    #[inline]
    fn new(rt: RtIn, (base, inc): (Base, Inc<LdStIncOffset>)) -> Self {
        Self(LdStArgs { rt: rt.into_reg(), addr: (base.into_reg(), inc) })
    }
}

// ── Fallible wrappers ─────────────────────────────────────────────────────────

impl<Rt, RtIn, BaseIn, Ext, Err> MakeLdrh<RtIn, (BaseIn, Result<Ext, Err>)>
    for Ldrh<LdStArgs<Rt, (RegOrSp64, Ext)>>
where
    Ldrh<LdStArgs<Rt, (RegOrSp64, Ext)>>: MakeLdrh<RtIn, (BaseIn, Ext)>,
    BaseIn: IntoReg<RegOrSp64>,
{
    type Output = Result<<Self as MakeLdrh<RtIn, (BaseIn, Ext)>>::Output, Err>;
    #[inline]
    fn new(rt: RtIn, (base, ext_res): (BaseIn, Result<Ext, Err>)) -> Self::Output {
        ext_res.map(|ext| <Self as MakeLdrh<RtIn, (BaseIn, Ext)>>::new(rt, (base, ext)))
    }
}

impl<Rt, RtIn, BaseIn, Ext, Err> MakeLdrh<RtIn, (Result<Ext, Err>, BaseIn)>
    for Ldrh<LdStArgs<Rt, (Ext, RegOrSp64)>>
where
    Ldrh<LdStArgs<Rt, (Ext, RegOrSp64)>>: MakeLdrh<RtIn, (Ext, BaseIn)>,
    BaseIn: IntoReg<RegOrSp64>,
{
    type Output = Result<<Self as MakeLdrh<RtIn, (Ext, BaseIn)>>::Output, Err>;
    #[inline]
    fn new(rt: RtIn, (ext_res, base): (Result<Ext, Err>, BaseIn)) -> Self::Output {
        ext_res.map(|ext| <Self as MakeLdrh<RtIn, (Ext, BaseIn)>>::new(rt, (ext, base)))
    }
}

impl<Rt, RtIn, Addr, Err> MakeLdrh<RtIn, Result<Addr, Err>>
    for Ldrh<LdStArgs<Rt, Addr>>
where
    Ldrh<LdStArgs<Rt, Addr>>: MakeLdrh<RtIn, Addr>,
{
    type Output = Result<<Self as MakeLdrh<RtIn, Addr>>::Output, Err>;
    #[inline]
    fn new(rt: RtIn, addr_res: Result<Addr, Err>) -> Self::Output {
        addr_res.map(|addr| <Self as MakeLdrh<RtIn, Addr>>::new(rt, addr))
    }
}

/// ldrh construction function.  See examples in the module documentation.
pub fn ldrh<RtIn, Rt, AddrIn, Addr>(
    dst: RtIn,
    addr: AddrIn,
) -> <Ldrh<LdStArgs<Rt, Addr>> as MakeLdrh<RtIn, AddrIn>>::Output
where
    Ldrh<LdStArgs<Rt, Addr>>: MakeLdrh<RtIn, AddrIn>,
{
    <Ldrh<LdStArgs<Rt, Addr>> as MakeLdrh<RtIn, AddrIn>>::new(dst, addr)
}

// === LDRH: extended 64-bit register offset ===

impl RawInstruction
    for Ldrh<LdStArgs<RegOrZero32, (RegOrSp64, Extended<HalfShift, RegOrZero64>)>>
{
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let (base, offset) = self.0.addr;
        LDRH_32_ldst_regoff(
            offset.offset.index(),
            (offset.extend as u8).into(),
            offset.shifted.into(),
            base.index(),
            self.0.rt.index(),
        )
    }
}

// === LDRH: extended 32-bit register offset ===

impl RawInstruction
    for Ldrh<LdStArgs<RegOrZero32, (RegOrSp64, Extended<HalfShift, RegOrZero32>)>>
{
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let (base, offset) = self.0.addr;
        LDRH_32_ldst_regoff(
            offset.offset.index(),
            (offset.extend as u8).into(),
            offset.shifted.into(),
            base.index(),
            self.0.rt.index(),
        )
    }
}

// === LDRH: bare 64-bit register offset ===

impl RawInstruction for Ldrh<LdStArgs<RegOrZero32, (RegOrSp64, RegOrZero64)>> {
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let (base, offset) = self.0.addr;
        LDRH_32_ldst_regoff(
            offset.index(),
            (LdStExtendOption64::default() as u8).into(),
            0b0.into(),
            base.index(),
            self.0.rt.index(),
        )
    }
}

// === LDRH: scaled immediate offset ===

impl RawInstruction for Ldrh<LdStArgs<RegOrZero32, (RegOrSp64, ScaledOffset16)>> {
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let (base, offset) = self.0.addr;
        LDRH_32_ldst_pos(offset.into(), base.index(), self.0.rt.index())
    }
}

// === LDRH: pre-increment ===

impl RawInstruction for Ldrh<LdStArgs<RegOrZero32, (Inc<LdStIncOffset>, RegOrSp64)>> {
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let (inc, base) = self.0.addr;
        LDRH_32_ldst_immpre(inc.offset.into(), base.index(), self.0.rt.index())
    }
}

// === LDRH: post-increment ===

impl RawInstruction for Ldrh<LdStArgs<RegOrZero32, (RegOrSp64, Inc<LdStIncOffset>)>> {
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let (base, inc) = self.0.addr;
        LDRH_32_ldst_immpost(inc.offset.into(), base.index(), self.0.rt.index())
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
        instructions::ldst::{inc, postinc, preinc},
    };
    use LdStExtendOption32::*;
    use LdStExtendOption64::*;
    use RegOrSp64::SP;
    use RegOrZero32::WZR;
    use RegOrZero64::XZR;

    const LDRH_REG_EXT_DB: &str = "78634902	ldrh w2, [x8, w3, uxtw]
78634902	ldrh w2, [x8, w3, uxtw #0]
78635902	ldrh w2, [x8, w3, uxtw #1]
78636902	ldrh w2, [x8, x3]
78636902	ldrh w2, [x8, x3, lsl #0]
78637902	ldrh w2, [x8, x3, lsl #1]
78636be2	ldrh w2, [sp, x3]
7863c902	ldrh w2, [x8, w3, sxtw]
7863c902	ldrh w2, [x8, w3, sxtw #0]
7863d902	ldrh w2, [x8, w3, sxtw #1]
7863e902	ldrh w2, [x8, x3, sxtx]
7863e902	ldrh w2, [x8, x3, sxtx #0]
7863f902	ldrh w2, [x8, x3, sxtx #1]
787f6902	ldrh w2, [x8, xzr]
787f6be2	ldrh w2, [sp, xzr]
";

    // 'ldrh (w2|x2), [(x8|sp), #0x190]'
    const LDRH_SCALED_IMM_DB: &str = "
79432102	ldrh w2, [x8, #0x190]
7943211f	ldrh wzr, [x8, #0x190]
794323e2	ldrh w2, [sp, #0x190]
79400102	ldrh w2, [x8]
";

    const LDRH_PRE_POST_INC_DB: &str = "
7842a441	ldrh w1, [x2], #0x2a
7842a45f	ldrh wzr, [x2], #0x2a
7842a7e1	ldrh w1, [sp], #0x2a
7842ac41	ldrh w1, [x2, #0x2a]!
7842ac5f	ldrh wzr, [x2, #0x2a]!
7842afe1	ldrh w1, [sp, #0x2a]!
785d6441	ldrh w1, [x2], #-0x2a
785d67e1	ldrh w1, [sp], #-0x2a
785d6c41	ldrh w1, [x2, #-0x2a]!
785d6fe1	ldrh w1, [sp, #-0x2a]!
";

    test_cases! {
        LDRH_REG_EXT_DB, untested_ldrh_reg_ext_db;
        test_ldrh_r32_r64_r32_sxtw, ldrh(W2, (X8, ext((W3, SXTW)))), "ldrh w2, [x8, w3, sxtw]";
        test_ldrh_r32_r64_r32_uxtw, ldrh(W2, (X8, ext((W3, UXTW)))), "ldrh w2, [x8, w3, uxtw]";
        test_ldrh_r32_r64_r64_sxtx, ldrh(W2, (X8, ext((X3, SXTX)))), "ldrh w2, [x8, x3, sxtx]";
        test_ldrh_r32_r64_r32_sxtw_0, ldrh(W2, (X8, ext((W3, SXTW, 0)))).unwrap(), "ldrh w2, [x8, w3, sxtw #0]";
        test_ldrh_r32_r64_r32_sxtw_1, ldrh(W2, (X8, ext((W3, SXTW, 1)))).unwrap(), "ldrh w2, [x8, w3, sxtw #1]";
        test_ldrh_r32_r64_r32_uxtw_0, ldrh(W2, (X8, ext((W3, UXTW, 0)))).unwrap(), "ldrh w2, [x8, w3, uxtw #0]";
        test_ldrh_r32_r64_r32_uxtw_1, ldrh(W2, (X8, ext((W3, UXTW, 1)))).unwrap(), "ldrh w2, [x8, w3, uxtw #1]";
        test_ldrh_r32_r64_r64_lsl_0, ldrh(W2, (X8, ext((X3, LSL, 0)))).unwrap(), "ldrh w2, [x8, x3, lsl #0]";
        test_ldrh_r32_r64_r64_lsl_1, ldrh(W2, (X8, ext((X3, LSL, 1)))).unwrap(), "ldrh w2, [x8, x3, lsl #1]";
        test_ldrh_r32_r64_r64_sxtx_0, ldrh(W2, (X8, ext((X3, SXTX, 0)))).unwrap(), "ldrh w2, [x8, x3, sxtx #0]";
        test_ldrh_r32_r64_r64_sxtx_1, ldrh(W2, (X8, ext((X3, SXTX, 1)))).unwrap(), "ldrh w2, [x8, x3, sxtx #1]";
        test_ldrh_r32_r64_r64, ldrh(W2, (X8, X3)), "ldrh w2, [x8, x3]";
        test_ldrh_r32_rsp_r64, ldrh(W2, (SP, X3)), "ldrh w2, [sp, x3]";
        test_ldrh_r32_r64_xzr, ldrh(W2, (X8, XZR)), "ldrh w2, [x8, xzr]";
        test_ldrh_r32_rsp_xzr, ldrh(W2, (SP, XZR)), "ldrh w2, [sp, xzr]";
    }

    test_cases! {
        LDRH_SCALED_IMM_DB, untested_ldrh_scaled_imm;
        test_ldrh_r32_r64_scaled_imm, ldrh(W2, (X8, UBitValue::<12, 1>::new(0x190).unwrap())), "ldrh w2, [x8, #0x190]";
        test_ldrh_r32_sp_scaled_imm, ldrh(W2, (SP, UBitValue::<12, 1>::new(0x190).unwrap())), "ldrh w2, [sp, #0x190]";
        test_ldrh_r32_r64_scaled_imm2, ldrh(W2, (X8, 0x190u32)).unwrap(), "ldrh w2, [x8, #0x190]";
        test_ldrh_wzr_r64_scaled_imm2, ldrh(WZR, (X8, 0x190u32)).unwrap(), "ldrh wzr, [x8, #0x190]";
        test_ldrh_r32_r64_scaled_imm3, ldrh(W2, (X8, 0x190i32)).unwrap(), "ldrh w2, [x8, #0x190]";
        test_ldrh_wzr_r64_scaled_imm3, ldrh(WZR, (X8, 0x190i32)).unwrap(), "ldrh wzr, [x8, #0x190]";
        test_ldrh_r32_r64_simple, ldrh(W2, (X8,)), "ldrh w2, [x8]";
    }

    test_cases! {
        LDRH_PRE_POST_INC_DB, untested_ldrh_pre_post_inc;
        test_ldrh_r32_r64_preinc, ldrh(W1, preinc(X2, 0x2a)).unwrap(), "ldrh w1, [x2, #0x2a]!";
        test_ldrh_r32_r64_postinc, ldrh(W1, postinc(X2, 0x2a)).unwrap(), "ldrh w1, [x2], #0x2a";
        test_ldrh_r32_sp_preinc, ldrh(W1, preinc(SP, 0x2a)).unwrap(), "ldrh w1, [sp, #0x2a]!";
        test_ldrh_r32_sp_postinc, ldrh(W1, postinc(SP, 0x2a)).unwrap(), "ldrh w1, [sp], #0x2a";
        test_ldrh_r32_r64_preinc_neg, ldrh(W1, preinc(X2, -0x2a)).unwrap(), "ldrh w1, [x2, #-0x2a]!";
        test_ldrh_r32_r64_postinc_neg, ldrh(W1, postinc(X2, -0x2a)).unwrap(), "ldrh w1, [x2], #-0x2a";
        test_ldrh_r32_sp_preinc_neg, ldrh(W1, preinc(SP, -0x2a)).unwrap(), "ldrh w1, [sp, #-0x2a]!";
        test_ldrh_r32_sp_postinc_neg, ldrh(W1, postinc(SP, -0x2a)).unwrap(), "ldrh w1, [sp], #-0x2a";
        test_ldrh_r32_sp_preinc2, ldrh(W1, preinc(SP, LdStIncOffset::new(0x2a).unwrap())), "ldrh w1, [sp, #0x2a]!";
        test_ldrh_r32_r64_pre_inc, ldrh(W1, (inc(0x2a), X2)).unwrap(), "ldrh w1, [x2, #0x2a]!";
        test_ldrh_r32_r64_post_inc, ldrh(W1, (X2, inc(0x2a))).unwrap(), "ldrh w1, [x2], #0x2a";
        test_ldrh_r32_sp_pre_inc, ldrh(W1, (inc(0x2a), SP)).unwrap(), "ldrh w1, [sp, #0x2a]!";
        test_ldrh_r32_sp_post_inc, ldrh(W1, (SP, inc(0x2a))).unwrap(), "ldrh w1, [sp], #0x2a";
        test_ldrh_r32_r64_pre_inc_neg, ldrh(W1, (inc(-0x2a), X2)).unwrap(), "ldrh w1, [x2, #-0x2a]!";
        test_ldrh_r32_r64_post_inc_neg, ldrh(W1, (X2, inc(-0x2a))).unwrap(), "ldrh w1, [x2], #-0x2a";
        test_ldrh_r32_sp_pre_inc_neg, ldrh(W1, (inc(-0x2a), SP)).unwrap(), "ldrh w1, [sp, #-0x2a]!";
        test_ldrh_r32_sp_post_inc_neg, ldrh(W1, (SP, inc(-0x2a))).unwrap(), "ldrh w1, [sp], #-0x2a";
        test_ldrh_r32_sp_pre_inc2, ldrh(W1, (inc(LdStIncOffset::new(0x2a).unwrap()), SP)), "ldrh w1, [sp, #0x2a]!";
        test_ldrh_wzr_r64_pre_inc, ldrh(WZR, (inc(0x2a), X2)).unwrap(), "ldrh wzr, [x2, #0x2a]!";
        test_ldrh_wzr_r64_post_inc, ldrh(WZR, (X2, inc(0x2a))).unwrap(), "ldrh wzr, [x2], #0x2a";
    }
}
