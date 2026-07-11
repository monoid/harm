/* Copyright (C) 2026 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use aarchmrs_instructions::A64::ldst::ldst_unpriv::LDTRSW_64_ldst_unpriv::LDTRSW_64_ldst_unpriv;

use crate::bits::BitError;
use crate::instructions::RawInstruction;
use crate::register::{IntoReg, RegOrSp64, RegOrZero64, Register};
use crate::sealed::Sealed;

use super::UnscaledOffset;

/// A `LDTRSW` instruction with a destination and an address.
pub struct Ldtrsw<Rt, Addr> {
    rt: Rt,
    addr: Addr,
}

impl<Rt, Addr> Ldtrsw<Rt, Addr> {
    pub fn rt(&self) -> &Rt {
        &self.rt
    }

    pub fn addr(&self) -> &Addr {
        &self.addr
    }
}

impl<Rt, Addr> Sealed for Ldtrsw<Rt, Addr> {}

/// Defines possible ways to construct a `ldtrsw` instruction.
pub trait MakeLdtrsw<Rt, Addr>: Sealed {
    /// Allows defining both fallible and infallible constructors.
    type Output;

    fn new(rt: Rt, addr: Addr) -> Self::Output;
}

define_unscaled_imm_offset_rules!(Ldtrsw, MakeLdtrsw, LDTRSW, RegOrZero64, 64, "ldst_unpriv");

pub fn ldtrsw<TargetInp, TargetOut, AddrInp, AddrOut>(
    dst: TargetInp,
    addr: AddrInp,
) -> <Ldtrsw<TargetOut, AddrOut> as MakeLdtrsw<TargetInp, AddrInp>>::Output
where
    Ldtrsw<TargetOut, AddrOut>: MakeLdtrsw<TargetInp, AddrInp>,
{
    Ldtrsw::new(dst, addr)
}

#[cfg(test)]
mod tests {
    use harm_test_utils::test_cases;

    use super::*;
    use crate::instructions::InstructionSeq;
    use crate::register::Reg64::*;
    use RegOrSp64::SP;
    use RegOrZero64::XZR;

    // 'ldtrsw (x1|w1|xzr|wzr), [(x2|sp), (-1|1|255|-256|0)]
    const LDTRSW_DB: &str = "
b89ff841	ldtrsw x1, [x2, -1]
b8801841	ldtrsw x1, [x2, 1]
b88ff841	ldtrsw x1, [x2, 255]
b8900841	ldtrsw x1, [x2, -256]
b8800841	ldtrsw x1, [x2, 0]
b8800841	ldtrsw x1, [x2]
b89ffbe1	ldtrsw x1, [sp, -1]
b8801be1	ldtrsw x1, [sp, 1]
b88ffbe1	ldtrsw x1, [sp, 255]
b8900be1	ldtrsw x1, [sp, -256]
b8800be1	ldtrsw x1, [sp, 0]
b89ff85f	ldtrsw xzr, [x2, -1]
b880185f	ldtrsw xzr, [x2, 1]
b88ff85f	ldtrsw xzr, [x2, 255]
b890085f	ldtrsw xzr, [x2, -256]
b880085f	ldtrsw xzr, [x2, 0]
b89ffbff	ldtrsw xzr, [sp, -1]
b8801bff	ldtrsw xzr, [sp, 1]
b88ffbff	ldtrsw xzr, [sp, 255]
b8900bff	ldtrsw xzr, [sp, -256]
b8800bff	ldtrsw xzr, [sp, 0]
";

    test_cases! {
        LDTRSW_DB, untested_ldtrsw_cases;
        test_ldtrsw_x1_x2_m1, ldtrsw(X1, (X2, -1)).unwrap(), "ldtrsw x1, [x2, -1]";
        test_ldtrsw_x1_x2_1, ldtrsw(X1, (X2, 1)).unwrap(), "ldtrsw x1, [x2, 1]";
        test_ldtrsw_x1_x2_255, ldtrsw(X1, (X2, 255)).unwrap(), "ldtrsw x1, [x2, 255]";
        test_ldtrsw_x1_x2_m256, ldtrsw(X1, (X2, -256)).unwrap(), "ldtrsw x1, [x2, -256]";
        test_ldtrsw_x1_x2_0, ldtrsw(X1, (X2, 0)).unwrap(), "ldtrsw x1, [x2, 0]";
        test_ldtrsw_x1_x2_simple, ldtrsw(X1, (X2,)), "ldtrsw x1, [x2]";
        test_ldtrsw_x1_sp_m1, ldtrsw(X1, (SP, -1)).unwrap(), "ldtrsw x1, [sp, -1]";
        test_ldtrsw_x1_sp_1, ldtrsw(X1, (SP, 1)).unwrap(), "ldtrsw x1, [sp, 1]";
        test_ldtrsw_x1_sp_255, ldtrsw(X1, (SP, 255)).unwrap(), "ldtrsw x1, [sp, 255]";
        test_ldtrsw_x1_sp_m256, ldtrsw(X1, (SP, -256)).unwrap(), "ldtrsw x1, [sp, -256]";
        test_ldtrsw_x1_sp_0, ldtrsw(X1, (SP, 0)).unwrap(), "ldtrsw x1, [sp, 0]";
        test_ldtrsw_xzr_x2_m1, ldtrsw(XZR, (X2, -1)).unwrap(), "ldtrsw xzr, [x2, -1]";
        test_ldtrsw_xzr_x2_1, ldtrsw(XZR, (X2, 1)).unwrap(), "ldtrsw xzr, [x2, 1]";
        test_ldtrsw_xzr_x2_255, ldtrsw(XZR, (X2, 255)).unwrap(), "ldtrsw xzr, [x2, 255]";
        test_ldtrsw_xzr_x2_m256, ldtrsw(XZR, (X2, -256)).unwrap(), "ldtrsw xzr, [x2, -256]";
        test_ldtrsw_xzr_x2_0, ldtrsw(XZR, (X2, 0)).unwrap(), "ldtrsw xzr, [x2, 0]";
        test_ldtrsw_xzr_sp_m1, ldtrsw(XZR, (SP, -1)).unwrap(), "ldtrsw xzr, [sp, -1]";
        test_ldtrsw_xzr_sp_1, ldtrsw(XZR, (SP, 1)).unwrap(), "ldtrsw xzr, [sp, 1]";
        test_ldtrsw_xzr_sp_255, ldtrsw(XZR, (SP, 255)).unwrap(), "ldtrsw xzr, [sp, 255]";
        test_ldtrsw_xzr_sp_m256, ldtrsw(XZR, (SP, -256)).unwrap(), "ldtrsw xzr, [sp, -256]";
        test_ldtrsw_xzr_sp_0, ldtrsw(XZR, (SP, 0)).unwrap(), "ldtrsw xzr, [sp, 0]";
    }
}
