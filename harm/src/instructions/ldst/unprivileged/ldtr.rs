/* Copyright (C) 2026 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use aarchmrs_instructions::A64::ldst::ldst_unpriv::{
    LDTR_32_ldst_unpriv::LDTR_32_ldst_unpriv, LDTR_64_ldst_unpriv::LDTR_64_ldst_unpriv,
};

use crate::bits::BitError;
use crate::instructions::RawInstruction;
use crate::register::{IntoReg, RegOrSp64, RegOrZero32, RegOrZero64, Register};
use crate::sealed::Sealed;

use super::UnscaledOffset;

/// A `LDTR` instruction with a destination and an address.
pub struct Ldtr<Rt, Addr> {
    rt: Rt,
    addr: Addr,
}

impl<Rt, Addr> Ldtr<Rt, Addr> {
    pub fn rt(&self) -> &Rt {
        &self.rt
    }

    pub fn addr(&self) -> &Addr {
        &self.addr
    }
}

impl<Rt, Addr> Sealed for Ldtr<Rt, Addr> {}

/// Defines possible ways to construct a `ldtr` instruction.
pub trait MakeLdtr<Rt, Addr>: Sealed {
    /// Allows defining both fallible and infallible constructors.
    type Output;

    fn new(rt: Rt, addr: Addr) -> Self::Output;
}

pub fn ldtr<TargetInp, TargetOut, AddrInp, AddrOut>(
    dst: TargetInp,
    addr: AddrInp,
) -> <Ldtr<TargetOut, AddrOut> as MakeLdtr<TargetInp, AddrInp>>::Output
where
    Ldtr<TargetOut, AddrOut>: MakeLdtr<TargetInp, AddrInp>,
{
    Ldtr::new(dst, addr)
}

define_unscaled_imm_offset_rules!(Ldtr, MakeLdtr, LDTR, RegOrZero64, 64, "ldst_unpriv");
define_unscaled_imm_offset_rules!(Ldtr, MakeLdtr, LDTR, RegOrZero32, 32, "ldst_unpriv");

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

    // 'ldtr (x1|w1|xzr|wzr), [(x2|sp), (-1|1|255|-256|0)]
    const LDTR_DB: &str = "
f85ff841	ldtr x1, [x2, -1]
f8401841	ldtr x1, [x2, 1]
f84ff841	ldtr x1, [x2, 255]
f8500841	ldtr x1, [x2, -256]
f8400841	ldtr x1, [x2, 0]
f8400841	ldtr x1, [x2]
f85ffbe1	ldtr x1, [sp, -1]
f8401be1	ldtr x1, [sp, 1]
f84ffbe1	ldtr x1, [sp, 255]
f8500be1	ldtr x1, [sp, -256]
f8400be1	ldtr x1, [sp, 0]
b85ff841	ldtr w1, [x2, -1]
b8401841	ldtr w1, [x2, 1]
b84ff841	ldtr w1, [x2, 255]
b8500841	ldtr w1, [x2, -256]
b8400841	ldtr w1, [x2, 0]
b85ffbe1	ldtr w1, [sp, -1]
b8401be1	ldtr w1, [sp, 1]
b84ffbe1	ldtr w1, [sp, 255]
b8500be1	ldtr w1, [sp, -256]
b8400be1	ldtr w1, [sp, 0]
f85ff85f	ldtr xzr, [x2, -1]
f840185f	ldtr xzr, [x2, 1]
f84ff85f	ldtr xzr, [x2, 255]
f850085f	ldtr xzr, [x2, -256]
f840085f	ldtr xzr, [x2, 0]
f85ffbff	ldtr xzr, [sp, -1]
f8401bff	ldtr xzr, [sp, 1]
f84ffbff	ldtr xzr, [sp, 255]
f8500bff	ldtr xzr, [sp, -256]
f8400bff	ldtr xzr, [sp, 0]
b85ff85f	ldtr wzr, [x2, -1]
b840185f	ldtr wzr, [x2, 1]
b84ff85f	ldtr wzr, [x2, 255]
b850085f	ldtr wzr, [x2, -256]
b840085f	ldtr wzr, [x2, 0]
b85ffbff	ldtr wzr, [sp, -1]
b8401bff	ldtr wzr, [sp, 1]
b84ffbff	ldtr wzr, [sp, 255]
b8500bff	ldtr wzr, [sp, -256]
b8400bff	ldtr wzr, [sp, 0]
";

    test_cases! {
        LDTR_DB, untested_ldtr_cases;
        test_ldtr_x1_x2_m1, ldtr(X1, (X2, -1)).unwrap(), "ldtr x1, [x2, -1]";
        test_ldtr_x1_x2_1, ldtr(X1, (X2, 1)).unwrap(), "ldtr x1, [x2, 1]";
        test_ldtr_x1_x2_255, ldtr(X1, (X2, 255)).unwrap(), "ldtr x1, [x2, 255]";
        test_ldtr_x1_x2_m256, ldtr(X1, (X2, -256)).unwrap(), "ldtr x1, [x2, -256]";
        test_ldtr_x1_x2_0, ldtr(X1, (X2, 0)).unwrap(), "ldtr x1, [x2, 0]";
        test_ldtr_x1_x2_simple, ldtr(X1, (X2,)), "ldtr x1, [x2]";
        test_ldtr_x1_sp_m1, ldtr(X1, (SP, -1)).unwrap(), "ldtr x1, [sp, -1]";
        test_ldtr_x1_sp_1, ldtr(X1, (SP, 1)).unwrap(), "ldtr x1, [sp, 1]";
        test_ldtr_x1_sp_255, ldtr(X1, (SP, 255)).unwrap(), "ldtr x1, [sp, 255]";
        test_ldtr_x1_sp_m256, ldtr(X1, (SP, -256)).unwrap(), "ldtr x1, [sp, -256]";
        test_ldtr_x1_sp_0, ldtr(X1, (SP, 0)).unwrap(), "ldtr x1, [sp, 0]";
        test_ldtr_w1_x2_m1, ldtr(W1, (X2, -1)).unwrap(), "ldtr w1, [x2, -1]";
        test_ldtr_w1_x2_1, ldtr(W1, (X2, 1)).unwrap(), "ldtr w1, [x2, 1]";
        test_ldtr_w1_x2_255, ldtr(W1, (X2, 255)).unwrap(), "ldtr w1, [x2, 255]";
        test_ldtr_w1_x2_m256, ldtr(W1, (X2, -256)).unwrap(), "ldtr w1, [x2, -256]";
        test_ldtr_w1_x2_0, ldtr(W1, (X2, 0)).unwrap(), "ldtr w1, [x2, 0]";
        test_ldtr_w1_sp_m1, ldtr(W1, (SP, -1)).unwrap(), "ldtr w1, [sp, -1]";
        test_ldtr_w1_sp_1, ldtr(W1, (SP, 1)).unwrap(), "ldtr w1, [sp, 1]";
        test_ldtr_w1_sp_255, ldtr(W1, (SP, 255)).unwrap(), "ldtr w1, [sp, 255]";
        test_ldtr_w1_sp_m256, ldtr(W1, (SP, -256)).unwrap(), "ldtr w1, [sp, -256]";
        test_ldtr_w1_sp_0, ldtr(W1, (SP, 0)).unwrap(), "ldtr w1, [sp, 0]";
        test_ldtr_xzr_x2_m1, ldtr(XZR, (X2, -1)).unwrap(), "ldtr xzr, [x2, -1]";
        test_ldtr_xzr_x2_1, ldtr(XZR, (X2, 1)).unwrap(), "ldtr xzr, [x2, 1]";
        test_ldtr_xzr_x2_255, ldtr(XZR, (X2, 255)).unwrap(), "ldtr xzr, [x2, 255]";
        test_ldtr_xzr_x2_m256, ldtr(XZR, (X2, -256)).unwrap(), "ldtr xzr, [x2, -256]";
        test_ldtr_xzr_x2_0, ldtr(XZR, (X2, 0)).unwrap(), "ldtr xzr, [x2, 0]";
        test_ldtr_xzr_sp_m1, ldtr(XZR, (SP, -1)).unwrap(), "ldtr xzr, [sp, -1]";
        test_ldtr_xzr_sp_1, ldtr(XZR, (SP, 1)).unwrap(), "ldtr xzr, [sp, 1]";
        test_ldtr_xzr_sp_255, ldtr(XZR, (SP, 255)).unwrap(), "ldtr xzr, [sp, 255]";
        test_ldtr_xzr_sp_m256, ldtr(XZR, (SP, -256)).unwrap(), "ldtr xzr, [sp, -256]";
        test_ldtr_xzr_sp_0, ldtr(XZR, (SP, 0)).unwrap(), "ldtr xzr, [sp, 0]";
        test_ldtr_wzr_x2_m1, ldtr(WZR, (X2, -1)).unwrap(), "ldtr wzr, [x2, -1]";
        test_ldtr_wzr_x2_1, ldtr(WZR, (X2, 1)).unwrap(), "ldtr wzr, [x2, 1]";
        test_ldtr_wzr_x2_255, ldtr(WZR, (X2, 255)).unwrap(), "ldtr wzr, [x2, 255]";
        test_ldtr_wzr_x2_m256, ldtr(WZR, (X2, -256)).unwrap(), "ldtr wzr, [x2, -256]";
        test_ldtr_wzr_x2_0, ldtr(WZR, (X2, 0)).unwrap(), "ldtr wzr, [x2, 0]";
        test_ldtr_wzr_sp_m1, ldtr(WZR, (SP, -1)).unwrap(), "ldtr wzr, [sp, -1]";
        test_ldtr_wzr_sp_1, ldtr(WZR, (SP, 1)).unwrap(), "ldtr wzr, [sp, 1]";
        test_ldtr_wzr_sp_255, ldtr(WZR, (SP, 255)).unwrap(), "ldtr wzr, [sp, 255]";
        test_ldtr_wzr_sp_m256, ldtr(WZR, (SP, -256)).unwrap(), "ldtr wzr, [sp, -256]";
        test_ldtr_wzr_sp_0, ldtr(WZR, (SP, 0)).unwrap(), "ldtr wzr, [sp, 0]";
    }
}
