/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

//! `LDRH` and related commands.
//!
//! The `ldrh` function returns an instance of `Instruction` for loading a halfword into 32-bit register from memory.
//! While `LDRH` instruction has different variants with various number of arguments, the `ldrh` function has two
//! arguments: a destination register and an "address" that encapsulates the rest of arguments: the base, offsets,
//! extensions, etc. Tuples are often used for the second argument, see the pattern in the examples below.
//!
//! The funciton is overloaded for various argument types. For some of them, and `Instruction` trait instance is
//! returned, for others, a `Result` if the aguments need validation. Such arugment combinations have `.unwrap()` in
//! examples.
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
//! ldrh(W1, (X2, X3));  // LDRH W1, [X2, X3] ; n.b. a 32-bit register offset requires an extend speicifer (sxtw or uxtw):
//! ldrh(W1, (X2, ext((W3, UXTW)))); // ldrh w1, [x2, w3, uxtw]
//! ldrh(W1, (X2, ext((W3, UXTW, LdStShift::Unshifted)))); // ldrh w1, [x2, w3, uxtw #0]
//! ldrh(W1, (X2, ext((W3, UXTW, LdStShift::Shifted)))); // ldrh w1, [x2, w3, uxtw #1]
//! ldrh(W1, (X2, ext((W3, UXTW)))); // ldrh w1, [x2, w3, uxtw]
//! ```
//!
//! Please note, that `uxtw` and `sxtw` can be used only with index 32-bit register, and shift can be either omited, 0
//! or 1. The `lsl` and `sxtx` can be used only with 64-bit index registers, and while they produce different bit
//! patterns, they are equivalent; shift can be only either omited, 0 or 1. Please note that we do allow `lsl` without
//! shift.
//!
//! # `LDRH`: Register base with immediate offset
//!
//! LDRH with register base with 2-byte aligned immediate offset. The offset has 12 significant bits available.
//!
//! You may also a `u32` offset value, and a error is returned if the value doesn't fit the offset pattern.
//!
//! Examples:
//! ```ignore
//! let word_aligned_offset: UBitValue<12, 2> = ...;
//! let dword_aligned_offset: UBitValue<12, 3> = ...;
//!
//! ldrh(W1, (X2, offset as u32)).unwrap(),
//! ldrh(W1, (X2, word_aligned_offset)),
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
//! // Equavalent to the lines above:
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

use super::{HalfShift, shift_extend::*};
use super::{Inc, LdStIncOffset, ScaledOffset16};
use crate::{
    bits::BitError,
    instructions::RawInstruction,
    register::{IntoCode, RegOrSp64, RegOrZero32, RegOrZero64},
};

/// A `LDRH` instruction with a destination and an address.
pub struct Ldrh<Rt, Addr> {
    rt: Rt,
    addr: Addr,
}

impl<Rt, Addr> Ldrh<Rt, Addr> {
    pub fn rt(&self) -> &Rt {
        &self.rt
    }

    pub fn addr(&self) -> &Addr {
        &self.addr
    }
}

/// Defines possible was to construct a `LDRH` instruction.
// TODO sealed trait?
pub trait MakeLdrh<Rt, Addr> {
    /// Allows defining both faillible and infallible constructors.
    type Output;
    fn new(rt: Rt, addr: Addr) -> Self::Output;
}

//
// ## LDRH (register offset)
//
define_reg_offset_rules!(Ldrh, MakeLdrh, LDRH, RegOrZero32, "32", HalfShift);

//
// ## LDRH (immediate offset)
//
define_imm_offset_rules!(Ldrh, MakeLdrh, LDRH, RegOrZero32, "32", ScaledOffset16);

//
// ## Faillible
//
define_fallible_rules!(LDRH, Ldrh, MakeLdrh);

/// ldrh construction function.  See examples in the module documentation.
pub fn ldrh<TargetInp, TargetOut, AddrInp, AddrOut>(
    dst: TargetInp,
    addr: AddrInp,
) -> <Ldrh<TargetOut, AddrOut> as MakeLdrh<TargetInp, AddrInp>>::Output
where
    Ldrh<TargetOut, AddrOut>: MakeLdrh<TargetInp, AddrInp>,
{
    Ldrh::new(dst, addr)
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
