/* Copyright (C) 2026 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use aarchmrs_instructions::A64::ldst::ldst_unpriv::{
    STTR_32_ldst_unpriv::STTR_32_ldst_unpriv, STTR_64_ldst_unpriv::STTR_64_ldst_unpriv,
};

use crate::bits::BitError;
use crate::instructions::RawInstruction;
use crate::register::{IntoReg, RegOrSp64, RegOrZero32, RegOrZero64, Register};
use crate::sealed::Sealed;

use super::UnscaledOffset;

/// A `sttr` instruction with a destination and an address.
pub struct Sttr<Rt, Addr> {
    rt: Rt,
    addr: Addr,
}

impl<Rt, Addr> Sttr<Rt, Addr> {
    pub fn rt(&self) -> &Rt {
        &self.rt
    }

    pub fn addr(&self) -> &Addr {
        &self.addr
    }
}

impl<Rt, Addr> Sealed for Sttr<Rt, Addr> {}

/// Defines possible ways to construct a `sttr` instruction.
pub trait MakeSttr<Rt, Addr>: Sealed {
    /// Allows defining both fallible and infallible constructors.
    type Output;

    fn new(rt: Rt, addr: Addr) -> Self::Output;
}

define_unscaled_imm_offset_rules!(Sttr, MakeSttr, STTR, RegOrZero64, 64, "ldst_unpriv");
define_unscaled_imm_offset_rules!(Sttr, MakeSttr, STTR, RegOrZero32, 32, "ldst_unpriv");

pub fn sttr<TargetInp, TargetOut, AddrInp, AddrOut>(
    dst: TargetInp,
    addr: AddrInp,
) -> <Sttr<TargetOut, AddrOut> as MakeSttr<TargetInp, AddrInp>>::Output
where
    Sttr<TargetOut, AddrOut>: MakeSttr<TargetInp, AddrInp>,
{
    Sttr::new(dst, addr)
}

#[cfg(test)]
mod tests {
    use harm_test_utils::test_cases;

    use super::*;
    use crate::instructions::InstructionSeq;
    use crate::register::Reg32::*;
    use crate::register::Reg64::*;
    use RegOrSp64::SP;
    use RegOrZero32::WZR;
    use RegOrZero64::XZR;

    // 'sttr (x1|w1|xzr|wzr), [(x2|sp), (-1|1|255|-256|0)]
    const STTR_DB: &str = "
f81ff841	sttr x1, [x2, -1]
f8001841	sttr x1, [x2, 1]
f80ff841	sttr x1, [x2, 255]
f8100841	sttr x1, [x2, -256]
f8000841	sttr x1, [x2, 0]
f8000841	sttr x1, [x2]
f81ffbe1	sttr x1, [sp, -1]
f8001be1	sttr x1, [sp, 1]
f80ffbe1	sttr x1, [sp, 255]
f8100be1	sttr x1, [sp, -256]
f8000be1	sttr x1, [sp, 0]
b81ff841	sttr w1, [x2, -1]
b8001841	sttr w1, [x2, 1]
b80ff841	sttr w1, [x2, 255]
b8100841	sttr w1, [x2, -256]
b8000841	sttr w1, [x2, 0]
b81ffbe1	sttr w1, [sp, -1]
b8001be1	sttr w1, [sp, 1]
b80ffbe1	sttr w1, [sp, 255]
b8100be1	sttr w1, [sp, -256]
b8000be1	sttr w1, [sp, 0]
f81ff85f	sttr xzr, [x2, -1]
f800185f	sttr xzr, [x2, 1]
f80ff85f	sttr xzr, [x2, 255]
f810085f	sttr xzr, [x2, -256]
f800085f	sttr xzr, [x2, 0]
f81ffbff	sttr xzr, [sp, -1]
f8001bff	sttr xzr, [sp, 1]
f80ffbff	sttr xzr, [sp, 255]
f8100bff	sttr xzr, [sp, -256]
f8000bff	sttr xzr, [sp, 0]
b81ff85f	sttr wzr, [x2, -1]
b800185f	sttr wzr, [x2, 1]
b80ff85f	sttr wzr, [x2, 255]
b810085f	sttr wzr, [x2, -256]
b800085f	sttr wzr, [x2, 0]
b81ffbff	sttr wzr, [sp, -1]
b8001bff	sttr wzr, [sp, 1]
b80ffbff	sttr wzr, [sp, 255]
b8100bff	sttr wzr, [sp, -256]
b8000bff	sttr wzr, [sp, 0]
";

    test_cases! {
        STTR_DB, untested_sttr_cases;
        test_sttr_x1_x2_m1, sttr(X1, (X2, -1)).unwrap(), "sttr x1, [x2, -1]";
        test_sttr_x1_x2_1, sttr(X1, (X2, 1)).unwrap(), "sttr x1, [x2, 1]";
        test_sttr_x1_x2_255, sttr(X1, (X2, 255)).unwrap(), "sttr x1, [x2, 255]";
        test_sttr_x1_x2_m256, sttr(X1, (X2, -256)).unwrap(), "sttr x1, [x2, -256]";
        test_sttr_x1_x2_0, sttr(X1, (X2, 0)).unwrap(), "sttr x1, [x2, 0]";
        test_sttr_x1_x2_simple, sttr(X1, (X2,)), "sttr x1, [x2]";
        test_sttr_x1_sp_m1, sttr(X1, (SP, -1)).unwrap(), "sttr x1, [sp, -1]";
        test_sttr_x1_sp_1, sttr(X1, (SP, 1)).unwrap(), "sttr x1, [sp, 1]";
        test_sttr_x1_sp_255, sttr(X1, (SP, 255)).unwrap(), "sttr x1, [sp, 255]";
        test_sttr_x1_sp_m256, sttr(X1, (SP, -256)).unwrap(), "sttr x1, [sp, -256]";
        test_sttr_x1_sp_0, sttr(X1, (SP, 0)).unwrap(), "sttr x1, [sp, 0]";
        test_sttr_w1_x2_m1, sttr(W1, (X2, -1)).unwrap(), "sttr w1, [x2, -1]";
        test_sttr_w1_x2_1, sttr(W1, (X2, 1)).unwrap(), "sttr w1, [x2, 1]";
        test_sttr_w1_x2_255, sttr(W1, (X2, 255)).unwrap(), "sttr w1, [x2, 255]";
        test_sttr_w1_x2_m256, sttr(W1, (X2, -256)).unwrap(), "sttr w1, [x2, -256]";
        test_sttr_w1_x2_0, sttr(W1, (X2, 0)).unwrap(), "sttr w1, [x2, 0]";
        test_sttr_w1_sp_m1, sttr(W1, (SP, -1)).unwrap(), "sttr w1, [sp, -1]";
        test_sttr_w1_sp_1, sttr(W1, (SP, 1)).unwrap(), "sttr w1, [sp, 1]";
        test_sttr_w1_sp_255, sttr(W1, (SP, 255)).unwrap(), "sttr w1, [sp, 255]";
        test_sttr_w1_sp_m256, sttr(W1, (SP, -256)).unwrap(), "sttr w1, [sp, -256]";
        test_sttr_w1_sp_0, sttr(W1, (SP, 0)).unwrap(), "sttr w1, [sp, 0]";
        test_sttr_xzr_x2_m1, sttr(XZR, (X2, -1)).unwrap(), "sttr xzr, [x2, -1]";
        test_sttr_xzr_x2_1, sttr(XZR, (X2, 1)).unwrap(), "sttr xzr, [x2, 1]";
        test_sttr_xzr_x2_255, sttr(XZR, (X2, 255)).unwrap(), "sttr xzr, [x2, 255]";
        test_sttr_xzr_x2_m256, sttr(XZR, (X2, -256)).unwrap(), "sttr xzr, [x2, -256]";
        test_sttr_xzr_x2_0, sttr(XZR, (X2, 0)).unwrap(), "sttr xzr, [x2, 0]";
        test_sttr_xzr_sp_m1, sttr(XZR, (SP, -1)).unwrap(), "sttr xzr, [sp, -1]";
        test_sttr_xzr_sp_1, sttr(XZR, (SP, 1)).unwrap(), "sttr xzr, [sp, 1]";
        test_sttr_xzr_sp_255, sttr(XZR, (SP, 255)).unwrap(), "sttr xzr, [sp, 255]";
        test_sttr_xzr_sp_m256, sttr(XZR, (SP, -256)).unwrap(), "sttr xzr, [sp, -256]";
        test_sttr_xzr_sp_0, sttr(XZR, (SP, 0)).unwrap(), "sttr xzr, [sp, 0]";
        test_sttr_wzr_x2_m1, sttr(WZR, (X2, -1)).unwrap(), "sttr wzr, [x2, -1]";
        test_sttr_wzr_x2_1, sttr(WZR, (X2, 1)).unwrap(), "sttr wzr, [x2, 1]";
        test_sttr_wzr_x2_255, sttr(WZR, (X2, 255)).unwrap(), "sttr wzr, [x2, 255]";
        test_sttr_wzr_x2_m256, sttr(WZR, (X2, -256)).unwrap(), "sttr wzr, [x2, -256]";
        test_sttr_wzr_x2_0, sttr(WZR, (X2, 0)).unwrap(), "sttr wzr, [x2, 0]";
        test_sttr_wzr_sp_m1, sttr(WZR, (SP, -1)).unwrap(), "sttr wzr, [sp, -1]";
        test_sttr_wzr_sp_1, sttr(WZR, (SP, 1)).unwrap(), "sttr wzr, [sp, 1]";
        test_sttr_wzr_sp_255, sttr(WZR, (SP, 255)).unwrap(), "sttr wzr, [sp, 255]";
        test_sttr_wzr_sp_m256, sttr(WZR, (SP, -256)).unwrap(), "sttr wzr, [sp, -256]";
        test_sttr_wzr_sp_0, sttr(WZR, (SP, 0)).unwrap(), "sttr wzr, [sp, 0]";
    }
}
