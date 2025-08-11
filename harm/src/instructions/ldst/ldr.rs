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
        LDR_32_ldst_immpost::LDR_32_ldst_immpost, LDR_64_ldst_immpost::LDR_64_ldst_immpost,
    },
    ldst_immpre::{LDR_32_ldst_immpre::LDR_32_ldst_immpre, LDR_64_ldst_immpre::LDR_64_ldst_immpre},
    ldst_pos::{LDR_32_ldst_pos::LDR_32_ldst_pos, LDR_64_ldst_pos::LDR_64_ldst_pos},
    ldst_regoff::{LDR_32_ldst_regoff::LDR_32_ldst_regoff, LDR_64_ldst_regoff::LDR_64_ldst_regoff},
    loadlit::{LDR_32_loadlit::LDR_32_loadlit, LDR_64_loadlit::LDR_64_loadlit},
};

use super::shift_extend::*;
use super::{Inc, LdStIncOffset, ScaledOffset32, ScaledOffset64};
use crate::bits::BitError;
use crate::instructions::RawInstruction;
use crate::register::{IntoCode, RegOrSp64, RegOrZero32, RegOrZero64};

/// A `LDR` instruction with a destination and an address.
pub struct Load<Rt, Addr> {
    rt: Rt,
    addr: Addr,
}

impl<Rt, Addr> Load<Rt, Addr> {
    pub fn rt(&self) -> &Rt {
        &self.rt
    }

    pub fn addr(&self) -> &Addr {
        &self.addr
    }
}

/// Defines possible was to construct a `Load` instruction.
// TODO sealed trait?
pub trait MakeLoad<Rt, Addr> {
    /// Allows defining both faillible and infallible constructors.
    type Output;
    fn new(rt: Rt, addr: Addr) -> Self::Output;
}
//
// ## LDR (register offset)
//
define_reg_offset_rules!(Load, MakeLoad, LDR, RegOrZero64, 64);
define_reg_offset_rules!(Load, MakeLoad, LDR, RegOrZero32, 32);

//
// ## LDR (immediate offset)
//
define_imm_offset_rules!(Load, MakeLoad, LDR, RegOrZero64, 64, ScaledOffset64);
define_imm_offset_rules!(Load, MakeLoad, LDR, RegOrZero32, 32, ScaledOffset32);

//
// ## LDR (PC-relative literal)
//
define_pc_offset_rules!(Load, MakeLoad, LDR, RegOrZero64, 64);
define_pc_offset_rules!(Load, MakeLoad, LDR, RegOrZero32, 32);

//
// ## Faillible
//
define_fallible_rules!(LDR, Load, MakeLoad);

/// ldr construction function.  See examples in the module documentation.
pub fn ldr<TargetInp, TargetOut, AddrInp, AddrOut>(
    dst: TargetInp,
    addr: AddrInp,
) -> <Load<TargetOut, AddrOut> as MakeLoad<TargetInp, AddrInp>>::Output
where
    Load<TargetOut, AddrOut>: MakeLoad<TargetInp, AddrInp>,
{
    Load::new(dst, addr)
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

    const LDR_REG_EXT_DB: &str = "
b8634902	ldr w2, [x8, w3, uxtw #0]
b8634902	ldr w2, [x8, w3, uxtw]
b8635902	ldr w2, [x8, w3, uxtw #2]
b8636902	ldr w2, [x8, x3, lsl #0]
b8636902	ldr w2, [x8, x3]
b8636be2	ldr w2, [sp, x3]
b8637902	ldr w2, [x8, x3, lsl #2]
b8637902	ldr w2, [x8, x3, lsl #2]
b863c902	ldr w2, [x8, w3, sxtw #0]
b863c902	ldr w2, [x8, w3, sxtw]
b863d902	ldr w2, [x8, w3, sxtw #2]
b863e902	ldr w2, [x8, x3, sxtx #0]
b863e902	ldr w2, [x8, x3, sxtx]
b863f902	ldr w2, [x8, x3, sxtx #2]
b87f6902	ldr w2, [x8, xzr]
b87f6be2	ldr w2, [sp, xzr]
f8634902	ldr x2, [x8, w3, uxtw #0]
f8634902	ldr x2, [x8, w3, uxtw]
f8635902	ldr x2, [x8, w3, uxtw #3]
f8636902	ldr x2, [x8, x3, lsl #0]
f8636902	ldr x2, [x8, x3]
f8636be2	ldr x2, [sp, x3]
f8637902	ldr x2, [x8, x3, lsl #3]
f863c902	ldr x2, [x8, w3, sxtw #0]
f863c902	ldr x2, [x8, w3, sxtw]
f863d902	ldr x2, [x8, w3, sxtw #3]
f863e902	ldr x2, [x8, x3, sxtx #0]
f863e902	ldr x2, [x8, x3, sxtx]
f863f902	ldr x2, [x8, x3, sxtx #3]
f869491f	ldr xzr, [x8, w9, uxtw]
f869691f	ldr xzr, [x8, x9]
f869691f	ldr xzr, [x8, x9]
f869c91f	ldr xzr, [x8, w9, sxtw]
f87f4902	ldr x2, [x8, wzr, uxtw]
f87f6902	ldr x2, [x8, xzr]
f87f6be2	ldr x2, [sp, xzr]
f87f7902	ldr x2, [x8, xzr, lsl #3]
f87fc902	ldr x2, [x8, wzr, sxtw]
f87fe902	ldr x2, [x8, xzr, sxtx]
";

    // 'ldr (w2|x2), [(x8|sp), #0x190]'
    const LDR_SCALED_IMM_DB: &str = "
b9419102	ldr w2, [x8, #0x190]
b941911f	ldr wzr, [x8, #0x190]
b94193e2	ldr w2, [sp, #0x190]
f940c902	ldr x2, [x8, #0x190]
f940cbe2	ldr x2, [sp, #0x190]
f940cbff	ldr xzr, [sp, #0x190]
";

    // NB: not a real syntax.
    const LDR_PC_RELATIVE_DB: &str = "
18000162	ldr w2, [pc, #44]
58000162	ldr x2, [pc, #44]
18fffea2	ldr w2, [pc, #-44]
58fffea2	ldr x2, [pc, #-44]
";

    const LDR_PRE_POST_INC_DB: &str = "
b842a441	ldr w1, [x2], #0x2a
b842a45f	ldr wzr, [x2], #0x2a
b842a7e1	ldr w1, [sp], #0x2a
b842ac41	ldr w1, [x2, #0x2a]!
b842ac5f	ldr wzr, [x2, #0x2a]!
b842afe1	ldr w1, [sp, #0x2a]!
b85d6441	ldr w1, [x2], #-0x2a
b85d67e1	ldr w1, [sp], #-0x2a
b85d6c41	ldr w1, [x2, #-0x2a]!
b85d6fe1	ldr w1, [sp, #-0x2a]!
f842a441	ldr x1, [x2], #0x2a
f842a45f	ldr xzr, [x2], #0x2a
f842a7e1	ldr x1, [sp], #0x2a
f842ac41	ldr x1, [x2, #0x2a]!
f842ac5f	ldr xzr, [x2, #0x2a]!
f842afe1	ldr x1, [sp, #0x2a]!
f85d6441	ldr x1, [x2], #-0x2a
f85d67e1	ldr x1, [sp], #-0x2a
f85d6c41	ldr x1, [x2, #-0x2a]!
f85d6fe1	ldr x1, [sp, #-0x2a]!
";
    test_cases! {
        LDR_REG_EXT_DB, untested_ldr_reg_ext_db;
        test_ldr_r64_r64_r32_sxtw, ldr(X2, (X8, ext((W3, SXTW)))), "ldr x2, [x8, w3, sxtw]";
        test_ldr_r64_r64_r32_uxtw, ldr(X2, (X8, ext((W3, UXTW)))), "ldr x2, [x8, w3, uxtw]";
        test_ldr_r32_r64_r32_sxtw, ldr(W2, (X8, ext((W3, SXTW)))), "ldr w2, [x8, w3, sxtw]";
        test_ldr_r32_r64_r32_uxtw, ldr(W2, (X8, ext((W3, UXTW)))), "ldr w2, [x8, w3, uxtw]";
        test_ldr_r32_r64_r64_sxtx, ldr(W2, (X8, ext((X3, SXTX)))), "ldr w2, [x8, x3, sxtx]";
        test_ldr_r64_r64_r64_sxtx, ldr(X2, (X8, ext((X3, SXTX)))), "ldr x2, [x8, x3, sxtx]";
        test_ldr_r64_r64_xzr_sxtx, ldr(X2, (X8, ext((XZR, SXTX)))), "ldr x2, [x8, xzr, sxtx]";
        test_ldr_r64_r64_xzr_lsl_3, ldr(X2, (X8, ext((XZR, LSL, 3)))).unwrap(), "ldr x2, [x8, xzr, lsl #3]";
        test_ldr_r64_r64_wzr_sxtw, ldr(X2, (X8, ext((WZR, SXTW)))), "ldr x2, [x8, wzr, sxtw]";
        test_ldr_r64_r64_wzr_uxtx, ldr(X2, (X8, ext((WZR, UXTW)))), "ldr x2, [x8, wzr, uxtw]";
        test_ldr_r32_r64_r32_sxtw_0, ldr(W2, (X8, ext((W3, SXTW, 0)))).unwrap(), "ldr w2, [x8, w3, sxtw #0]";
        test_ldr_r32_r64_r32_sxtw_2, ldr(W2, (X8, ext((W3, SXTW, 2)))).unwrap(), "ldr w2, [x8, w3, sxtw #2]";
        test_ldr_r32_r64_r32_uxtw_0, ldr(W2, (X8, ext((W3, UXTW, 0)))).unwrap(), "ldr w2, [x8, w3, uxtw #0]";
        test_ldr_r32_r64_r32_uxtw_2, ldr(W2, (X8, ext((W3, UXTW, 2)))).unwrap(), "ldr w2, [x8, w3, uxtw #2]";
        test_ldr_r64_r64_r32_sxtw_0, ldr(X2, (X8, ext((W3, SXTW, 0)))).unwrap(), "ldr x2, [x8, w3, sxtw #0]";
        test_ldr_r64_r64_r32_sxtw_3, ldr(X2, (X8, ext((W3, SXTW, 3)))).unwrap(), "ldr x2, [x8, w3, sxtw #3]";
        test_ldr_r64_r64_r32_uxtw_0, ldr(X2, (X8, ext((W3, UXTW, 0)))).unwrap(), "ldr x2, [x8, w3, uxtw #0]";
        test_ldr_r64_r64_r32_uxtw_3, ldr(X2, (X8, ext((W3, UXTW, 3)))).unwrap(), "ldr x2, [x8, w3, uxtw #3]";
        test_ldr_r32_r64_r64_lsl_0, ldr(W2, (X8, ext((X3, LSL, 0)))).unwrap(), "ldr w2, [x8, x3, lsl #0]";
        test_ldr_r32_r64_r64_lsl_2, ldr(W2, (X8, ext((X3, LSL, 2)))).unwrap(), "ldr w2, [x8, x3, lsl #2]";
        test_ldr_r32_r64_r64_sxtx_0, ldr(W2, (X8, ext((X3, SXTX, 0)))).unwrap(), "ldr w2, [x8, x3, sxtx #0]";
        test_ldr_r32_r64_r64_sxtx_2, ldr(W2, (X8, ext((X3, SXTX, 2)))).unwrap(), "ldr w2, [x8, x3, sxtx #2]";
        test_ldr_r64_r64_r64_lsl_0, ldr(X2, (X8, ext((X3, LSL, 0)))).unwrap(), "ldr x2, [x8, x3, lsl #0]";
        test_ldr_r64_r64_r64_lsl_3, ldr(X2, (X8, ext((X3, LSL, 3)))).unwrap(), "ldr x2, [x8, x3, lsl #3]";
        test_ldr_r64_r64_r32_sxtx_0, ldr(X2, (X8, ext((X3, SXTX, 0)))).unwrap(), "ldr x2, [x8, x3, sxtx #0]";
        test_ldr_r64_r64_r32_sxtx_3, ldr(X2, (X8, ext((X3, SXTX, 3)))).unwrap(), "ldr x2, [x8, x3, sxtx #3]";
        test_ldr_r32_r64_r64, ldr(W2, (X8, X3)), "ldr w2, [x8, x3]";
        test_ldr_r32_rsp_r64, ldr(W2, (SP, X3)), "ldr w2, [sp, x3]";
        test_ldr_r64_r64_r64, ldr(X2, (X8, X3)), "ldr x2, [x8, x3]";
        test_ldr_r64_rsp_r64, ldr(X2, (SP, X3)), "ldr x2, [sp, x3]";
        test_ldr_r32_r64_xzr, ldr(W2, (X8, XZR)), "ldr w2, [x8, xzr]";
        test_ldr_r32_rsp_xzr, ldr(W2, (SP, XZR)), "ldr w2, [sp, xzr]";
        test_ldr_r64_r64_xzr, ldr(X2, (X8, XZR)), "ldr x2, [x8, xzr]";
        test_ldr_r64_rsp_xzr, ldr(X2, (SP, XZR)), "ldr x2, [sp, xzr]";
        test_ldr_xzr_r64_r64, ldr(XZR, (X8, X9)), "ldr xzr, [x8, x9]";
        test_ldr_wzr_r64_r64, ldr(XZR, (X8, X9)), "ldr xzr, [x8, x9]";
        test_ldr_xzr_r64_r32_sxtw, ldr(XZR, (X8, ext((W9, SXTW)))), "ldr xzr, [x8, w9, sxtw]";
        test_ldr_wzr_r64_r32_uxtw, ldr(XZR, (X8, ext((W9, UXTW)))), "ldr xzr, [x8, w9, uxtw]";
    }

    test_cases! {
        LDR_SCALED_IMM_DB, untested_ldr_scaled_imm;
        test_ldr_r32_r64_scaled_imm, ldr(W2, (X8, UBitValue::<12, 2>::new(0x190).unwrap())), "ldr w2, [x8, #0x190]";
        test_ldr_r32_sp_scaled_imm, ldr(W2, (SP, UBitValue::<12, 2>::new(0x190).unwrap())), "ldr w2, [sp, #0x190]";
        test_ldr_r64_r64_scaled_imm, ldr(X2, (X8, UBitValue::<12, 3>::new(0x190).unwrap())), "ldr x2, [x8, #0x190]";
        test_ldr_r64_sp_scaled_imm, ldr(X2, (SP, UBitValue::<12, 3>::new(0x190).unwrap())), "ldr x2, [sp, #0x190]";
        test_ldr_r32_r64_scaled_imm2, ldr(W2, (X8, 0x190u32)).unwrap(), "ldr w2, [x8, #0x190]";
        test_ldr_r64_sp_scaled_imm2, ldr(X2, (SP, 0x190u32)).unwrap(), "ldr x2, [sp, #0x190]";
        test_ldr_wzr_r64_scaled_imm2, ldr(WZR, (X8, 0x190u32)).unwrap(), "ldr wzr, [x8, #0x190]";
        test_ldr_xzr_sp_scaled_imm2, ldr(XZR, (SP, 0x190u32)).unwrap(), "ldr xzr, [sp, #0x190]";
        test_ldr_r32_r64_scaled_imm3, ldr(W2, (X8, 0x190i32)).unwrap(), "ldr w2, [x8, #0x190]";
        test_ldr_r64_sp_scaled_imm3, ldr(X2, (SP, 0x190i32)).unwrap(), "ldr x2, [sp, #0x190]";
        test_ldr_wzr_r64_scaled_imm3, ldr(WZR, (X8, 0x190i32)).unwrap(), "ldr wzr, [x8, #0x190]";
        test_ldr_xzr_sp_scaled_imm3, ldr(XZR, (SP, 0x190i32)).unwrap(), "ldr xzr, [sp, #0x190]";
    }

    test_cases! {
        LDR_PC_RELATIVE_DB, untested_ldr_pc_relative;
            test_ldr_r32_pc_relative, ldr(W2, (Pc, LdStPcOffset::new(44).unwrap())), "ldr w2, [pc, #44]";
            test_ldr_r64_pc_relative, ldr(X2, (Pc, LdStPcOffset::new(44).unwrap())), "ldr x2, [pc, #44]";
            test_ldr_r32_pc_relative_neg, ldr(W2, (Pc, LdStPcOffset::new(-44).unwrap())), "ldr w2, [pc, #-44]";
            test_ldr_r64_pc_relative_neg, ldr(X2, (Pc, LdStPcOffset::new(-44).unwrap())), "ldr x2, [pc, #-44]";
            test_ldr_r64_pc_relative2, ldr(X2, (Pc, 44)).unwrap(), "ldr x2, [pc, #44]";
    }

    test_cases! {
        LDR_PRE_POST_INC_DB, untested_ldr_pre_post_inc;
        test_ldr_r32_r64_preinc, ldr(W1, preinc(X2, 0x2a)).unwrap(), "ldr w1, [x2, #0x2a]!";
        test_ldr_r32_r64_postinc, ldr(W1, postinc(X2, 0x2a)).unwrap(), "ldr w1, [x2], #0x2a";
        test_ldr_r64_r64_preinc, ldr(X1, preinc(X2, 0x2a)).unwrap(), "ldr x1, [x2, #0x2a]!";
        test_ldr_r64_r64_postinc, ldr(X1, postinc(X2, 0x2a)).unwrap(), "ldr x1, [x2], #0x2a";
        test_ldr_r32_sp_preinc, ldr(W1, preinc(SP, 0x2a)).unwrap(), "ldr w1, [sp, #0x2a]!";
        test_ldr_r32_sp_postinc, ldr(W1, postinc(SP, 0x2a)).unwrap(), "ldr w1, [sp], #0x2a";
        test_ldr_r64_sp_preinc, ldr(X1, preinc(SP, 0x2a)).unwrap(), "ldr x1, [sp, #0x2a]!";
        test_ldr_r64_sp_postinc, ldr(X1, postinc(SP, 0x2a)).unwrap(), "ldr x1, [sp], #0x2a";
        test_ldr_r32_r64_preinc_neg, ldr(W1, preinc(X2, -0x2a)).unwrap(), "ldr w1, [x2, #-0x2a]!";
        test_ldr_r32_r64_postinc_neg, ldr(W1, postinc(X2, -0x2a)).unwrap(), "ldr w1, [x2], #-0x2a";
        test_ldr_r64_r64_preinc_neg, ldr(X1, preinc(X2, -0x2a)).unwrap(), "ldr x1, [x2, #-0x2a]!";
        test_ldr_r64_r64_postinc_neg, ldr(X1, postinc(X2, -0x2a)).unwrap(), "ldr x1, [x2], #-0x2a";
        test_ldr_r32_sp_preinc_neg, ldr(W1, preinc(SP, -0x2a)).unwrap(), "ldr w1, [sp, #-0x2a]!";
        test_ldr_r32_sp_postinc_neg, ldr(W1, postinc(SP, -0x2a)).unwrap(), "ldr w1, [sp], #-0x2a";
        test_ldr_r64_sp_preinc_neg, ldr(X1, preinc(SP, -0x2a)).unwrap(), "ldr x1, [sp, #-0x2a]!";
        test_ldr_r64_sp_postinc_neg, ldr(X1, postinc(SP, -0x2a)).unwrap(), "ldr x1, [sp], #-0x2a";
        test_ldr_r32_sp_preinc2, ldr(W1, preinc(SP, LdStIncOffset::new(0x2a).unwrap())), "ldr w1, [sp, #0x2a]!";
        test_ldr_r64_r64_preinc_neg2, ldr(X1, preinc(X2, LdStIncOffset::new(-0x2a).unwrap())), "ldr x1, [x2, #-0x2a]!";
        test_ldr_r32_r64_pre_inc, ldr(W1, (inc(0x2a), X2)).unwrap(), "ldr w1, [x2, #0x2a]!";
        test_ldr_r32_r64_post_inc, ldr(W1, (X2, inc(0x2a))).unwrap(), "ldr w1, [x2], #0x2a";
        test_ldr_r64_r64_pre_inc, ldr(X1, (inc(0x2a), X2)).unwrap(), "ldr x1, [x2, #0x2a]!";
        test_ldr_r64_r64_post_inc, ldr(X1, (X2, inc(0x2a))).unwrap(), "ldr x1, [x2], #0x2a";
        test_ldr_r32_sp_pre_inc, ldr(W1, (inc(0x2a), SP)).unwrap(), "ldr w1, [sp, #0x2a]!";
        test_ldr_r32_sp_post_inc, ldr(W1, (SP, inc(0x2a))).unwrap(), "ldr w1, [sp], #0x2a";
        test_ldr_r64_sp_pre_inc, ldr(X1, (inc(0x2a), SP)).unwrap(), "ldr x1, [sp, #0x2a]!";
        test_ldr_r64_sp_post_inc, ldr(X1, (SP, inc(0x2a))).unwrap(), "ldr x1, [sp], #0x2a";
        test_ldr_r32_r64_pre_inc_neg, ldr(W1, (inc(-0x2a), X2)).unwrap(), "ldr w1, [x2, #-0x2a]!";
        test_ldr_r32_r64_post_inc_neg, ldr(W1, (X2, inc(-0x2a))).unwrap(), "ldr w1, [x2], #-0x2a";
        test_ldr_r64_r64_pre_inc_neg, ldr(X1, (inc(-0x2a), X2)).unwrap(), "ldr x1, [x2, #-0x2a]!";
        test_ldr_r64_r64_post_inc_neg, ldr(X1, (X2, inc(-0x2a))).unwrap(), "ldr x1, [x2], #-0x2a";
        test_ldr_r32_sp_pre_inc_neg, ldr(W1, (inc(-0x2a), SP)).unwrap(), "ldr w1, [sp, #-0x2a]!";
        test_ldr_r32_sp_post_inc_neg, ldr(W1, (SP, inc(-0x2a))).unwrap(), "ldr w1, [sp], #-0x2a";
        test_ldr_r64_sp_pre_inc_neg, ldr(X1, (inc(-0x2a), SP)).unwrap(), "ldr x1, [sp, #-0x2a]!";
        test_ldr_r64_sp_post_inc_neg, ldr(X1, (SP, inc(-0x2a))).unwrap(), "ldr x1, [sp], #-0x2a";
        test_ldr_r32_sp_pre_inc2, ldr(W1, (inc(LdStIncOffset::new(0x2a).unwrap()), SP)), "ldr w1, [sp, #0x2a]!";
        test_ldr_r64_r64_pre_inc_neg2, ldr(X1, (inc(LdStIncOffset::new(-0x2a).unwrap()), X2)), "ldr x1, [x2, #-0x2a]!";
        test_ldr_xzr_r64_pre_inc, ldr(XZR, (inc(0x2a), X2)).unwrap(), "ldr xzr, [x2, #0x2a]!";
        test_ldr_xzr_r64_post_inc, ldr(XZR, (X2, inc(0x2a))).unwrap(), "ldr xzr, [x2], #0x2a";
        test_ldr_wzr_r64_pre_inc, ldr(WZR, (inc(0x2a), X2)).unwrap(), "ldr wzr, [x2, #0x2a]!";
        test_ldr_wzr_r64_post_inc, ldr(WZR, (X2, inc(0x2a))).unwrap(), "ldr wzr, [x2], #0x2a";
    }
}
