/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

//! `STRH` and related commands.
//!
//! The `strh` function returns an instance of `Instruction` for storing a halfword of 32-bit register into memory.
//! While `STRH` instruction has different variants with various number of arguments, the `strh` function has two
//! arguments: a destination register and an "address" that encapsulates the rest of arguments: the base, offsets,
//! extensions, etc. Tuples are often used for the second argument, see the pattern in the examples below.
//!
//! The funciton is overloaded for various argument types. For some of them, and `Instruction` trait instance is
//! returned, for others, a `Result` if the aguments need validation. Such arugment combinations have `.unwrap()` in
//! examples.
//!
//! # `STRH`: Register base with register offset
//!
//! # Examples:
//! ```
//! # use harm::instructions::ldst::{strh, ext, LdStExtendOption32, LdStShift};
//! use harm_types::A64::register::Reg32::*;
//! use harm_types::A64::register::Reg64::*;
//! use LdStExtendOption32::*;
//!
//! strh(W1, X2);        // STRH W1, [X2]
//! strh(W1, (X2,));     // STRH W1, [X2]
//! strh(W1, (X2, X3));  // STRH W1, [X2, X3] ; n.b. a 32-bit register offset requires an extend speicifer (sxtw or uxtw):
//! strh(W1, (X2, ext((W3, UXTW)))); // strh w1, [x2, w3, uxtw]
//! strh(W1, (X2, ext((W3, UXTW, LdStShift::Unshifted)))); // strh w1, [x2, w3, uxtw #0]
//! strh(W1, (X2, ext((W3, UXTW, LdStShift::Shifted)))); // strh w1, [x2, w3, uxtw #1]
//! strh(W1, (X2, ext((W3, UXTW)))); // strh w1, [x2, w3, uxtw]
//! ```
//!
//! Please note, that `uxtw` and `sxtw` can be used only with index 32-bit register, and shift can be either omited, 0
//! or 1. The `lsl` and `sxtx` can be used only with 64-bit index registers, and while they produce different bit
//! patterns, they are equivalent; shift can be only either omited, 0 or 1. Please note that we do allow `lsl` without
//! shift.
//!
//! # `STRH`: Register base with immediate offset
//!
//! STRH with register base with 2-byte aligned immediate offset. The offset has 12 significant bits available.
//!
//! You may also a `u32` offset value, and a error is returned if the value doesn't fit the offset pattern.
//!
//! Examples:
//! ```ignore
//! let word_aligned_offset: UBitValue<12, 2> = ...;
//! let dword_aligned_offset: UBitValue<12, 3> = ...;
//!
//! strh(W1, (X2, offset as u32)).unwrap(),
//! strh(W1, (X2, word_aligned_offset)),
//! ```
//!
//! Pre-increment and post-increment variants have the following syntax:
//! ```
//! # use harm::instructions::ldst::{strh, inc, preinc, postinc, LdStIncOffset};
//! use harm_types::A64::register::Reg32::*;
//! use harm_types::A64::register::Reg64::*;
//! let offset = LdStIncOffset::new(4).unwrap();
//! strh(W1, (inc(offset), X2));       // preincrement, STRH W1, [X2, #4]!
//! strh(W1, (X2, inc(offset)));       // postincrement, STRH W1, [X2], #4
//! // Equavalent to the lines above:
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

use super::{HalfShift, shift_extend::*};
use super::{Inc, LdStIncOffset, ScaledOffset16};
use crate::{
    bits::BitError,
    instructions::RawInstruction,
    register::{IntoReg, RegOrSp64, RegOrZero32, RegOrZero64, Register},
    sealed::Sealed,
};

/// A `STRH` instruction with a destination and an address.
pub struct Strh<Rt, Addr> {
    rt: Rt,
    addr: Addr,
}

impl<Rt, Addr> Strh<Rt, Addr> {
    pub fn rt(&self) -> &Rt {
        &self.rt
    }

    pub fn addr(&self) -> &Addr {
        &self.addr
    }
}

impl<Rt, Addr> Sealed for Strh<Rt, Addr> {}

/// Defines possible was to construct a `STRH` instruction.
pub trait MakeStrh<Rt, Addr>: Sealed {
    /// Allows defining both faillible and infallible constructors.
    type Output;
    fn new(rt: Rt, addr: Addr) -> Self::Output;
}

//
// ## STRH (register offset)
//
define_reg_offset_rules!(Strh, MakeStrh, STRH, RegOrZero32, "32", HalfShift);

//
// ## STRH (immediate offset)
//
define_imm_offset_rules!(Strh, MakeStrh, STRH, RegOrZero32, "32", ScaledOffset16);

//
// ## Faillible
//
define_fallible_rules!(STRH, Strh, MakeStrh);

/// strh construction function.  See examples in the module documentation.
pub fn strh<TargetInp, TargetOut, AddrInp, AddrOut>(
    dst: TargetInp,
    addr: AddrInp,
) -> <Strh<TargetOut, AddrOut> as MakeStrh<TargetInp, AddrInp>>::Output
where
    Strh<TargetOut, AddrOut>: MakeStrh<TargetInp, AddrInp>,
{
    Strh::new(dst, addr)
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
