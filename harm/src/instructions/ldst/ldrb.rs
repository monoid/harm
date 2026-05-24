/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

//! `LDRB` and related commands.
//!
//! The `ldrb` function returns an instance of `Instruction` for loading a byte into a 32-bit
//! register from memory. While `LDRB` has different variants with various addressing modes, the
//! `ldrb` function takes two arguments: a destination register and an "address" that encapsulates
//! the rest: the base, offsets, extensions, etc. Tuples are often used for the second argument,
//! see the pattern in the examples below.
//!
//! The function is overloaded for various argument types. For some of them an `Instruction` trait
//! instance is returned; for others a `Result` if the arguments need validation. Such argument
//! combinations have `.unwrap()` in examples.
//!
//! # `LDRB`: Register base with register offset
//!
//! # Examples:
//! ```
//! # use harm::instructions::ldst::{ldrb, ext, LdStExtendOption32, LdStShift};
//! use harm::register::Reg32::*;
//! use harm::register::Reg64::*;
//! use LdStExtendOption32::*;
//!
//! ldrb(W1, X2);        // LDRB W1, [X2]
//! ldrb(W1, (X2,));     // LDRB W1, [X2]
//! ldrb(W1, (X2, X3));  // LDRB W1, [X2, X3]
//! ldrb(W1, (X2, ext((W3, UXTW)))); // ldrb w1, [x2, w3, uxtw]
//! ldrb(W1, (X2, ext((W3, UXTW, LdStShift::Shifted)))); // ldrb w1, [x2, w3, uxtw #0]
//! ```
//!
//! Please note that `uxtw` and `sxtw` can be used only with a 32-bit index register, and shift
//! can be only absent or 0.  The `lsl` and `sxtx` can be used only with 64-bit index registers;
//! shift can be only either absent or 0.
//!
//! # `LDRB`: Register base with immediate offset
//!
//! LDRB with a register base and an immediate unsigned byte offset. The offset has 12 significant
//! bits available (no alignment requirement — byte width).
//!
//! You may also pass a `u32` offset value; an error is returned if the value doesn't fit.
//!
//! Examples:
//! ```ignore
//! let byte_offset: ScaledOffset8 = ...;
//!
//! ldrb(W1, (X2, offset as u32)).unwrap(),
//! ldrb(W1, (X2, byte_offset)),
//! ```
//!
//! Pre-increment and post-increment variants have the following syntax:
//! ```
//! # use harm::instructions::ldst::{ldrb, inc, preinc, postinc, LdStIncOffset};
//! use harm::register::Reg32::*;
//! use harm::register::Reg64::*;
//! let offset = LdStIncOffset::new(4).unwrap();
//! ldrb(W1, (inc(offset), X2));       // preincrement, LDRB W1, [X2, #4]!
//! ldrb(W1, (X2, inc(offset)));       // postincrement, LDRB W1, [X2], #4
//! // Equivalent to the lines above:
//! ldrb(W1, preinc(X2, offset));      // preincrement, LDRB W1, [X2, #4]!
//! ldrb(W1, postinc(X2, offset));     // postincrement, LDRB W1, [X2], #4
//! // Fallible variants:
//! ldrb(W1, (inc(4), X2)).unwrap();   // preincrement, LDRB W1, [X2, #4]!
//! ldrb(W1, postinc(X2, 4)).unwrap(); // postincrement, LDRB W1, [X2], #4
//! ```

use aarchmrs_instructions::A64::ldst::{
    ldst_immpost::LDRB_32_ldst_immpost::LDRB_32_ldst_immpost,
    ldst_immpre::LDRB_32_ldst_immpre::LDRB_32_ldst_immpre,
    ldst_pos::LDRB_32_ldst_pos::LDRB_32_ldst_pos,
    ldst_regoff::LDRB_32B_ldst_regoff::LDRB_32B_ldst_regoff,
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

/// A `ldrb` instruction with a destination and an address.
#[derive(Debug, Copy, Clone)]
pub struct Ldrb<Args>(pub Args);

impl<Args: Sealed> Sealed for Ldrb<Args> {}

/// Defines possible ways to construct a `ldrb` instruction.
pub trait MakeLdrb<RtIn, AddrIn>: Sealed {
    type Output;
    fn new(rt: RtIn, addr: AddrIn) -> Self::Output;
}

// ── Register offset: extended 64-bit register ────────────────────────────────

impl<RtIn, Base, Ext> MakeLdrb<RtIn, (Base, Ext)>
    for Ldrb<LdStArgs<RegOrZero32, (RegOrSp64, Extended<ByteShift, RegOrZero64>)>>
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

impl<RtIn, Base, Ext> MakeLdrb<RtIn, (Base, Ext)>
    for Ldrb<LdStArgs<RegOrZero32, (RegOrSp64, Extended<ByteShift, RegOrZero32>)>>
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

impl<RtIn, Base, OffsetReg> MakeLdrb<RtIn, (Base, OffsetReg)>
    for Ldrb<LdStArgs<RegOrZero32, (RegOrSp64, RegOrZero64)>>
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

impl<RtIn, Base> MakeLdrb<RtIn, Base>
    for Ldrb<LdStArgs<RegOrZero32, (RegOrSp64, ScaledOffset8)>>
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

impl<RtIn, Base> MakeLdrb<RtIn, (Base,)>
    for Ldrb<LdStArgs<RegOrZero32, (RegOrSp64, ScaledOffset8)>>
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

impl<RtIn, Base> MakeLdrb<RtIn, (Base, ScaledOffset8)>
    for Ldrb<LdStArgs<RegOrZero32, (RegOrSp64, ScaledOffset8)>>
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

impl<RtIn, Base> MakeLdrb<RtIn, (Base, u32)>
    for Ldrb<LdStArgs<RegOrZero32, (RegOrSp64, ScaledOffset8)>>
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

impl<RtIn, Base> MakeLdrb<RtIn, (Base, i32)>
    for Ldrb<LdStArgs<RegOrZero32, (RegOrSp64, ScaledOffset8)>>
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

impl<RtIn, Base> MakeLdrb<RtIn, (Inc<LdStIncOffset>, Base)>
    for Ldrb<LdStArgs<RegOrZero32, (Inc<LdStIncOffset>, RegOrSp64)>>
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

impl<RtIn, Base> MakeLdrb<RtIn, (Base, Inc<LdStIncOffset>)>
    for Ldrb<LdStArgs<RegOrZero32, (RegOrSp64, Inc<LdStIncOffset>)>>
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

impl<Rt, RtIn, BaseIn, Ext, Err> MakeLdrb<RtIn, (BaseIn, Result<Ext, Err>)>
    for Ldrb<LdStArgs<Rt, (RegOrSp64, Ext)>>
where
    Ldrb<LdStArgs<Rt, (RegOrSp64, Ext)>>: MakeLdrb<RtIn, (BaseIn, Ext)>,
    BaseIn: IntoReg<RegOrSp64>,
{
    type Output = Result<<Self as MakeLdrb<RtIn, (BaseIn, Ext)>>::Output, Err>;
    #[inline]
    fn new(rt: RtIn, (base, ext_res): (BaseIn, Result<Ext, Err>)) -> Self::Output {
        ext_res.map(|ext| <Self as MakeLdrb<RtIn, (BaseIn, Ext)>>::new(rt, (base, ext)))
    }
}

impl<Rt, RtIn, BaseIn, Ext, Err> MakeLdrb<RtIn, (Result<Ext, Err>, BaseIn)>
    for Ldrb<LdStArgs<Rt, (Ext, RegOrSp64)>>
where
    Ldrb<LdStArgs<Rt, (Ext, RegOrSp64)>>: MakeLdrb<RtIn, (Ext, BaseIn)>,
    BaseIn: IntoReg<RegOrSp64>,
{
    type Output = Result<<Self as MakeLdrb<RtIn, (Ext, BaseIn)>>::Output, Err>;
    #[inline]
    fn new(rt: RtIn, (ext_res, base): (Result<Ext, Err>, BaseIn)) -> Self::Output {
        ext_res.map(|ext| <Self as MakeLdrb<RtIn, (Ext, BaseIn)>>::new(rt, (ext, base)))
    }
}

impl<Rt, RtIn, Addr, Err> MakeLdrb<RtIn, Result<Addr, Err>>
    for Ldrb<LdStArgs<Rt, Addr>>
where
    Ldrb<LdStArgs<Rt, Addr>>: MakeLdrb<RtIn, Addr>,
{
    type Output = Result<<Self as MakeLdrb<RtIn, Addr>>::Output, Err>;
    #[inline]
    fn new(rt: RtIn, addr_res: Result<Addr, Err>) -> Self::Output {
        addr_res.map(|addr| <Self as MakeLdrb<RtIn, Addr>>::new(rt, addr))
    }
}

/// ldrb construction function.  See examples in the module documentation.
pub fn ldrb<RtIn, Rt, AddrIn, Addr>(
    dst: RtIn,
    addr: AddrIn,
) -> <Ldrb<LdStArgs<Rt, Addr>> as MakeLdrb<RtIn, AddrIn>>::Output
where
    Ldrb<LdStArgs<Rt, Addr>>: MakeLdrb<RtIn, AddrIn>,
{
    <Ldrb<LdStArgs<Rt, Addr>> as MakeLdrb<RtIn, AddrIn>>::new(dst, addr)
}

// === LDRB: extended 64-bit register offset ===

impl RawInstruction
    for Ldrb<LdStArgs<RegOrZero32, (RegOrSp64, Extended<ByteShift, RegOrZero64>)>>
{
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let (base, offset) = self.0.addr;
        LDRB_32B_ldst_regoff(
            offset.offset.index(),
            (offset.extend as u8).into(),
            offset.shifted.into(),
            base.index(),
            self.0.rt.index(),
        )
    }
}

// === LDRB: extended 32-bit register offset ===

impl RawInstruction
    for Ldrb<LdStArgs<RegOrZero32, (RegOrSp64, Extended<ByteShift, RegOrZero32>)>>
{
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let (base, offset) = self.0.addr;
        LDRB_32B_ldst_regoff(
            offset.offset.index(),
            (offset.extend as u8).into(),
            offset.shifted.into(),
            base.index(),
            self.0.rt.index(),
        )
    }
}

// === LDRB: bare 64-bit register offset ===

impl RawInstruction for Ldrb<LdStArgs<RegOrZero32, (RegOrSp64, RegOrZero64)>> {
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let (base, offset) = self.0.addr;
        LDRB_32B_ldst_regoff(
            offset.index(),
            (LdStExtendOption64::default() as u8).into(),
            0b0.into(),
            base.index(),
            self.0.rt.index(),
        )
    }
}

// === LDRB: scaled immediate offset ===

impl RawInstruction for Ldrb<LdStArgs<RegOrZero32, (RegOrSp64, ScaledOffset8)>> {
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let (base, offset) = self.0.addr;
        LDRB_32_ldst_pos(offset.into(), base.index(), self.0.rt.index())
    }
}

// === LDRB: pre-increment ===

impl RawInstruction for Ldrb<LdStArgs<RegOrZero32, (Inc<LdStIncOffset>, RegOrSp64)>> {
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let (inc, base) = self.0.addr;
        LDRB_32_ldst_immpre(inc.offset.into(), base.index(), self.0.rt.index())
    }
}

// === LDRB: post-increment ===

impl RawInstruction for Ldrb<LdStArgs<RegOrZero32, (RegOrSp64, Inc<LdStIncOffset>)>> {
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let (base, inc) = self.0.addr;
        LDRB_32_ldst_immpost(inc.offset.into(), base.index(), self.0.rt.index())
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

    const LDRB_REG_EXT_DB: &str = "
38634902	ldrb w2, [x8, w3, uxtw]
38635902	ldrb w2, [x8, w3, uxtw #0]
38636902	ldrb w2, [x8, x3]
38637902	ldrb w2, [x8, x3, lsl #0]
38636be2	ldrb w2, [sp, x3]
3863c902	ldrb w2, [x8, w3, sxtw]
3863d902	ldrb w2, [x8, w3, sxtw #0]
3863e902	ldrb w2, [x8, x3, sxtx]
3863f902	ldrb w2, [x8, x3, sxtx #0]
387f6902	ldrb w2, [x8, xzr]
387f6be2	ldrb w2, [sp, xzr]

";

    // 'ldrb (w2|x2), [(x8|sp), #0x190]'
    const LDRB_SCALED_IMM_DB: &str = "
39464102	ldrb w2, [x8, #0x190]
3946411f	ldrb wzr, [x8, #0x190]
394643e2	ldrb w2, [sp, #0x190]
39400102	ldrb w2, [x8]
";

    const LDRB_PRE_POST_INC_DB: &str = "
3842a441	ldrb w1, [x2], #0x2a
3842a45f	ldrb wzr, [x2], #0x2a
3842a7e1	ldrb w1, [sp], #0x2a
3842ac41	ldrb w1, [x2, #0x2a]!
3842ac5f	ldrb wzr, [x2, #0x2a]!
3842afe1	ldrb w1, [sp, #0x2a]!
385d6441	ldrb w1, [x2], #-0x2a
385d67e1	ldrb w1, [sp], #-0x2a
385d6c41	ldrb w1, [x2, #-0x2a]!
385d6fe1	ldrb w1, [sp, #-0x2a]!
";

    test_cases! {
        LDRB_REG_EXT_DB, untested_ldrb_reg_ext_db;
        test_ldrb_r32_r64_r32_sxtw, ldrb(W2, (X8, ext((W3, SXTW)))), "ldrb w2, [x8, w3, sxtw]";
        test_ldrb_r32_r64_r32_uxtw, ldrb(W2, (X8, ext((W3, UXTW)))), "ldrb w2, [x8, w3, uxtw]";
        test_ldrb_r32_r64_r64_sxtx, ldrb(W2, (X8, ext((X3, SXTX)))), "ldrb w2, [x8, x3, sxtx]";
        test_ldrb_r32_r64_r32_sxtw_0, ldrb(W2, (X8, ext((W3, SXTW, 0)))).unwrap(), "ldrb w2, [x8, w3, sxtw #0]";
        test_ldrb_r32_r64_r32_uxtw_0, ldrb(W2, (X8, ext((W3, UXTW, 0)))).unwrap(), "ldrb w2, [x8, w3, uxtw #0]";
        test_ldrb_r32_r64_r64_lsl_0, ldrb(W2, (X8, ext((X3, LSL, 0)))).unwrap(), "ldrb w2, [x8, x3, lsl #0]";
        test_ldrb_r32_r64_r64_sxtx_0, ldrb(W2, (X8, ext((X3, SXTX, 0)))).unwrap(), "ldrb w2, [x8, x3, sxtx #0]";
        test_ldrb_r32_r64_r64, ldrb(W2, (X8, X3)), "ldrb w2, [x8, x3]";
        test_ldrb_r32_rsp_r64, ldrb(W2, (SP, X3)), "ldrb w2, [sp, x3]";
        test_ldrb_r32_r64_xzr, ldrb(W2, (X8, XZR)), "ldrb w2, [x8, xzr]";
        test_ldrb_r32_rsp_xzr, ldrb(W2, (SP, XZR)), "ldrb w2, [sp, xzr]";
    }

    test_cases! {
        LDRB_SCALED_IMM_DB, untested_ldrb_scaled_imm;
        test_ldrb_r32_r64_scaled_imm, ldrb(W2, (X8, UBitValue::<12>::new(0x190).unwrap())), "ldrb w2, [x8, #0x190]";
        test_ldrb_r32_sp_scaled_imm, ldrb(W2, (SP, UBitValue::<12>::new(0x190).unwrap())), "ldrb w2, [sp, #0x190]";
        test_ldrb_r32_r64_scaled_imm2, ldrb(W2, (X8, 0x190u32)).unwrap(), "ldrb w2, [x8, #0x190]";
        test_ldrb_wzr_r64_scaled_imm2, ldrb(WZR, (X8, 0x190u32)).unwrap(), "ldrb wzr, [x8, #0x190]";
        test_ldrb_r32_r64_scaled_imm3, ldrb(W2, (X8, 0x190i32)).unwrap(), "ldrb w2, [x8, #0x190]";
        test_ldrb_wzr_r64_scaled_imm3, ldrb(WZR, (X8, 0x190i32)).unwrap(), "ldrb wzr, [x8, #0x190]";
        test_ldrb_r32_r64_simple, ldrb(W2, (X8,)), "ldrb w2, [x8]";
    }

    test_cases! {
        LDRB_PRE_POST_INC_DB, untested_ldrb_pre_post_inc;
        test_ldrb_r32_r64_preinc, ldrb(W1, preinc(X2, 0x2a)).unwrap(), "ldrb w1, [x2, #0x2a]!";
        test_ldrb_r32_r64_postinc, ldrb(W1, postinc(X2, 0x2a)).unwrap(), "ldrb w1, [x2], #0x2a";
        test_ldrb_r32_sp_preinc, ldrb(W1, preinc(SP, 0x2a)).unwrap(), "ldrb w1, [sp, #0x2a]!";
        test_ldrb_r32_sp_postinc, ldrb(W1, postinc(SP, 0x2a)).unwrap(), "ldrb w1, [sp], #0x2a";
        test_ldrb_r32_r64_preinc_neg, ldrb(W1, preinc(X2, -0x2a)).unwrap(), "ldrb w1, [x2, #-0x2a]!";
        test_ldrb_r32_r64_postinc_neg, ldrb(W1, postinc(X2, -0x2a)).unwrap(), "ldrb w1, [x2], #-0x2a";
        test_ldrb_r32_sp_preinc_neg, ldrb(W1, preinc(SP, -0x2a)).unwrap(), "ldrb w1, [sp, #-0x2a]!";
        test_ldrb_r32_sp_postinc_neg, ldrb(W1, postinc(SP, -0x2a)).unwrap(), "ldrb w1, [sp], #-0x2a";
        test_ldrb_r32_sp_preinc2, ldrb(W1, preinc(SP, LdStIncOffset::new(0x2a).unwrap())), "ldrb w1, [sp, #0x2a]!";
        test_ldrb_r32_r64_pre_inc, ldrb(W1, (inc(0x2a), X2)).unwrap(), "ldrb w1, [x2, #0x2a]!";
        test_ldrb_r32_r64_post_inc, ldrb(W1, (X2, inc(0x2a))).unwrap(), "ldrb w1, [x2], #0x2a";
        test_ldrb_r32_sp_pre_inc, ldrb(W1, (inc(0x2a), SP)).unwrap(), "ldrb w1, [sp, #0x2a]!";
        test_ldrb_r32_sp_post_inc, ldrb(W1, (SP, inc(0x2a))).unwrap(), "ldrb w1, [sp], #0x2a";
        test_ldrb_r32_r64_pre_inc_neg, ldrb(W1, (inc(-0x2a), X2)).unwrap(), "ldrb w1, [x2, #-0x2a]!";
        test_ldrb_r32_r64_post_inc_neg, ldrb(W1, (X2, inc(-0x2a))).unwrap(), "ldrb w1, [x2], #-0x2a";
        test_ldrb_r32_sp_pre_inc_neg, ldrb(W1, (inc(-0x2a), SP)).unwrap(), "ldrb w1, [sp, #-0x2a]!";
        test_ldrb_r32_sp_post_inc_neg, ldrb(W1, (SP, inc(-0x2a))).unwrap(), "ldrb w1, [sp], #-0x2a";
        test_ldrb_r32_sp_pre_inc2, ldrb(W1, (inc(LdStIncOffset::new(0x2a).unwrap()), SP)), "ldrb w1, [sp, #0x2a]!";
        test_ldrb_wzr_r64_pre_inc, ldrb(WZR, (inc(0x2a), X2)).unwrap(), "ldrb wzr, [x2, #0x2a]!";
        test_ldrb_wzr_r64_post_inc, ldrb(WZR, (X2, inc(0x2a))).unwrap(), "ldrb wzr, [x2], #0x2a";
    }
}
