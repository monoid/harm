/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use aarchmrs_instructions::A64::ldst::ldst_unscaled::{
    LDUR_32_ldst_unscaled::LDUR_32_ldst_unscaled, LDUR_64_ldst_unscaled::LDUR_64_ldst_unscaled,
};

use crate::bits::BitError;
use crate::instructions::RawInstruction;
use crate::register::{IntoReg, RegOrSp64, RegOrZero32, RegOrZero64, Register};
use crate::sealed::Sealed;

use super::UnscaledOffset;

/// A `LDUR` instruction with a destination and an address.
pub struct Ldur<Rt, Addr> {
    rt: Rt,
    addr: Addr,
}

impl<Rt, Addr> Ldur<Rt, Addr> {
    pub fn rt(&self) -> &Rt {
        &self.rt
    }

    pub fn addr(&self) -> &Addr {
        &self.addr
    }
}

impl<Rt, Addr> Sealed for Ldur<Rt, Addr> {}

/// Defines possible was to construct a `ldur` instruction.
pub trait MakeLdur<Rt, Addr>: Sealed {
    /// Allows defining both faillible and infallible constructors.
    type Output;

    fn new(rt: Rt, addr: Addr) -> Self::Output;
}

define_unscaled_imm_offset_rules!(Ldur, MakeLdur, LDUR, RegOrZero64, 64);
define_unscaled_imm_offset_rules!(Ldur, MakeLdur, LDUR, RegOrZero32, 32);

pub fn ldur<TargetInp, TargetOut, AddrInp, AddrOut>(
    dst: TargetInp,
    addr: AddrInp,
) -> <Ldur<TargetOut, AddrOut> as MakeLdur<TargetInp, AddrInp>>::Output
where
    Ldur<TargetOut, AddrOut>: MakeLdur<TargetInp, AddrInp>,
{
    Ldur::new(dst, addr)
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

    // 'ldur (x1|w1|xzr|wzr), [(x2|sp), (-1|1|255|-256|0)]
    const LDUR_DB: &str = "
f85ff041	ldur x1, [x2, -1]
f8401041	ldur x1, [x2, 1]
f84ff041	ldur x1, [x2, 255]
f8500041	ldur x1, [x2, -256]
f8400041	ldur x1, [x2, 0]
f8400041	ldur x1, [x2]
f85ff3e1	ldur x1, [sp, -1]
f84013e1	ldur x1, [sp, 1]
f84ff3e1	ldur x1, [sp, 255]
f85003e1	ldur x1, [sp, -256]
f84003e1	ldur x1, [sp, 0]
b85ff041	ldur w1, [x2, -1]
b8401041	ldur w1, [x2, 1]
b84ff041	ldur w1, [x2, 255]
b8500041	ldur w1, [x2, -256]
b8400041	ldur w1, [x2, 0]
b85ff3e1	ldur w1, [sp, -1]
b84013e1	ldur w1, [sp, 1]
b84ff3e1	ldur w1, [sp, 255]
b85003e1	ldur w1, [sp, -256]
b84003e1	ldur w1, [sp, 0]
f85ff05f	ldur xzr, [x2, -1]
f840105f	ldur xzr, [x2, 1]
f84ff05f	ldur xzr, [x2, 255]
f850005f	ldur xzr, [x2, -256]
f840005f	ldur xzr, [x2, 0]
f85ff3ff	ldur xzr, [sp, -1]
f84013ff	ldur xzr, [sp, 1]
f84ff3ff	ldur xzr, [sp, 255]
f85003ff	ldur xzr, [sp, -256]
f84003ff	ldur xzr, [sp, 0]
b85ff05f	ldur wzr, [x2, -1]
b840105f	ldur wzr, [x2, 1]
b84ff05f	ldur wzr, [x2, 255]
b850005f	ldur wzr, [x2, -256]
b840005f	ldur wzr, [x2, 0]
b85ff3ff	ldur wzr, [sp, -1]
b84013ff	ldur wzr, [sp, 1]
b84ff3ff	ldur wzr, [sp, 255]
b85003ff	ldur wzr, [sp, -256]
b84003ff	ldur wzr, [sp, 0]
";

    test_cases! {
        LDUR_DB, untested_ldur_cases;
        test_ldur_x1_x2_m1, ldur(X1, (X2, -1)).unwrap(), "ldur x1, [x2, -1]";
        test_ldur_x1_x2_1, ldur(X1, (X2, 1)).unwrap(), "ldur x1, [x2, 1]";
        test_ldur_x1_x2_255, ldur(X1, (X2, 255)).unwrap(), "ldur x1, [x2, 255]";
        test_ldur_x1_x2_m256, ldur(X1, (X2, -256)).unwrap(), "ldur x1, [x2, -256]";
        test_ldur_x1_x2_0, ldur(X1, (X2, 0)).unwrap(), "ldur x1, [x2, 0]";
        test_ldur_x1_x2_simple, ldur(X1, (X2,)), "ldur x1, [x2]";
        test_ldur_x1_sp_m1, ldur(X1, (SP, -1)).unwrap(), "ldur x1, [sp, -1]";
        test_ldur_x1_sp_1, ldur(X1, (SP, 1)).unwrap(), "ldur x1, [sp, 1]";
        test_ldur_x1_sp_255, ldur(X1, (SP, 255)).unwrap(), "ldur x1, [sp, 255]";
        test_ldur_x1_sp_m256, ldur(X1, (SP, -256)).unwrap(), "ldur x1, [sp, -256]";
        test_ldur_x1_sp_0, ldur(X1, (SP, 0)).unwrap(), "ldur x1, [sp, 0]";
        test_ldur_w1_x2_m1, ldur(W1, (X2, -1)).unwrap(), "ldur w1, [x2, -1]";
        test_ldur_w1_x2_1, ldur(W1, (X2, 1)).unwrap(), "ldur w1, [x2, 1]";
        test_ldur_w1_x2_255, ldur(W1, (X2, 255)).unwrap(), "ldur w1, [x2, 255]";
        test_ldur_w1_x2_m256, ldur(W1, (X2, -256)).unwrap(), "ldur w1, [x2, -256]";
        test_ldur_w1_x2_0, ldur(W1, (X2, 0)).unwrap(), "ldur w1, [x2, 0]";
        test_ldur_w1_sp_m1, ldur(W1, (SP, -1)).unwrap(), "ldur w1, [sp, -1]";
        test_ldur_w1_sp_1, ldur(W1, (SP, 1)).unwrap(), "ldur w1, [sp, 1]";
        test_ldur_w1_sp_255, ldur(W1, (SP, 255)).unwrap(), "ldur w1, [sp, 255]";
        test_ldur_w1_sp_m256, ldur(W1, (SP, -256)).unwrap(), "ldur w1, [sp, -256]";
        test_ldur_w1_sp_0, ldur(W1, (SP, 0)).unwrap(), "ldur w1, [sp, 0]";
        test_ldur_xzr_x2_m1, ldur(XZR, (X2, -1)).unwrap(), "ldur xzr, [x2, -1]";
        test_ldur_xzr_x2_1, ldur(XZR, (X2, 1)).unwrap(), "ldur xzr, [x2, 1]";
        test_ldur_xzr_x2_255, ldur(XZR, (X2, 255)).unwrap(), "ldur xzr, [x2, 255]";
        test_ldur_xzr_x2_m256, ldur(XZR, (X2, -256)).unwrap(), "ldur xzr, [x2, -256]";
        test_ldur_xzr_x2_0, ldur(XZR, (X2, 0)).unwrap(), "ldur xzr, [x2, 0]";
        test_ldur_xzr_sp_m1, ldur(XZR, (SP, -1)).unwrap(), "ldur xzr, [sp, -1]";
        test_ldur_xzr_sp_1, ldur(XZR, (SP, 1)).unwrap(), "ldur xzr, [sp, 1]";
        test_ldur_xzr_sp_255, ldur(XZR, (SP, 255)).unwrap(), "ldur xzr, [sp, 255]";
        test_ldur_xzr_sp_m256, ldur(XZR, (SP, -256)).unwrap(), "ldur xzr, [sp, -256]";
        test_ldur_xzr_sp_0, ldur(XZR, (SP, 0)).unwrap(), "ldur xzr, [sp, 0]";
        test_ldur_wzr_x2_m1, ldur(WZR, (X2, -1)).unwrap(), "ldur wzr, [x2, -1]";
        test_ldur_wzr_x2_1, ldur(WZR, (X2, 1)).unwrap(), "ldur wzr, [x2, 1]";
        test_ldur_wzr_x2_255, ldur(WZR, (X2, 255)).unwrap(), "ldur wzr, [x2, 255]";
        test_ldur_wzr_x2_m256, ldur(WZR, (X2, -256)).unwrap(), "ldur wzr, [x2, -256]";
        test_ldur_wzr_x2_0, ldur(WZR, (X2, 0)).unwrap(), "ldur wzr, [x2, 0]";
        test_ldur_wzr_sp_m1, ldur(WZR, (SP, -1)).unwrap(), "ldur wzr, [sp, -1]";
        test_ldur_wzr_sp_1, ldur(WZR, (SP, 1)).unwrap(), "ldur wzr, [sp, 1]";
        test_ldur_wzr_sp_255, ldur(WZR, (SP, 255)).unwrap(), "ldur wzr, [sp, 255]";
        test_ldur_wzr_sp_m256, ldur(WZR, (SP, -256)).unwrap(), "ldur wzr, [sp, -256]";
        test_ldur_wzr_sp_0, ldur(WZR, (SP, 0)).unwrap(), "ldur wzr, [sp, 0]";
    }
}
