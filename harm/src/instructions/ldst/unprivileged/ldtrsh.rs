/* Copyright (C) 2026 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use aarchmrs_instructions::A64::ldst::ldst_unpriv::{
    LDTRSH_32_ldst_unpriv::LDTRSH_32_ldst_unpriv, LDTRSH_64_ldst_unpriv::LDTRSH_64_ldst_unpriv,
};

use crate::bits::BitError;
use crate::instructions::RawInstruction;
use crate::register::{IntoReg, RegOrSp64, RegOrZero32, RegOrZero64, Register};
use crate::sealed::Sealed;

use super::UnscaledOffset;

/// A `LDTRSH` instruction with a destination and an address.
pub struct Ldtrsh<Rt, Addr> {
    rt: Rt,
    addr: Addr,
}

impl<Rt, Addr> Ldtrsh<Rt, Addr> {
    pub fn rt(&self) -> &Rt {
        &self.rt
    }

    pub fn addr(&self) -> &Addr {
        &self.addr
    }
}

impl<Rt, Addr> Sealed for Ldtrsh<Rt, Addr> {}

/// Defines possible ways to construct a `ldtrsh` instruction.
pub trait MakeLdtrsh<Rt, Addr>: Sealed {
    /// Allows defining both fallible and infallible constructors.
    type Output;

    fn new(rt: Rt, addr: Addr) -> Self::Output;
}

define_unscaled_imm_offset_rules!(Ldtrsh, MakeLdtrsh, LDTRSH, RegOrZero64, 64, "ldst_unpriv");
define_unscaled_imm_offset_rules!(Ldtrsh, MakeLdtrsh, LDTRSH, RegOrZero32, 32, "ldst_unpriv");

pub fn ldtrsh<TargetInp, TargetOut, AddrInp, AddrOut>(
    dst: TargetInp,
    addr: AddrInp,
) -> <Ldtrsh<TargetOut, AddrOut> as MakeLdtrsh<TargetInp, AddrInp>>::Output
where
    Ldtrsh<TargetOut, AddrOut>: MakeLdtrsh<TargetInp, AddrInp>,
{
    Ldtrsh::new(dst, addr)
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

    // 'ldtrsh (x1|w1|xzr|wzr), [(x2|sp), (-1|1|255|-256|0)]
    const LDTRSH_DB: &str = "
789ff841	ldtrsh x1, [x2, -1]
78801841	ldtrsh x1, [x2, 1]
788ff841	ldtrsh x1, [x2, 255]
78900841	ldtrsh x1, [x2, -256]
78800841	ldtrsh x1, [x2, 0]
78800841	ldtrsh x1, [x2]
789ffbe1	ldtrsh x1, [sp, -1]
78801be1	ldtrsh x1, [sp, 1]
788ffbe1	ldtrsh x1, [sp, 255]
78900be1	ldtrsh x1, [sp, -256]
78800be1	ldtrsh x1, [sp, 0]
78dff841	ldtrsh w1, [x2, -1]
78c01841	ldtrsh w1, [x2, 1]
78cff841	ldtrsh w1, [x2, 255]
78d00841	ldtrsh w1, [x2, -256]
78c00841	ldtrsh w1, [x2, 0]
78dffbe1	ldtrsh w1, [sp, -1]
78c01be1	ldtrsh w1, [sp, 1]
78cffbe1	ldtrsh w1, [sp, 255]
78d00be1	ldtrsh w1, [sp, -256]
78c00be1	ldtrsh w1, [sp, 0]
789ff85f	ldtrsh xzr, [x2, -1]
7880185f	ldtrsh xzr, [x2, 1]
788ff85f	ldtrsh xzr, [x2, 255]
7890085f	ldtrsh xzr, [x2, -256]
7880085f	ldtrsh xzr, [x2, 0]
789ffbff	ldtrsh xzr, [sp, -1]
78801bff	ldtrsh xzr, [sp, 1]
788ffbff	ldtrsh xzr, [sp, 255]
78900bff	ldtrsh xzr, [sp, -256]
78800bff	ldtrsh xzr, [sp, 0]
78dff85f	ldtrsh wzr, [x2, -1]
78c0185f	ldtrsh wzr, [x2, 1]
78cff85f	ldtrsh wzr, [x2, 255]
78d0085f	ldtrsh wzr, [x2, -256]
78c0085f	ldtrsh wzr, [x2, 0]
78dffbff	ldtrsh wzr, [sp, -1]
78c01bff	ldtrsh wzr, [sp, 1]
78cffbff	ldtrsh wzr, [sp, 255]
78d00bff	ldtrsh wzr, [sp, -256]
78c00bff	ldtrsh wzr, [sp, 0]
";

    test_cases! {
        LDTRSH_DB, untested_ldtrsh_cases;
        test_ldtrsh_x1_x2_m1, ldtrsh(X1, (X2, -1)).unwrap(), "ldtrsh x1, [x2, -1]";
        test_ldtrsh_x1_x2_1, ldtrsh(X1, (X2, 1)).unwrap(), "ldtrsh x1, [x2, 1]";
        test_ldtrsh_x1_x2_255, ldtrsh(X1, (X2, 255)).unwrap(), "ldtrsh x1, [x2, 255]";
        test_ldtrsh_x1_x2_m256, ldtrsh(X1, (X2, -256)).unwrap(), "ldtrsh x1, [x2, -256]";
        test_ldtrsh_x1_x2_0, ldtrsh(X1, (X2, 0)).unwrap(), "ldtrsh x1, [x2, 0]";
        test_ldtrsh_x1_x2_simple, ldtrsh(X1, (X2,)), "ldtrsh x1, [x2]";
        test_ldtrsh_x1_sp_m1, ldtrsh(X1, (SP, -1)).unwrap(), "ldtrsh x1, [sp, -1]";
        test_ldtrsh_x1_sp_1, ldtrsh(X1, (SP, 1)).unwrap(), "ldtrsh x1, [sp, 1]";
        test_ldtrsh_x1_sp_255, ldtrsh(X1, (SP, 255)).unwrap(), "ldtrsh x1, [sp, 255]";
        test_ldtrsh_x1_sp_m256, ldtrsh(X1, (SP, -256)).unwrap(), "ldtrsh x1, [sp, -256]";
        test_ldtrsh_x1_sp_0, ldtrsh(X1, (SP, 0)).unwrap(), "ldtrsh x1, [sp, 0]";
        test_ldtrsh_w1_x2_m1, ldtrsh(W1, (X2, -1)).unwrap(), "ldtrsh w1, [x2, -1]";
        test_ldtrsh_w1_x2_1, ldtrsh(W1, (X2, 1)).unwrap(), "ldtrsh w1, [x2, 1]";
        test_ldtrsh_w1_x2_255, ldtrsh(W1, (X2, 255)).unwrap(), "ldtrsh w1, [x2, 255]";
        test_ldtrsh_w1_x2_m256, ldtrsh(W1, (X2, -256)).unwrap(), "ldtrsh w1, [x2, -256]";
        test_ldtrsh_w1_x2_0, ldtrsh(W1, (X2, 0)).unwrap(), "ldtrsh w1, [x2, 0]";
        test_ldtrsh_w1_sp_m1, ldtrsh(W1, (SP, -1)).unwrap(), "ldtrsh w1, [sp, -1]";
        test_ldtrsh_w1_sp_1, ldtrsh(W1, (SP, 1)).unwrap(), "ldtrsh w1, [sp, 1]";
        test_ldtrsh_w1_sp_255, ldtrsh(W1, (SP, 255)).unwrap(), "ldtrsh w1, [sp, 255]";
        test_ldtrsh_w1_sp_m256, ldtrsh(W1, (SP, -256)).unwrap(), "ldtrsh w1, [sp, -256]";
        test_ldtrsh_w1_sp_0, ldtrsh(W1, (SP, 0)).unwrap(), "ldtrsh w1, [sp, 0]";
        test_ldtrsh_xzr_x2_m1, ldtrsh(XZR, (X2, -1)).unwrap(), "ldtrsh xzr, [x2, -1]";
        test_ldtrsh_xzr_x2_1, ldtrsh(XZR, (X2, 1)).unwrap(), "ldtrsh xzr, [x2, 1]";
        test_ldtrsh_xzr_x2_255, ldtrsh(XZR, (X2, 255)).unwrap(), "ldtrsh xzr, [x2, 255]";
        test_ldtrsh_xzr_x2_m256, ldtrsh(XZR, (X2, -256)).unwrap(), "ldtrsh xzr, [x2, -256]";
        test_ldtrsh_xzr_x2_0, ldtrsh(XZR, (X2, 0)).unwrap(), "ldtrsh xzr, [x2, 0]";
        test_ldtrsh_xzr_sp_m1, ldtrsh(XZR, (SP, -1)).unwrap(), "ldtrsh xzr, [sp, -1]";
        test_ldtrsh_xzr_sp_1, ldtrsh(XZR, (SP, 1)).unwrap(), "ldtrsh xzr, [sp, 1]";
        test_ldtrsh_xzr_sp_255, ldtrsh(XZR, (SP, 255)).unwrap(), "ldtrsh xzr, [sp, 255]";
        test_ldtrsh_xzr_sp_m256, ldtrsh(XZR, (SP, -256)).unwrap(), "ldtrsh xzr, [sp, -256]";
        test_ldtrsh_xzr_sp_0, ldtrsh(XZR, (SP, 0)).unwrap(), "ldtrsh xzr, [sp, 0]";
        test_ldtrsh_wzr_x2_m1, ldtrsh(WZR, (X2, -1)).unwrap(), "ldtrsh wzr, [x2, -1]";
        test_ldtrsh_wzr_x2_1, ldtrsh(WZR, (X2, 1)).unwrap(), "ldtrsh wzr, [x2, 1]";
        test_ldtrsh_wzr_x2_255, ldtrsh(WZR, (X2, 255)).unwrap(), "ldtrsh wzr, [x2, 255]";
        test_ldtrsh_wzr_x2_m256, ldtrsh(WZR, (X2, -256)).unwrap(), "ldtrsh wzr, [x2, -256]";
        test_ldtrsh_wzr_x2_0, ldtrsh(WZR, (X2, 0)).unwrap(), "ldtrsh wzr, [x2, 0]";
        test_ldtrsh_wzr_sp_m1, ldtrsh(WZR, (SP, -1)).unwrap(), "ldtrsh wzr, [sp, -1]";
        test_ldtrsh_wzr_sp_1, ldtrsh(WZR, (SP, 1)).unwrap(), "ldtrsh wzr, [sp, 1]";
        test_ldtrsh_wzr_sp_255, ldtrsh(WZR, (SP, 255)).unwrap(), "ldtrsh wzr, [sp, 255]";
        test_ldtrsh_wzr_sp_m256, ldtrsh(WZR, (SP, -256)).unwrap(), "ldtrsh wzr, [sp, -256]";
        test_ldtrsh_wzr_sp_0, ldtrsh(WZR, (SP, 0)).unwrap(), "ldtrsh wzr, [sp, 0]";
    }
}
