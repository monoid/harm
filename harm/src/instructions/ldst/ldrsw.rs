/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

//! `LDRSW` and related commands.
//!
//! The `ldrsw` function returns an instance of `Instruction` for loading a 32-bit or 64-bit register from memory. While
//! `LDRSW` instruction has different variants with various number of arguments, the `ldrsw` function has two arguments: a
//! destination register and an "address" that encapsulates the rest of arguments: the base, offsets, extensions, etc.
//! Tuples are often used for the second argument, see the pattern in the examples below.
//!
//! The funciton is overloaded for various argument types. For some of them, and `Instruction` trait instance is
//! returned, for others, a `Result` if the aguments need validation. Such arugment combinations have `.unwrap()` in
//! examples.
//!
//! # `LDRSW`: Register base with register offset
//!
//! # Examples:
//! ```
//! # use harm::instructions::ldst::{ldrsw, ext, LdStExtendOption32, LdStShift};
//! use harm_types::A64::register::Reg32::*;
//! use harm_types::A64::register::Reg64::*;
//! use LdStExtendOption32::*;
//!
//! ldrsw(X1, X2);        // LDRSW W1, [X2]
//! ldrsw(X1, (X2,));     // LDRSW W1, [X2]
//! ldrsw(X1, (X2, X3));  // LDRSW W1, [X2, X3] ; n.b. a 32-bit register index requires an extend speicifer (sxtw or uxtw):
//! ldrsw(X1, (X2, ext((W3, UXTW)))); // ldrsw x1, [x2, w3, uxtw]
//! ldrsw(X1, (X2, ext((W3, UXTW, LdStShift::Shifted)))); // ldrsw x1, [x2, w3, uxtw #2]
//! ```
//!
//! Please note, that `uxtw` and `sxtw` can be used only with 32-bit register, and shift can be only either 0
//! (unshifted) or 2 (shifted). The `lsl` and `sxtx` can be used only with 64-bit registers, and while they produce
//! different bit patterns, they are equivalent; shift can be only either 0 (unshifted) or 3 (shifted).
//!
//! # `LDRSW`: Register base with immediate offset
//!
//! LDRSW with register base with immediate offset has an unsigned offset aligned by destination register size.
//! For example, if the desination register is `W1`, the offset has be aligned by 4 bytes (two lower bits are clear),
//! and if it is `X1`, the offset has to be aligned by 8 bytes (three lower bits are clear).  The offset has 12
//! significan bits available.
//!
//! You may also a `u32` offset value, and a error is returned if the value doesn't fit the offset pattern.
//!
//! Examples:
//! ```ignore
//! let word_aligned_offset: UBitValue<12, 2> = ...;
//! let dword_aligned_offset: UBitValue<12, 3> = ...;
//!
//! ldrsw(W1, (X2, offset as u32)).unwrap(),
//! ldrsw(X1, (X2, offset as u32)).unwrap(),
//! ldrsw(W1, (X2, word_aligned_offset)),
//! ldrsw(X1, (X2, dword_aligned_offset)),
//! ```
//!
//! Pre-increment and post-increment variants have the following syntax:
//! ```
//! # use harm::instructions::ldst::{ldrsw, inc, preinc, postinc, LdStIncOffset};
//! use harm_types::A64::register::Reg32::*;
//! use harm_types::A64::register::Reg64::*;
//! let offset = LdStIncOffset::new(4).unwrap();
//! ldrsw(X1, (inc(offset), X2));       // preincrement, LDRSW X1, [X2, #4]!
//! ldrsw(X1, (X2, inc(offset)));       // postincrement, LDRSW X1, [X2], #4
//! // Equavalent to the lines above:
//! ldrsw(X1, preinc(X2, offset));      // preincrement, LDRSW X1, [X2, #4]!
//! ldrsw(X1, postinc(X2, offset));     // postincrement, LDRSW X1, [X2], #4
//! // Fallible variants:
//! ldrsw(X1, (inc(4), X2)).unwrap();   // preincrement, LDRSW X1, [X2, #4]!
//! ldrsw(X1, postinc(X2, 4)).unwrap(); // postincrement, LDRSW X1, [X2], #4
//! ```
//!
//! # `LDRSW`: PC base with immediate offset
//!
//! An immediate signed offset of `SBitValue<12, 2>` is added to `PC`, i.e. the offset is relative to the instruction's
//! address.  The 2-bit alignment is the same for both 32-bit and 64-bit variants.
//!
//! ```ignore
//! let bit_offset: PcOffset = ...;
//! let raw_offset: i32 = ...;
//!
//! ldrsw(W1, pc(bit_offset)),
//! ldrsw(W1, (Pc, bit_offset)),
//! ldrsw(W1, (Pc, raw_offset)).unwrap(),
//! ```
//!
//! # `LDUR`: Register base with unaligned immediate offset
//!
//! `LDUR` is similar to `LDRSW` with register base with immediate offset, but its offset is **signed** 9 bit wide. For
//! convenience, a `SBitValue<9>` value can be used with `ldrsw` as well, and both signed and unsigned raw values can be
//! encoded as LDUR if they fit into the range and cannot be encoded with `LDRSW` with immediate offset (TODO it makes the
//! code little bit more complex, unless we use an enum).
//!
//! ```ignore
//! let bit_offset: SBitOffset<9> = ...;
//! let raw_offset: i32 = ...;
//!
//! ldrsw(W1, (X2, bit_offset)),
//! ldur(W1, (X2, bit_offset)),
//! ldrsw(X1, (X2, bit_offset)),
//! ldur(X1, (X2, bit_offset)),
//! ldrsw(W1, (X2, raw_offset)).unwap(),
//! ldur(W1, (X2, raw_offset)).unwap(),
//! ldrsw(X1, (X2, raw_offset)).unwap(),
//! ldur(X1, (X2, raw_offset)).unwap(),
//! ```

use aarchmrs_instructions::A64::ldst::{
    ldst_immpost::LDRSW_64_ldst_immpost::LDRSW_64_ldst_immpost,
    ldst_immpre::LDRSW_64_ldst_immpre::LDRSW_64_ldst_immpre,
    ldst_pos::LDRSW_64_ldst_pos::LDRSW_64_ldst_pos,
    ldst_regoff::LDRSW_64_ldst_regoff::LDRSW_64_ldst_regoff,
    loadlit::LDRSW_64_loadlit::LDRSW_64_loadlit,
};

use super::shift_extend::*;
use super::{Inc, LdStIncOffset, ScaledOffset32};
use crate::bits::BitError;
use crate::instructions::RawInstruction;
use crate::register::{IntoReg, RegOrSp64, RegOrZero32, RegOrZero64, Register};
use crate::sealed::Sealed;

/// A `ldrsw` instruction with a destination and an address.
pub struct Ldrsw<Rt, Addr> {
    rt: Rt,
    addr: Addr,
}

impl<Rt, Addr> Ldrsw<Rt, Addr> {
    pub fn rt(&self) -> &Rt {
        &self.rt
    }

    pub fn addr(&self) -> &Addr {
        &self.addr
    }
}

impl<Rt, Addr> Sealed for Ldrsw<Rt, Addr> {}

/// Defines possible was to construct a `ldsrw` instruction.
pub trait MakeLdrsw<Rt, Addr>: Sealed {
    /// Allows defining both faillible and infallible constructors.
    type Output;
    fn new(rt: Rt, addr: Addr) -> Self::Output;
}
//
// ## LDRSW (register offset)
//
define_reg_offset_rules!(Ldrsw, MakeLdrsw, LDRSW, RegOrZero64, 64, RegOrZero32);

//
// ## LDRSW (immediate offset)
//
define_imm_offset_rules!(Ldrsw, MakeLdrsw, LDRSW, RegOrZero64, 64, ScaledOffset32);

//
// ## LDRSW (PC-relative literal)
//
define_pc_offset_rules!(Ldrsw, MakeLdrsw, LDRSW, RegOrZero64, 64);

//
// ## Faillible
//
define_fallible_rules!(LDRSW, Ldrsw, MakeLdrsw);

/// ldrsw construction function.  See examples in the module documentation.
pub fn ldrsw<TargetInp, TargetOut, AddrInp, AddrOut>(
    dst: TargetInp,
    addr: AddrInp,
) -> <Ldrsw<TargetOut, AddrOut> as MakeLdrsw<TargetInp, AddrInp>>::Output
where
    Ldrsw<TargetOut, AddrOut>: MakeLdrsw<TargetInp, AddrInp>,
{
    Ldrsw::new(dst, addr)
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
        instructions::ldst::{LdStPcOffset, Pc, inc, postinc, preinc},
    };
    use LdStExtendOption32::*;
    use LdStExtendOption64::*;
    use RegOrSp64::SP;
    use RegOrZero32::WZR;
    use RegOrZero64::XZR;

    const LDRSW_REG_EXT_DB: &str = "
b8a34902	ldrsw x2, [x8, w3, uxtw #0]
b8a34902	ldrsw x2, [x8, w3, uxtw]
b8a35902	ldrsw x2, [x8, w3, uxtw #2]
b8a36902	ldrsw x2, [x8, x3, lsl #0]
b8a36902	ldrsw x2, [x8, x3]
b8a36be2	ldrsw x2, [sp, x3]
b8a37902	ldrsw x2, [x8, x3, lsl #2]
b8a3c902	ldrsw x2, [x8, w3, sxtw #0]
b8a3c902	ldrsw x2, [x8, w3, sxtw]
b8a3d902	ldrsw x2, [x8, w3, sxtw #2]
b8a3e902	ldrsw x2, [x8, x3, sxtx #0]
b8a3e902	ldrsw x2, [x8, x3, sxtx]
b8a3f902	ldrsw x2, [x8, x3, sxtx #2]
b8a9491f	ldrsw xzr, [x8, w9, uxtw]
b8a9691f	ldrsw xzr, [x8, x9]
b8a9691f	ldrsw xzr, [x8, x9]
b8a9c91f	ldrsw xzr, [x8, w9, sxtw]
b8bf4902	ldrsw x2, [x8, wzr, uxtw]
b8bf6902	ldrsw x2, [x8, xzr]
b8bf6be2	ldrsw x2, [sp, xzr]
b8bf7902	ldrsw x2, [x8, xzr, lsl #2]
b8bfc902	ldrsw x2, [x8, wzr, sxtw]
b8bfe902	ldrsw x2, [x8, xzr, sxtx]
";

    // 'ldrsw (w2|x2), [(x8|sp), #0x190]'
    const LDRSW_SCALED_IMM_DB: &str = "
b9819102	ldrsw x2, [x8, #0x190]
b98193e2	ldrsw x2, [sp, #0x190]
b98193ff	ldrsw xzr, [sp, #0x190]
b9800102	ldrsw x2, [x8]
";

    // NB: not a real syntax.
    const LDRSW_PC_RELATIVE_DB: &str = "
98000162	ldrsw x2, [pc, #44]
98fffea2	ldrsw x2, [pc, #-44]
";

    const LDRSW_PRE_POST_INC_DB: &str = "
b882a441	ldrsw x1, [x2], #0x2a
b882a45f	ldrsw xzr, [x2], #0x2a
b882a7e1	ldrsw x1, [sp], #0x2a
b882ac41	ldrsw x1, [x2, #0x2a]!
b882ac5f	ldrsw xzr, [x2, #0x2a]!
b882afe1	ldrsw x1, [sp, #0x2a]!
b89d6441	ldrsw x1, [x2], #-0x2a
b89d67e1	ldrsw x1, [sp], #-0x2a
b89d6c41	ldrsw x1, [x2, #-0x2a]!
b89d6fe1	ldrsw x1, [sp, #-0x2a]!
";
    test_cases! {
        LDRSW_REG_EXT_DB, untested_ldrsw_reg_ext_db;
        test_ldrsw_r64_r64_r32_sxtw, ldrsw(X2, (X8, ext((W3, SXTW)))), "ldrsw x2, [x8, w3, sxtw]";
        test_ldrsw_r64_r64_r32_uxtw, ldrsw(X2, (X8, ext((W3, UXTW)))), "ldrsw x2, [x8, w3, uxtw]";
        test_ldrsw_r64_r64_r64_sxtx, ldrsw(X2, (X8, ext((X3, SXTX)))), "ldrsw x2, [x8, x3, sxtx]";
        test_ldrsw_r64_r64_xzr_sxtx, ldrsw(X2, (X8, ext((XZR, SXTX)))), "ldrsw x2, [x8, xzr, sxtx]";
        test_ldrsw_r64_r64_xzr_lsl_2, ldrsw(X2, (X8, ext((XZR, LSL, 2)))).unwrap(), "ldrsw x2, [x8, xzr, lsl #2]";
        test_ldrsw_r64_r64_wzr_sxtw, ldrsw(X2, (X8, ext((WZR, SXTW)))), "ldrsw x2, [x8, wzr, sxtw]";
        test_ldrsw_r64_r64_wzr_uxtx, ldrsw(X2, (X8, ext((WZR, UXTW)))), "ldrsw x2, [x8, wzr, uxtw]";
        test_ldrsw_r64_r64_r32_sxtw_0, ldrsw(X2, (X8, ext((W3, SXTW, 0)))).unwrap(), "ldrsw x2, [x8, w3, sxtw #0]";
        test_ldrsw_r64_r64_r32_sxtw_2, ldrsw(X2, (X8, ext((W3, SXTW, 2)))).unwrap(), "ldrsw x2, [x8, w3, sxtw #2]";
        test_ldrsw_r64_r64_r32_uxtw_0, ldrsw(X2, (X8, ext((W3, UXTW, 0)))).unwrap(), "ldrsw x2, [x8, w3, uxtw #0]";
        test_ldrsw_r64_r64_r32_uxtw_2, ldrsw(X2, (X8, ext((W3, UXTW, 2)))).unwrap(), "ldrsw x2, [x8, w3, uxtw #2]";
        test_ldrsw_r64_r64_r64_lsl_0, ldrsw(X2, (X8, ext((X3, LSL, 0)))).unwrap(), "ldrsw x2, [x8, x3, lsl #0]";
        test_ldrsw_r64_r64_r64_lsl_2, ldrsw(X2, (X8, ext((X3, LSL, 2)))).unwrap(), "ldrsw x2, [x8, x3, lsl #2]";
        test_ldrsw_r64_r64_r32_sxtx_0, ldrsw(X2, (X8, ext((X3, SXTX, 0)))).unwrap(), "ldrsw x2, [x8, x3, sxtx #0]";
        test_ldrsw_r64_r64_r32_sxtx_2, ldrsw(X2, (X8, ext((X3, SXTX, 2)))).unwrap(), "ldrsw x2, [x8, x3, sxtx #2]";
        test_ldrsw_r64_r64_r64, ldrsw(X2, (X8, X3)), "ldrsw x2, [x8, x3]";
        test_ldrsw_r64_rsp_r64, ldrsw(X2, (SP, X3)), "ldrsw x2, [sp, x3]";
        test_ldrsw_r64_r64_xzr, ldrsw(X2, (X8, XZR)), "ldrsw x2, [x8, xzr]";
        test_ldrsw_r64_rsp_xzr, ldrsw(X2, (SP, XZR)), "ldrsw x2, [sp, xzr]";
        test_ldrsw_xzr_r64_r64, ldrsw(XZR, (X8, X9)), "ldrsw xzr, [x8, x9]";
        test_ldrsw_wzr_r64_r64, ldrsw(XZR, (X8, X9)), "ldrsw xzr, [x8, x9]";
        test_ldrsw_xzr_r64_r32_sxtw, ldrsw(XZR, (X8, ext((W9, SXTW)))), "ldrsw xzr, [x8, w9, sxtw]";
        test_ldrsw_wzr_r64_r32_uxtw, ldrsw(XZR, (X8, ext((W9, UXTW)))), "ldrsw xzr, [x8, w9, uxtw]";
    }

    test_cases! {
        LDRSW_SCALED_IMM_DB, untested_ldrsw_scaled_imm;
        test_ldrsw_r64_r64_scaled_imm, ldrsw(X2, (X8, UBitValue::<12, 2>::new(0x190).unwrap())), "ldrsw x2, [x8, #0x190]";
        test_ldrsw_r64_sp_scaled_imm, ldrsw(X2, (SP, UBitValue::<12, 2>::new(0x190).unwrap())), "ldrsw x2, [sp, #0x190]";
        test_ldrsw_r64_sp_scaled_imm2, ldrsw(X2, (SP, 0x190u32)).unwrap(), "ldrsw x2, [sp, #0x190]";
        test_ldrsw_xzr_sp_scaled_imm2, ldrsw(XZR, (SP, 0x190u32)).unwrap(), "ldrsw xzr, [sp, #0x190]";
        test_ldrsw_r64_sp_scaled_imm3, ldrsw(X2, (SP, 0x190i32)).unwrap(), "ldrsw x2, [sp, #0x190]";
        test_ldrsw_xzr_sp_scaled_imm3, ldrsw(XZR, (SP, 0x190i32)).unwrap(), "ldrsw xzr, [sp, #0x190]";
        test_ldrsw_r64_r64_simple, ldrsw(X2, (X8,)), "ldrsw x2, [x8]";
    }

    test_cases! {
        LDRSW_PC_RELATIVE_DB, untested_ldrsw_pc_relative;
        test_ldrsw_r64_pc_relative, ldrsw(X2, (Pc, LdStPcOffset::new(44).unwrap())), "ldrsw x2, [pc, #44]";
        test_ldrsw_r64_pc_relative_neg, ldrsw(X2, (Pc, LdStPcOffset::new(-44).unwrap())), "ldrsw x2, [pc, #-44]";
        test_ldrsw_r64_pc_relative2, ldrsw(X2, (Pc, 44)).unwrap(), "ldrsw x2, [pc, #44]";
    }

    test_cases! {
        LDRSW_PRE_POST_INC_DB, untested_ldrsw_pre_post_inc;
        test_ldrsw_r64_r64_preinc, ldrsw(X1, preinc(X2, 0x2a)).unwrap(), "ldrsw x1, [x2, #0x2a]!";
        test_ldrsw_r64_r64_postinc, ldrsw(X1, postinc(X2, 0x2a)).unwrap(), "ldrsw x1, [x2], #0x2a";
        test_ldrsw_r64_sp_preinc, ldrsw(X1, preinc(SP, 0x2a)).unwrap(), "ldrsw x1, [sp, #0x2a]!";
        test_ldrsw_r64_sp_postinc, ldrsw(X1, postinc(SP, 0x2a)).unwrap(), "ldrsw x1, [sp], #0x2a";
        test_ldrsw_r64_r64_preinc_neg, ldrsw(X1, preinc(X2, -0x2a)).unwrap(), "ldrsw x1, [x2, #-0x2a]!";
        test_ldrsw_r64_r64_postinc_neg, ldrsw(X1, postinc(X2, -0x2a)).unwrap(), "ldrsw x1, [x2], #-0x2a";
        test_ldrsw_r64_sp_preinc_neg, ldrsw(X1, preinc(SP, -0x2a)).unwrap(), "ldrsw x1, [sp, #-0x2a]!";
        test_ldrsw_r64_sp_postinc_neg, ldrsw(X1, postinc(SP, -0x2a)).unwrap(), "ldrsw x1, [sp], #-0x2a";
        test_ldrsw_r64_r64_preinc_neg2, ldrsw(X1, preinc(X2, LdStIncOffset::new(-0x2a).unwrap())), "ldrsw x1, [x2, #-0x2a]!";
        test_ldrsw_r64_r64_pre_inc, ldrsw(X1, (inc(0x2a), X2)).unwrap(), "ldrsw x1, [x2, #0x2a]!";
        test_ldrsw_r64_r64_post_inc, ldrsw(X1, (X2, inc(0x2a))).unwrap(), "ldrsw x1, [x2], #0x2a";
        test_ldrsw_r64_sp_pre_inc, ldrsw(X1, (inc(0x2a), SP)).unwrap(), "ldrsw x1, [sp, #0x2a]!";
        test_ldrsw_r64_sp_post_inc, ldrsw(X1, (SP, inc(0x2a))).unwrap(), "ldrsw x1, [sp], #0x2a";
        test_ldrsw_r64_r64_pre_inc_neg, ldrsw(X1, (inc(-0x2a), X2)).unwrap(), "ldrsw x1, [x2, #-0x2a]!";
        test_ldrsw_r64_r64_post_inc_neg, ldrsw(X1, (X2, inc(-0x2a))).unwrap(), "ldrsw x1, [x2], #-0x2a";
        test_ldrsw_r64_sp_pre_inc_neg, ldrsw(X1, (inc(-0x2a), SP)).unwrap(), "ldrsw x1, [sp, #-0x2a]!";
        test_ldrsw_r64_sp_post_inc_neg, ldrsw(X1, (SP, inc(-0x2a))).unwrap(), "ldrsw x1, [sp], #-0x2a";
        test_ldrsw_r64_r64_pre_inc_neg2, ldrsw(X1, (inc(LdStIncOffset::new(-0x2a).unwrap()), X2)), "ldrsw x1, [x2, #-0x2a]!";
        test_ldrsw_xzr_r64_pre_inc, ldrsw(XZR, (inc(0x2a), X2)).unwrap(), "ldrsw xzr, [x2, #0x2a]!";
        test_ldrsw_xzr_r64_post_inc, ldrsw(XZR, (X2, inc(0x2a))).unwrap(), "ldrsw xzr, [x2], #0x2a";
    }
}
