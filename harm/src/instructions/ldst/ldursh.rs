/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use aarchmrs_instructions::A64::ldst::ldst_unscaled::{
    LDURSH_32_ldst_unscaled::LDURSH_32_ldst_unscaled,
    LDURSH_64_ldst_unscaled::LDURSH_64_ldst_unscaled,
};

use crate::bits::BitError;
use crate::instructions::RawInstruction;
use crate::register::{IntoReg, RegOrSp64, RegOrZero32, RegOrZero64, Register};
use crate::sealed::Sealed;

use super::UnscaledOffset;

/// A `LDURSH` instruction with a destination and an address.
pub struct Ldursh<Rt, Addr> {
    rt: Rt,
    addr: Addr,
}

impl<Rt, Addr> Ldursh<Rt, Addr> {
    pub fn rt(&self) -> &Rt {
        &self.rt
    }

    pub fn addr(&self) -> &Addr {
        &self.addr
    }
}

impl<Rt, Addr> Sealed for Ldursh<Rt, Addr> {}

/// Defines possible was to construct a `ldursh` instruction.
pub trait MakeLdursh<Rt, Addr>: Sealed {
    /// Allows defining both faillible and infallible constructors.
    type Output;

    fn new(rt: Rt, addr: Addr) -> Self::Output;
}

define_unscaled_imm_offset_rules!(Ldursh, MakeLdursh, LDURSH, RegOrZero64, 64);
define_unscaled_imm_offset_rules!(Ldursh, MakeLdursh, LDURSH, RegOrZero32, 32);

pub fn ldursh<TargetInp, TargetOut, AddrInp, AddrOut>(
    dst: TargetInp,
    addr: AddrInp,
) -> <Ldursh<TargetOut, AddrOut> as MakeLdursh<TargetInp, AddrInp>>::Output
where
    Ldursh<TargetOut, AddrOut>: MakeLdursh<TargetInp, AddrInp>,
{
    Ldursh::new(dst, addr)
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

    // 'ldursh (x1|w1|xzr|wzr), [(x2|sp), (-1|1|255|-256|0)]
    const LDURSH_DB: &str = "
789ff041	ldursh x1, [x2, -1]
78801041	ldursh x1, [x2, 1]
788ff041	ldursh x1, [x2, 255]
78900041	ldursh x1, [x2, -256]
78800041	ldursh x1, [x2, 0]
78800041	ldursh x1, [x2]
789ff3e1	ldursh x1, [sp, -1]
788013e1	ldursh x1, [sp, 1]
788ff3e1	ldursh x1, [sp, 255]
789003e1	ldursh x1, [sp, -256]
788003e1	ldursh x1, [sp, 0]
78dff041	ldursh w1, [x2, -1]
78c01041	ldursh w1, [x2, 1]
78cff041	ldursh w1, [x2, 255]
78d00041	ldursh w1, [x2, -256]
78c00041	ldursh w1, [x2, 0]
78dff3e1	ldursh w1, [sp, -1]
78c013e1	ldursh w1, [sp, 1]
78cff3e1	ldursh w1, [sp, 255]
78d003e1	ldursh w1, [sp, -256]
78c003e1	ldursh w1, [sp, 0]
789ff05f	ldursh xzr, [x2, -1]
7880105f	ldursh xzr, [x2, 1]
788ff05f	ldursh xzr, [x2, 255]
7890005f	ldursh xzr, [x2, -256]
7880005f	ldursh xzr, [x2, 0]
789ff3ff	ldursh xzr, [sp, -1]
788013ff	ldursh xzr, [sp, 1]
788ff3ff	ldursh xzr, [sp, 255]
789003ff	ldursh xzr, [sp, -256]
788003ff	ldursh xzr, [sp, 0]
78dff05f	ldursh wzr, [x2, -1]
78c0105f	ldursh wzr, [x2, 1]
78cff05f	ldursh wzr, [x2, 255]
78d0005f	ldursh wzr, [x2, -256]
78c0005f	ldursh wzr, [x2, 0]
78dff3ff	ldursh wzr, [sp, -1]
78c013ff	ldursh wzr, [sp, 1]
78cff3ff	ldursh wzr, [sp, 255]
78d003ff	ldursh wzr, [sp, -256]
78c003ff	ldursh wzr, [sp, 0]
";

    test_cases! {
        LDURSH_DB, untested_ldursh_cases;
        test_ldursh_x1_x2_m1, ldursh(X1, (X2, -1)).unwrap(), "ldursh x1, [x2, -1]";
        test_ldursh_x1_x2_1, ldursh(X1, (X2, 1)).unwrap(), "ldursh x1, [x2, 1]";
        test_ldursh_x1_x2_255, ldursh(X1, (X2, 255)).unwrap(), "ldursh x1, [x2, 255]";
        test_ldursh_x1_x2_m256, ldursh(X1, (X2, -256)).unwrap(), "ldursh x1, [x2, -256]";
        test_ldursh_x1_x2_0, ldursh(X1, (X2, 0)).unwrap(), "ldursh x1, [x2, 0]";
        test_ldursh_x1_x2_simple, ldursh(X1, (X2,)), "ldursh x1, [x2]";
        test_ldursh_x1_sp_m1, ldursh(X1, (SP, -1)).unwrap(), "ldursh x1, [sp, -1]";
        test_ldursh_x1_sp_1, ldursh(X1, (SP, 1)).unwrap(), "ldursh x1, [sp, 1]";
        test_ldursh_x1_sp_255, ldursh(X1, (SP, 255)).unwrap(), "ldursh x1, [sp, 255]";
        test_ldursh_x1_sp_m256, ldursh(X1, (SP, -256)).unwrap(), "ldursh x1, [sp, -256]";
        test_ldursh_x1_sp_0, ldursh(X1, (SP, 0)).unwrap(), "ldursh x1, [sp, 0]";
        test_ldursh_w1_x2_m1, ldursh(W1, (X2, -1)).unwrap(), "ldursh w1, [x2, -1]";
        test_ldursh_w1_x2_1, ldursh(W1, (X2, 1)).unwrap(), "ldursh w1, [x2, 1]";
        test_ldursh_w1_x2_255, ldursh(W1, (X2, 255)).unwrap(), "ldursh w1, [x2, 255]";
        test_ldursh_w1_x2_m256, ldursh(W1, (X2, -256)).unwrap(), "ldursh w1, [x2, -256]";
        test_ldursh_w1_x2_0, ldursh(W1, (X2, 0)).unwrap(), "ldursh w1, [x2, 0]";
        test_ldursh_w1_sp_m1, ldursh(W1, (SP, -1)).unwrap(), "ldursh w1, [sp, -1]";
        test_ldursh_w1_sp_1, ldursh(W1, (SP, 1)).unwrap(), "ldursh w1, [sp, 1]";
        test_ldursh_w1_sp_255, ldursh(W1, (SP, 255)).unwrap(), "ldursh w1, [sp, 255]";
        test_ldursh_w1_sp_m256, ldursh(W1, (SP, -256)).unwrap(), "ldursh w1, [sp, -256]";
        test_ldursh_w1_sp_0, ldursh(W1, (SP, 0)).unwrap(), "ldursh w1, [sp, 0]";
        test_ldursh_xzr_x2_m1, ldursh(XZR, (X2, -1)).unwrap(), "ldursh xzr, [x2, -1]";
        test_ldursh_xzr_x2_1, ldursh(XZR, (X2, 1)).unwrap(), "ldursh xzr, [x2, 1]";
        test_ldursh_xzr_x2_255, ldursh(XZR, (X2, 255)).unwrap(), "ldursh xzr, [x2, 255]";
        test_ldursh_xzr_x2_m256, ldursh(XZR, (X2, -256)).unwrap(), "ldursh xzr, [x2, -256]";
        test_ldursh_xzr_x2_0, ldursh(XZR, (X2, 0)).unwrap(), "ldursh xzr, [x2, 0]";
        test_ldursh_xzr_sp_m1, ldursh(XZR, (SP, -1)).unwrap(), "ldursh xzr, [sp, -1]";
        test_ldursh_xzr_sp_1, ldursh(XZR, (SP, 1)).unwrap(), "ldursh xzr, [sp, 1]";
        test_ldursh_xzr_sp_255, ldursh(XZR, (SP, 255)).unwrap(), "ldursh xzr, [sp, 255]";
        test_ldursh_xzr_sp_m256, ldursh(XZR, (SP, -256)).unwrap(), "ldursh xzr, [sp, -256]";
        test_ldursh_xzr_sp_0, ldursh(XZR, (SP, 0)).unwrap(), "ldursh xzr, [sp, 0]";
        test_ldursh_wzr_x2_m1, ldursh(WZR, (X2, -1)).unwrap(), "ldursh wzr, [x2, -1]";
        test_ldursh_wzr_x2_1, ldursh(WZR, (X2, 1)).unwrap(), "ldursh wzr, [x2, 1]";
        test_ldursh_wzr_x2_255, ldursh(WZR, (X2, 255)).unwrap(), "ldursh wzr, [x2, 255]";
        test_ldursh_wzr_x2_m256, ldursh(WZR, (X2, -256)).unwrap(), "ldursh wzr, [x2, -256]";
        test_ldursh_wzr_x2_0, ldursh(WZR, (X2, 0)).unwrap(), "ldursh wzr, [x2, 0]";
        test_ldursh_wzr_sp_m1, ldursh(WZR, (SP, -1)).unwrap(), "ldursh wzr, [sp, -1]";
        test_ldursh_wzr_sp_1, ldursh(WZR, (SP, 1)).unwrap(), "ldursh wzr, [sp, 1]";
        test_ldursh_wzr_sp_255, ldursh(WZR, (SP, 255)).unwrap(), "ldursh wzr, [sp, 255]";
        test_ldursh_wzr_sp_m256, ldursh(WZR, (SP, -256)).unwrap(), "ldursh wzr, [sp, -256]";
        test_ldursh_wzr_sp_0, ldursh(WZR, (SP, 0)).unwrap(), "ldursh wzr, [sp, 0]";
    }
}
