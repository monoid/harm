/* Copyright (C) 2026 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use aarchmrs_instructions::A64::ldst::ldst_unpriv::STTRH_32_ldst_unpriv::STTRH_32_ldst_unpriv;

use crate::bits::BitError;
use crate::instructions::RawInstruction;
use crate::register::{IntoReg, RegOrSp64, RegOrZero32, Register};
use crate::sealed::Sealed;

use super::UnscaledOffset;

/// A `STTRH` instruction with a destination and an address.
pub struct Sttrh<Rt, Addr> {
    rt: Rt,
    addr: Addr,
}

impl<Rt, Addr> Sttrh<Rt, Addr> {
    pub fn rt(&self) -> &Rt {
        &self.rt
    }

    pub fn addr(&self) -> &Addr {
        &self.addr
    }
}

impl<Rt, Addr> Sealed for Sttrh<Rt, Addr> {}

/// Defines possible ways to construct a `sttrh` instruction.
pub trait MakeSttrh<Rt, Addr>: Sealed {
    /// Allows defining both fallible and infallible constructors.
    type Output;

    fn new(rt: Rt, addr: Addr) -> Self::Output;
}

define_unscaled_imm_offset_rules!(Sttrh, MakeSttrh, STTRH, RegOrZero32, 32, "ldst_unpriv");

pub fn sttrh<TargetInp, TargetOut, AddrInp, AddrOut>(
    dst: TargetInp,
    addr: AddrInp,
) -> <Sttrh<TargetOut, AddrOut> as MakeSttrh<TargetInp, AddrInp>>::Output
where
    Sttrh<TargetOut, AddrOut>: MakeSttrh<TargetInp, AddrInp>,
{
    Sttrh::new(dst, addr)
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

    const STTRH_DB: &str = "
781ff841	sttrh w1, [x2, -1]
78001841	sttrh w1, [x2, 1]
780ff841	sttrh w1, [x2, 255]
78100841	sttrh w1, [x2, -256]
78000841	sttrh w1, [x2, 0]
78000841	sttrh w1, [x2]
781ffbe1	sttrh w1, [sp, -1]
78001be1	sttrh w1, [sp, 1]
780ffbe1	sttrh w1, [sp, 255]
78100be1	sttrh w1, [sp, -256]
78000be1	sttrh w1, [sp, 0]
781ff85f	sttrh wzr, [x2, -1]
7800185f	sttrh wzr, [x2, 1]
780ff85f	sttrh wzr, [x2, 255]
7810085f	sttrh wzr, [x2, -256]
7800085f	sttrh wzr, [x2, 0]
781ffbff	sttrh wzr, [sp, -1]
78001bff	sttrh wzr, [sp, 1]
780ffbff	sttrh wzr, [sp, 255]
78100bff	sttrh wzr, [sp, -256]
78000bff	sttrh wzr, [sp, 0]
";

    test_cases! {
        STTRH_DB, untested_sttrh_cases;
        test_sttrh_w1_x2_m1, sttrh(W1, (X2, -1)).unwrap(), "sttrh w1, [x2, -1]";
        test_sttrh_w1_x2_1, sttrh(W1, (X2, 1)).unwrap(), "sttrh w1, [x2, 1]";
        test_sttrh_w1_x2_255, sttrh(W1, (X2, 255)).unwrap(), "sttrh w1, [x2, 255]";
        test_sttrh_w1_x2_m256, sttrh(W1, (X2, -256)).unwrap(), "sttrh w1, [x2, -256]";
        test_sttrh_w1_x2_0, sttrh(W1, (X2, 0)).unwrap(), "sttrh w1, [x2, 0]";
        test_sttrh_w1_x2_simple, sttrh(W1, (X2,)), "sttrh w1, [x2]";
        test_sttrh_w1_sp_m1, sttrh(W1, (SP, -1)).unwrap(), "sttrh w1, [sp, -1]";
        test_sttrh_w1_sp_1, sttrh(W1, (SP, 1)).unwrap(), "sttrh w1, [sp, 1]";
        test_sttrh_w1_sp_255, sttrh(W1, (SP, 255)).unwrap(), "sttrh w1, [sp, 255]";
        test_sttrh_w1_sp_m256, sttrh(W1, (SP, -256)).unwrap(), "sttrh w1, [sp, -256]";
        test_sttrh_w1_sp_0, sttrh(W1, (SP, 0)).unwrap(), "sttrh w1, [sp, 0]";
        test_sttrh_wzr_x2_m1, sttrh(WZR, (X2, -1)).unwrap(), "sttrh wzr, [x2, -1]";
        test_sttrh_wzr_x2_1, sttrh(WZR, (X2, 1)).unwrap(), "sttrh wzr, [x2, 1]";
        test_sttrh_wzr_x2_255, sttrh(WZR, (X2, 255)).unwrap(), "sttrh wzr, [x2, 255]";
        test_sttrh_wzr_x2_m256, sttrh(WZR, (X2, -256)).unwrap(), "sttrh wzr, [x2, -256]";
        test_sttrh_wzr_x2_0, sttrh(WZR, (X2, 0)).unwrap(), "sttrh wzr, [x2, 0]";
        test_sttrh_wzr_sp_m1, sttrh(WZR, (SP, -1)).unwrap(), "sttrh wzr, [sp, -1]";
        test_sttrh_wzr_sp_1, sttrh(WZR, (SP, 1)).unwrap(), "sttrh wzr, [sp, 1]";
        test_sttrh_wzr_sp_255, sttrh(WZR, (SP, 255)).unwrap(), "sttrh wzr, [sp, 255]";
        test_sttrh_wzr_sp_m256, sttrh(WZR, (SP, -256)).unwrap(), "sttrh wzr, [sp, -256]";
        test_sttrh_wzr_sp_0, sttrh(WZR, (SP, 0)).unwrap(), "sttrh wzr, [sp, 0]";
    }
}
