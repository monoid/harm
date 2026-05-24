/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

//! `STRB` and related commands.
//!
//! The `strb` function returns an instance of `Instruction` for storing a byte from a 32-bit
//! register to memory. While `STRB` has different variants with various addressing modes, the
//! `strb` function takes two arguments: a source register and an "address" that encapsulates the
//! rest: the base, offsets, extensions, etc. Tuples are often used for the second argument, see
//! the pattern in the examples below.
//!
//! The function is overloaded for various argument types. For some of them an `Instruction` trait
//! instance is returned; for others a `Result` if the arguments need validation. Such argument
//! combinations have `.unwrap()` in examples.
//!
//! # `STRB`: Register base with register offset
//!
//! # Examples:
//! ```
//! # use harm::instructions::ldst::{strb, ext, LdStExtendOption32, LdStShift};
//! use harm::register::Reg32::*;
//! use harm::register::Reg64::*;
//! use LdStExtendOption32::*;
//!
//! strb(W1, X2);        // STRB W1, [X2]
//! strb(W1, (X2,));     // STRB W1, [X2]
//! strb(W1, (X2, X3));  // STRB W1, [X2, X3]
//! strb(W1, (X2, ext((W3, UXTW)))); // strb w1, [x2, w3, uxtw]
//! strb(W1, (X2, ext((W3, UXTW, LdStShift::Shifted)))); // strb w1, [x2, w3, uxtw #0]
//! ```
//!
//! Please note that `uxtw` and `sxtw` can be used only with a 32-bit index register, and shift
//! can be only absent or 0.  The `lsl` and `sxtx` can be used only with 64-bit index registers;
//! shift can be only either absent or 0.
//!
//! # `STRB`: Register base with immediate offset
//!
//! STRB with a register base and an immediate unsigned byte offset. The offset has 12 significant
//! bits available (no alignment requirement — byte width).
//!
//! You may also pass a `u32` offset value; an error is returned if the value doesn't fit.
//!
//! Examples:
//! ```ignore
//! let byte_offset: ScaledOffset8 = ...;
//!
//! strb(W1, (X2, offset as u32)).unwrap(),
//! strb(W1, (X2, byte_offset)),
//! ```
//!
//! Pre-increment and post-increment variants have the following syntax:
//! ```
//! # use harm::instructions::ldst::{strb, inc, preinc, postinc, LdStIncOffset};
//! use harm::register::Reg32::*;
//! use harm::register::Reg64::*;
//! let offset = LdStIncOffset::new(4).unwrap();
//! strb(W1, (inc(offset), X2));       // preincrement, STRB W1, [X2, #4]!
//! strb(W1, (X2, inc(offset)));       // postincrement, STRB W1, [X2], #4
//! // Equivalent to the lines above:
//! strb(W1, preinc(X2, offset));      // preincrement, STRB W1, [X2, #4]!
//! strb(W1, postinc(X2, offset));     // postincrement, STRB W1, [X2], #4
//! // Fallible variants:
//! strb(W1, (inc(4), X2)).unwrap();   // preincrement, STRB W1, [X2, #4]!
//! strb(W1, postinc(X2, 4)).unwrap(); // postincrement, STRB W1, [X2], #4
//! ```

use aarchmrs_instructions::A64::ldst::{
    ldst_immpost::STRB_32_ldst_immpost::STRB_32_ldst_immpost,
    ldst_immpre::STRB_32_ldst_immpre::STRB_32_ldst_immpre,
    ldst_pos::STRB_32_ldst_pos::STRB_32_ldst_pos,
    ldst_regoff::STRB_32B_ldst_regoff::STRB_32B_ldst_regoff,
};

use super::args::LdStArgs;
use super::shift_extend::*;
use super::{ByteShift, Inc, LdStIncOffset, ScaledOffset8};
use crate::{
    bits::BitError,
    instructions::RawInstruction,
    register::{IntoReg, RegOrSp64, RegOrZero32, RegOrZero64, Register},
    sealed::Sealed,
};

/// A `strb` instruction with a source register and an address.
#[derive(Debug, Copy, Clone)]
pub struct Strb<Args>(pub Args);

impl<Args: Sealed> Sealed for Strb<Args> {}

/// Defines possible ways to construct a `strb` instruction.
pub trait MakeStrb<RtIn, AddrIn>: Sealed {
    type Output;
    fn new(rt: RtIn, addr: AddrIn) -> Self::Output;
}

// ── Register offset: extended 64-bit register ────────────────────────────────

impl<RtIn, Base, Ext> MakeStrb<RtIn, (Base, Ext)>
    for Strb<LdStArgs<RegOrZero32, (RegOrSp64, Extended<ByteShift, RegOrZero64>)>>
where
    RtIn: IntoReg<RegOrZero32>,
    Base: IntoReg<RegOrSp64>,
    Ext: Into<Extended<ByteShift, RegOrZero64>>,
{
    type Output = Self;
    #[inline]
    fn new(rt: RtIn, (base, ext): (Base, Ext)) -> Self {
        Self(LdStArgs { rt: rt.into_reg(), addr: (base.into_reg(), ext.into()) })
    }
}

// ── Register offset: extended 32-bit register ────────────────────────────────

impl<RtIn, Base, Ext> MakeStrb<RtIn, (Base, Ext)>
    for Strb<LdStArgs<RegOrZero32, (RegOrSp64, Extended<ByteShift, RegOrZero32>)>>
where
    RtIn: IntoReg<RegOrZero32>,
    Base: IntoReg<RegOrSp64>,
    Ext: Into<Extended<ByteShift, RegOrZero32>>,
{
    type Output = Self;
    #[inline]
    fn new(rt: RtIn, (base, ext): (Base, Ext)) -> Self {
        Self(LdStArgs { rt: rt.into_reg(), addr: (base.into_reg(), ext.into()) })
    }
}

// ── Register offset: bare 64-bit register ────────────────────────────────────

impl<RtIn, Base, OffsetReg> MakeStrb<RtIn, (Base, OffsetReg)>
    for Strb<LdStArgs<RegOrZero32, (RegOrSp64, RegOrZero64)>>
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

impl<RtIn, Base> MakeStrb<RtIn, Base>
    for Strb<LdStArgs<RegOrZero32, (RegOrSp64, ScaledOffset8)>>
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

impl<RtIn, Base> MakeStrb<RtIn, (Base,)>
    for Strb<LdStArgs<RegOrZero32, (RegOrSp64, ScaledOffset8)>>
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

impl<RtIn, Base> MakeStrb<RtIn, (Base, ScaledOffset8)>
    for Strb<LdStArgs<RegOrZero32, (RegOrSp64, ScaledOffset8)>>
where
    RtIn: IntoReg<RegOrZero32>,
    Base: IntoReg<RegOrSp64>,
{
    type Output = Self;
    #[inline]
    fn new(rt: RtIn, (base, offset): (Base, ScaledOffset8)) -> Self {
        Self(LdStArgs { rt: rt.into_reg(), addr: (base.into_reg(), offset) })
    }
}

// ── Scaled immediate offset: u32 (fallible) ──────────────────────────────────

impl<RtIn, Base> MakeStrb<RtIn, (Base, u32)>
    for Strb<LdStArgs<RegOrZero32, (RegOrSp64, ScaledOffset8)>>
where
    RtIn: IntoReg<RegOrZero32>,
    Base: IntoReg<RegOrSp64>,
{
    type Output = Result<Self, BitError>;
    #[inline]
    fn new(rt: RtIn, (base, offset): (Base, u32)) -> Self::Output {
        ScaledOffset8::try_from(offset)
            .map(|offset| Self(LdStArgs { rt: rt.into_reg(), addr: (base.into_reg(), offset) }))
    }
}

// ── Scaled immediate offset: i32 (fallible) ──────────────────────────────────

impl<RtIn, Base> MakeStrb<RtIn, (Base, i32)>
    for Strb<LdStArgs<RegOrZero32, (RegOrSp64, ScaledOffset8)>>
where
    RtIn: IntoReg<RegOrZero32>,
    Base: IntoReg<RegOrSp64>,
{
    type Output = Result<Self, BitError>;
    #[inline]
    fn new(rt: RtIn, (base, offset): (Base, i32)) -> Self::Output {
        ScaledOffset8::try_from(offset)
            .map(|offset| Self(LdStArgs { rt: rt.into_reg(), addr: (base.into_reg(), offset) }))
    }
}

// ── Pre-increment ─────────────────────────────────────────────────────────────

impl<RtIn, Base> MakeStrb<RtIn, (Inc<LdStIncOffset>, Base)>
    for Strb<LdStArgs<RegOrZero32, (Inc<LdStIncOffset>, RegOrSp64)>>
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

impl<RtIn, Base> MakeStrb<RtIn, (Base, Inc<LdStIncOffset>)>
    for Strb<LdStArgs<RegOrZero32, (RegOrSp64, Inc<LdStIncOffset>)>>
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

impl<Rt, RtIn, BaseIn, Ext, Err> MakeStrb<RtIn, (BaseIn, Result<Ext, Err>)>
    for Strb<LdStArgs<Rt, (RegOrSp64, Ext)>>
where
    Strb<LdStArgs<Rt, (RegOrSp64, Ext)>>: MakeStrb<RtIn, (BaseIn, Ext)>,
    BaseIn: IntoReg<RegOrSp64>,
{
    type Output = Result<<Self as MakeStrb<RtIn, (BaseIn, Ext)>>::Output, Err>;
    #[inline]
    fn new(rt: RtIn, (base, ext_res): (BaseIn, Result<Ext, Err>)) -> Self::Output {
        ext_res.map(|ext| <Self as MakeStrb<RtIn, (BaseIn, Ext)>>::new(rt, (base, ext)))
    }
}

impl<Rt, RtIn, BaseIn, Ext, Err> MakeStrb<RtIn, (Result<Ext, Err>, BaseIn)>
    for Strb<LdStArgs<Rt, (Ext, RegOrSp64)>>
where
    Strb<LdStArgs<Rt, (Ext, RegOrSp64)>>: MakeStrb<RtIn, (Ext, BaseIn)>,
    BaseIn: IntoReg<RegOrSp64>,
{
    type Output = Result<<Self as MakeStrb<RtIn, (Ext, BaseIn)>>::Output, Err>;
    #[inline]
    fn new(rt: RtIn, (ext_res, base): (Result<Ext, Err>, BaseIn)) -> Self::Output {
        ext_res.map(|ext| <Self as MakeStrb<RtIn, (Ext, BaseIn)>>::new(rt, (ext, base)))
    }
}

impl<Rt, RtIn, Addr, Err> MakeStrb<RtIn, Result<Addr, Err>>
    for Strb<LdStArgs<Rt, Addr>>
where
    Strb<LdStArgs<Rt, Addr>>: MakeStrb<RtIn, Addr>,
{
    type Output = Result<<Self as MakeStrb<RtIn, Addr>>::Output, Err>;
    #[inline]
    fn new(rt: RtIn, addr_res: Result<Addr, Err>) -> Self::Output {
        addr_res.map(|addr| <Self as MakeStrb<RtIn, Addr>>::new(rt, addr))
    }
}

/// strb construction function.  See examples in the module documentation.
pub fn strb<RtIn, Rt, AddrIn, Addr>(
    dst: RtIn,
    addr: AddrIn,
) -> <Strb<LdStArgs<Rt, Addr>> as MakeStrb<RtIn, AddrIn>>::Output
where
    Strb<LdStArgs<Rt, Addr>>: MakeStrb<RtIn, AddrIn>,
{
    <Strb<LdStArgs<Rt, Addr>> as MakeStrb<RtIn, AddrIn>>::new(dst, addr)
}

// === STRB: extended 64-bit register offset ===

impl RawInstruction
    for Strb<LdStArgs<RegOrZero32, (RegOrSp64, Extended<ByteShift, RegOrZero64>)>>
{
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let (base, offset) = self.0.addr;
        STRB_32B_ldst_regoff(
            offset.offset.index(),
            (offset.extend as u8).into(),
            offset.shifted.into(),
            base.index(),
            self.0.rt.index(),
        )
    }
}

// === STRB: extended 32-bit register offset ===

impl RawInstruction
    for Strb<LdStArgs<RegOrZero32, (RegOrSp64, Extended<ByteShift, RegOrZero32>)>>
{
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let (base, offset) = self.0.addr;
        STRB_32B_ldst_regoff(
            offset.offset.index(),
            (offset.extend as u8).into(),
            offset.shifted.into(),
            base.index(),
            self.0.rt.index(),
        )
    }
}

// === STRB: bare 64-bit register offset ===

impl RawInstruction for Strb<LdStArgs<RegOrZero32, (RegOrSp64, RegOrZero64)>> {
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let (base, offset) = self.0.addr;
        STRB_32B_ldst_regoff(
            offset.index(),
            (LdStExtendOption64::default() as u8).into(),
            0b0.into(),
            base.index(),
            self.0.rt.index(),
        )
    }
}

// === STRB: scaled immediate offset ===

impl RawInstruction for Strb<LdStArgs<RegOrZero32, (RegOrSp64, ScaledOffset8)>> {
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let (base, offset) = self.0.addr;
        STRB_32_ldst_pos(offset.into(), base.index(), self.0.rt.index())
    }
}

// === STRB: pre-increment ===

impl RawInstruction for Strb<LdStArgs<RegOrZero32, (Inc<LdStIncOffset>, RegOrSp64)>> {
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let (inc, base) = self.0.addr;
        STRB_32_ldst_immpre(inc.offset.into(), base.index(), self.0.rt.index())
    }
}

// === STRB: post-increment ===

impl RawInstruction for Strb<LdStArgs<RegOrZero32, (RegOrSp64, Inc<LdStIncOffset>)>> {
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let (base, inc) = self.0.addr;
        STRB_32_ldst_immpost(inc.offset.into(), base.index(), self.0.rt.index())
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

    const STRB_REG_EXT_DB: &str = "
38234902	strb w2, [x8, w3, uxtw]
38235902	strb w2, [x8, w3, uxtw #0]
38236902	strb w2, [x8, x3]
38237902	strb w2, [x8, x3, lsl #0]
38236be2	strb w2, [sp, x3]
3823c902	strb w2, [x8, w3, sxtw]
3823d902	strb w2, [x8, w3, sxtw #0]
3823e902	strb w2, [x8, x3, sxtx]
3823f902	strb w2, [x8, x3, sxtx #0]
383f6902	strb w2, [x8, xzr]
383f6be2	strb w2, [sp, xzr]
";

    // 'strb (w2|x2), [(x8|sp), #0x190]'
    const STRB_SCALED_IMM_DB: &str = "
39064102	strb w2, [x8, #0x190]
3906411f	strb wzr, [x8, #0x190]
390643e2	strb w2, [sp, #0x190]
39000102	strb w2, [x8]
";

    const STRB_PRE_POST_INC_DB: &str = "
3802a441	strb w1, [x2], #0x2a
3802a45f	strb wzr, [x2], #0x2a
3802a7e1	strb w1, [sp], #0x2a
3802ac41	strb w1, [x2, #0x2a]!
3802ac5f	strb wzr, [x2, #0x2a]!
3802afe1	strb w1, [sp, #0x2a]!
381d6441	strb w1, [x2], #-0x2a
381d67e1	strb w1, [sp], #-0x2a
381d6c41	strb w1, [x2, #-0x2a]!
381d6fe1	strb w1, [sp, #-0x2a]!
";

    test_cases! {
        STRB_REG_EXT_DB, untested_strb_reg_ext_db;
        test_strb_r32_r64_r32_sxtw, strb(W2, (X8, ext((W3, SXTW)))), "strb w2, [x8, w3, sxtw]";
        test_strb_r32_r64_r32_uxtw, strb(W2, (X8, ext((W3, UXTW)))), "strb w2, [x8, w3, uxtw]";
        test_strb_r32_r64_r64_sxtx, strb(W2, (X8, ext((X3, SXTX)))), "strb w2, [x8, x3, sxtx]";
        test_strb_r32_r64_r32_sxtw_0, strb(W2, (X8, ext((W3, SXTW, 0)))).unwrap(), "strb w2, [x8, w3, sxtw #0]";
        test_strb_r32_r64_r32_uxtw_0, strb(W2, (X8, ext((W3, UXTW, 0)))).unwrap(), "strb w2, [x8, w3, uxtw #0]";
        test_strb_r32_r64_r64_lsl_0, strb(W2, (X8, ext((X3, LSL, 0)))).unwrap(), "strb w2, [x8, x3, lsl #0]";
        test_strb_r32_r64_r64_sxtx_0, strb(W2, (X8, ext((X3, SXTX, 0)))).unwrap(), "strb w2, [x8, x3, sxtx #0]";
        test_strb_r32_r64_r64, strb(W2, (X8, X3)), "strb w2, [x8, x3]";
        test_strb_r32_rsp_r64, strb(W2, (SP, X3)), "strb w2, [sp, x3]";
        test_strb_r32_r64_xzr, strb(W2, (X8, XZR)), "strb w2, [x8, xzr]";
        test_strb_r32_rsp_xzr, strb(W2, (SP, XZR)), "strb w2, [sp, xzr]";
    }

    test_cases! {
        STRB_SCALED_IMM_DB, untested_strb_scaled_imm;
        test_strb_r32_r64_scaled_imm, strb(W2, (X8, UBitValue::<12>::new(0x190).unwrap())), "strb w2, [x8, #0x190]";
        test_strb_r32_sp_scaled_imm, strb(W2, (SP, UBitValue::<12>::new(0x190).unwrap())), "strb w2, [sp, #0x190]";
        test_strb_r32_r64_scaled_imm2, strb(W2, (X8, 0x190u32)).unwrap(), "strb w2, [x8, #0x190]";
        test_strb_wzr_r64_scaled_imm2, strb(WZR, (X8, 0x190u32)).unwrap(), "strb wzr, [x8, #0x190]";
        test_strb_r32_r64_scaled_imm3, strb(W2, (X8, 0x190i32)).unwrap(), "strb w2, [x8, #0x190]";
        test_strb_wzr_r64_scaled_imm3, strb(WZR, (X8, 0x190i32)).unwrap(), "strb wzr, [x8, #0x190]";
        test_strb_r32_r64_simple, strb(W2, (X8,)), "strb w2, [x8]";
    }

    test_cases! {
        STRB_PRE_POST_INC_DB, untested_strb_pre_post_inc;
        test_strb_r32_r64_preinc, strb(W1, preinc(X2, 0x2a)).unwrap(), "strb w1, [x2, #0x2a]!";
        test_strb_r32_r64_postinc, strb(W1, postinc(X2, 0x2a)).unwrap(), "strb w1, [x2], #0x2a";
        test_strb_r32_sp_preinc, strb(W1, preinc(SP, 0x2a)).unwrap(), "strb w1, [sp, #0x2a]!";
        test_strb_r32_sp_postinc, strb(W1, postinc(SP, 0x2a)).unwrap(), "strb w1, [sp], #0x2a";
        test_strb_r32_r64_preinc_neg, strb(W1, preinc(X2, -0x2a)).unwrap(), "strb w1, [x2, #-0x2a]!";
        test_strb_r32_r64_postinc_neg, strb(W1, postinc(X2, -0x2a)).unwrap(), "strb w1, [x2], #-0x2a";
        test_strb_r32_sp_preinc_neg, strb(W1, preinc(SP, -0x2a)).unwrap(), "strb w1, [sp, #-0x2a]!";
        test_strb_r32_sp_postinc_neg, strb(W1, postinc(SP, -0x2a)).unwrap(), "strb w1, [sp], #-0x2a";
        test_strb_r32_sp_preinc2, strb(W1, preinc(SP, LdStIncOffset::new(0x2a).unwrap())), "strb w1, [sp, #0x2a]!";
        test_strb_r32_r64_pre_inc, strb(W1, (inc(0x2a), X2)).unwrap(), "strb w1, [x2, #0x2a]!";
        test_strb_r32_r64_post_inc, strb(W1, (X2, inc(0x2a))).unwrap(), "strb w1, [x2], #0x2a";
        test_strb_r32_sp_pre_inc, strb(W1, (inc(0x2a), SP)).unwrap(), "strb w1, [sp, #0x2a]!";
        test_strb_r32_sp_post_inc, strb(W1, (SP, inc(0x2a))).unwrap(), "strb w1, [sp], #0x2a";
        test_strb_r32_r64_pre_inc_neg, strb(W1, (inc(-0x2a), X2)).unwrap(), "strb w1, [x2, #-0x2a]!";
        test_strb_r32_r64_post_inc_neg, strb(W1, (X2, inc(-0x2a))).unwrap(), "strb w1, [x2], #-0x2a";
        test_strb_r32_sp_pre_inc_neg, strb(W1, (inc(-0x2a), SP)).unwrap(), "strb w1, [sp, #-0x2a]!";
        test_strb_r32_sp_post_inc_neg, strb(W1, (SP, inc(-0x2a))).unwrap(), "strb w1, [sp], #-0x2a";
        test_strb_r32_sp_pre_inc2, strb(W1, (inc(LdStIncOffset::new(0x2a).unwrap()), SP)), "strb w1, [sp, #0x2a]!";
        test_strb_wzr_r64_pre_inc, strb(WZR, (inc(0x2a), X2)).unwrap(), "strb wzr, [x2, #0x2a]!";
        test_strb_wzr_r64_post_inc, strb(WZR, (X2, inc(0x2a))).unwrap(), "strb wzr, [x2], #0x2a";
    }
}
