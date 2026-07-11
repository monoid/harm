/* Copyright (C) 2026 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use aarchmrs_instructions::A64::ldst::ldst_unpriv::LDTRH_32_ldst_unpriv::LDTRH_32_ldst_unpriv;

use crate::bits::BitError;
use crate::instructions::RawInstruction;
use crate::register::{IntoReg, RegOrSp64, RegOrZero32, Register};
use crate::sealed::Sealed;

use super::UnscaledOffset;

/// A `LDTRH` instruction with a destination and an address.
pub struct Ldtrh<Rt, Addr> {
    rt: Rt,
    addr: Addr,
}

impl<Rt, Addr> Ldtrh<Rt, Addr> {
    pub fn rt(&self) -> &Rt {
        &self.rt
    }

    pub fn addr(&self) -> &Addr {
        &self.addr
    }
}

impl<Rt, Addr> Sealed for Ldtrh<Rt, Addr> {}

/// Defines possible ways to construct a `ldtrh` instruction.
pub trait MakeLdtrh<Rt, Addr>: Sealed {
    /// Allows defining both fallible and infallible constructors.
    type Output;

    fn new(rt: Rt, addr: Addr) -> Self::Output;
}

define_unscaled_imm_offset_rules!(Ldtrh, MakeLdtrh, LDTRH, RegOrZero32, 32, "ldst_unpriv");

pub fn ldtrh<TargetInp, TargetOut, AddrInp, AddrOut>(
    dst: TargetInp,
    addr: AddrInp,
) -> <Ldtrh<TargetOut, AddrOut> as MakeLdtrh<TargetInp, AddrInp>>::Output
where
    Ldtrh<TargetOut, AddrOut>: MakeLdtrh<TargetInp, AddrInp>,
{
    Ldtrh::new(dst, addr)
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

    const LDTRH_DB: &str = "
785ff841	ldtrh w1, [x2, -1]
78401841	ldtrh w1, [x2, 1]
784ff841	ldtrh w1, [x2, 255]
78500841	ldtrh w1, [x2, -256]
78400841	ldtrh w1, [x2, 0]
78400841	ldtrh w1, [x2]
785ffbe1	ldtrh w1, [sp, -1]
78401be1	ldtrh w1, [sp, 1]
784ffbe1	ldtrh w1, [sp, 255]
78500be1	ldtrh w1, [sp, -256]
78400be1	ldtrh w1, [sp, 0]
785ff85f	ldtrh wzr, [x2, -1]
7840185f	ldtrh wzr, [x2, 1]
784ff85f	ldtrh wzr, [x2, 255]
7850085f	ldtrh wzr, [x2, -256]
7840085f	ldtrh wzr, [x2, 0]
785ffbff	ldtrh wzr, [sp, -1]
78401bff	ldtrh wzr, [sp, 1]
784ffbff	ldtrh wzr, [sp, 255]
78500bff	ldtrh wzr, [sp, -256]
78400bff	ldtrh wzr, [sp, 0]
";

    test_cases! {
        LDTRH_DB, untested_ldtrh_cases;
        test_ldtrh_w1_x2_m1, ldtrh(W1, (X2, -1)).unwrap(), "ldtrh w1, [x2, -1]";
        test_ldtrh_w1_x2_1, ldtrh(W1, (X2, 1)).unwrap(), "ldtrh w1, [x2, 1]";
        test_ldtrh_w1_x2_255, ldtrh(W1, (X2, 255)).unwrap(), "ldtrh w1, [x2, 255]";
        test_ldtrh_w1_x2_m256, ldtrh(W1, (X2, -256)).unwrap(), "ldtrh w1, [x2, -256]";
        test_ldtrh_w1_x2_0, ldtrh(W1, (X2, 0)).unwrap(), "ldtrh w1, [x2, 0]";
        test_ldtrh_w1_x2_simple, ldtrh(W1, (X2,)), "ldtrh w1, [x2]";
        test_ldtrh_w1_sp_m1, ldtrh(W1, (SP, -1)).unwrap(), "ldtrh w1, [sp, -1]";
        test_ldtrh_w1_sp_1, ldtrh(W1, (SP, 1)).unwrap(), "ldtrh w1, [sp, 1]";
        test_ldtrh_w1_sp_255, ldtrh(W1, (SP, 255)).unwrap(), "ldtrh w1, [sp, 255]";
        test_ldtrh_w1_sp_m256, ldtrh(W1, (SP, -256)).unwrap(), "ldtrh w1, [sp, -256]";
        test_ldtrh_w1_sp_0, ldtrh(W1, (SP, 0)).unwrap(), "ldtrh w1, [sp, 0]";
        test_ldtrh_wzr_x2_m1, ldtrh(WZR, (X2, -1)).unwrap(), "ldtrh wzr, [x2, -1]";
        test_ldtrh_wzr_x2_1, ldtrh(WZR, (X2, 1)).unwrap(), "ldtrh wzr, [x2, 1]";
        test_ldtrh_wzr_x2_255, ldtrh(WZR, (X2, 255)).unwrap(), "ldtrh wzr, [x2, 255]";
        test_ldtrh_wzr_x2_m256, ldtrh(WZR, (X2, -256)).unwrap(), "ldtrh wzr, [x2, -256]";
        test_ldtrh_wzr_x2_0, ldtrh(WZR, (X2, 0)).unwrap(), "ldtrh wzr, [x2, 0]";
        test_ldtrh_wzr_sp_m1, ldtrh(WZR, (SP, -1)).unwrap(), "ldtrh wzr, [sp, -1]";
        test_ldtrh_wzr_sp_1, ldtrh(WZR, (SP, 1)).unwrap(), "ldtrh wzr, [sp, 1]";
        test_ldtrh_wzr_sp_255, ldtrh(WZR, (SP, 255)).unwrap(), "ldtrh wzr, [sp, 255]";
        test_ldtrh_wzr_sp_m256, ldtrh(WZR, (SP, -256)).unwrap(), "ldtrh wzr, [sp, -256]";
        test_ldtrh_wzr_sp_0, ldtrh(WZR, (SP, 0)).unwrap(), "ldtrh wzr, [sp, 0]";
    }
}
