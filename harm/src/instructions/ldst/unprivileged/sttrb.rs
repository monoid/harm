/* Copyright (C) 2026 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use aarchmrs_instructions::A64::ldst::ldst_unpriv::STTRB_32_ldst_unpriv::STTRB_32_ldst_unpriv;

use crate::bits::BitError;
use crate::instructions::RawInstruction;
use crate::register::{IntoReg, RegOrSp64, RegOrZero32, Register};
use crate::sealed::Sealed;

use super::UnscaledOffset;

/// A `STTRB` instruction with a destination and an address.
pub struct Sttrb<Rt, Addr> {
    rt: Rt,
    addr: Addr,
}

impl<Rt, Addr> Sttrb<Rt, Addr> {
    pub fn rt(&self) -> &Rt {
        &self.rt
    }

    pub fn addr(&self) -> &Addr {
        &self.addr
    }
}

impl<Rt, Addr> Sealed for Sttrb<Rt, Addr> {}

/// Defines possible ways to construct a `sttrb` instruction.
pub trait MakeSttrb<Rt, Addr>: Sealed {
    /// Allows defining both fallible and infallible constructors.
    type Output;

    fn new(rt: Rt, addr: Addr) -> Self::Output;
}

define_unscaled_imm_offset_rules!(Sttrb, MakeSttrb, STTRB, RegOrZero32, 32, "ldst_unpriv");

pub fn sttrb<TargetInp, TargetOut, AddrInp, AddrOut>(
    dst: TargetInp,
    addr: AddrInp,
) -> <Sttrb<TargetOut, AddrOut> as MakeSttrb<TargetInp, AddrInp>>::Output
where
    Sttrb<TargetOut, AddrOut>: MakeSttrb<TargetInp, AddrInp>,
{
    Sttrb::new(dst, addr)
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

    const STTRB_DB: &str = "
381ff841	sttrb w1, [x2, -1]
38001841	sttrb w1, [x2, 1]
380ff841	sttrb w1, [x2, 255]
38100841	sttrb w1, [x2, -256]
38000841	sttrb w1, [x2, 0]
38000841	sttrb w1, [x2]
381ffbe1	sttrb w1, [sp, -1]
38001be1	sttrb w1, [sp, 1]
380ffbe1	sttrb w1, [sp, 255]
38100be1	sttrb w1, [sp, -256]
38000be1	sttrb w1, [sp, 0]
381ff85f	sttrb wzr, [x2, -1]
3800185f	sttrb wzr, [x2, 1]
380ff85f	sttrb wzr, [x2, 255]
3810085f	sttrb wzr, [x2, -256]
3800085f	sttrb wzr, [x2, 0]
381ffbff	sttrb wzr, [sp, -1]
38001bff	sttrb wzr, [sp, 1]
380ffbff	sttrb wzr, [sp, 255]
38100bff	sttrb wzr, [sp, -256]
38000bff	sttrb wzr, [sp, 0]
";

    test_cases! {
        STTRB_DB, untested_sttrb_cases;
        test_sttrb_w1_x2_m1, sttrb(W1, (X2, -1)).unwrap(), "sttrb w1, [x2, -1]";
        test_sttrb_w1_x2_1, sttrb(W1, (X2, 1)).unwrap(), "sttrb w1, [x2, 1]";
        test_sttrb_w1_x2_255, sttrb(W1, (X2, 255)).unwrap(), "sttrb w1, [x2, 255]";
        test_sttrb_w1_x2_m256, sttrb(W1, (X2, -256)).unwrap(), "sttrb w1, [x2, -256]";
        test_sttrb_w1_x2_0, sttrb(W1, (X2, 0)).unwrap(), "sttrb w1, [x2, 0]";
        test_sttrb_w1_x2_simple, sttrb(W1, (X2,)), "sttrb w1, [x2]";
        test_sttrb_w1_sp_m1, sttrb(W1, (SP, -1)).unwrap(), "sttrb w1, [sp, -1]";
        test_sttrb_w1_sp_1, sttrb(W1, (SP, 1)).unwrap(), "sttrb w1, [sp, 1]";
        test_sttrb_w1_sp_255, sttrb(W1, (SP, 255)).unwrap(), "sttrb w1, [sp, 255]";
        test_sttrb_w1_sp_m256, sttrb(W1, (SP, -256)).unwrap(), "sttrb w1, [sp, -256]";
        test_sttrb_w1_sp_0, sttrb(W1, (SP, 0)).unwrap(), "sttrb w1, [sp, 0]";
        test_sttrb_wzr_x2_m1, sttrb(WZR, (X2, -1)).unwrap(), "sttrb wzr, [x2, -1]";
        test_sttrb_wzr_x2_1, sttrb(WZR, (X2, 1)).unwrap(), "sttrb wzr, [x2, 1]";
        test_sttrb_wzr_x2_255, sttrb(WZR, (X2, 255)).unwrap(), "sttrb wzr, [x2, 255]";
        test_sttrb_wzr_x2_m256, sttrb(WZR, (X2, -256)).unwrap(), "sttrb wzr, [x2, -256]";
        test_sttrb_wzr_x2_0, sttrb(WZR, (X2, 0)).unwrap(), "sttrb wzr, [x2, 0]";
        test_sttrb_wzr_sp_m1, sttrb(WZR, (SP, -1)).unwrap(), "sttrb wzr, [sp, -1]";
        test_sttrb_wzr_sp_1, sttrb(WZR, (SP, 1)).unwrap(), "sttrb wzr, [sp, 1]";
        test_sttrb_wzr_sp_255, sttrb(WZR, (SP, 255)).unwrap(), "sttrb wzr, [sp, 255]";
        test_sttrb_wzr_sp_m256, sttrb(WZR, (SP, -256)).unwrap(), "sttrb wzr, [sp, -256]";
        test_sttrb_wzr_sp_0, sttrb(WZR, (SP, 0)).unwrap(), "sttrb wzr, [sp, 0]";
    }
}
