/* Copyright (C) 2026 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use aarchmrs_instructions::A64::ldst::ldst_unpriv::{
    LDTRSB_32_ldst_unpriv::LDTRSB_32_ldst_unpriv, LDTRSB_64_ldst_unpriv::LDTRSB_64_ldst_unpriv,
};

use crate::bits::BitError;
use crate::instructions::RawInstruction;
use crate::register::{IntoReg, RegOrSp64, RegOrZero32, RegOrZero64, Register};
use crate::sealed::Sealed;

use super::UnscaledOffset;

/// A `LDTRSB` instruction with a destination and an address.
pub struct Ldtrsb<Rt, Addr> {
    rt: Rt,
    addr: Addr,
}

impl<Rt, Addr> Ldtrsb<Rt, Addr> {
    pub fn rt(&self) -> &Rt {
        &self.rt
    }

    pub fn addr(&self) -> &Addr {
        &self.addr
    }
}

impl<Rt, Addr> Sealed for Ldtrsb<Rt, Addr> {}

/// Defines possible ways to construct a `ldtrsb` instruction.
pub trait MakeLdtrsb<Rt, Addr>: Sealed {
    /// Allows defining both fallible and infallible constructors.
    type Output;

    fn new(rt: Rt, addr: Addr) -> Self::Output;
}

define_unscaled_imm_offset_rules!(Ldtrsb, MakeLdtrsb, LDTRSB, RegOrZero64, 64, "ldst_unpriv");
define_unscaled_imm_offset_rules!(Ldtrsb, MakeLdtrsb, LDTRSB, RegOrZero32, 32, "ldst_unpriv");

pub fn ldtrsb<TargetInp, TargetOut, AddrInp, AddrOut>(
    dst: TargetInp,
    addr: AddrInp,
) -> <Ldtrsb<TargetOut, AddrOut> as MakeLdtrsb<TargetInp, AddrInp>>::Output
where
    Ldtrsb<TargetOut, AddrOut>: MakeLdtrsb<TargetInp, AddrInp>,
{
    Ldtrsb::new(dst, addr)
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

    // 'ldtrsb (x1|w1|xzr|wzr), [(x2|sp), (-1|1|255|-256|0)]
    const LDTRSB_DB: &str = "
389ff841	ldtrsb x1, [x2, -1]
38801841	ldtrsb x1, [x2, 1]
388ff841	ldtrsb x1, [x2, 255]
38900841	ldtrsb x1, [x2, -256]
38800841	ldtrsb x1, [x2, 0]
38800841	ldtrsb x1, [x2]
389ffbe1	ldtrsb x1, [sp, -1]
38801be1	ldtrsb x1, [sp, 1]
388ffbe1	ldtrsb x1, [sp, 255]
38900be1	ldtrsb x1, [sp, -256]
38800be1	ldtrsb x1, [sp, 0]
38dff841	ldtrsb w1, [x2, -1]
38c01841	ldtrsb w1, [x2, 1]
38cff841	ldtrsb w1, [x2, 255]
38d00841	ldtrsb w1, [x2, -256]
38c00841	ldtrsb w1, [x2, 0]
38dffbe1	ldtrsb w1, [sp, -1]
38c01be1	ldtrsb w1, [sp, 1]
38cffbe1	ldtrsb w1, [sp, 255]
38d00be1	ldtrsb w1, [sp, -256]
38c00be1	ldtrsb w1, [sp, 0]
389ff85f	ldtrsb xzr, [x2, -1]
3880185f	ldtrsb xzr, [x2, 1]
388ff85f	ldtrsb xzr, [x2, 255]
3890085f	ldtrsb xzr, [x2, -256]
3880085f	ldtrsb xzr, [x2, 0]
389ffbff	ldtrsb xzr, [sp, -1]
38801bff	ldtrsb xzr, [sp, 1]
388ffbff	ldtrsb xzr, [sp, 255]
38900bff	ldtrsb xzr, [sp, -256]
38800bff	ldtrsb xzr, [sp, 0]
38dff85f	ldtrsb wzr, [x2, -1]
38c0185f	ldtrsb wzr, [x2, 1]
38cff85f	ldtrsb wzr, [x2, 255]
38d0085f	ldtrsb wzr, [x2, -256]
38c0085f	ldtrsb wzr, [x2, 0]
38dffbff	ldtrsb wzr, [sp, -1]
38c01bff	ldtrsb wzr, [sp, 1]
38cffbff	ldtrsb wzr, [sp, 255]
38d00bff	ldtrsb wzr, [sp, -256]
38c00bff	ldtrsb wzr, [sp, 0]
";

    test_cases! {
        LDTRSB_DB, untested_ldtrsb_cases;
        test_ldtrsb_x1_x2_m1, ldtrsb(X1, (X2, -1)).unwrap(), "ldtrsb x1, [x2, -1]";
        test_ldtrsb_x1_x2_1, ldtrsb(X1, (X2, 1)).unwrap(), "ldtrsb x1, [x2, 1]";
        test_ldtrsb_x1_x2_255, ldtrsb(X1, (X2, 255)).unwrap(), "ldtrsb x1, [x2, 255]";
        test_ldtrsb_x1_x2_m256, ldtrsb(X1, (X2, -256)).unwrap(), "ldtrsb x1, [x2, -256]";
        test_ldtrsb_x1_x2_0, ldtrsb(X1, (X2, 0)).unwrap(), "ldtrsb x1, [x2, 0]";
        test_ldtrsb_x1_x2_simple, ldtrsb(X1, (X2,)), "ldtrsb x1, [x2]";
        test_ldtrsb_x1_sp_m1, ldtrsb(X1, (SP, -1)).unwrap(), "ldtrsb x1, [sp, -1]";
        test_ldtrsb_x1_sp_1, ldtrsb(X1, (SP, 1)).unwrap(), "ldtrsb x1, [sp, 1]";
        test_ldtrsb_x1_sp_255, ldtrsb(X1, (SP, 255)).unwrap(), "ldtrsb x1, [sp, 255]";
        test_ldtrsb_x1_sp_m256, ldtrsb(X1, (SP, -256)).unwrap(), "ldtrsb x1, [sp, -256]";
        test_ldtrsb_x1_sp_0, ldtrsb(X1, (SP, 0)).unwrap(), "ldtrsb x1, [sp, 0]";
        test_ldtrsb_w1_x2_m1, ldtrsb(W1, (X2, -1)).unwrap(), "ldtrsb w1, [x2, -1]";
        test_ldtrsb_w1_x2_1, ldtrsb(W1, (X2, 1)).unwrap(), "ldtrsb w1, [x2, 1]";
        test_ldtrsb_w1_x2_255, ldtrsb(W1, (X2, 255)).unwrap(), "ldtrsb w1, [x2, 255]";
        test_ldtrsb_w1_x2_m256, ldtrsb(W1, (X2, -256)).unwrap(), "ldtrsb w1, [x2, -256]";
        test_ldtrsb_w1_x2_0, ldtrsb(W1, (X2, 0)).unwrap(), "ldtrsb w1, [x2, 0]";
        test_ldtrsb_w1_sp_m1, ldtrsb(W1, (SP, -1)).unwrap(), "ldtrsb w1, [sp, -1]";
        test_ldtrsb_w1_sp_1, ldtrsb(W1, (SP, 1)).unwrap(), "ldtrsb w1, [sp, 1]";
        test_ldtrsb_w1_sp_255, ldtrsb(W1, (SP, 255)).unwrap(), "ldtrsb w1, [sp, 255]";
        test_ldtrsb_w1_sp_m256, ldtrsb(W1, (SP, -256)).unwrap(), "ldtrsb w1, [sp, -256]";
        test_ldtrsb_w1_sp_0, ldtrsb(W1, (SP, 0)).unwrap(), "ldtrsb w1, [sp, 0]";
        test_ldtrsb_xzr_x2_m1, ldtrsb(XZR, (X2, -1)).unwrap(), "ldtrsb xzr, [x2, -1]";
        test_ldtrsb_xzr_x2_1, ldtrsb(XZR, (X2, 1)).unwrap(), "ldtrsb xzr, [x2, 1]";
        test_ldtrsb_xzr_x2_255, ldtrsb(XZR, (X2, 255)).unwrap(), "ldtrsb xzr, [x2, 255]";
        test_ldtrsb_xzr_x2_m256, ldtrsb(XZR, (X2, -256)).unwrap(), "ldtrsb xzr, [x2, -256]";
        test_ldtrsb_xzr_x2_0, ldtrsb(XZR, (X2, 0)).unwrap(), "ldtrsb xzr, [x2, 0]";
        test_ldtrsb_xzr_sp_m1, ldtrsb(XZR, (SP, -1)).unwrap(), "ldtrsb xzr, [sp, -1]";
        test_ldtrsb_xzr_sp_1, ldtrsb(XZR, (SP, 1)).unwrap(), "ldtrsb xzr, [sp, 1]";
        test_ldtrsb_xzr_sp_255, ldtrsb(XZR, (SP, 255)).unwrap(), "ldtrsb xzr, [sp, 255]";
        test_ldtrsb_xzr_sp_m256, ldtrsb(XZR, (SP, -256)).unwrap(), "ldtrsb xzr, [sp, -256]";
        test_ldtrsb_xzr_sp_0, ldtrsb(XZR, (SP, 0)).unwrap(), "ldtrsb xzr, [sp, 0]";
        test_ldtrsb_wzr_x2_m1, ldtrsb(WZR, (X2, -1)).unwrap(), "ldtrsb wzr, [x2, -1]";
        test_ldtrsb_wzr_x2_1, ldtrsb(WZR, (X2, 1)).unwrap(), "ldtrsb wzr, [x2, 1]";
        test_ldtrsb_wzr_x2_255, ldtrsb(WZR, (X2, 255)).unwrap(), "ldtrsb wzr, [x2, 255]";
        test_ldtrsb_wzr_x2_m256, ldtrsb(WZR, (X2, -256)).unwrap(), "ldtrsb wzr, [x2, -256]";
        test_ldtrsb_wzr_x2_0, ldtrsb(WZR, (X2, 0)).unwrap(), "ldtrsb wzr, [x2, 0]";
        test_ldtrsb_wzr_sp_m1, ldtrsb(WZR, (SP, -1)).unwrap(), "ldtrsb wzr, [sp, -1]";
        test_ldtrsb_wzr_sp_1, ldtrsb(WZR, (SP, 1)).unwrap(), "ldtrsb wzr, [sp, 1]";
        test_ldtrsb_wzr_sp_255, ldtrsb(WZR, (SP, 255)).unwrap(), "ldtrsb wzr, [sp, 255]";
        test_ldtrsb_wzr_sp_m256, ldtrsb(WZR, (SP, -256)).unwrap(), "ldtrsb wzr, [sp, -256]";
        test_ldtrsb_wzr_sp_0, ldtrsb(WZR, (SP, 0)).unwrap(), "ldtrsb wzr, [sp, 0]";
    }
}
