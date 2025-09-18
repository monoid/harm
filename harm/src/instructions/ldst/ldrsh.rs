/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

//! `LDRSH` and related commands.
//!
//! The `ldrsh` function returns an instance of `Instruction` for loading a byte into a 32-bit or 64-bit register from
//! memory. While `LDRSH` instruction has different variants with various number of arguments, the `ldrsh` function has
//! two arguments: a destination register and an "address" that encapsulates the rest of arguments: the base, offsets,
//! extensions, etc. Tuples are often used for the second argument, see the pattern in the examples below.
//!
//! The funciton is overloaded for various argument types. For some of them, and `Instruction` trait instance is
//! returned, for others, a `Result` if the aguments need validation. Such arugment combinations have `.unwrap()` in
//! examples.
//!
//! # `LDRSH`: Register base with register offset
//!
//! # Examples:
//! ```
//! # use harm::instructions::ldst::{ldrsh, ext, LdStExtendOption32, LdStShift};
//! use harm::register::Reg32::*;
//! use harm::register::Reg64::*;
//! use LdStExtendOption32::*;
//!
//! ldrsh(W1, X2);        // LDRSH W1, [X2]
//! ldrsh(W1, (X2,));     // LDRSH W1, [X2]
//! ldrsh(W1, (X2, X3));  // LDRSH W1, [X2, X3] ; n.b. a 32-bit register offset requires an extend speicifer (sxtw or uxtw):
//! ldrsh(W1, (X2, ext((W3, UXTW)))); // ldrsh w1, [x2, w3, uxtw]
//! ldrsh(W1, (X2, ext((W3, UXTW, LdStShift::Shifted)))); // ldrsh w1, [x2, w3, uxtw #0]
//! ldrsh(W1, (X2, ext((W3, UXTW)))); // ldrsh w1, [x2, w3, uxtw]
//! ldrsh(X1, (X2, ext((W3, UXTW)))); // ldrsh x1, [x2, w3, uxtw]
//! ldrsh(X1, (X2, ext((W3, UXTW, LdStShift::Shifted)))); // ldrsh x1, [x2, w3, uxtw #0]
//! ldrsh(X1, (X2, ext((W3, UXTW, 0)))).unwrap(); // ldrsh x1, [x2, w3, uxtw #0]
//! ```
//!
//! Please note, that `uxtw` and `sxtw` can be used only with 32-bit register, and shift can be either omited
//! (unshifted) or 0 (shifted), which is the same for this instruction. The `lsl` and `sxtx` can be used only with
//! 64-bit registers, and while they produce different bit patterns, they are equivalent; shift can be either omited
//! (unshifted) or 0 (shifted), which is the same for this instruction.
//!
//! # `LDRSH`: Register base with immediate offset
//!
//! LDRSH with register base with immediate offset has an unsigned offset. The offset has 12
//! significan bits available.
//!
//! You may also a `u32` offset value, and a error is returned if the value doesn't fit the offset pattern.
//!
//! Examples:
//! ```ignore
//! let word_aligned_offset: UBitValue<12, 1> = ...;
//! let dword_aligned_offset: UBitValue<12, 1> = ...;
//!
//! ldrsh(W1, (X2, offset as u32)).unwrap(),
//! ldrsh(X1, (X2, offset as u32)).unwrap(),
//! ldrsh(W1, (X2, word_aligned_offset)),
//! ldrsh(X1, (X2, dword_aligned_offset)),
//! ```
//!
//! Pre-increment and post-increment variants have the following syntax:
//! ```
//! # use harm::instructions::ldst::{ldrsh, inc, preinc, postinc, LdStIncOffset};
//! use harm::register::Reg32::*;
//! use harm::register::Reg64::*;
//! let offset = LdStIncOffset::new(4).unwrap();
//! ldrsh(W1, (inc(offset), X2));       // preincrement, LDRSH W1, [X2, #4]!
//! ldrsh(W1, (X2, inc(offset)));       // postincrement, LDRSH W1, [X2], #4
//! // Equavalent to the lines above:
//! ldrsh(W1, preinc(X2, offset));      // preincrement, LDRSH W1, [X2, #4]!
//! ldrsh(W1, postinc(X2, offset));     // postincrement, LDRSH W1, [X2], #4
//! // Fallible variants:
//! ldrsh(W1, (inc(4), X2)).unwrap();   // preincrement, LDRSH W1, [X2, #4]!
//! ldrsh(W1, postinc(X2, 4)).unwrap(); // postincrement, LDRSH W1, [X2], #4
//! ```
//!

use aarchmrs_instructions::A64::ldst::{
    ldst_immpost::{
        LDRSH_32_ldst_immpost::LDRSH_32_ldst_immpost, LDRSH_64_ldst_immpost::LDRSH_64_ldst_immpost,
    },
    ldst_immpre::{
        LDRSH_32_ldst_immpre::LDRSH_32_ldst_immpre, LDRSH_64_ldst_immpre::LDRSH_64_ldst_immpre,
    },
    ldst_pos::{LDRSH_32_ldst_pos::LDRSH_32_ldst_pos, LDRSH_64_ldst_pos::LDRSH_64_ldst_pos},
    ldst_regoff::{
        LDRSH_32_ldst_regoff::LDRSH_32_ldst_regoff, LDRSH_64_ldst_regoff::LDRSH_64_ldst_regoff,
    },
};

use super::shift_extend::*;
use super::{HalfShift, Inc, LdStIncOffset, ScaledOffset16};
use crate::{
    bits::BitError,
    instructions::RawInstruction,
    register::{RegOrSp64, RegOrZero32, RegOrZero64, Register},
    sealed::Sealed,
};

/// A `LDRSH` instruction with a destination and an address.
pub struct Ldrsh<Rt, Addr> {
    rt: Rt,
    addr: Addr,
}

impl<Rt, Addr> Ldrsh<Rt, Addr> {
    pub fn rt(&self) -> &Rt {
        &self.rt
    }

    pub fn addr(&self) -> &Addr {
        &self.addr
    }
}

impl<Rt, Addr> Sealed for Ldrsh<Rt, Addr> {}

/// Defines possible was to construct a `Ldrsh` instruction.
pub trait MakeLdrsh<Rt, Addr>: Sealed {
    /// Allows defining both faillible and infallible constructors.
    type Output;
    fn new(rt: Rt, addr: Addr) -> Self::Output;
}
//
// ## LDRSH (register offset)
//
define_reg_offset_rules!(Ldrsh, MakeLdrsh, LDRSH, RegOrZero64, 64, HalfShift);
define_reg_offset_rules!(Ldrsh, MakeLdrsh, LDRSH, RegOrZero32, 32, HalfShift);

//
// ## LDRSH (immediate offset)
//
define_imm_offset_rules!(Ldrsh, MakeLdrsh, LDRSH, RegOrZero64, 64, ScaledOffset16);
define_imm_offset_rules!(Ldrsh, MakeLdrsh, LDRSH, RegOrZero32, 32, ScaledOffset16);

//
// ## Faillible
//
define_fallible_rules!(LDRSH, Ldrsh, MakeLdrsh);

/// ldrsh construction function.  See examples in the module documentation.
pub fn ldrsh<TargetInp, TargetOut, AddrInp, AddrOut>(
    dst: TargetInp,
    addr: AddrInp,
) -> <Ldrsh<TargetOut, AddrOut> as MakeLdrsh<TargetInp, AddrInp>>::Output
where
    Ldrsh<TargetOut, AddrOut>: MakeLdrsh<TargetInp, AddrInp>,
{
    Ldrsh::new(dst, addr)
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

    const LDRSH_REG_EXT_DB: &str = "
78e34902	ldrsh w2, [x8, w3, uxtw #0]
78e34902	ldrsh w2, [x8, w3, uxtw]
78e36902	ldrsh w2, [x8, x3, lsl #0]
78e36902	ldrsh w2, [x8, x3]
78e36be2	ldrsh w2, [sp, x3]
78e3c902	ldrsh w2, [x8, w3, sxtw #0]
78e3c902	ldrsh w2, [x8, w3, sxtw]
78e3e902	ldrsh w2, [x8, x3, sxtx #0]
78e3e902	ldrsh w2, [x8, x3, sxtx]
78ff6902	ldrsh w2, [x8, xzr]
78ff6be2	ldrsh w2, [sp, xzr]
78a34902	ldrsh x2, [x8, w3, uxtw #0]
78a34902	ldrsh x2, [x8, w3, uxtw]
78a36902	ldrsh x2, [x8, x3, lsl #0]
78a36902	ldrsh x2, [x8, x3]
78a36be2	ldrsh x2, [sp, x3]
78a3c902	ldrsh x2, [x8, w3, sxtw #0]
78a3c902	ldrsh x2, [x8, w3, sxtw]
78a3e902	ldrsh x2, [x8, x3, sxtx #0]
78a3e902	ldrsh x2, [x8, x3, sxtx]
78a9491f	ldrsh xzr, [x8, w9, uxtw]
78a9691f	ldrsh xzr, [x8, x9]
78a9691f	ldrsh xzr, [x8, x9]
78a9c91f	ldrsh xzr, [x8, w9, sxtw]
78bf4902	ldrsh x2, [x8, wzr, uxtw]
78bf6902	ldrsh x2, [x8, xzr]
78bf6be2	ldrsh x2, [sp, xzr]
78bfc902	ldrsh x2, [x8, wzr, sxtw]
78bfe902	ldrsh x2, [x8, xzr, sxtx]
";

    // 'ldrsh (w2|x2), [(x8|sp), #0x190]'
    const LDRSH_SCALED_IMM_DB: &str = "
79c32102	ldrsh w2, [x8, #0x190]
79c3211f	ldrsh wzr, [x8, #0x190]
79c323e2	ldrsh w2, [sp, #0x190]
79832102	ldrsh x2, [x8, #0x190]
798323e2	ldrsh x2, [sp, #0x190]
798323ff	ldrsh xzr, [sp, #0x190]
79800102	ldrsh x2, [x8]
";

    const LDRSH_PRE_POST_INC_DB: &str = "
78c2a441	ldrsh w1, [x2], #0x2a
78c2a45f	ldrsh wzr, [x2], #0x2a
78c2a7e1	ldrsh w1, [sp], #0x2a
78c2ac41	ldrsh w1, [x2, #0x2a]!
78c2ac5f	ldrsh wzr, [x2, #0x2a]!
78c2afe1	ldrsh w1, [sp, #0x2a]!
78dd6441	ldrsh w1, [x2], #-0x2a
78dd67e1	ldrsh w1, [sp], #-0x2a
78dd6c41	ldrsh w1, [x2, #-0x2a]!
78dd6fe1	ldrsh w1, [sp, #-0x2a]!
7882a441	ldrsh x1, [x2], #0x2a
7882a45f	ldrsh xzr, [x2], #0x2a
7882a7e1	ldrsh x1, [sp], #0x2a
7882ac41	ldrsh x1, [x2, #0x2a]!
7882ac5f	ldrsh xzr, [x2, #0x2a]!
7882afe1	ldrsh x1, [sp, #0x2a]!
789d6441	ldrsh x1, [x2], #-0x2a
789d67e1	ldrsh x1, [sp], #-0x2a
789d6c41	ldrsh x1, [x2, #-0x2a]!
789d6fe1	ldrsh x1, [sp, #-0x2a]!
";

    test_cases! {
        LDRSH_REG_EXT_DB, untested_ldrsh_reg_ext_db;
        test_ldrsh_r64_r64_r32_sxtw, ldrsh(X2, (X8, ext((W3, SXTW)))), "ldrsh x2, [x8, w3, sxtw]";
        test_ldrsh_r64_r64_r32_uxtw, ldrsh(X2, (X8, ext((W3, UXTW)))), "ldrsh x2, [x8, w3, uxtw]";
        test_ldrsh_r32_r64_r32_sxtw, ldrsh(W2, (X8, ext((W3, SXTW)))), "ldrsh w2, [x8, w3, sxtw]";
        test_ldrsh_r32_r64_r32_uxtw, ldrsh(W2, (X8, ext((W3, UXTW)))), "ldrsh w2, [x8, w3, uxtw]";
        test_ldrsh_r32_r64_r64_sxtx, ldrsh(W2, (X8, ext((X3, SXTX)))), "ldrsh w2, [x8, x3, sxtx]";
        test_ldrsh_r64_r64_r64_sxtx, ldrsh(X2, (X8, ext((X3, SXTX)))), "ldrsh x2, [x8, x3, sxtx]";
        test_ldrsh_r64_r64_xzr_sxtx, ldrsh(X2, (X8, ext((XZR, SXTX)))), "ldrsh x2, [x8, xzr, sxtx]";
        test_ldrsh_r64_r64_wzr_sxtw, ldrsh(X2, (X8, ext((WZR, SXTW)))), "ldrsh x2, [x8, wzr, sxtw]";
        test_ldrsh_r64_r64_wzr_uxtx, ldrsh(X2, (X8, ext((WZR, UXTW)))), "ldrsh x2, [x8, wzr, uxtw]";
        test_ldrsh_r32_r64_r32_sxtw_0, ldrsh(W2, (X8, ext((W3, SXTW, 0)))).unwrap(), "ldrsh w2, [x8, w3, sxtw #0]";
        test_ldrsh_r32_r64_r32_uxtw_0, ldrsh(W2, (X8, ext((W3, UXTW, 0)))).unwrap(), "ldrsh w2, [x8, w3, uxtw #0]";
        test_ldrsh_r64_r64_r32_sxtw_0, ldrsh(X2, (X8, ext((W3, SXTW, 0)))).unwrap(), "ldrsh x2, [x8, w3, sxtw #0]";
        test_ldrsh_r64_r64_r32_uxtw_0, ldrsh(X2, (X8, ext((W3, UXTW, 0)))).unwrap(), "ldrsh x2, [x8, w3, uxtw #0]";
        test_ldrsh_r32_r64_r64_lsl_0, ldrsh(W2, (X8, ext((X3, LSL, 0)))).unwrap(), "ldrsh w2, [x8, x3, lsl #0]";
        test_ldrsh_r32_r64_r64_sxtx_0, ldrsh(W2, (X8, ext((X3, SXTX, 0)))).unwrap(), "ldrsh w2, [x8, x3, sxtx #0]";
        test_ldrsh_r64_r64_r64_lsl_0, ldrsh(X2, (X8, ext((X3, LSL, 0)))).unwrap(), "ldrsh x2, [x8, x3, lsl #0]";
        test_ldrsh_r64_r64_r32_sxtx_0, ldrsh(X2, (X8, ext((X3, SXTX, 0)))).unwrap(), "ldrsh x2, [x8, x3, sxtx #0]";
        test_ldrsh_r32_r64_r64, ldrsh(W2, (X8, X3)), "ldrsh w2, [x8, x3]";
        test_ldrsh_r32_rsp_r64, ldrsh(W2, (SP, X3)), "ldrsh w2, [sp, x3]";
        test_ldrsh_r64_r64_r64, ldrsh(X2, (X8, X3)), "ldrsh x2, [x8, x3]";
        test_ldrsh_r64_rsp_r64, ldrsh(X2, (SP, X3)), "ldrsh x2, [sp, x3]";
        test_ldrsh_r32_r64_xzr, ldrsh(W2, (X8, XZR)), "ldrsh w2, [x8, xzr]";
        test_ldrsh_r32_rsp_xzr, ldrsh(W2, (SP, XZR)), "ldrsh w2, [sp, xzr]";
        test_ldrsh_r64_r64_xzr, ldrsh(X2, (X8, XZR)), "ldrsh x2, [x8, xzr]";
        test_ldrsh_r64_rsp_xzr, ldrsh(X2, (SP, XZR)), "ldrsh x2, [sp, xzr]";
        test_ldrsh_xzr_r64_r64, ldrsh(XZR, (X8, X9)), "ldrsh xzr, [x8, x9]";
        test_ldrsh_wzr_r64_r64, ldrsh(XZR, (X8, X9)), "ldrsh xzr, [x8, x9]";
        test_ldrsh_xzr_r64_r32_sxtw, ldrsh(XZR, (X8, ext((W9, SXTW)))), "ldrsh xzr, [x8, w9, sxtw]";
        test_ldrsh_wzr_r64_r32_uxtw, ldrsh(XZR, (X8, ext((W9, UXTW)))), "ldrsh xzr, [x8, w9, uxtw]";
    }

    test_cases! {
        LDRSH_SCALED_IMM_DB, untested_ldrsh_scaled_imm;
        test_ldrsh_r32_r64_scaled_imm, ldrsh(W2, (X8, UBitValue::<12, 1>::new(0x190).unwrap())), "ldrsh w2, [x8, #0x190]";
        test_ldrsh_r32_sp_scaled_imm, ldrsh(W2, (SP, UBitValue::<12, 1>::new(0x190).unwrap())), "ldrsh w2, [sp, #0x190]";
        test_ldrsh_r64_r64_scaled_imm, ldrsh(X2, (X8, UBitValue::<12, 1>::new(0x190).unwrap())), "ldrsh x2, [x8, #0x190]";
        test_ldrsh_r64_sp_scaled_imm, ldrsh(X2, (SP, UBitValue::<12, 1>::new(0x190).unwrap())), "ldrsh x2, [sp, #0x190]";
        test_ldrsh_r32_r64_scaled_imm2, ldrsh(W2, (X8, 0x190u32)).unwrap(), "ldrsh w2, [x8, #0x190]";
        test_ldrsh_r64_sp_scaled_imm2, ldrsh(X2, (SP, 0x190u32)).unwrap(), "ldrsh x2, [sp, #0x190]";
        test_ldrsh_wzr_r64_scaled_imm2, ldrsh(WZR, (X8, 0x190u32)).unwrap(), "ldrsh wzr, [x8, #0x190]";
        test_ldrsh_xzr_sp_scaled_imm2, ldrsh(XZR, (SP, 0x190u32)).unwrap(), "ldrsh xzr, [sp, #0x190]";
        test_ldrsh_r32_r64_scaled_imm3, ldrsh(W2, (X8, 0x190i32)).unwrap(), "ldrsh w2, [x8, #0x190]";
        test_ldrsh_r64_sp_scaled_imm3, ldrsh(X2, (SP, 0x190i32)).unwrap(), "ldrsh x2, [sp, #0x190]";
        test_ldrsh_wzr_r64_scaled_imm3, ldrsh(WZR, (X8, 0x190i32)).unwrap(), "ldrsh wzr, [x8, #0x190]";
        test_ldrsh_xzr_sp_scaled_imm3, ldrsh(XZR, (SP, 0x190i32)).unwrap(), "ldrsh xzr, [sp, #0x190]";
        test_ldrsh_r64_r64_simple, ldrsh(X2, (X8,)), "ldrsh x2, [x8]";
    }

    test_cases! {
        LDRSH_PRE_POST_INC_DB, untested_ldrsh_pre_post_inc;
        test_ldrsh_r32_r64_preinc, ldrsh(W1, preinc(X2, 0x2a)).unwrap(), "ldrsh w1, [x2, #0x2a]!";
        test_ldrsh_r32_r64_postinc, ldrsh(W1, postinc(X2, 0x2a)).unwrap(), "ldrsh w1, [x2], #0x2a";
        test_ldrsh_r64_r64_preinc, ldrsh(X1, preinc(X2, 0x2a)).unwrap(), "ldrsh x1, [x2, #0x2a]!";
        test_ldrsh_r64_r64_postinc, ldrsh(X1, postinc(X2, 0x2a)).unwrap(), "ldrsh x1, [x2], #0x2a";
        test_ldrsh_r32_sp_preinc, ldrsh(W1, preinc(SP, 0x2a)).unwrap(), "ldrsh w1, [sp, #0x2a]!";
        test_ldrsh_r32_sp_postinc, ldrsh(W1, postinc(SP, 0x2a)).unwrap(), "ldrsh w1, [sp], #0x2a";
        test_ldrsh_r64_sp_preinc, ldrsh(X1, preinc(SP, 0x2a)).unwrap(), "ldrsh x1, [sp, #0x2a]!";
        test_ldrsh_r64_sp_postinc, ldrsh(X1, postinc(SP, 0x2a)).unwrap(), "ldrsh x1, [sp], #0x2a";
        test_ldrsh_r32_r64_preinc_neg, ldrsh(W1, preinc(X2, -0x2a)).unwrap(), "ldrsh w1, [x2, #-0x2a]!";
        test_ldrsh_r32_r64_postinc_neg, ldrsh(W1, postinc(X2, -0x2a)).unwrap(), "ldrsh w1, [x2], #-0x2a";
        test_ldrsh_r64_r64_preinc_neg, ldrsh(X1, preinc(X2, -0x2a)).unwrap(), "ldrsh x1, [x2, #-0x2a]!";
        test_ldrsh_r64_r64_postinc_neg, ldrsh(X1, postinc(X2, -0x2a)).unwrap(), "ldrsh x1, [x2], #-0x2a";
        test_ldrsh_r32_sp_preinc_neg, ldrsh(W1, preinc(SP, -0x2a)).unwrap(), "ldrsh w1, [sp, #-0x2a]!";
        test_ldrsh_r32_sp_postinc_neg, ldrsh(W1, postinc(SP, -0x2a)).unwrap(), "ldrsh w1, [sp], #-0x2a";
        test_ldrsh_r64_sp_preinc_neg, ldrsh(X1, preinc(SP, -0x2a)).unwrap(), "ldrsh x1, [sp, #-0x2a]!";
        test_ldrsh_r64_sp_postinc_neg, ldrsh(X1, postinc(SP, -0x2a)).unwrap(), "ldrsh x1, [sp], #-0x2a";
        test_ldrsh_r32_sp_preinc2, ldrsh(W1, preinc(SP, LdStIncOffset::new(0x2a).unwrap())), "ldrsh w1, [sp, #0x2a]!";
        test_ldrsh_r64_r64_preinc_neg2, ldrsh(X1, preinc(X2, LdStIncOffset::new(-0x2a).unwrap())), "ldrsh x1, [x2, #-0x2a]!";
        test_ldrsh_r32_r64_pre_inc, ldrsh(W1, (inc(0x2a), X2)).unwrap(), "ldrsh w1, [x2, #0x2a]!";
        test_ldrsh_r32_r64_post_inc, ldrsh(W1, (X2, inc(0x2a))).unwrap(), "ldrsh w1, [x2], #0x2a";
        test_ldrsh_r64_r64_pre_inc, ldrsh(X1, (inc(0x2a), X2)).unwrap(), "ldrsh x1, [x2, #0x2a]!";
        test_ldrsh_r64_r64_post_inc, ldrsh(X1, (X2, inc(0x2a))).unwrap(), "ldrsh x1, [x2], #0x2a";
        test_ldrsh_r32_sp_pre_inc, ldrsh(W1, (inc(0x2a), SP)).unwrap(), "ldrsh w1, [sp, #0x2a]!";
        test_ldrsh_r32_sp_post_inc, ldrsh(W1, (SP, inc(0x2a))).unwrap(), "ldrsh w1, [sp], #0x2a";
        test_ldrsh_r64_sp_pre_inc, ldrsh(X1, (inc(0x2a), SP)).unwrap(), "ldrsh x1, [sp, #0x2a]!";
        test_ldrsh_r64_sp_post_inc, ldrsh(X1, (SP, inc(0x2a))).unwrap(), "ldrsh x1, [sp], #0x2a";
        test_ldrsh_r32_r64_pre_inc_neg, ldrsh(W1, (inc(-0x2a), X2)).unwrap(), "ldrsh w1, [x2, #-0x2a]!";
        test_ldrsh_r32_r64_post_inc_neg, ldrsh(W1, (X2, inc(-0x2a))).unwrap(), "ldrsh w1, [x2], #-0x2a";
        test_ldrsh_r64_r64_pre_inc_neg, ldrsh(X1, (inc(-0x2a), X2)).unwrap(), "ldrsh x1, [x2, #-0x2a]!";
        test_ldrsh_r64_r64_post_inc_neg, ldrsh(X1, (X2, inc(-0x2a))).unwrap(), "ldrsh x1, [x2], #-0x2a";
        test_ldrsh_r32_sp_pre_inc_neg, ldrsh(W1, (inc(-0x2a), SP)).unwrap(), "ldrsh w1, [sp, #-0x2a]!";
        test_ldrsh_r32_sp_post_inc_neg, ldrsh(W1, (SP, inc(-0x2a))).unwrap(), "ldrsh w1, [sp], #-0x2a";
        test_ldrsh_r64_sp_pre_inc_neg, ldrsh(X1, (inc(-0x2a), SP)).unwrap(), "ldrsh x1, [sp, #-0x2a]!";
        test_ldrsh_r64_sp_post_inc_neg, ldrsh(X1, (SP, inc(-0x2a))).unwrap(), "ldrsh x1, [sp], #-0x2a";
        test_ldrsh_r32_sp_pre_inc2, ldrsh(W1, (inc(LdStIncOffset::new(0x2a).unwrap()), SP)), "ldrsh w1, [sp, #0x2a]!";
        test_ldrsh_r64_r64_pre_inc_neg2, ldrsh(X1, (inc(LdStIncOffset::new(-0x2a).unwrap()), X2)), "ldrsh x1, [x2, #-0x2a]!";
        test_ldrsh_xzr_r64_pre_inc, ldrsh(XZR, (inc(0x2a), X2)).unwrap(), "ldrsh xzr, [x2, #0x2a]!";
        test_ldrsh_xzr_r64_post_inc, ldrsh(XZR, (X2, inc(0x2a))).unwrap(), "ldrsh xzr, [x2], #0x2a";
        test_ldrsh_wzr_r64_pre_inc, ldrsh(WZR, (inc(0x2a), X2)).unwrap(), "ldrsh wzr, [x2, #0x2a]!";
        test_ldrsh_wzr_r64_post_inc, ldrsh(WZR, (X2, inc(0x2a))).unwrap(), "ldrsh wzr, [x2], #0x2a";
    }
}
