/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

//! `STRB` and related commands.
//!
//! The `strb` function returns an instance of `Instruction` for storing a byte from 32-bit register to memory. While
//! `STRB` instruction has different variants with various number of arguments, the `strb` function has two arguments: a
//! destination register and an "address" that encapsulates the rest of arguments: the base, offsets, extensions, etc.
//! Tuples are often used for the second argument, see the pattern in the examples below.
//!
//! The funciton is overloaded for various argument types. For some of them, and `Instruction` trait instance is
//! returned, for others, a `Result` if the aguments need validation. Such arugment combinations have `.unwrap()` in
//! examples.
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
//! strb(W1, (X2, X3));  // STRB W1, [X2, X3] ; n.b. a 32-bit register offset requires an extend speicifer (sxtw or uxtw):
//! strb(W1, (X2, ext((W3, UXTW)))); // strb w1, [x2, w3, uxtw]
//! strb(W1, (X2, ext((W3, UXTW, LdStShift::Shifted)))); // strb w1, [x2, w3, uxtw #0]
//! strb(W1, (X2, ext((W3, UXTW)))); // strb w1, [x2, w3, uxtw]
//! ```
//!
//! Please note, that `uxtw` and `sxtw` can be used only with index 32-bit register, and shift can be only absent or 0.
//! The `lsl` and `sxtx` can be used only with 64-bit index registers, and while they produce different bit patterns,
//! they are equivalent; shift can be only either absent or 0. Please note that we do allow `lsl` without shift.
//!
//! # `STRB`: Register base with immediate offset
//!
//! STRB with register base with immediate offset. The offset has 12 significant bits available.
//!
//! You may also a `u32` offset value, and a error is returned if the value doesn't fit the offset pattern.
//!
//! Examples:
//! ```ignore
//! let word_aligned_offset: UBitValue<12, 2> = ...;
//! let dword_aligned_offset: UBitValue<12, 3> = ...;
//!
//! strb(W1, (X2, offset as u32)).unwrap(),
//! strb(W1, (X2, word_aligned_offset)),
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
//! // Equavalent to the lines above:
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

use super::shift_extend::*;
use super::{ByteShift, Inc, LdStIncOffset, ScaledOffset8};
use crate::{
    bits::BitError,
    instructions::RawInstruction,
    register::{RegOrSp64, RegOrZero32, RegOrZero64, Register},
    sealed::Sealed,
};

/// A `STRB` instruction with a destination and an address.
pub struct Strb<Rt, Addr> {
    rt: Rt,
    addr: Addr,
}

impl<Rt, Addr> Strb<Rt, Addr> {
    pub fn rt(&self) -> &Rt {
        &self.rt
    }

    pub fn addr(&self) -> &Addr {
        &self.addr
    }
}

impl<Rt, Addr> Sealed for Strb<Rt, Addr> {}

/// Defines possible was to construct a `STRB` instruction.
pub trait MakeStrb<Rt, Addr>: Sealed {
    /// Allows defining both faillible and infallible constructors.
    type Output;
    fn new(rt: Rt, addr: Addr) -> Self::Output;
}

//
// ## STRB (register offset)
//
define_reg_offset_rules!(Strb, MakeStrb, STRB, RegOrZero32, "32B", ByteShift);

//
// ## STRB (immediate offset)
//
define_imm_offset_rules!(Strb, MakeStrb, STRB, RegOrZero32, "32", ScaledOffset8);

//
// ## Faillible
//
define_fallible_rules!(STRB, Strb, MakeStrb);

/// strb construction function.  See examples in the module documentation.
pub fn strb<TargetInp, TargetOut, AddrInp, AddrOut>(
    dst: TargetInp,
    addr: AddrInp,
) -> <Strb<TargetOut, AddrOut> as MakeStrb<TargetInp, AddrInp>>::Output
where
    Strb<TargetOut, AddrOut>: MakeStrb<TargetInp, AddrInp>,
{
    Strb::new(dst, addr)
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
