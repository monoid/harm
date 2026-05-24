/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use aarchmrs_instructions::A64::ldst::ldst_unscaled::LDURH_32_ldst_unscaled::LDURH_32_ldst_unscaled;

use super::args::{LdStArgs, MakeUrhArgs};
use super::UnscaledOffset;
use crate::{
    instructions::RawInstruction,
    outcome::Outcome,
    register::{RegOrSp64, RegOrZero32, Register},
};

/// A `ldurh` instruction with a destination and an address.
#[derive(Debug, Copy, Clone)]
pub struct Ldurh<Args>(pub Args);

/// ldurh construction function.  See examples in the module documentation.
pub fn ldurh<RtIn, Rt, AddrIn, Addr>(
    dst: RtIn,
    addr: AddrIn,
) -> <<LdStArgs<Rt, Addr> as MakeUrhArgs<RtIn, AddrIn>>::Outcome as Outcome>::Output<
    Ldurh<LdStArgs<Rt, Addr>>,
>
where
    LdStArgs<Rt, Addr>: MakeUrhArgs<RtIn, AddrIn>,
    <LdStArgs<Rt, Addr> as MakeUrhArgs<RtIn, AddrIn>>::Outcome:
        Outcome<Inner = LdStArgs<Rt, Addr>>,
{
    <LdStArgs<Rt, Addr> as MakeUrhArgs<RtIn, AddrIn>>::new(dst, addr).map(Ldurh)
}

impl RawInstruction for Ldurh<LdStArgs<RegOrZero32, (RegOrSp64, UnscaledOffset)>> {
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let (base, offset) = self.0.addr;
        LDURH_32_ldst_unscaled(offset.into(), base.index(), self.0.rt.index())
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

    const LDURH_DB: &str = "
785ff041	ldurh w1, [x2, -1]
78401041	ldurh w1, [x2, 1]
784ff041	ldurh w1, [x2, 255]
78500041	ldurh w1, [x2, -256]
78400041	ldurh w1, [x2, 0]
78400041	ldurh w1, [x2]
785ff3e1	ldurh w1, [sp, -1]
784013e1	ldurh w1, [sp, 1]
784ff3e1	ldurh w1, [sp, 255]
785003e1	ldurh w1, [sp, -256]
784003e1	ldurh w1, [sp, 0]
785ff05f	ldurh wzr, [x2, -1]
7840105f	ldurh wzr, [x2, 1]
784ff05f	ldurh wzr, [x2, 255]
7850005f	ldurh wzr, [x2, -256]
7840005f	ldurh wzr, [x2, 0]
785ff3ff	ldurh wzr, [sp, -1]
784013ff	ldurh wzr, [sp, 1]
784ff3ff	ldurh wzr, [sp, 255]
785003ff	ldurh wzr, [sp, -256]
784003ff	ldurh wzr, [sp, 0]
";

    test_cases! {
        LDURH_DB, untested_ldurh_cases;
        test_ldurh_w1_x2_m1, ldurh(W1, (X2, -1)).unwrap(), "ldurh w1, [x2, -1]";
        test_ldurh_w1_x2_1, ldurh(W1, (X2, 1)).unwrap(), "ldurh w1, [x2, 1]";
        test_ldurh_w1_x2_255, ldurh(W1, (X2, 255)).unwrap(), "ldurh w1, [x2, 255]";
        test_ldurh_w1_x2_m256, ldurh(W1, (X2, -256)).unwrap(), "ldurh w1, [x2, -256]";
        test_ldurh_w1_x2_0, ldurh(W1, (X2, 0)).unwrap(), "ldurh w1, [x2, 0]";
        test_ldurh_w1_x2_simple, ldurh(W1, (X2,)), "ldurh w1, [x2]";
        test_ldurh_w1_sp_m1, ldurh(W1, (SP, -1)).unwrap(), "ldurh w1, [sp, -1]";
        test_ldurh_w1_sp_1, ldurh(W1, (SP, 1)).unwrap(), "ldurh w1, [sp, 1]";
        test_ldurh_w1_sp_255, ldurh(W1, (SP, 255)).unwrap(), "ldurh w1, [sp, 255]";
        test_ldurh_w1_sp_m256, ldurh(W1, (SP, -256)).unwrap(), "ldurh w1, [sp, -256]";
        test_ldurh_w1_sp_0, ldurh(W1, (SP, 0)).unwrap(), "ldurh w1, [sp, 0]";
        test_ldurh_wzr_x2_m1, ldurh(WZR, (X2, -1)).unwrap(), "ldurh wzr, [x2, -1]";
        test_ldurh_wzr_x2_1, ldurh(WZR, (X2, 1)).unwrap(), "ldurh wzr, [x2, 1]";
        test_ldurh_wzr_x2_255, ldurh(WZR, (X2, 255)).unwrap(), "ldurh wzr, [x2, 255]";
        test_ldurh_wzr_x2_m256, ldurh(WZR, (X2, -256)).unwrap(), "ldurh wzr, [x2, -256]";
        test_ldurh_wzr_x2_0, ldurh(WZR, (X2, 0)).unwrap(), "ldurh wzr, [x2, 0]";
        test_ldurh_wzr_sp_m1, ldurh(WZR, (SP, -1)).unwrap(), "ldurh wzr, [sp, -1]";
        test_ldurh_wzr_sp_1, ldurh(WZR, (SP, 1)).unwrap(), "ldurh wzr, [sp, 1]";
        test_ldurh_wzr_sp_255, ldurh(WZR, (SP, 255)).unwrap(), "ldurh wzr, [sp, 255]";
        test_ldurh_wzr_sp_m256, ldurh(WZR, (SP, -256)).unwrap(), "ldurh wzr, [sp, -256]";
        test_ldurh_wzr_sp_0, ldurh(WZR, (SP, 0)).unwrap(), "ldurh wzr, [sp, 0]";
    }
}
