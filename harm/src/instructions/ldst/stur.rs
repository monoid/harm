/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use aarchmrs_instructions::A64::ldst::ldst_unscaled::{
    STUR_32_ldst_unscaled::STUR_32_ldst_unscaled, STUR_64_ldst_unscaled::STUR_64_ldst_unscaled,
};

use crate::bits::BitError;
use crate::instructions::RawInstruction;
use crate::register::{RegOrSp64, RegOrZero32, RegOrZero64, Register};
use crate::sealed::Sealed;

use super::UnscaledOffset;

/// A `stur` instruction with a destination and an address.
pub struct Stur<Rt, Addr> {
    rt: Rt,
    addr: Addr,
}

impl<Rt, Addr> Stur<Rt, Addr> {
    pub fn rt(&self) -> &Rt {
        &self.rt
    }

    pub fn addr(&self) -> &Addr {
        &self.addr
    }
}

impl<Rt, Addr> Sealed for Stur<Rt, Addr> {}

/// Defines possible was to construct a `stur` instruction.
pub trait MakeStur<Rt, Addr>: Sealed {
    /// Allows defining both faillible and infallible constructors.
    type Output;

    fn new(rt: Rt, addr: Addr) -> Self::Output;
}

define_unscaled_imm_offset_rules!(Stur, MakeStur, STUR, RegOrZero64, 64);
define_unscaled_imm_offset_rules!(Stur, MakeStur, STUR, RegOrZero32, 32);

pub fn stur<TargetInp, TargetOut, AddrInp, AddrOut>(
    dst: TargetInp,
    addr: AddrInp,
) -> <Stur<TargetOut, AddrOut> as MakeStur<TargetInp, AddrInp>>::Output
where
    Stur<TargetOut, AddrOut>: MakeStur<TargetInp, AddrInp>,
{
    Stur::new(dst, addr)
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

    // 'stur (x1|w1|xzr|wzr), [(x2|sp), (-1|1|255|-256|0)]
    const STUR_DB: &str = "
f81ff041	stur x1, [x2, -1]
f8001041	stur x1, [x2, 1]
f80ff041	stur x1, [x2, 255]
f8100041	stur x1, [x2, -256]
f8000041	stur x1, [x2, 0]
f8000041	stur x1, [x2]
f81ff3e1	stur x1, [sp, -1]
f80013e1	stur x1, [sp, 1]
f80ff3e1	stur x1, [sp, 255]
f81003e1	stur x1, [sp, -256]
f80003e1	stur x1, [sp, 0]
b81ff041	stur w1, [x2, -1]
b8001041	stur w1, [x2, 1]
b80ff041	stur w1, [x2, 255]
b8100041	stur w1, [x2, -256]
b8000041	stur w1, [x2, 0]
b81ff3e1	stur w1, [sp, -1]
b80013e1	stur w1, [sp, 1]
b80ff3e1	stur w1, [sp, 255]
b81003e1	stur w1, [sp, -256]
b80003e1	stur w1, [sp, 0]
f81ff05f	stur xzr, [x2, -1]
f800105f	stur xzr, [x2, 1]
f80ff05f	stur xzr, [x2, 255]
f810005f	stur xzr, [x2, -256]
f800005f	stur xzr, [x2, 0]
f81ff3ff	stur xzr, [sp, -1]
f80013ff	stur xzr, [sp, 1]
f80ff3ff	stur xzr, [sp, 255]
f81003ff	stur xzr, [sp, -256]
f80003ff	stur xzr, [sp, 0]
b81ff05f	stur wzr, [x2, -1]
b800105f	stur wzr, [x2, 1]
b80ff05f	stur wzr, [x2, 255]
b810005f	stur wzr, [x2, -256]
b800005f	stur wzr, [x2, 0]
b81ff3ff	stur wzr, [sp, -1]
b80013ff	stur wzr, [sp, 1]
b80ff3ff	stur wzr, [sp, 255]
b81003ff	stur wzr, [sp, -256]
b80003ff	stur wzr, [sp, 0]
";

    test_cases! {
        STUR_DB, untested_stur_cases;
        test_stur_x1_x2_m1, stur(X1, (X2, -1)).unwrap(), "stur x1, [x2, -1]";
        test_stur_x1_x2_1, stur(X1, (X2, 1)).unwrap(), "stur x1, [x2, 1]";
        test_stur_x1_x2_255, stur(X1, (X2, 255)).unwrap(), "stur x1, [x2, 255]";
        test_stur_x1_x2_m256, stur(X1, (X2, -256)).unwrap(), "stur x1, [x2, -256]";
        test_stur_x1_x2_0, stur(X1, (X2, 0)).unwrap(), "stur x1, [x2, 0]";
        test_stur_x1_x2_simple, stur(X1, (X2,)), "stur x1, [x2]";
        test_stur_x1_sp_m1, stur(X1, (SP, -1)).unwrap(), "stur x1, [sp, -1]";
        test_stur_x1_sp_1, stur(X1, (SP, 1)).unwrap(), "stur x1, [sp, 1]";
        test_stur_x1_sp_255, stur(X1, (SP, 255)).unwrap(), "stur x1, [sp, 255]";
        test_stur_x1_sp_m256, stur(X1, (SP, -256)).unwrap(), "stur x1, [sp, -256]";
        test_stur_x1_sp_0, stur(X1, (SP, 0)).unwrap(), "stur x1, [sp, 0]";
        test_stur_w1_x2_m1, stur(W1, (X2, -1)).unwrap(), "stur w1, [x2, -1]";
        test_stur_w1_x2_1, stur(W1, (X2, 1)).unwrap(), "stur w1, [x2, 1]";
        test_stur_w1_x2_255, stur(W1, (X2, 255)).unwrap(), "stur w1, [x2, 255]";
        test_stur_w1_x2_m256, stur(W1, (X2, -256)).unwrap(), "stur w1, [x2, -256]";
        test_stur_w1_x2_0, stur(W1, (X2, 0)).unwrap(), "stur w1, [x2, 0]";
        test_stur_w1_sp_m1, stur(W1, (SP, -1)).unwrap(), "stur w1, [sp, -1]";
        test_stur_w1_sp_1, stur(W1, (SP, 1)).unwrap(), "stur w1, [sp, 1]";
        test_stur_w1_sp_255, stur(W1, (SP, 255)).unwrap(), "stur w1, [sp, 255]";
        test_stur_w1_sp_m256, stur(W1, (SP, -256)).unwrap(), "stur w1, [sp, -256]";
        test_stur_w1_sp_0, stur(W1, (SP, 0)).unwrap(), "stur w1, [sp, 0]";
        test_stur_xzr_x2_m1, stur(XZR, (X2, -1)).unwrap(), "stur xzr, [x2, -1]";
        test_stur_xzr_x2_1, stur(XZR, (X2, 1)).unwrap(), "stur xzr, [x2, 1]";
        test_stur_xzr_x2_255, stur(XZR, (X2, 255)).unwrap(), "stur xzr, [x2, 255]";
        test_stur_xzr_x2_m256, stur(XZR, (X2, -256)).unwrap(), "stur xzr, [x2, -256]";
        test_stur_xzr_x2_0, stur(XZR, (X2, 0)).unwrap(), "stur xzr, [x2, 0]";
        test_stur_xzr_sp_m1, stur(XZR, (SP, -1)).unwrap(), "stur xzr, [sp, -1]";
        test_stur_xzr_sp_1, stur(XZR, (SP, 1)).unwrap(), "stur xzr, [sp, 1]";
        test_stur_xzr_sp_255, stur(XZR, (SP, 255)).unwrap(), "stur xzr, [sp, 255]";
        test_stur_xzr_sp_m256, stur(XZR, (SP, -256)).unwrap(), "stur xzr, [sp, -256]";
        test_stur_xzr_sp_0, stur(XZR, (SP, 0)).unwrap(), "stur xzr, [sp, 0]";
        test_stur_wzr_x2_m1, stur(WZR, (X2, -1)).unwrap(), "stur wzr, [x2, -1]";
        test_stur_wzr_x2_1, stur(WZR, (X2, 1)).unwrap(), "stur wzr, [x2, 1]";
        test_stur_wzr_x2_255, stur(WZR, (X2, 255)).unwrap(), "stur wzr, [x2, 255]";
        test_stur_wzr_x2_m256, stur(WZR, (X2, -256)).unwrap(), "stur wzr, [x2, -256]";
        test_stur_wzr_x2_0, stur(WZR, (X2, 0)).unwrap(), "stur wzr, [x2, 0]";
        test_stur_wzr_sp_m1, stur(WZR, (SP, -1)).unwrap(), "stur wzr, [sp, -1]";
        test_stur_wzr_sp_1, stur(WZR, (SP, 1)).unwrap(), "stur wzr, [sp, 1]";
        test_stur_wzr_sp_255, stur(WZR, (SP, 255)).unwrap(), "stur wzr, [sp, 255]";
        test_stur_wzr_sp_m256, stur(WZR, (SP, -256)).unwrap(), "stur wzr, [sp, -256]";
        test_stur_wzr_sp_0, stur(WZR, (SP, 0)).unwrap(), "stur wzr, [sp, 0]";
    }
}
