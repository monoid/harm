/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use aarchmrs_instructions::A64::ldst::ldst_unscaled::STURB_32_ldst_unscaled::STURB_32_ldst_unscaled;

use crate::bits::BitError;
use crate::instructions::RawInstruction;
use crate::register::{IntoCode, RegOrSp64, RegOrZero32};
use crate::sealed::Sealed;

use super::UnscaledOffset;

/// A `STURB` instruction with a destination and an address.
pub struct Sturb<Rt, Addr> {
    rt: Rt,
    addr: Addr,
}

impl<Rt, Addr> Sturb<Rt, Addr> {
    pub fn rt(&self) -> &Rt {
        &self.rt
    }

    pub fn addr(&self) -> &Addr {
        &self.addr
    }
}

impl<Rt, Addr> Sealed for Sturb<Rt, Addr> {}

/// Defines possible was to construct a `sturb` instruction.
pub trait MakeSturb<Rt, Addr>: Sealed {
    /// Allows defining both faillible and infallible constructors.
    type Output;

    fn new(rt: Rt, addr: Addr) -> Self::Output;
}

define_unscaled_imm_offset_rules!(Sturb, MakeSturb, STURB, RegOrZero32, 32);

pub fn sturb<TargetInp, TargetOut, AddrInp, AddrOut>(
    dst: TargetInp,
    addr: AddrInp,
) -> <Sturb<TargetOut, AddrOut> as MakeSturb<TargetInp, AddrInp>>::Output
where
    Sturb<TargetOut, AddrOut>: MakeSturb<TargetInp, AddrInp>,
{
    Sturb::new(dst, addr)
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

    const STURB_DB: &str = "
381ff041	sturb w1, [x2, -1]
38001041	sturb w1, [x2, 1]
380ff041	sturb w1, [x2, 255]
38100041	sturb w1, [x2, -256]
38000041	sturb w1, [x2, 0]
38000041	sturb w1, [x2]
381ff3e1	sturb w1, [sp, -1]
380013e1	sturb w1, [sp, 1]
380ff3e1	sturb w1, [sp, 255]
381003e1	sturb w1, [sp, -256]
380003e1	sturb w1, [sp, 0]
381ff05f	sturb wzr, [x2, -1]
3800105f	sturb wzr, [x2, 1]
380ff05f	sturb wzr, [x2, 255]
3810005f	sturb wzr, [x2, -256]
3800005f	sturb wzr, [x2, 0]
381ff3ff	sturb wzr, [sp, -1]
380013ff	sturb wzr, [sp, 1]
380ff3ff	sturb wzr, [sp, 255]
381003ff	sturb wzr, [sp, -256]
380003ff	sturb wzr, [sp, 0]
";

    test_cases! {
        STURB_DB, untested_sturb_cases;
        test_sturb_w1_x2_m1, sturb(W1, (X2, -1)).unwrap(), "sturb w1, [x2, -1]";
        test_sturb_w1_x2_1, sturb(W1, (X2, 1)).unwrap(), "sturb w1, [x2, 1]";
        test_sturb_w1_x2_255, sturb(W1, (X2, 255)).unwrap(), "sturb w1, [x2, 255]";
        test_sturb_w1_x2_m256, sturb(W1, (X2, -256)).unwrap(), "sturb w1, [x2, -256]";
        test_sturb_w1_x2_0, sturb(W1, (X2, 0)).unwrap(), "sturb w1, [x2, 0]";
        test_sturb_w1_x2_simple, sturb(W1, (X2,)), "sturb w1, [x2]";
        test_sturb_w1_sp_m1, sturb(W1, (SP, -1)).unwrap(), "sturb w1, [sp, -1]";
        test_sturb_w1_sp_1, sturb(W1, (SP, 1)).unwrap(), "sturb w1, [sp, 1]";
        test_sturb_w1_sp_255, sturb(W1, (SP, 255)).unwrap(), "sturb w1, [sp, 255]";
        test_sturb_w1_sp_m256, sturb(W1, (SP, -256)).unwrap(), "sturb w1, [sp, -256]";
        test_sturb_w1_sp_0, sturb(W1, (SP, 0)).unwrap(), "sturb w1, [sp, 0]";
        test_sturb_wzr_x2_m1, sturb(WZR, (X2, -1)).unwrap(), "sturb wzr, [x2, -1]";
        test_sturb_wzr_x2_1, sturb(WZR, (X2, 1)).unwrap(), "sturb wzr, [x2, 1]";
        test_sturb_wzr_x2_255, sturb(WZR, (X2, 255)).unwrap(), "sturb wzr, [x2, 255]";
        test_sturb_wzr_x2_m256, sturb(WZR, (X2, -256)).unwrap(), "sturb wzr, [x2, -256]";
        test_sturb_wzr_x2_0, sturb(WZR, (X2, 0)).unwrap(), "sturb wzr, [x2, 0]";
        test_sturb_wzr_sp_m1, sturb(WZR, (SP, -1)).unwrap(), "sturb wzr, [sp, -1]";
        test_sturb_wzr_sp_1, sturb(WZR, (SP, 1)).unwrap(), "sturb wzr, [sp, 1]";
        test_sturb_wzr_sp_255, sturb(WZR, (SP, 255)).unwrap(), "sturb wzr, [sp, 255]";
        test_sturb_wzr_sp_m256, sturb(WZR, (SP, -256)).unwrap(), "sturb wzr, [sp, -256]";
        test_sturb_wzr_sp_0, sturb(WZR, (SP, 0)).unwrap(), "sturb wzr, [sp, 0]";
    }
}
