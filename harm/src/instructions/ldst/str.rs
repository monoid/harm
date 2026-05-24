/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

//! `STR` and related commands.
//!
//! The `str` function returns an instance of `Instruction` for storing a 32-bit or 64-bit register to memory.
//! `STR` has different variants with various addressing modes; the `str` function takes two arguments: a source
//! register and an "address" that encapsulates the rest of arguments: the base, offsets, extensions, etc.
//! Tuples are often used for the second argument, see the pattern in the examples below.
//!
//! The function is overloaded for various argument types. For some of them, an `Instruction` trait instance is
//! returned, for others, a `Result` if the arguments need validation. Such argument combinations have `.unwrap()` in
//! examples.
//!
//! # `STR`: Register base with register offset
//!
//! # Examples:
//! ```
//! # use harm::instructions::ldst::{str, ext, LdStExtendOption32, LdStShift};
//! use harm::register::Reg32::*;
//! use harm::register::Reg64::*;
//! use LdStExtendOption32::*;
//!
//! str(W1, X2);        // STR W1, [X2]
//! str(W1, (X2,));     // STR W1, [X2]
//! str(W1, (X2, X3));  // STR W1, [X2, X3] ; n.b. a 32-bit register offset requires an extend specifier (sxtw or uxtw):
//! str(W1, (X2, ext((W3, UXTW)))); // str w1, [x2, w3, uxtw]
//! str(W1, (X2, ext((W3, UXTW, LdStShift::Shifted)))); // str w1, [x2, w3, uxtw #2]
//! str(X1, (X2, ext((W3, UXTW)))); // str x1, [x2, w3, uxtw]
//! str(X1, (X2, ext((W3, UXTW, LdStShift::Shifted)))); // str x1, [x2, w3, uxtw #3]
//! str(X1, (X2, ext((W3, UXTW, 3)))).unwrap(); // str x1, [x2, w3, uxtw #3]
//! ```
//!
//! # `STR`: Register base with immediate offset
//!
//! STR with register base with immediate offset has an unsigned offset aligned by source register size.
//! For example, if the source register is `W1`, the offset must be aligned by 4 bytes (two lower bits are clear),
//! and if it is `X1`, the offset must be aligned by 8 bytes (three lower bits are clear).
//!
//! You may also pass a `u32` offset value, and an error is returned if the value doesn't fit the offset pattern.
//!
//! Examples:
//! ```ignore
//! let word_aligned_offset: UBitValue<12, 2> = ...;
//! let dword_aligned_offset: UBitValue<12, 3> = ...;
//!
//! str(W1, (X2, offset as u32)).unwrap(),
//! str(X1, (X2, offset as u32)).unwrap(),
//! str(W1, (X2, word_aligned_offset)),
//! str(X1, (X2, dword_aligned_offset)),
//! ```
//!
//! Pre-increment and post-increment variants have the following syntax:
//! ```
//! # use harm::instructions::ldst::{str, inc, preinc, postinc, LdStIncOffset};
//! use harm::register::Reg32::*;
//! use harm::register::Reg64::*;
//! let offset = LdStIncOffset::new(4).unwrap();
//! str(W1, (inc(offset), X2));       // preincrement, STR W1, [X2, #4]!
//! str(W1, (X2, inc(offset)));       // postincrement, STR W1, [X2], #4
//! // Equivalent to the lines above:
//! str(W1, preinc(X2, offset));      // preincrement, STR W1, [X2, #4]!
//! str(W1, postinc(X2, offset));     // postincrement, STR W1, [X2], #4
//! // Fallible variants:
//! str(W1, (inc(4), X2)).unwrap();   // preincrement, STR W1, [X2, #4]!
//! str(W1, postinc(X2, 4)).unwrap(); // postincrement, STR W1, [X2], #4
//! ```

use aarchmrs_instructions::A64::ldst::{
    ldst_immpost::{
        STR_32_ldst_immpost::STR_32_ldst_immpost, STR_64_ldst_immpost::STR_64_ldst_immpost,
    },
    ldst_immpre::{STR_32_ldst_immpre::STR_32_ldst_immpre, STR_64_ldst_immpre::STR_64_ldst_immpre},
    ldst_pos::{STR_32_ldst_pos::STR_32_ldst_pos, STR_64_ldst_pos::STR_64_ldst_pos},
    ldst_regoff::{STR_32_ldst_regoff::STR_32_ldst_regoff, STR_64_ldst_regoff::STR_64_ldst_regoff},
};

use super::args::{LdStArgs, MakeLdStArgs};
use super::shift_extend::*;
use super::{Inc, LdStIncOffset, ScaledOffset32, ScaledOffset64};
use crate::instructions::RawInstruction;
use crate::outcome::Outcome;
use crate::register::{RegOrSp64, RegOrZero32, RegOrZero64, Register};
use crate::sealed::Sealed;

/// A `str` instruction with a source register and an address.
#[derive(Debug, Copy, Clone)]
pub struct Str<Args>(pub Args);

impl<Args: Sealed> Sealed for Str<Args> {}

/// str construction function. See examples in the module documentation.
pub fn str<RtIn, Rt, AddrIn, Addr>(
    src: RtIn,
    addr: AddrIn,
) -> <<LdStArgs<Rt, Addr> as MakeLdStArgs<RtIn, AddrIn>>::Outcome as Outcome>::Output<
    Str<LdStArgs<Rt, Addr>>,
>
where
    LdStArgs<Rt, Addr>: MakeLdStArgs<RtIn, AddrIn>,
    <LdStArgs<Rt, Addr> as MakeLdStArgs<RtIn, AddrIn>>::Outcome:
        Outcome<Inner = LdStArgs<Rt, Addr>>,
{
    <LdStArgs<Rt, Addr> as MakeLdStArgs<RtIn, AddrIn>>::new(src, addr).map(Str)
}

// === STR 64-bit: register offset ===

impl RawInstruction
    for Str<LdStArgs<RegOrZero64, (RegOrSp64, Extended<RegOrZero64, RegOrZero64>)>>
{
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let (base, offset) = self.0.addr;
        STR_64_ldst_regoff(
            offset.offset.index(),
            (offset.extend as u8).into(),
            offset.shifted.into(),
            base.index(),
            self.0.rt.index(),
        )
    }
}

impl RawInstruction
    for Str<LdStArgs<RegOrZero64, (RegOrSp64, Extended<RegOrZero64, RegOrZero32>)>>
{
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let (base, offset) = self.0.addr;
        STR_64_ldst_regoff(
            offset.offset.index(),
            (offset.extend as u8).into(),
            offset.shifted.into(),
            base.index(),
            self.0.rt.index(),
        )
    }
}

impl RawInstruction for Str<LdStArgs<RegOrZero64, (RegOrSp64, RegOrZero64)>> {
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let (base, offset) = self.0.addr;
        STR_64_ldst_regoff(
            offset.index(),
            (LdStExtendOption64::default() as u8).into(),
            0b0.into(),
            base.index(),
            self.0.rt.index(),
        )
    }
}

// === STR 32-bit: register offset ===

impl RawInstruction
    for Str<LdStArgs<RegOrZero32, (RegOrSp64, Extended<RegOrZero32, RegOrZero64>)>>
{
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let (base, offset) = self.0.addr;
        STR_32_ldst_regoff(
            offset.offset.index(),
            (offset.extend as u8).into(),
            offset.shifted.into(),
            base.index(),
            self.0.rt.index(),
        )
    }
}

impl RawInstruction
    for Str<LdStArgs<RegOrZero32, (RegOrSp64, Extended<RegOrZero32, RegOrZero32>)>>
{
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let (base, offset) = self.0.addr;
        STR_32_ldst_regoff(
            offset.offset.index(),
            (offset.extend as u8).into(),
            offset.shifted.into(),
            base.index(),
            self.0.rt.index(),
        )
    }
}

impl RawInstruction for Str<LdStArgs<RegOrZero32, (RegOrSp64, RegOrZero64)>> {
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let (base, offset) = self.0.addr;
        STR_32_ldst_regoff(
            offset.index(),
            (LdStExtendOption64::default() as u8).into(),
            0b0.into(),
            base.index(),
            self.0.rt.index(),
        )
    }
}

// === STR 64-bit: scaled immediate offset ===

impl RawInstruction for Str<LdStArgs<RegOrZero64, (RegOrSp64, ScaledOffset64)>> {
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let (base, offset) = self.0.addr;
        STR_64_ldst_pos(offset.into(), base.index(), self.0.rt.index())
    }
}

impl RawInstruction for Str<LdStArgs<RegOrZero64, (Inc<LdStIncOffset>, RegOrSp64)>> {
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let (inc, base) = self.0.addr;
        STR_64_ldst_immpre(inc.offset.into(), base.index(), self.0.rt.index())
    }
}

impl RawInstruction for Str<LdStArgs<RegOrZero64, (RegOrSp64, Inc<LdStIncOffset>)>> {
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let (base, inc) = self.0.addr;
        STR_64_ldst_immpost(inc.offset.into(), base.index(), self.0.rt.index())
    }
}

// === STR 32-bit: scaled immediate offset ===

impl RawInstruction for Str<LdStArgs<RegOrZero32, (RegOrSp64, ScaledOffset32)>> {
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let (base, offset) = self.0.addr;
        STR_32_ldst_pos(offset.into(), base.index(), self.0.rt.index())
    }
}

impl RawInstruction for Str<LdStArgs<RegOrZero32, (Inc<LdStIncOffset>, RegOrSp64)>> {
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let (inc, base) = self.0.addr;
        STR_32_ldst_immpre(inc.offset.into(), base.index(), self.0.rt.index())
    }
}

impl RawInstruction for Str<LdStArgs<RegOrZero32, (RegOrSp64, Inc<LdStIncOffset>)>> {
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let (base, inc) = self.0.addr;
        STR_32_ldst_immpost(inc.offset.into(), base.index(), self.0.rt.index())
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
