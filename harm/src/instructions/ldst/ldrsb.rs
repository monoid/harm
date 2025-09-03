/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

//! `LDRSB` and related commands.
//!
//! The `ldrsb` function returns an instance of `Instruction` for loading a byte into a 32-bit or 64-bit register from
//! memory. While `LDRSB` instruction has different variants with various number of arguments, the `ldrsb` function has
//! two arguments: a destination register and an "address" that encapsulates the rest of arguments: the base, offsets,
//! extensions, etc. Tuples are often used for the second argument, see the pattern in the examples below.
//!
//! The funciton is overloaded for various argument types. For some of them, and `Instruction` trait instance is
//! returned, for others, a `Result` if the aguments need validation. Such arugment combinations have `.unwrap()` in
//! examples.
//!
//! # `LDRSB`: Register base with register offset
//!
//! # Examples:
//! ```
//! # use harm::instructions::ldst::{ldrsb, ext, LdStExtendOption32, LdStShift};
//! use harm::register::Reg32::*;
//! use harm::register::Reg64::*;
//! use LdStExtendOption32::*;
//!
//! ldrsb(W1, X2);        // LDRSB W1, [X2]
//! ldrsb(W1, (X2,));     // LDRSB W1, [X2]
//! ldrsb(W1, (X2, X3));  // LDRSB W1, [X2, X3] ; n.b. a 32-bit register offset requires an extend speicifer (sxtw or uxtw):
//! ldrsb(W1, (X2, ext((W3, UXTW)))); // ldrsb w1, [x2, w3, uxtw]
//! ldrsb(W1, (X2, ext((W3, UXTW, LdStShift::Shifted)))); // ldrsb w1, [x2, w3, uxtw #0]
//! ldrsb(W1, (X2, ext((W3, UXTW)))); // ldrsb w1, [x2, w3, uxtw]
//! ldrsb(X1, (X2, ext((W3, UXTW)))); // ldrsb x1, [x2, w3, uxtw]
//! ldrsb(X1, (X2, ext((W3, UXTW, LdStShift::Shifted)))); // ldrsb x1, [x2, w3, uxtw #0]
//! ldrsb(X1, (X2, ext((W3, UXTW, 0)))).unwrap(); // ldrsb x1, [x2, w3, uxtw #0]
//! ```
//!
//! Please note, that `uxtw` and `sxtw` can be used only with 32-bit register, and shift can be either omited
//! (unshifted) or 0 (shifted), which is the same for this instruction. The `lsl` and `sxtx` can be used only with
//! 64-bit registers, and while they produce different bit patterns, they are equivalent; shift can be either omited
//! (unshifted) or 0 (shifted), which is the same for this instruction.
//!
//! # `LDRSB`: Register base with immediate offset
//!
//! LDRSB with register base with immediate offset has an unsigned offset. The offset has 12
//! significan bits available.
//!
//! You may also a `u32` offset value, and a error is returned if the value doesn't fit the offset pattern.
//!
//! Examples:
//! ```ignore
//! let word_aligned_offset: UBitValue<12, 0> = ...;
//! let dword_aligned_offset: UBitValue<12, 0> = ...;
//!
//! ldrsb(W1, (X2, offset as u32)).unwrap(),
//! ldrsb(X1, (X2, offset as u32)).unwrap(),
//! ldrsb(W1, (X2, word_aligned_offset)),
//! ldrsb(X1, (X2, dword_aligned_offset)),
//! ```
//!
//! Pre-increment and post-increment variants have the following syntax:
//! ```
//! # use harm::instructions::ldst::{ldrsb, inc, preinc, postinc, LdStIncOffset};
//! use harm::register::Reg32::*;
//! use harm::register::Reg64::*;
//! let offset = LdStIncOffset::new(4).unwrap();
//! ldrsb(W1, (inc(offset), X2));       // preincrement, LDRSB W1, [X2, #4]!
//! ldrsb(W1, (X2, inc(offset)));       // postincrement, LDRSB W1, [X2], #4
//! // Equavalent to the lines above:
//! ldrsb(W1, preinc(X2, offset));      // preincrement, LDRSB W1, [X2, #4]!
//! ldrsb(W1, postinc(X2, offset));     // postincrement, LDRSB W1, [X2], #4
//! // Fallible variants:
//! ldrsb(W1, (inc(4), X2)).unwrap();   // preincrement, LDRSB W1, [X2, #4]!
//! ldrsb(W1, postinc(X2, 4)).unwrap(); // postincrement, LDRSB W1, [X2], #4
//! ```
//!

use aarchmrs_instructions::A64::ldst::{
    ldst_immpost::{
        LDRSB_32_ldst_immpost::LDRSB_32_ldst_immpost, LDRSB_64_ldst_immpost::LDRSB_64_ldst_immpost,
    },
    ldst_immpre::{
        LDRSB_32_ldst_immpre::LDRSB_32_ldst_immpre, LDRSB_64_ldst_immpre::LDRSB_64_ldst_immpre,
    },
    ldst_pos::{LDRSB_32_ldst_pos::LDRSB_32_ldst_pos, LDRSB_64_ldst_pos::LDRSB_64_ldst_pos},
    ldst_regoff::{
        LDRSB_32B_ldst_regoff::LDRSB_32B_ldst_regoff, LDRSB_64B_ldst_regoff::LDRSB_64B_ldst_regoff,
    },
};

use super::shift_extend::*;
use super::{ByteShift, Inc, LdStIncOffset, ScaledOffset8};
use crate::bits::BitError;
use crate::instructions::RawInstruction;
use crate::register::{IntoCode, RegOrSp64, RegOrZero32, RegOrZero64};

/// A `LDRSB` instruction with a destination and an address.
pub struct Ldrsb<Rt, Addr> {
    rt: Rt,
    addr: Addr,
}

impl<Rt, Addr> Ldrsb<Rt, Addr> {
    pub fn rt(&self) -> &Rt {
        &self.rt
    }

    pub fn addr(&self) -> &Addr {
        &self.addr
    }
}

/// Defines possible was to construct a `Ldrsb` instruction.
// TODO sealed trait?
pub trait MakeLdrsb<Rt, Addr> {
    /// Allows defining both faillible and infallible constructors.
    type Output;
    fn new(rt: Rt, addr: Addr) -> Self::Output;
}
//
// ## LDRSB (register offset)
//
define_reg_offset_rules!(Ldrsb, MakeLdrsb, LDRSB, RegOrZero64, "64B", ByteShift);
define_reg_offset_rules!(Ldrsb, MakeLdrsb, LDRSB, RegOrZero32, "32B", ByteShift);

//
// ## LDRSB (immediate offset)
//
define_simple_rules!(
    Ldrsb,
    MakeLdrsb,
    RegOrZero64,
    ScaledOffset8,
    ScaledOffset8::default()
);
define_simple_rules!(
    Ldrsb,
    MakeLdrsb,
    RegOrZero32,
    ScaledOffset8,
    ScaledOffset8::default()
);
define_imm_offset_rules!(Ldrsb, MakeLdrsb, LDRSB, RegOrZero64, 64, ScaledOffset8);
define_imm_offset_rules!(Ldrsb, MakeLdrsb, LDRSB, RegOrZero32, 32, ScaledOffset8);

//
// ## Faillible
//
define_fallible_rules!(LDRSB, Ldrsb, MakeLdrsb);

/// ldrsb construction function.  See examples in the module documentation.
pub fn ldrsb<TargetInp, TargetOut, AddrInp, AddrOut>(
    dst: TargetInp,
    addr: AddrInp,
) -> <Ldrsb<TargetOut, AddrOut> as MakeLdrsb<TargetInp, AddrInp>>::Output
where
    Ldrsb<TargetOut, AddrOut>: MakeLdrsb<TargetInp, AddrInp>,
{
    Ldrsb::new(dst, addr)
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

    const LDRSB_REG_EXT_DB: &str = "
38e35902	ldrsb w2, [x8, w3, uxtw #0]
38e34902	ldrsb w2, [x8, w3, uxtw]
38e37902	ldrsb w2, [x8, x3, lsl #0]
38e36902	ldrsb w2, [x8, x3]
38e36be2	ldrsb w2, [sp, x3]
38e3d902	ldrsb w2, [x8, w3, sxtw #0]
38e3c902	ldrsb w2, [x8, w3, sxtw]
38e3f902	ldrsb w2, [x8, x3, sxtx #0]
38e3e902	ldrsb w2, [x8, x3, sxtx]
38ff6902	ldrsb w2, [x8, xzr]
38ff6be2	ldrsb w2, [sp, xzr]
38a35902	ldrsb x2, [x8, w3, uxtw #0]
38a34902	ldrsb x2, [x8, w3, uxtw]
38a37902	ldrsb x2, [x8, x3, lsl #0]
38a36902	ldrsb x2, [x8, x3]
38a36be2	ldrsb x2, [sp, x3]
38a3d902	ldrsb x2, [x8, w3, sxtw #0]
38a3c902	ldrsb x2, [x8, w3, sxtw]
38a3f902	ldrsb x2, [x8, x3, sxtx #0]
38a3e902	ldrsb x2, [x8, x3, sxtx]
38a9491f	ldrsb xzr, [x8, w9, uxtw]
38a9691f	ldrsb xzr, [x8, x9]
38a9691f	ldrsb xzr, [x8, x9]
38a9c91f	ldrsb xzr, [x8, w9, sxtw]
38bf4902	ldrsb x2, [x8, wzr, uxtw]
38bf6902	ldrsb x2, [x8, xzr]
38bf6be2	ldrsb x2, [sp, xzr]
38bfc902	ldrsb x2, [x8, wzr, sxtw]
38bfe902	ldrsb x2, [x8, xzr, sxtx]
";

    // 'ldrsb (w2|x2), [(x8|sp), #0x190]'
    const LDRSB_SCALED_IMM_DB: &str = "
39c64102	ldrsb w2, [x8, #0x190]
39c6411f	ldrsb wzr, [x8, #0x190]
39c643e2	ldrsb w2, [sp, #0x190]
39864102	ldrsb x2, [x8, #0x190]
398643e2	ldrsb x2, [sp, #0x190]
398643ff	ldrsb xzr, [sp, #0x190]
39800102	ldrsb x2, [x8]
";

    const LDRSB_PRE_POST_INC_DB: &str = "
38c2a441	ldrsb w1, [x2], #0x2a
38c2a45f	ldrsb wzr, [x2], #0x2a
38c2a7e1	ldrsb w1, [sp], #0x2a
38c2ac41	ldrsb w1, [x2, #0x2a]!
38c2ac5f	ldrsb wzr, [x2, #0x2a]!
38c2afe1	ldrsb w1, [sp, #0x2a]!
38dd6441	ldrsb w1, [x2], #-0x2a
38dd67e1	ldrsb w1, [sp], #-0x2a
38dd6c41	ldrsb w1, [x2, #-0x2a]!
38dd6fe1	ldrsb w1, [sp, #-0x2a]!
3882a441	ldrsb x1, [x2], #0x2a
3882a45f	ldrsb xzr, [x2], #0x2a
3882a7e1	ldrsb x1, [sp], #0x2a
3882ac41	ldrsb x1, [x2, #0x2a]!
3882ac5f	ldrsb xzr, [x2, #0x2a]!
3882afe1	ldrsb x1, [sp, #0x2a]!
389d6441	ldrsb x1, [x2], #-0x2a
389d67e1	ldrsb x1, [sp], #-0x2a
389d6c41	ldrsb x1, [x2, #-0x2a]!
389d6fe1	ldrsb x1, [sp, #-0x2a]!
";

    test_cases! {
        LDRSB_REG_EXT_DB, untested_ldrsb_reg_ext_db;
        test_ldrsb_r64_r64_r32_sxtw, ldrsb(X2, (X8, ext((W3, SXTW)))), "ldrsb x2, [x8, w3, sxtw]";
        test_ldrsb_r64_r64_r32_uxtw, ldrsb(X2, (X8, ext((W3, UXTW)))), "ldrsb x2, [x8, w3, uxtw]";
        test_ldrsb_r32_r64_r32_sxtw, ldrsb(W2, (X8, ext((W3, SXTW)))), "ldrsb w2, [x8, w3, sxtw]";
        test_ldrsb_r32_r64_r32_uxtw, ldrsb(W2, (X8, ext((W3, UXTW)))), "ldrsb w2, [x8, w3, uxtw]";
        test_ldrsb_r32_r64_r64_sxtx, ldrsb(W2, (X8, ext((X3, SXTX)))), "ldrsb w2, [x8, x3, sxtx]";
        test_ldrsb_r64_r64_r64_sxtx, ldrsb(X2, (X8, ext((X3, SXTX)))), "ldrsb x2, [x8, x3, sxtx]";
        test_ldrsb_r64_r64_xzr_sxtx, ldrsb(X2, (X8, ext((XZR, SXTX)))), "ldrsb x2, [x8, xzr, sxtx]";
        test_ldrsb_r64_r64_wzr_sxtw, ldrsb(X2, (X8, ext((WZR, SXTW)))), "ldrsb x2, [x8, wzr, sxtw]";
        test_ldrsb_r64_r64_wzr_uxtx, ldrsb(X2, (X8, ext((WZR, UXTW)))), "ldrsb x2, [x8, wzr, uxtw]";
        test_ldrsb_r32_r64_r32_sxtw_0, ldrsb(W2, (X8, ext((W3, SXTW, 0)))).unwrap(), "ldrsb w2, [x8, w3, sxtw #0]";
        test_ldrsb_r32_r64_r32_uxtw_0, ldrsb(W2, (X8, ext((W3, UXTW, 0)))).unwrap(), "ldrsb w2, [x8, w3, uxtw #0]";
        test_ldrsb_r64_r64_r32_sxtw_0, ldrsb(X2, (X8, ext((W3, SXTW, 0)))).unwrap(), "ldrsb x2, [x8, w3, sxtw #0]";
        test_ldrsb_r64_r64_r32_uxtw_0, ldrsb(X2, (X8, ext((W3, UXTW, 0)))).unwrap(), "ldrsb x2, [x8, w3, uxtw #0]";
        test_ldrsb_r32_r64_r64_lsl_0, ldrsb(W2, (X8, ext((X3, LSL, 0)))).unwrap(), "ldrsb w2, [x8, x3, lsl #0]";
        test_ldrsb_r32_r64_r64_sxtx_0, ldrsb(W2, (X8, ext((X3, SXTX, 0)))).unwrap(), "ldrsb w2, [x8, x3, sxtx #0]";
        test_ldrsb_r64_r64_r64_lsl_0, ldrsb(X2, (X8, ext((X3, LSL, 0)))).unwrap(), "ldrsb x2, [x8, x3, lsl #0]";
        test_ldrsb_r64_r64_r32_sxtx_0, ldrsb(X2, (X8, ext((X3, SXTX, 0)))).unwrap(), "ldrsb x2, [x8, x3, sxtx #0]";
        test_ldrsb_r32_r64_r64, ldrsb(W2, (X8, X3)), "ldrsb w2, [x8, x3]";
        test_ldrsb_r32_rsp_r64, ldrsb(W2, (SP, X3)), "ldrsb w2, [sp, x3]";
        test_ldrsb_r64_r64_r64, ldrsb(X2, (X8, X3)), "ldrsb x2, [x8, x3]";
        test_ldrsb_r64_rsp_r64, ldrsb(X2, (SP, X3)), "ldrsb x2, [sp, x3]";
        test_ldrsb_r32_r64_xzr, ldrsb(W2, (X8, XZR)), "ldrsb w2, [x8, xzr]";
        test_ldrsb_r32_rsp_xzr, ldrsb(W2, (SP, XZR)), "ldrsb w2, [sp, xzr]";
        test_ldrsb_r64_r64_xzr, ldrsb(X2, (X8, XZR)), "ldrsb x2, [x8, xzr]";
        test_ldrsb_r64_rsp_xzr, ldrsb(X2, (SP, XZR)), "ldrsb x2, [sp, xzr]";
        test_ldrsb_xzr_r64_r64, ldrsb(XZR, (X8, X9)), "ldrsb xzr, [x8, x9]";
        test_ldrsb_wzr_r64_r64, ldrsb(XZR, (X8, X9)), "ldrsb xzr, [x8, x9]";
        test_ldrsb_xzr_r64_r32_sxtw, ldrsb(XZR, (X8, ext((W9, SXTW)))), "ldrsb xzr, [x8, w9, sxtw]";
        test_ldrsb_wzr_r64_r32_uxtw, ldrsb(XZR, (X8, ext((W9, UXTW)))), "ldrsb xzr, [x8, w9, uxtw]";
    }

    test_cases! {
        LDRSB_SCALED_IMM_DB, untested_ldrsb_scaled_imm;
        test_ldrsb_r32_r64_scaled_imm, ldrsb(W2, (X8, UBitValue::<12>::new(0x190).unwrap())), "ldrsb w2, [x8, #0x190]";
        test_ldrsb_r32_sp_scaled_imm, ldrsb(W2, (SP, UBitValue::<12>::new(0x190).unwrap())), "ldrsb w2, [sp, #0x190]";
        test_ldrsb_r64_r64_scaled_imm, ldrsb(X2, (X8, UBitValue::<12>::new(0x190).unwrap())), "ldrsb x2, [x8, #0x190]";
        test_ldrsb_r64_sp_scaled_imm, ldrsb(X2, (SP, UBitValue::<12>::new(0x190).unwrap())), "ldrsb x2, [sp, #0x190]";
        test_ldrsb_r32_r64_scaled_imm2, ldrsb(W2, (X8, 0x190u32)).unwrap(), "ldrsb w2, [x8, #0x190]";
        test_ldrsb_r64_sp_scaled_imm2, ldrsb(X2, (SP, 0x190u32)).unwrap(), "ldrsb x2, [sp, #0x190]";
        test_ldrsb_wzr_r64_scaled_imm2, ldrsb(WZR, (X8, 0x190u32)).unwrap(), "ldrsb wzr, [x8, #0x190]";
        test_ldrsb_xzr_sp_scaled_imm2, ldrsb(XZR, (SP, 0x190u32)).unwrap(), "ldrsb xzr, [sp, #0x190]";
        test_ldrsb_r32_r64_scaled_imm3, ldrsb(W2, (X8, 0x190i32)).unwrap(), "ldrsb w2, [x8, #0x190]";
        test_ldrsb_r64_sp_scaled_imm3, ldrsb(X2, (SP, 0x190i32)).unwrap(), "ldrsb x2, [sp, #0x190]";
        test_ldrsb_wzr_r64_scaled_imm3, ldrsb(WZR, (X8, 0x190i32)).unwrap(), "ldrsb wzr, [x8, #0x190]";
        test_ldrsb_xzr_sp_scaled_imm3, ldrsb(XZR, (SP, 0x190i32)).unwrap(), "ldrsb xzr, [sp, #0x190]";
        test_ldrsb_r64_r64_simple, ldrsb(X2, (X8,)), "ldrsb x2, [x8]";
    }

    test_cases! {
        LDRSB_PRE_POST_INC_DB, untested_ldrsb_pre_post_inc;
        test_ldrsb_r32_r64_preinc, ldrsb(W1, preinc(X2, 0x2a)).unwrap(), "ldrsb w1, [x2, #0x2a]!";
        test_ldrsb_r32_r64_postinc, ldrsb(W1, postinc(X2, 0x2a)).unwrap(), "ldrsb w1, [x2], #0x2a";
        test_ldrsb_r64_r64_preinc, ldrsb(X1, preinc(X2, 0x2a)).unwrap(), "ldrsb x1, [x2, #0x2a]!";
        test_ldrsb_r64_r64_postinc, ldrsb(X1, postinc(X2, 0x2a)).unwrap(), "ldrsb x1, [x2], #0x2a";
        test_ldrsb_r32_sp_preinc, ldrsb(W1, preinc(SP, 0x2a)).unwrap(), "ldrsb w1, [sp, #0x2a]!";
        test_ldrsb_r32_sp_postinc, ldrsb(W1, postinc(SP, 0x2a)).unwrap(), "ldrsb w1, [sp], #0x2a";
        test_ldrsb_r64_sp_preinc, ldrsb(X1, preinc(SP, 0x2a)).unwrap(), "ldrsb x1, [sp, #0x2a]!";
        test_ldrsb_r64_sp_postinc, ldrsb(X1, postinc(SP, 0x2a)).unwrap(), "ldrsb x1, [sp], #0x2a";
        test_ldrsb_r32_r64_preinc_neg, ldrsb(W1, preinc(X2, -0x2a)).unwrap(), "ldrsb w1, [x2, #-0x2a]!";
        test_ldrsb_r32_r64_postinc_neg, ldrsb(W1, postinc(X2, -0x2a)).unwrap(), "ldrsb w1, [x2], #-0x2a";
        test_ldrsb_r64_r64_preinc_neg, ldrsb(X1, preinc(X2, -0x2a)).unwrap(), "ldrsb x1, [x2, #-0x2a]!";
        test_ldrsb_r64_r64_postinc_neg, ldrsb(X1, postinc(X2, -0x2a)).unwrap(), "ldrsb x1, [x2], #-0x2a";
        test_ldrsb_r32_sp_preinc_neg, ldrsb(W1, preinc(SP, -0x2a)).unwrap(), "ldrsb w1, [sp, #-0x2a]!";
        test_ldrsb_r32_sp_postinc_neg, ldrsb(W1, postinc(SP, -0x2a)).unwrap(), "ldrsb w1, [sp], #-0x2a";
        test_ldrsb_r64_sp_preinc_neg, ldrsb(X1, preinc(SP, -0x2a)).unwrap(), "ldrsb x1, [sp, #-0x2a]!";
        test_ldrsb_r64_sp_postinc_neg, ldrsb(X1, postinc(SP, -0x2a)).unwrap(), "ldrsb x1, [sp], #-0x2a";
        test_ldrsb_r32_sp_preinc2, ldrsb(W1, preinc(SP, LdStIncOffset::new(0x2a).unwrap())), "ldrsb w1, [sp, #0x2a]!";
        test_ldrsb_r64_r64_preinc_neg2, ldrsb(X1, preinc(X2, LdStIncOffset::new(-0x2a).unwrap())), "ldrsb x1, [x2, #-0x2a]!";
        test_ldrsb_r32_r64_pre_inc, ldrsb(W1, (inc(0x2a), X2)).unwrap(), "ldrsb w1, [x2, #0x2a]!";
        test_ldrsb_r32_r64_post_inc, ldrsb(W1, (X2, inc(0x2a))).unwrap(), "ldrsb w1, [x2], #0x2a";
        test_ldrsb_r64_r64_pre_inc, ldrsb(X1, (inc(0x2a), X2)).unwrap(), "ldrsb x1, [x2, #0x2a]!";
        test_ldrsb_r64_r64_post_inc, ldrsb(X1, (X2, inc(0x2a))).unwrap(), "ldrsb x1, [x2], #0x2a";
        test_ldrsb_r32_sp_pre_inc, ldrsb(W1, (inc(0x2a), SP)).unwrap(), "ldrsb w1, [sp, #0x2a]!";
        test_ldrsb_r32_sp_post_inc, ldrsb(W1, (SP, inc(0x2a))).unwrap(), "ldrsb w1, [sp], #0x2a";
        test_ldrsb_r64_sp_pre_inc, ldrsb(X1, (inc(0x2a), SP)).unwrap(), "ldrsb x1, [sp, #0x2a]!";
        test_ldrsb_r64_sp_post_inc, ldrsb(X1, (SP, inc(0x2a))).unwrap(), "ldrsb x1, [sp], #0x2a";
        test_ldrsb_r32_r64_pre_inc_neg, ldrsb(W1, (inc(-0x2a), X2)).unwrap(), "ldrsb w1, [x2, #-0x2a]!";
        test_ldrsb_r32_r64_post_inc_neg, ldrsb(W1, (X2, inc(-0x2a))).unwrap(), "ldrsb w1, [x2], #-0x2a";
        test_ldrsb_r64_r64_pre_inc_neg, ldrsb(X1, (inc(-0x2a), X2)).unwrap(), "ldrsb x1, [x2, #-0x2a]!";
        test_ldrsb_r64_r64_post_inc_neg, ldrsb(X1, (X2, inc(-0x2a))).unwrap(), "ldrsb x1, [x2], #-0x2a";
        test_ldrsb_r32_sp_pre_inc_neg, ldrsb(W1, (inc(-0x2a), SP)).unwrap(), "ldrsb w1, [sp, #-0x2a]!";
        test_ldrsb_r32_sp_post_inc_neg, ldrsb(W1, (SP, inc(-0x2a))).unwrap(), "ldrsb w1, [sp], #-0x2a";
        test_ldrsb_r64_sp_pre_inc_neg, ldrsb(X1, (inc(-0x2a), SP)).unwrap(), "ldrsb x1, [sp, #-0x2a]!";
        test_ldrsb_r64_sp_post_inc_neg, ldrsb(X1, (SP, inc(-0x2a))).unwrap(), "ldrsb x1, [sp], #-0x2a";
        test_ldrsb_r32_sp_pre_inc2, ldrsb(W1, (inc(LdStIncOffset::new(0x2a).unwrap()), SP)), "ldrsb w1, [sp, #0x2a]!";
        test_ldrsb_r64_r64_pre_inc_neg2, ldrsb(X1, (inc(LdStIncOffset::new(-0x2a).unwrap()), X2)), "ldrsb x1, [x2, #-0x2a]!";
        test_ldrsb_xzr_r64_pre_inc, ldrsb(XZR, (inc(0x2a), X2)).unwrap(), "ldrsb xzr, [x2, #0x2a]!";
        test_ldrsb_xzr_r64_post_inc, ldrsb(XZR, (X2, inc(0x2a))).unwrap(), "ldrsb xzr, [x2], #0x2a";
        test_ldrsb_wzr_r64_pre_inc, ldrsb(WZR, (inc(0x2a), X2)).unwrap(), "ldrsb wzr, [x2, #0x2a]!";
        test_ldrsb_wzr_r64_post_inc, ldrsb(WZR, (X2, inc(0x2a))).unwrap(), "ldrsb wzr, [x2], #0x2a";
    }
}
