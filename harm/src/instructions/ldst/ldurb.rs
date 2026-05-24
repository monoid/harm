/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use aarchmrs_instructions::A64::ldst::ldst_unscaled::LDURB_32_ldst_unscaled::LDURB_32_ldst_unscaled;

use super::args::{LdStArgs, MakeUrbArgs};
use super::UnscaledOffset;
use crate::{
    instructions::RawInstruction,
    outcome::Outcome,
    register::{RegOrSp64, RegOrZero32, Register},
};

/// A `ldurb` instruction with a destination and an address.
#[derive(Debug, Copy, Clone)]
pub struct Ldurb<Args>(pub Args);

/// ldurb construction function.  See examples in the module documentation.
pub fn ldurb<RtIn, Rt, AddrIn, Addr>(
    dst: RtIn,
    addr: AddrIn,
) -> <<LdStArgs<Rt, Addr> as MakeUrbArgs<RtIn, AddrIn>>::Outcome as Outcome>::Output<
    Ldurb<LdStArgs<Rt, Addr>>,
>
where
    LdStArgs<Rt, Addr>: MakeUrbArgs<RtIn, AddrIn>,
    <LdStArgs<Rt, Addr> as MakeUrbArgs<RtIn, AddrIn>>::Outcome:
        Outcome<Inner = LdStArgs<Rt, Addr>>,
{
    <LdStArgs<Rt, Addr> as MakeUrbArgs<RtIn, AddrIn>>::new(dst, addr).map(Ldurb)
}

impl RawInstruction for Ldurb<LdStArgs<RegOrZero32, (RegOrSp64, UnscaledOffset)>> {
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let (base, offset) = self.0.addr;
        LDURB_32_ldst_unscaled(offset.into(), base.index(), self.0.rt.index())
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

    const LDURB_DB: &str = "
385ff041	ldurb w1, [x2, -1]
38401041	ldurb w1, [x2, 1]
384ff041	ldurb w1, [x2, 255]
38500041	ldurb w1, [x2, -256]
38400041	ldurb w1, [x2, 0]
38400041	ldurb w1, [x2]
385ff3e1	ldurb w1, [sp, -1]
384013e1	ldurb w1, [sp, 1]
384ff3e1	ldurb w1, [sp, 255]
385003e1	ldurb w1, [sp, -256]
384003e1	ldurb w1, [sp, 0]
385ff05f	ldurb wzr, [x2, -1]
3840105f	ldurb wzr, [x2, 1]
384ff05f	ldurb wzr, [x2, 255]
3850005f	ldurb wzr, [x2, -256]
3840005f	ldurb wzr, [x2, 0]
385ff3ff	ldurb wzr, [sp, -1]
384013ff	ldurb wzr, [sp, 1]
384ff3ff	ldurb wzr, [sp, 255]
385003ff	ldurb wzr, [sp, -256]
384003ff	ldurb wzr, [sp, 0]
";

    test_cases! {
        LDURB_DB, untested_ldurb_cases;
        test_ldurb_w1_x2_m1, ldurb(W1, (X2, -1)).unwrap(), "ldurb w1, [x2, -1]";
        test_ldurb_w1_x2_1, ldurb(W1, (X2, 1)).unwrap(), "ldurb w1, [x2, 1]";
        test_ldurb_w1_x2_255, ldurb(W1, (X2, 255)).unwrap(), "ldurb w1, [x2, 255]";
        test_ldurb_w1_x2_m256, ldurb(W1, (X2, -256)).unwrap(), "ldurb w1, [x2, -256]";
        test_ldurb_w1_x2_0, ldurb(W1, (X2, 0)).unwrap(), "ldurb w1, [x2, 0]";
        test_ldurb_w1_x2_simple, ldurb(W1, (X2,)), "ldurb w1, [x2]";
        test_ldurb_w1_sp_m1, ldurb(W1, (SP, -1)).unwrap(), "ldurb w1, [sp, -1]";
        test_ldurb_w1_sp_1, ldurb(W1, (SP, 1)).unwrap(), "ldurb w1, [sp, 1]";
        test_ldurb_w1_sp_255, ldurb(W1, (SP, 255)).unwrap(), "ldurb w1, [sp, 255]";
        test_ldurb_w1_sp_m256, ldurb(W1, (SP, -256)).unwrap(), "ldurb w1, [sp, -256]";
        test_ldurb_w1_sp_0, ldurb(W1, (SP, 0)).unwrap(), "ldurb w1, [sp, 0]";
        test_ldurb_wzr_x2_m1, ldurb(WZR, (X2, -1)).unwrap(), "ldurb wzr, [x2, -1]";
        test_ldurb_wzr_x2_1, ldurb(WZR, (X2, 1)).unwrap(), "ldurb wzr, [x2, 1]";
        test_ldurb_wzr_x2_255, ldurb(WZR, (X2, 255)).unwrap(), "ldurb wzr, [x2, 255]";
        test_ldurb_wzr_x2_m256, ldurb(WZR, (X2, -256)).unwrap(), "ldurb wzr, [x2, -256]";
        test_ldurb_wzr_x2_0, ldurb(WZR, (X2, 0)).unwrap(), "ldurb wzr, [x2, 0]";
        test_ldurb_wzr_sp_m1, ldurb(WZR, (SP, -1)).unwrap(), "ldurb wzr, [sp, -1]";
        test_ldurb_wzr_sp_1, ldurb(WZR, (SP, 1)).unwrap(), "ldurb wzr, [sp, 1]";
        test_ldurb_wzr_sp_255, ldurb(WZR, (SP, 255)).unwrap(), "ldurb wzr, [sp, 255]";
        test_ldurb_wzr_sp_m256, ldurb(WZR, (SP, -256)).unwrap(), "ldurb wzr, [sp, -256]";
        test_ldurb_wzr_sp_0, ldurb(WZR, (SP, 0)).unwrap(), "ldurb wzr, [sp, 0]";
    }
}
