/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

//! `LDRB` and related commands.
//!
//! The `ldrb` function returns an instance of `Instruction` for loading a byte into 32-bit register from memory. While
//! `LDRB` instruction has different variants with various number of arguments, the `ldrb` function has two arguments: a
//! destination register and an "address" that encapsulates the rest of arguments: the base, offsets, extensions, etc.
//! Tuples are often used for the second argument, see the pattern in the examples below.
//!
//! The funciton is overloaded for various argument types. For some of them, and `Instruction` trait instance is
//! returned, for others, a `Result` if the aguments need validation. Such arugment combinations have `.unwrap()` in
//! examples.
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
//! ldrb(W1, (X2, X3));  // LDRB W1, [X2, X3] ; n.b. a 32-bit register offset requires an extend speicifer (sxtw or uxtw):
//! ldrb(W1, (X2, ext((W3, UXTW)))); // ldrb w1, [x2, w3, uxtw]
//! ldrb(W1, (X2, ext((W3, UXTW, LdStShift::Shifted)))); // ldrb w1, [x2, w3, uxtw #0]
//! ldrb(W1, (X2, ext((W3, UXTW)))); // ldrb w1, [x2, w3, uxtw]
//! ```
//!
//! Please note, that `uxtw` and `sxtw` can be used only with index 32-bit register, and shift can be only absent or 0.
//! The `lsl` and `sxtx` can be used only with 64-bit index registers, and while they produce different bit patterns,
//! they are equivalent; shift can be only either absent or 0. Please note that we do allow `lsl` without shift.
//!
//! # `LDRB`: Register base with immediate offset
//!
//! LDRB with register base with immediate offset. The offset has 12 significant bits available.
//!
//! You may also a `u32` offset value, and a error is returned if the value doesn't fit the offset pattern.
//!
//! Examples:
//! ```ignore
//! let word_aligned_offset: UBitValue<12, 2> = ...;
//! let dword_aligned_offset: UBitValue<12, 3> = ...;
//!
//! ldrb(W1, (X2, offset as u32)).unwrap(),
//! ldrb(W1, (X2, word_aligned_offset)),
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
//! // Equavalent to the lines above:
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

use super::shift_extend::*;
use super::{ByteShift, Inc, LdStIncOffset, ScaledOffset8};
use crate::{
    bits::BitError,
    instructions::RawInstruction,
    register::{IntoCode, RegOrSp64, RegOrZero32, RegOrZero64},
};

/// A `LDRB` instruction with a destination and an address.
pub struct Ldrb<Rt, Addr> {
    rt: Rt,
    addr: Addr,
}

impl<Rt, Addr> Ldrb<Rt, Addr> {
    pub fn rt(&self) -> &Rt {
        &self.rt
    }

    pub fn addr(&self) -> &Addr {
        &self.addr
    }
}

/// Defines possible was to construct a `LDRB` instruction.
// TODO sealed trait?
pub trait MakeLdrb<Rt, Addr> {
    /// Allows defining both faillible and infallible constructors.
    type Output;
    fn new(rt: Rt, addr: Addr) -> Self::Output;
}

//
// ## LDRB (register offset)
//
define_reg_offset_rules!(Ldrb, MakeLdrb, LDRB, RegOrZero32, "32B", ByteShift);

//
// ## LDRB (immediate offset)
//
define_imm_offset_rules!(Ldrb, MakeLdrb, LDRB, RegOrZero32, "32", ScaledOffset8);

//
// ## Faillible
//
define_fallible_rules!(LDRB, Ldrb, MakeLdrb);

/// ldrb construction function.  See examples in the module documentation.
pub fn ldrb<TargetInp, TargetOut, AddrInp, AddrOut>(
    dst: TargetInp,
    addr: AddrInp,
) -> <Ldrb<TargetOut, AddrOut> as MakeLdrb<TargetInp, AddrInp>>::Output
where
    Ldrb<TargetOut, AddrOut>: MakeLdrb<TargetInp, AddrInp>,
{
    Ldrb::new(dst, addr)
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
