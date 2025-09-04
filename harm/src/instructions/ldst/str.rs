/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

//! `LDR` and related commands.
//!
//! The `ldr` function returns an instance of `Instruction` for loading a 32-bit or 64-bit register from memory. While
//! `LDR` instruction has different variants with various number of arguments, the `ldr` function has two arguments: a
//! destination register and an "address" that encapsulates the rest of arguments: the base, offsets, extensions, etc.
//! Tuples are often used for the second argument, see the pattern in the examples below.
//!
//! The funciton is overloaded for various argument types. For some of them, and `Instruction` trait instance is
//! returned, for others, a `Result` if the aguments need validation. Such arugment combinations have `.unwrap()` in
//! examples.
//!
//! # `LDR`: Register base with register offset
//!
//! # Examples:
//! ```
//! # use harm::instructions::ldst::{ldr, ext, LdStExtendOption32, LdStShift};
//! use harm::register::Reg32::*;
//! use harm::register::Reg64::*;
//! use LdStExtendOption32::*;
//!
//! ldr(W1, X2);        // LDR W1, [X2]
//! ldr(W1, (X2,));     // LDR W1, [X2]
//! ldr(W1, (X2, X3));  // LDR W1, [X2, X3] ; n.b. a 32-bit register offset requires an extend speicifer (sxtw or uxtw):
//! ldr(W1, (X2, ext((W3, UXTW)))); // ldr w1, [x2, w3, uxtw]
//! ldr(W1, (X2, ext((W3, UXTW, LdStShift::Shifted)))); // ldr w1, [x2, w3, uxtw #2]
//! ldr(W1, (X2, ext((W3, UXTW)))); // ldr w1, [x2, w3, uxtw]
//! ldr(X1, (X2, ext((W3, UXTW)))); // ldr x1, [x2, w3, uxtw]
//! ldr(X1, (X2, ext((W3, UXTW, LdStShift::Shifted)))); // ldr x1, [x2, w3, uxtw #3]
//! ldr(X1, (X2, ext((W3, UXTW, 3)))).unwrap(); // ldr x1, [x2, w3, uxtw #3]
//! ```
//!
//! Please note, that `uxtw` and `sxtw` can be used only with 32-bit register, and shift can be only either 0
//! (unshifted) or 2 (shifted). The `lsl` and `sxtx` can be used only with 64-bit registers, and while they produce
//! different bit patterns, they are equivalent; shift can be only either 0 (unshifted) or 3 (shifted).
//!
//! # `LDR`: Register base with immediate offset
//!
//! LDR with register base with immediate offset has an unsigned offset aligned by destination register size.
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
//! ldr(W1, (X2, offset as u32)).unwrap(),
//! ldr(X1, (X2, offset as u32)).unwrap(),
//! ldr(W1, (X2, word_aligned_offset)),
//! ldr(X1, (X2, dword_aligned_offset)),
//! ```
//!
//! Pre-increment and post-increment variants have the following syntax:
//! ```
//! # use harm::instructions::ldst::{ldr, inc, preinc, postinc, LdStIncOffset};
//! use harm::register::Reg32::*;
//! use harm::register::Reg64::*;
//! let offset = LdStIncOffset::new(4).unwrap();
//! ldr(W1, (inc(offset), X2));       // preincrement, LDR W1, [X2, #4]!
//! ldr(W1, (X2, inc(offset)));       // postincrement, LDR W1, [X2], #4
//! // Equavalent to the lines above:
//! ldr(W1, preinc(X2, offset));      // preincrement, LDR W1, [X2, #4]!
//! ldr(W1, postinc(X2, offset));     // postincrement, LDR W1, [X2], #4
//! // Fallible variants:
//! ldr(W1, (inc(4), X2)).unwrap();   // preincrement, LDR W1, [X2, #4]!
//! ldr(W1, postinc(X2, 4)).unwrap(); // postincrement, LDR W1, [X2], #4
//! ```
//!
//! # `LDR`: PC base with immediate offset
//!
//! An immediate signed offset of `SBitValue<12, 2>` is added to `PC`, i.e. the offset is relative to the instruction's
//! address.  The 2-bit alignment is the same for both 32-bit and 64-bit variants.
//!
//! ```ignore
//! let bit_offset: PcOffset = ...;
//! let raw_offset: i32 = ...;
//!
//! ldr(W1, pc(bit_offset)),
//! ldr(W1, (Pc, bit_offset)),
//! ldr(W1, (Pc, raw_offset)).unwrap(),
//! ```
//!
//! # `LDUR`: Register base with unaligned immediate offset
//!
//! `LDUR` is similar to `LDR` with register base with immediate offset, but its offset is **signed** 9 bit wide. For
//! convenience, a `SBitValue<9>` value can be used with `ldr` as well, and both signed and unsigned raw values can be
//! encoded as LDUR if they fit into the range and cannot be encoded with `LDR` with immediate offset (TODO it makes the
//! code little bit more complex, unless we use an enum).
//!
//! ```ignore
//! let bit_offset: SBitOffset<9> = ...;
//! let raw_offset: i32 = ...;
//!
//! ldr(W1, (X2, bit_offset)),
//! ldur(W1, (X2, bit_offset)),
//! ldr(X1, (X2, bit_offset)),
//! ldur(X1, (X2, bit_offset)),
//! ldr(W1, (X2, raw_offset)).unwap(),
//! ldur(W1, (X2, raw_offset)).unwap(),
//! ldr(X1, (X2, raw_offset)).unwap(),
//! ldur(X1, (X2, raw_offset)).unwap(),
//! ```

use aarchmrs_instructions::A64::ldst::{
    ldst_immpost::{
        STR_32_ldst_immpost::STR_32_ldst_immpost, STR_64_ldst_immpost::STR_64_ldst_immpost,
    },
    ldst_immpre::{STR_32_ldst_immpre::STR_32_ldst_immpre, STR_64_ldst_immpre::STR_64_ldst_immpre},
    ldst_pos::{STR_32_ldst_pos::STR_32_ldst_pos, STR_64_ldst_pos::STR_64_ldst_pos},
    ldst_regoff::{STR_32_ldst_regoff::STR_32_ldst_regoff, STR_64_ldst_regoff::STR_64_ldst_regoff},
};

use super::shift_extend::*;
use super::{Inc, LdStIncOffset, ScaledOffset32, ScaledOffset64};
use crate::{
    bits::BitError,
    instructions::RawInstruction,
    register::{IntoCode, RegOrSp64, RegOrZero32, RegOrZero64},
};

/// A `STR` instruction with a destination and an address.
pub struct Str<Rt, Addr> {
    rt: Rt,
    addr: Addr,
}

impl<Rt, Addr> Str<Rt, Addr> {
    pub fn rt(&self) -> &Rt {
        &self.rt
    }

    pub fn addr(&self) -> &Addr {
        &self.addr
    }
}

/// Defines possible was to construct a `STR` instruction.
// TODO sealed trait?
pub trait MakeStr<Rt, Addr> {
    /// Allows defining both faillible and infallible constructors.
    type Output;
    fn new(rt: Rt, addr: Addr) -> Self::Output;
}
//
// ## LDR (register offset)
//
define_reg_offset_rules!(Str, MakeStr, Str, RegOrZero64, 64);
define_reg_offset_rules!(Str, MakeStr, Str, RegOrZero32, 32);

//
// ## LDR (immediate offset)
//
define_imm_offset_rules!(Str, MakeStr, Str, RegOrZero64, 64, ScaledOffset64);
define_imm_offset_rules!(Str, MakeStr, Str, RegOrZero32, 32, ScaledOffset32);

//
// ## Faillible
//
define_fallible_rules!(STR, Str, MakeStr);

/// ldr construction function.  See examples in the module documentation.
pub fn str<TargetInp, TargetOut, AddrInp, AddrOut>(
    dst: TargetInp,
    addr: AddrInp,
) -> <Str<TargetOut, AddrOut> as MakeStr<TargetInp, AddrInp>>::Output
where
    Str<TargetOut, AddrOut>: MakeStr<TargetInp, AddrInp>,
{
    Str::new(dst, addr)
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

    const STR_REG_EXT_DB: &str = "
b8234902	str w2, [x8, w3, uxtw]
b8234902	str w2, [x8, w3, uxtw #0]
b8235902	str w2, [x8, w3, uxtw #2]
b8236902	str w2, [x8, x3]
b8236902	str w2, [x8, x3, lsl #0]
b8236be2	str w2, [sp, x3]
b8237902	str w2, [x8, x3, lsl #2]
b8237902	str w2, [x8, x3, lsl #2]
b823c902	str w2, [x8, w3, sxtw]
b823c902	str w2, [x8, w3, sxtw #0]
b823d902	str w2, [x8, w3, sxtw #2]
b823e902	str w2, [x8, x3, sxtx]
b823e902	str w2, [x8, x3, sxtx #0]
b823f902	str w2, [x8, x3, sxtx #2]
b83f6902	str w2, [x8, xzr]
b83f6be2	str w2, [sp, xzr]
f8234902	str x2, [x8, w3, uxtw]
f8234902	str x2, [x8, w3, uxtw #0]
f8235902	str x2, [x8, w3, uxtw #3]
f8236902	str x2, [x8, x3, lsl #0]
f8236902	str x2, [x8, x3]
f8236be2	str x2, [sp, x3]
f8237902	str x2, [x8, x3, lsl #3]
f823c902	str x2, [x8, w3, sxtw]
f823c902	str x2, [x8, w3, sxtw #0]
f823d902	str x2, [x8, w3, sxtw #3]
f823e902	str x2, [x8, x3, sxtx]
f823e902	str x2, [x8, x3, sxtx #0]
f823f902	str x2, [x8, x3, sxtx #3]
f829491f	str xzr, [x8, w9, uxtw]
f829691f	str xzr, [x8, x9]
f829691f	str xzr, [x8, x9]
f829c91f	str xzr, [x8, w9, sxtw]
f83f4902	str x2, [x8, wzr, uxtw]
f83f6902	str x2, [x8, xzr]
f83f6be2	str x2, [sp, xzr]
f83f7902	str x2, [x8, xzr, lsl #3]
f83fc902	str x2, [x8, wzr, sxtw]
f83fe902	str x2, [x8, xzr, sxtx]
";

    // 'str (w2|x2), [(x8|sp), #0x190]'
    const STR_SCALED_IMM_DB: &str = "
b9019102	str w2, [x8, #0x190]
b901911f	str wzr, [x8, #0x190]
b90193e2	str w2, [sp, #0x190]
f900c902	str x2, [x8, #0x190]
f9000102	str x2, [x8]
f900cbe2	str x2, [sp, #0x190]
f900cbff	str xzr, [sp, #0x190]
";

    const STR_PRE_POST_INC_DB: &str = "
b802a441	str w1, [x2], #0x2a
b802a45f	str wzr, [x2], #0x2a
b802a7e1	str w1, [sp], #0x2a
b802ac41	str w1, [x2, #0x2a]!
b802ac5f	str wzr, [x2, #0x2a]!
b802afe1	str w1, [sp, #0x2a]!
b81d6441	str w1, [x2], #-0x2a
b81d67e1	str w1, [sp], #-0x2a
b81d6c41	str w1, [x2, #-0x2a]!
b81d6fe1	str w1, [sp, #-0x2a]!
f802a441	str x1, [x2], #0x2a
f802a45f	str xzr, [x2], #0x2a
f802a7e1	str x1, [sp], #0x2a
f802ac41	str x1, [x2, #0x2a]!
f802ac5f	str xzr, [x2, #0x2a]!
f802afe1	str x1, [sp, #0x2a]!
f81d6441	str x1, [x2], #-0x2a
f81d67e1	str x1, [sp], #-0x2a
f81d6c41	str x1, [x2, #-0x2a]!
f81d6fe1	str x1, [sp, #-0x2a]!
";

    test_cases! {
        STR_REG_EXT_DB, untested_str_reg_ext_db;
        test_str_r64_r64_r32_sxtw, str(X2, (X8, ext((W3, SXTW)))), "str x2, [x8, w3, sxtw]";
        test_str_r64_r64_r32_uxtw, str(X2, (X8, ext((W3, UXTW)))), "str x2, [x8, w3, uxtw]";
        test_str_r32_r64_r32_sxtw, str(W2, (X8, ext((W3, SXTW)))), "str w2, [x8, w3, sxtw]";
        test_str_r32_r64_r32_uxtw, str(W2, (X8, ext((W3, UXTW)))), "str w2, [x8, w3, uxtw]";
        test_str_r32_r64_r64_sxtx, str(W2, (X8, ext((X3, SXTX)))), "str w2, [x8, x3, sxtx]";
        test_str_r64_r64_r64_sxtx, str(X2, (X8, ext((X3, SXTX)))), "str x2, [x8, x3, sxtx]";
        test_str_r64_r64_xzr_sxtx, str(X2, (X8, ext((XZR, SXTX)))), "str x2, [x8, xzr, sxtx]";
        test_str_r64_r64_xzr_lsl_3, str(X2, (X8, ext((XZR, LSL, 3)))).unwrap(), "str x2, [x8, xzr, lsl #3]";
        test_str_r64_r64_wzr_sxtw, str(X2, (X8, ext((WZR, SXTW)))), "str x2, [x8, wzr, sxtw]";
        test_str_r64_r64_wzr_uxtx, str(X2, (X8, ext((WZR, UXTW)))), "str x2, [x8, wzr, uxtw]";
        test_str_r32_r64_r32_sxtw_0, str(W2, (X8, ext((W3, SXTW, 0)))).unwrap(), "str w2, [x8, w3, sxtw #0]";
        test_str_r32_r64_r32_sxtw_2, str(W2, (X8, ext((W3, SXTW, 2)))).unwrap(), "str w2, [x8, w3, sxtw #2]";
        test_str_r32_r64_r32_uxtw_0, str(W2, (X8, ext((W3, UXTW, 0)))).unwrap(), "str w2, [x8, w3, uxtw #0]";
        test_str_r32_r64_r32_uxtw_2, str(W2, (X8, ext((W3, UXTW, 2)))).unwrap(), "str w2, [x8, w3, uxtw #2]";
        test_str_r64_r64_r32_sxtw_0, str(X2, (X8, ext((W3, SXTW, 0)))).unwrap(), "str x2, [x8, w3, sxtw #0]";
        test_str_r64_r64_r32_sxtw_3, str(X2, (X8, ext((W3, SXTW, 3)))).unwrap(), "str x2, [x8, w3, sxtw #3]";
        test_str_r64_r64_r32_uxtw_0, str(X2, (X8, ext((W3, UXTW, 0)))).unwrap(), "str x2, [x8, w3, uxtw #0]";
        test_str_r64_r64_r32_uxtw_3, str(X2, (X8, ext((W3, UXTW, 3)))).unwrap(), "str x2, [x8, w3, uxtw #3]";
        test_str_r32_r64_r64_lsl_0, str(W2, (X8, ext((X3, LSL, 0)))).unwrap(), "str w2, [x8, x3, lsl #0]";
        test_str_r32_r64_r64_lsl_2, str(W2, (X8, ext((X3, LSL, 2)))).unwrap(), "str w2, [x8, x3, lsl #2]";
        test_str_r32_r64_r64_sxtx_0, str(W2, (X8, ext((X3, SXTX, 0)))).unwrap(), "str w2, [x8, x3, sxtx #0]";
        test_str_r32_r64_r64_sxtx_2, str(W2, (X8, ext((X3, SXTX, 2)))).unwrap(), "str w2, [x8, x3, sxtx #2]";
        test_str_r64_r64_r64_lsl_0, str(X2, (X8, ext((X3, LSL, 0)))).unwrap(), "str x2, [x8, x3, lsl #0]";
        test_str_r64_r64_r64_lsl_3, str(X2, (X8, ext((X3, LSL, 3)))).unwrap(), "str x2, [x8, x3, lsl #3]";
        test_str_r64_r64_r32_sxtx_0, str(X2, (X8, ext((X3, SXTX, 0)))).unwrap(), "str x2, [x8, x3, sxtx #0]";
        test_str_r64_r64_r32_sxtx_3, str(X2, (X8, ext((X3, SXTX, 3)))).unwrap(), "str x2, [x8, x3, sxtx #3]";
        test_str_r32_r64_r64, str(W2, (X8, X3)), "str w2, [x8, x3]";
        test_str_r32_rsp_r64, str(W2, (SP, X3)), "str w2, [sp, x3]";
        test_str_r64_r64_r64, str(X2, (X8, X3)), "str x2, [x8, x3]";
        test_str_r64_rsp_r64, str(X2, (SP, X3)), "str x2, [sp, x3]";
        test_str_r32_r64_xzr, str(W2, (X8, XZR)), "str w2, [x8, xzr]";
        test_str_r32_rsp_xzr, str(W2, (SP, XZR)), "str w2, [sp, xzr]";
        test_str_r64_r64_xzr, str(X2, (X8, XZR)), "str x2, [x8, xzr]";
        test_str_r64_rsp_xzr, str(X2, (SP, XZR)), "str x2, [sp, xzr]";
        test_str_xzr_r64_r64, str(XZR, (X8, X9)), "str xzr, [x8, x9]";
        test_str_wzr_r64_r64, str(XZR, (X8, X9)), "str xzr, [x8, x9]";
        test_str_xzr_r64_r32_sxtw, str(XZR, (X8, ext((W9, SXTW)))), "str xzr, [x8, w9, sxtw]";
        test_str_wzr_r64_r32_uxtw, str(XZR, (X8, ext((W9, UXTW)))), "str xzr, [x8, w9, uxtw]";
    }

    test_cases! {
        STR_SCALED_IMM_DB, untested_str_scaled_imm;
        test_str_r32_r64_scaled_imm, str(W2, (X8, UBitValue::<12, 2>::new(0x190).unwrap())), "str w2, [x8, #0x190]";
        test_str_r32_sp_scaled_imm, str(W2, (SP, UBitValue::<12, 2>::new(0x190).unwrap())), "str w2, [sp, #0x190]";
        test_str_r64_r64_scaled_imm, str(X2, (X8, UBitValue::<12, 3>::new(0x190).unwrap())), "str x2, [x8, #0x190]";
        test_str_r64_sp_scaled_imm, str(X2, (SP, UBitValue::<12, 3>::new(0x190).unwrap())), "str x2, [sp, #0x190]";
        test_str_r32_r64_scaled_imm2, str(W2, (X8, 0x190u32)).unwrap(), "str w2, [x8, #0x190]";
        test_str_r64_sp_scaled_imm2, str(X2, (SP, 0x190u32)).unwrap(), "str x2, [sp, #0x190]";
        test_str_wzr_r64_scaled_imm2, str(WZR, (X8, 0x190u32)).unwrap(), "str wzr, [x8, #0x190]";
        test_str_xzr_sp_scaled_imm2, str(XZR, (SP, 0x190u32)).unwrap(), "str xzr, [sp, #0x190]";
        test_str_r32_r64_scaled_imm3, str(W2, (X8, 0x190i32)).unwrap(), "str w2, [x8, #0x190]";
        test_str_r64_sp_scaled_imm3, str(X2, (SP, 0x190i32)).unwrap(), "str x2, [sp, #0x190]";
        test_str_wzr_r64_scaled_imm3, str(WZR, (X8, 0x190i32)).unwrap(), "str wzr, [x8, #0x190]";
        test_str_xzr_sp_scaled_imm3, str(XZR, (SP, 0x190i32)).unwrap(), "str xzr, [sp, #0x190]";
        test_str_r64_r64_simple, str(X2, (X8,)), "str x2, [x8]";
    }

    test_cases! {
        STR_PRE_POST_INC_DB, untested_str_pre_post_inc;
        test_str_r32_r64_preinc, str(W1, preinc(X2, 0x2a)).unwrap(), "str w1, [x2, #0x2a]!";
        test_str_r32_r64_postinc, str(W1, postinc(X2, 0x2a)).unwrap(), "str w1, [x2], #0x2a";
        test_str_r64_r64_preinc, str(X1, preinc(X2, 0x2a)).unwrap(), "str x1, [x2, #0x2a]!";
        test_str_r64_r64_postinc, str(X1, postinc(X2, 0x2a)).unwrap(), "str x1, [x2], #0x2a";
        test_str_r32_sp_preinc, str(W1, preinc(SP, 0x2a)).unwrap(), "str w1, [sp, #0x2a]!";
        test_str_r32_sp_postinc, str(W1, postinc(SP, 0x2a)).unwrap(), "str w1, [sp], #0x2a";
        test_str_r64_sp_preinc, str(X1, preinc(SP, 0x2a)).unwrap(), "str x1, [sp, #0x2a]!";
        test_str_r64_sp_postinc, str(X1, postinc(SP, 0x2a)).unwrap(), "str x1, [sp], #0x2a";
        test_str_r32_r64_preinc_neg, str(W1, preinc(X2, -0x2a)).unwrap(), "str w1, [x2, #-0x2a]!";
        test_str_r32_r64_postinc_neg, str(W1, postinc(X2, -0x2a)).unwrap(), "str w1, [x2], #-0x2a";
        test_str_r64_r64_preinc_neg, str(X1, preinc(X2, -0x2a)).unwrap(), "str x1, [x2, #-0x2a]!";
        test_str_r64_r64_postinc_neg, str(X1, postinc(X2, -0x2a)).unwrap(), "str x1, [x2], #-0x2a";
        test_str_r32_sp_preinc_neg, str(W1, preinc(SP, -0x2a)).unwrap(), "str w1, [sp, #-0x2a]!";
        test_str_r32_sp_postinc_neg, str(W1, postinc(SP, -0x2a)).unwrap(), "str w1, [sp], #-0x2a";
        test_str_r64_sp_preinc_neg, str(X1, preinc(SP, -0x2a)).unwrap(), "str x1, [sp, #-0x2a]!";
        test_str_r64_sp_postinc_neg, str(X1, postinc(SP, -0x2a)).unwrap(), "str x1, [sp], #-0x2a";
        test_str_r32_sp_preinc2, str(W1, preinc(SP, LdStIncOffset::new(0x2a).unwrap())), "str w1, [sp, #0x2a]!";
        test_str_r64_r64_preinc_neg2, str(X1, preinc(X2, LdStIncOffset::new(-0x2a).unwrap())), "str x1, [x2, #-0x2a]!";
        test_str_r32_r64_pre_inc, str(W1, (inc(0x2a), X2)).unwrap(), "str w1, [x2, #0x2a]!";
        test_str_r32_r64_post_inc, str(W1, (X2, inc(0x2a))).unwrap(), "str w1, [x2], #0x2a";
        test_str_r64_r64_pre_inc, str(X1, (inc(0x2a), X2)).unwrap(), "str x1, [x2, #0x2a]!";
        test_str_r64_r64_post_inc, str(X1, (X2, inc(0x2a))).unwrap(), "str x1, [x2], #0x2a";
        test_str_r32_sp_pre_inc, str(W1, (inc(0x2a), SP)).unwrap(), "str w1, [sp, #0x2a]!";
        test_str_r32_sp_post_inc, str(W1, (SP, inc(0x2a))).unwrap(), "str w1, [sp], #0x2a";
        test_str_r64_sp_pre_inc, str(X1, (inc(0x2a), SP)).unwrap(), "str x1, [sp, #0x2a]!";
        test_str_r64_sp_post_inc, str(X1, (SP, inc(0x2a))).unwrap(), "str x1, [sp], #0x2a";
        test_str_r32_r64_pre_inc_neg, str(W1, (inc(-0x2a), X2)).unwrap(), "str w1, [x2, #-0x2a]!";
        test_str_r32_r64_post_inc_neg, str(W1, (X2, inc(-0x2a))).unwrap(), "str w1, [x2], #-0x2a";
        test_str_r64_r64_pre_inc_neg, str(X1, (inc(-0x2a), X2)).unwrap(), "str x1, [x2, #-0x2a]!";
        test_str_r64_r64_post_inc_neg, str(X1, (X2, inc(-0x2a))).unwrap(), "str x1, [x2], #-0x2a";
        test_str_r32_sp_pre_inc_neg, str(W1, (inc(-0x2a), SP)).unwrap(), "str w1, [sp, #-0x2a]!";
        test_str_r32_sp_post_inc_neg, str(W1, (SP, inc(-0x2a))).unwrap(), "str w1, [sp], #-0x2a";
        test_str_r64_sp_pre_inc_neg, str(X1, (inc(-0x2a), SP)).unwrap(), "str x1, [sp, #-0x2a]!";
        test_str_r64_sp_post_inc_neg, str(X1, (SP, inc(-0x2a))).unwrap(), "str x1, [sp], #-0x2a";
        test_str_r32_sp_pre_inc2, str(W1, (inc(LdStIncOffset::new(0x2a).unwrap()), SP)), "str w1, [sp, #0x2a]!";
        test_str_r64_r64_pre_inc_neg2, str(X1, (inc(LdStIncOffset::new(-0x2a).unwrap()), X2)), "str x1, [x2, #-0x2a]!";
        test_str_xzr_r64_pre_inc, str(XZR, (inc(0x2a), X2)).unwrap(), "str xzr, [x2, #0x2a]!";
        test_str_xzr_r64_post_inc, str(XZR, (X2, inc(0x2a))).unwrap(), "str xzr, [x2], #0x2a";
        test_str_wzr_r64_pre_inc, str(WZR, (inc(0x2a), X2)).unwrap(), "str wzr, [x2, #0x2a]!";
        test_str_wzr_r64_post_inc, str(WZR, (X2, inc(0x2a))).unwrap(), "str wzr, [x2], #0x2a";
    }
}
