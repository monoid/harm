/* Copyright (C) 2026 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use aarchmrs_instructions::A64::ldst::ldst_unpriv::LDTRB_32_ldst_unpriv::LDTRB_32_ldst_unpriv;

use crate::bits::BitError;
use crate::instructions::RawInstruction;
use crate::register::{IntoReg, RegOrSp64, RegOrZero32, Register};
use crate::sealed::Sealed;

use super::UnscaledOffset;

/// A `LDTRB` instruction with a destination and an address.
pub struct Ldtrb<Rt, Addr> {
    rt: Rt,
    addr: Addr,
}

impl<Rt, Addr> Ldtrb<Rt, Addr> {
    pub fn rt(&self) -> &Rt {
        &self.rt
    }

    pub fn addr(&self) -> &Addr {
        &self.addr
    }
}

impl<Rt, Addr> Sealed for Ldtrb<Rt, Addr> {}

/// Defines possible ways to construct a `ldtrb` instruction.
pub trait MakeLdtrb<Rt, Addr>: Sealed {
    /// Allows defining both fallible and infallible constructors.
    type Output;

    fn new(rt: Rt, addr: Addr) -> Self::Output;
}

define_unscaled_imm_offset_rules!(Ldtrb, MakeLdtrb, LDTRB, RegOrZero32, 32, "ldst_unpriv");

pub fn ldtrb<TargetInp, TargetOut, AddrInp, AddrOut>(
    dst: TargetInp,
    addr: AddrInp,
) -> <Ldtrb<TargetOut, AddrOut> as MakeLdtrb<TargetInp, AddrInp>>::Output
where
    Ldtrb<TargetOut, AddrOut>: MakeLdtrb<TargetInp, AddrInp>,
{
    Ldtrb::new(dst, addr)
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

    const LDTRB_DB: &str = "
385ff841	ldtrb w1, [x2, -1]
38401841	ldtrb w1, [x2, 1]
384ff841	ldtrb w1, [x2, 255]
38500841	ldtrb w1, [x2, -256]
38400841	ldtrb w1, [x2, 0]
38400841	ldtrb w1, [x2]
385ffbe1	ldtrb w1, [sp, -1]
38401be1	ldtrb w1, [sp, 1]
384ffbe1	ldtrb w1, [sp, 255]
38500be1	ldtrb w1, [sp, -256]
38400be1	ldtrb w1, [sp, 0]
385ff85f	ldtrb wzr, [x2, -1]
3840185f	ldtrb wzr, [x2, 1]
384ff85f	ldtrb wzr, [x2, 255]
3850085f	ldtrb wzr, [x2, -256]
3840085f	ldtrb wzr, [x2, 0]
385ffbff	ldtrb wzr, [sp, -1]
38401bff	ldtrb wzr, [sp, 1]
384ffbff	ldtrb wzr, [sp, 255]
38500bff	ldtrb wzr, [sp, -256]
38400bff	ldtrb wzr, [sp, 0]
";

    test_cases! {
        LDTRB_DB, untested_ldtrb_cases;
        test_ldtrb_w1_x2_m1, ldtrb(W1, (X2, -1)).unwrap(), "ldtrb w1, [x2, -1]";
        test_ldtrb_w1_x2_1, ldtrb(W1, (X2, 1)).unwrap(), "ldtrb w1, [x2, 1]";
        test_ldtrb_w1_x2_255, ldtrb(W1, (X2, 255)).unwrap(), "ldtrb w1, [x2, 255]";
        test_ldtrb_w1_x2_m256, ldtrb(W1, (X2, -256)).unwrap(), "ldtrb w1, [x2, -256]";
        test_ldtrb_w1_x2_0, ldtrb(W1, (X2, 0)).unwrap(), "ldtrb w1, [x2, 0]";
        test_ldtrb_w1_x2_simple, ldtrb(W1, (X2,)), "ldtrb w1, [x2]";
        test_ldtrb_w1_sp_m1, ldtrb(W1, (SP, -1)).unwrap(), "ldtrb w1, [sp, -1]";
        test_ldtrb_w1_sp_1, ldtrb(W1, (SP, 1)).unwrap(), "ldtrb w1, [sp, 1]";
        test_ldtrb_w1_sp_255, ldtrb(W1, (SP, 255)).unwrap(), "ldtrb w1, [sp, 255]";
        test_ldtrb_w1_sp_m256, ldtrb(W1, (SP, -256)).unwrap(), "ldtrb w1, [sp, -256]";
        test_ldtrb_w1_sp_0, ldtrb(W1, (SP, 0)).unwrap(), "ldtrb w1, [sp, 0]";
        test_ldtrb_wzr_x2_m1, ldtrb(WZR, (X2, -1)).unwrap(), "ldtrb wzr, [x2, -1]";
        test_ldtrb_wzr_x2_1, ldtrb(WZR, (X2, 1)).unwrap(), "ldtrb wzr, [x2, 1]";
        test_ldtrb_wzr_x2_255, ldtrb(WZR, (X2, 255)).unwrap(), "ldtrb wzr, [x2, 255]";
        test_ldtrb_wzr_x2_m256, ldtrb(WZR, (X2, -256)).unwrap(), "ldtrb wzr, [x2, -256]";
        test_ldtrb_wzr_x2_0, ldtrb(WZR, (X2, 0)).unwrap(), "ldtrb wzr, [x2, 0]";
        test_ldtrb_wzr_sp_m1, ldtrb(WZR, (SP, -1)).unwrap(), "ldtrb wzr, [sp, -1]";
        test_ldtrb_wzr_sp_1, ldtrb(WZR, (SP, 1)).unwrap(), "ldtrb wzr, [sp, 1]";
        test_ldtrb_wzr_sp_255, ldtrb(WZR, (SP, 255)).unwrap(), "ldtrb wzr, [sp, 255]";
        test_ldtrb_wzr_sp_m256, ldtrb(WZR, (SP, -256)).unwrap(), "ldtrb wzr, [sp, -256]";
        test_ldtrb_wzr_sp_0, ldtrb(WZR, (SP, 0)).unwrap(), "ldtrb wzr, [sp, 0]";
    }
}
