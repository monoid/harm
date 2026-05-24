/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use aarchmrs_instructions::A64::ldst::ldst_unscaled::{
    LDURSH_32_ldst_unscaled::LDURSH_32_ldst_unscaled,
    LDURSH_64_ldst_unscaled::LDURSH_64_ldst_unscaled,
};

use super::args::{LdStArgs, MakeUrshArgs};
use super::UnscaledOffset;
use crate::{
    instructions::RawInstruction,
    outcome::Outcome,
    register::{RegOrSp64, RegOrZero32, RegOrZero64, Register},
};

/// A `ldursh` instruction with a destination and an address.
#[derive(Debug, Copy, Clone)]
pub struct Ldursh<Args>(pub Args);

/// ldursh construction function.  See examples in the module documentation.
pub fn ldursh<RtIn, Rt, AddrIn, Addr>(
    dst: RtIn,
    addr: AddrIn,
) -> <<LdStArgs<Rt, Addr> as MakeUrshArgs<RtIn, AddrIn>>::Outcome as Outcome>::Output<
    Ldursh<LdStArgs<Rt, Addr>>,
>
where
    LdStArgs<Rt, Addr>: MakeUrshArgs<RtIn, AddrIn>,
    <LdStArgs<Rt, Addr> as MakeUrshArgs<RtIn, AddrIn>>::Outcome:
        Outcome<Inner = LdStArgs<Rt, Addr>>,
{
    <LdStArgs<Rt, Addr> as MakeUrshArgs<RtIn, AddrIn>>::new(dst, addr).map(Ldursh)
}

impl RawInstruction for Ldursh<LdStArgs<RegOrZero64, (RegOrSp64, UnscaledOffset)>> {
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let (base, offset) = self.0.addr;
        LDURSH_64_ldst_unscaled(offset.into(), base.index(), self.0.rt.index())
    }
}

impl RawInstruction for Ldursh<LdStArgs<RegOrZero32, (RegOrSp64, UnscaledOffset)>> {
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let (base, offset) = self.0.addr;
        LDURSH_32_ldst_unscaled(offset.into(), base.index(), self.0.rt.index())
    }
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
