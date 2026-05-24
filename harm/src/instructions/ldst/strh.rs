/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

//! `STRH` and related commands.
//!
//! The `strh` function returns an instance of `Instruction` for storing a halfword from a 32-bit
//! register to memory. While `STRH` has different variants with various addressing modes, the
//! `strh` function takes two arguments: a source register and an "address" that encapsulates the
//! rest: the base, offsets, extensions, etc. Tuples are often used for the second argument, see
//! the pattern in the examples below.
//!
//! The function is overloaded for various argument types. For some of them an `Instruction` trait
//! instance is returned; for others a `Result` if the arguments need validation. Such argument
//! combinations have `.unwrap()` in examples.
//!
//! # `STRH`: Register base with register offset
//!
//! # Examples:
//! ```
//! # use harm::instructions::ldst::{strh, ext, LdStExtendOption32, LdStShift};
//! use harm::register::Reg32::*;
//! use harm::register::Reg64::*;
//! use LdStExtendOption32::*;
//!
//! strh(W1, X2);        // STRH W1, [X2]
//! strh(W1, (X2,));     // STRH W1, [X2]
//! strh(W1, (X2, X3));  // STRH W1, [X2, X3]
//! strh(W1, (X2, ext((W3, UXTW)))); // strh w1, [x2, w3, uxtw]
//! strh(W1, (X2, ext((W3, UXTW, LdStShift::Unshifted)))); // strh w1, [x2, w3, uxtw #0]
//! strh(W1, (X2, ext((W3, UXTW, LdStShift::Shifted)))); // strh w1, [x2, w3, uxtw #1]
//! ```
//!
//! Please note that `uxtw` and `sxtw` can be used only with a 32-bit index register, and shift
//! can be absent, 0, or 1.  The `lsl` and `sxtx` can be used only with 64-bit index registers;
//! shift can be absent, 0, or 1.
//!
//! # `STRH`: Register base with immediate offset
//!
//! STRH with a register base and a 2-byte-aligned immediate offset. The offset has 12 significant
//! bits available.
//!
//! You may also pass a `u32` offset value; an error is returned if the value doesn't fit.
//!
//! Examples:
//! ```ignore
//! let halfword_offset: ScaledOffset16 = ...;
//!
//! strh(W1, (X2, offset as u32)).unwrap(),
//! strh(W1, (X2, halfword_offset)),
//! ```
//!
//! Pre-increment and post-increment variants have the following syntax:
//! ```
//! # use harm::instructions::ldst::{strh, inc, preinc, postinc, LdStIncOffset};
//! use harm::register::Reg32::*;
//! use harm::register::Reg64::*;
//! let offset = LdStIncOffset::new(4).unwrap();
//! strh(W1, (inc(offset), X2));       // preincrement, STRH W1, [X2, #4]!
//! strh(W1, (X2, inc(offset)));       // postincrement, STRH W1, [X2], #4
//! // Equivalent to the lines above:
//! strh(W1, preinc(X2, offset));      // preincrement, STRH W1, [X2, #4]!
//! strh(W1, postinc(X2, offset));     // postincrement, STRH W1, [X2], #4
//! // Fallible variants:
//! strh(W1, (inc(4), X2)).unwrap();   // preincrement, STRH W1, [X2, #4]!
//! strh(W1, postinc(X2, 4)).unwrap(); // postincrement, STRH W1, [X2], #4
//! ```

use aarchmrs_instructions::A64::ldst::{
    ldst_immpost::STRH_32_ldst_immpost::STRH_32_ldst_immpost,
    ldst_immpre::STRH_32_ldst_immpre::STRH_32_ldst_immpre,
    ldst_pos::STRH_32_ldst_pos::STRH_32_ldst_pos,
    ldst_regoff::STRH_32_ldst_regoff::STRH_32_ldst_regoff,
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

/// A `strh` instruction with a source register and an address.
#[derive(Debug, Copy, Clone)]
pub struct Strh<Args>(pub Args);

impl<Args: Sealed> Sealed for Strh<Args> {}

/// Defines possible ways to construct a `strh` instruction.
pub trait MakeStrh<RtIn, AddrIn>: Sealed {
    type Output;
    fn new(rt: RtIn, addr: AddrIn) -> Self::Output;
}

// ── Register offset: extended 64-bit register ────────────────────────────────

impl<RtIn, Base, Ext> MakeStrh<RtIn, (Base, Ext)>
    for Strh<LdStArgs<RegOrZero32, (RegOrSp64, Extended<HalfShift, RegOrZero64>)>>
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

impl<RtIn, Base, Ext> MakeStrh<RtIn, (Base, Ext)>
    for Strh<LdStArgs<RegOrZero32, (RegOrSp64, Extended<HalfShift, RegOrZero32>)>>
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

impl<RtIn, Base, OffsetReg> MakeStrh<RtIn, (Base, OffsetReg)>
    for Strh<LdStArgs<RegOrZero32, (RegOrSp64, RegOrZero64)>>
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

impl<RtIn, Base> MakeStrh<RtIn, Base>
    for Strh<LdStArgs<RegOrZero32, (RegOrSp64, ScaledOffset16)>>
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

impl<RtIn, Base> MakeStrh<RtIn, (Base,)>
    for Strh<LdStArgs<RegOrZero32, (RegOrSp64, ScaledOffset16)>>
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

impl<RtIn, Base> MakeStrh<RtIn, (Base, ScaledOffset16)>
    for Strh<LdStArgs<RegOrZero32, (RegOrSp64, ScaledOffset16)>>
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

impl<RtIn, Base> MakeStrh<RtIn, (Base, u32)>
    for Strh<LdStArgs<RegOrZero32, (RegOrSp64, ScaledOffset16)>>
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

impl<RtIn, Base> MakeStrh<RtIn, (Base, i32)>
    for Strh<LdStArgs<RegOrZero32, (RegOrSp64, ScaledOffset16)>>
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

impl<RtIn, Base> MakeStrh<RtIn, (Inc<LdStIncOffset>, Base)>
    for Strh<LdStArgs<RegOrZero32, (Inc<LdStIncOffset>, RegOrSp64)>>
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

impl<RtIn, Base> MakeStrh<RtIn, (Base, Inc<LdStIncOffset>)>
    for Strh<LdStArgs<RegOrZero32, (RegOrSp64, Inc<LdStIncOffset>)>>
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

impl<Rt, RtIn, BaseIn, Ext, Err> MakeStrh<RtIn, (BaseIn, Result<Ext, Err>)>
    for Strh<LdStArgs<Rt, (RegOrSp64, Ext)>>
where
    Strh<LdStArgs<Rt, (RegOrSp64, Ext)>>: MakeStrh<RtIn, (BaseIn, Ext)>,
    BaseIn: IntoReg<RegOrSp64>,
{
    type Output = Result<<Self as MakeStrh<RtIn, (BaseIn, Ext)>>::Output, Err>;
    #[inline]
    fn new(rt: RtIn, (base, ext_res): (BaseIn, Result<Ext, Err>)) -> Self::Output {
        ext_res.map(|ext| <Self as MakeStrh<RtIn, (BaseIn, Ext)>>::new(rt, (base, ext)))
    }
}

impl<Rt, RtIn, BaseIn, Ext, Err> MakeStrh<RtIn, (Result<Ext, Err>, BaseIn)>
    for Strh<LdStArgs<Rt, (Ext, RegOrSp64)>>
where
    Strh<LdStArgs<Rt, (Ext, RegOrSp64)>>: MakeStrh<RtIn, (Ext, BaseIn)>,
    BaseIn: IntoReg<RegOrSp64>,
{
    type Output = Result<<Self as MakeStrh<RtIn, (Ext, BaseIn)>>::Output, Err>;
    #[inline]
    fn new(rt: RtIn, (ext_res, base): (Result<Ext, Err>, BaseIn)) -> Self::Output {
        ext_res.map(|ext| <Self as MakeStrh<RtIn, (Ext, BaseIn)>>::new(rt, (ext, base)))
    }
}

impl<Rt, RtIn, Addr, Err> MakeStrh<RtIn, Result<Addr, Err>>
    for Strh<LdStArgs<Rt, Addr>>
where
    Strh<LdStArgs<Rt, Addr>>: MakeStrh<RtIn, Addr>,
{
    type Output = Result<<Self as MakeStrh<RtIn, Addr>>::Output, Err>;
    #[inline]
    fn new(rt: RtIn, addr_res: Result<Addr, Err>) -> Self::Output {
        addr_res.map(|addr| <Self as MakeStrh<RtIn, Addr>>::new(rt, addr))
    }
}

/// strh construction function.  See examples in the module documentation.
pub fn strh<RtIn, Rt, AddrIn, Addr>(
    dst: RtIn,
    addr: AddrIn,
) -> <Strh<LdStArgs<Rt, Addr>> as MakeStrh<RtIn, AddrIn>>::Output
where
    Strh<LdStArgs<Rt, Addr>>: MakeStrh<RtIn, AddrIn>,
{
    <Strh<LdStArgs<Rt, Addr>> as MakeStrh<RtIn, AddrIn>>::new(dst, addr)
}

// === STRH: extended 64-bit register offset ===

impl RawInstruction
    for Strh<LdStArgs<RegOrZero32, (RegOrSp64, Extended<HalfShift, RegOrZero64>)>>
{
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let (base, offset) = self.0.addr;
        STRH_32_ldst_regoff(
            offset.offset.index(),
            (offset.extend as u8).into(),
            offset.shifted.into(),
            base.index(),
            self.0.rt.index(),
        )
    }
}

// === STRH: extended 32-bit register offset ===

impl RawInstruction
    for Strh<LdStArgs<RegOrZero32, (RegOrSp64, Extended<HalfShift, RegOrZero32>)>>
{
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let (base, offset) = self.0.addr;
        STRH_32_ldst_regoff(
            offset.offset.index(),
            (offset.extend as u8).into(),
            offset.shifted.into(),
            base.index(),
            self.0.rt.index(),
        )
    }
}

// === STRH: bare 64-bit register offset ===

impl RawInstruction for Strh<LdStArgs<RegOrZero32, (RegOrSp64, RegOrZero64)>> {
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let (base, offset) = self.0.addr;
        STRH_32_ldst_regoff(
            offset.index(),
            (LdStExtendOption64::default() as u8).into(),
            0b0.into(),
            base.index(),
            self.0.rt.index(),
        )
    }
}

// === STRH: scaled immediate offset ===

impl RawInstruction for Strh<LdStArgs<RegOrZero32, (RegOrSp64, ScaledOffset16)>> {
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let (base, offset) = self.0.addr;
        STRH_32_ldst_pos(offset.into(), base.index(), self.0.rt.index())
    }
}

// === STRH: pre-increment ===

impl RawInstruction for Strh<LdStArgs<RegOrZero32, (Inc<LdStIncOffset>, RegOrSp64)>> {
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let (inc, base) = self.0.addr;
        STRH_32_ldst_immpre(inc.offset.into(), base.index(), self.0.rt.index())
    }
}

// === STRH: post-increment ===

impl RawInstruction for Strh<LdStArgs<RegOrZero32, (RegOrSp64, Inc<LdStIncOffset>)>> {
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let (base, inc) = self.0.addr;
        STRH_32_ldst_immpost(inc.offset.into(), base.index(), self.0.rt.index())
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

    const STRH_REG_EXT_DB: &str = "
78234902	strh w2, [x8, w3, uxtw]
78234902	strh w2, [x8, w3, uxtw #0]
78235902	strh w2, [x8, w3, uxtw #1]
78236902	strh w2, [x8, x3]
78236902	strh w2, [x8, x3, lsl #0]
78237902	strh w2, [x8, x3, lsl #1]
78236be2	strh w2, [sp, x3]
7823c902	strh w2, [x8, w3, sxtw]
7823c902	strh w2, [x8, w3, sxtw #0]
7823d902	strh w2, [x8, w3, sxtw #1]
7823e902	strh w2, [x8, x3, sxtx]
7823e902	strh w2, [x8, x3, sxtx #0]
7823f902	strh w2, [x8, x3, sxtx #1]
783f6902	strh w2, [x8, xzr]
783f6be2	strh w2, [sp, xzr]
";

    // 'strh (w2|x2), [(x8|sp), #0x190]'
    const STRH_SCALED_IMM_DB: &str = "
79032102	strh w2, [x8, #0x190]
7903211f	strh wzr, [x8, #0x190]
790323e2	strh w2, [sp, #0x190]
79000102	strh w2, [x8]
";

    const STRH_PRE_POST_INC_DB: &str = "
7802a441	strh w1, [x2], #0x2a
7802a45f	strh wzr, [x2], #0x2a
7802a7e1	strh w1, [sp], #0x2a
7802ac41	strh w1, [x2, #0x2a]!
7802ac5f	strh wzr, [x2, #0x2a]!
7802afe1	strh w1, [sp, #0x2a]!
781d6441	strh w1, [x2], #-0x2a
781d67e1	strh w1, [sp], #-0x2a
781d6c41	strh w1, [x2, #-0x2a]!
781d6fe1	strh w1, [sp, #-0x2a]!
";

    test_cases! {
        STRH_REG_EXT_DB, untested_strh_reg_ext_db;
        test_strh_r32_r64_r32_sxtw, strh(W2, (X8, ext((W3, SXTW)))), "strh w2, [x8, w3, sxtw]";
        test_strh_r32_r64_r32_uxtw, strh(W2, (X8, ext((W3, UXTW)))), "strh w2, [x8, w3, uxtw]";
        test_strh_r32_r64_r64_sxtx, strh(W2, (X8, ext((X3, SXTX)))), "strh w2, [x8, x3, sxtx]";
        test_strh_r32_r64_r32_sxtw_0, strh(W2, (X8, ext((W3, SXTW, 0)))).unwrap(), "strh w2, [x8, w3, sxtw #0]";
        test_strh_r32_r64_r32_sxtw_1, strh(W2, (X8, ext((W3, SXTW, 1)))).unwrap(), "strh w2, [x8, w3, sxtw #1]";
        test_strh_r32_r64_r32_uxtw_0, strh(W2, (X8, ext((W3, UXTW, 0)))).unwrap(), "strh w2, [x8, w3, uxtw #0]";
        test_strh_r32_r64_r32_uxtw_1, strh(W2, (X8, ext((W3, UXTW, 1)))).unwrap(), "strh w2, [x8, w3, uxtw #1]";
        test_strh_r32_r64_r64_lsl_0, strh(W2, (X8, ext((X3, LSL, 0)))).unwrap(), "strh w2, [x8, x3, lsl #0]";
        test_strh_r32_r64_r64_lsl_1, strh(W2, (X8, ext((X3, LSL, 1)))).unwrap(), "strh w2, [x8, x3, lsl #1]";
        test_strh_r32_r64_r64_sxtx_0, strh(W2, (X8, ext((X3, SXTX, 0)))).unwrap(), "strh w2, [x8, x3, sxtx #0]";
        test_strh_r32_r64_r64_sxtx_1, strh(W2, (X8, ext((X3, SXTX, 1)))).unwrap(), "strh w2, [x8, x3, sxtx #1]";
        test_strh_r32_r64_r64, strh(W2, (X8, X3)), "strh w2, [x8, x3]";
        test_strh_r32_rsp_r64, strh(W2, (SP, X3)), "strh w2, [sp, x3]";
        test_strh_r32_r64_xzr, strh(W2, (X8, XZR)), "strh w2, [x8, xzr]";
        test_strh_r32_rsp_xzr, strh(W2, (SP, XZR)), "strh w2, [sp, xzr]";
    }

    test_cases! {
        STRH_SCALED_IMM_DB, untested_strh_scaled_imm;
        test_strh_r32_r64_scaled_imm, strh(W2, (X8, UBitValue::<12, 1>::new(0x190).unwrap())), "strh w2, [x8, #0x190]";
        test_strh_r32_sp_scaled_imm, strh(W2, (SP, UBitValue::<12, 1>::new(0x190).unwrap())), "strh w2, [sp, #0x190]";
        test_strh_r32_r64_scaled_imm2, strh(W2, (X8, 0x190u32)).unwrap(), "strh w2, [x8, #0x190]";
        test_strh_wzr_r64_scaled_imm2, strh(WZR, (X8, 0x190u32)).unwrap(), "strh wzr, [x8, #0x190]";
        test_strh_r32_r64_scaled_imm3, strh(W2, (X8, 0x190i32)).unwrap(), "strh w2, [x8, #0x190]";
        test_strh_wzr_r64_scaled_imm3, strh(WZR, (X8, 0x190i32)).unwrap(), "strh wzr, [x8, #0x190]";
        test_strh_r32_r64_simple, strh(W2, (X8,)), "strh w2, [x8]";
    }

    test_cases! {
        STRH_PRE_POST_INC_DB, untested_strh_pre_post_inc;
        test_strh_r32_r64_preinc, strh(W1, preinc(X2, 0x2a)).unwrap(), "strh w1, [x2, #0x2a]!";
        test_strh_r32_r64_postinc, strh(W1, postinc(X2, 0x2a)).unwrap(), "strh w1, [x2], #0x2a";
        test_strh_r32_sp_preinc, strh(W1, preinc(SP, 0x2a)).unwrap(), "strh w1, [sp, #0x2a]!";
        test_strh_r32_sp_postinc, strh(W1, postinc(SP, 0x2a)).unwrap(), "strh w1, [sp], #0x2a";
        test_strh_r32_r64_preinc_neg, strh(W1, preinc(X2, -0x2a)).unwrap(), "strh w1, [x2, #-0x2a]!";
        test_strh_r32_r64_postinc_neg, strh(W1, postinc(X2, -0x2a)).unwrap(), "strh w1, [x2], #-0x2a";
        test_strh_r32_sp_preinc_neg, strh(W1, preinc(SP, -0x2a)).unwrap(), "strh w1, [sp, #-0x2a]!";
        test_strh_r32_sp_postinc_neg, strh(W1, postinc(SP, -0x2a)).unwrap(), "strh w1, [sp], #-0x2a";
        test_strh_r32_sp_preinc2, strh(W1, preinc(SP, LdStIncOffset::new(0x2a).unwrap())), "strh w1, [sp, #0x2a]!";
        test_strh_r32_r64_pre_inc, strh(W1, (inc(0x2a), X2)).unwrap(), "strh w1, [x2, #0x2a]!";
        test_strh_r32_r64_post_inc, strh(W1, (X2, inc(0x2a))).unwrap(), "strh w1, [x2], #0x2a";
        test_strh_r32_sp_pre_inc, strh(W1, (inc(0x2a), SP)).unwrap(), "strh w1, [sp, #0x2a]!";
        test_strh_r32_sp_post_inc, strh(W1, (SP, inc(0x2a))).unwrap(), "strh w1, [sp], #0x2a";
        test_strh_r32_r64_pre_inc_neg, strh(W1, (inc(-0x2a), X2)).unwrap(), "strh w1, [x2, #-0x2a]!";
        test_strh_r32_r64_post_inc_neg, strh(W1, (X2, inc(-0x2a))).unwrap(), "strh w1, [x2], #-0x2a";
        test_strh_r32_sp_pre_inc_neg, strh(W1, (inc(-0x2a), SP)).unwrap(), "strh w1, [sp, #-0x2a]!";
        test_strh_r32_sp_post_inc_neg, strh(W1, (SP, inc(-0x2a))).unwrap(), "strh w1, [sp], #-0x2a";
        test_strh_r32_sp_pre_inc2, strh(W1, (inc(LdStIncOffset::new(0x2a).unwrap()), SP)), "strh w1, [sp, #0x2a]!";
        test_strh_wzr_r64_pre_inc, strh(WZR, (inc(0x2a), X2)).unwrap(), "strh wzr, [x2, #0x2a]!";
        test_strh_wzr_r64_post_inc, strh(WZR, (X2, inc(0x2a))).unwrap(), "strh wzr, [x2], #0x2a";
    }
}
