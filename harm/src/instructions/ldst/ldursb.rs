/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use aarchmrs_instructions::A64::ldst::ldst_unscaled::{
    LDURSB_32_ldst_unscaled::LDURSB_32_ldst_unscaled,
    LDURSB_64_ldst_unscaled::LDURSB_64_ldst_unscaled,
};

use super::args::{LdStArgs, MakeUrsbArgs};
use super::UnscaledOffset;
use crate::{
    instructions::RawInstruction,
    outcome::Outcome,
    register::{RegOrSp64, RegOrZero32, RegOrZero64, Register},
};

/// A `ldursb` instruction with a destination and an address.
#[derive(Debug, Copy, Clone)]
pub struct Ldursb<Args>(pub Args);

/// ldursb construction function.  See examples in the module documentation.
pub fn ldursb<RtIn, Rt, AddrIn, Addr>(
    dst: RtIn,
    addr: AddrIn,
) -> <<LdStArgs<Rt, Addr> as MakeUrsbArgs<RtIn, AddrIn>>::Outcome as Outcome>::Output<
    Ldursb<LdStArgs<Rt, Addr>>,
>
where
    LdStArgs<Rt, Addr>: MakeUrsbArgs<RtIn, AddrIn>,
    <LdStArgs<Rt, Addr> as MakeUrsbArgs<RtIn, AddrIn>>::Outcome:
        Outcome<Inner = LdStArgs<Rt, Addr>>,
{
    <LdStArgs<Rt, Addr> as MakeUrsbArgs<RtIn, AddrIn>>::new(dst, addr).map(Ldursb)
}

impl RawInstruction for Ldursb<LdStArgs<RegOrZero64, (RegOrSp64, UnscaledOffset)>> {
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let (base, offset) = self.0.addr;
        LDURSB_64_ldst_unscaled(offset.into(), base.index(), self.0.rt.index())
    }
}

impl RawInstruction for Ldursb<LdStArgs<RegOrZero32, (RegOrSp64, UnscaledOffset)>> {
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let (base, offset) = self.0.addr;
        LDURSB_32_ldst_unscaled(offset.into(), base.index(), self.0.rt.index())
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

    const LDURSB_DB: &str = "
389ff041	ldursb x1, [x2, -1]
38801041	ldursb x1, [x2, 1]
388ff041	ldursb x1, [x2, 255]
38900041	ldursb x1, [x2, -256]
38800041	ldursb x1, [x2, 0]
38800041	ldursb x1, [x2]
389ff3e1	ldursb x1, [sp, -1]
388013e1	ldursb x1, [sp, 1]
388ff3e1	ldursb x1, [sp, 255]
389003e1	ldursb x1, [sp, -256]
388003e1	ldursb x1, [sp, 0]
38dff041	ldursb w1, [x2, -1]
38c01041	ldursb w1, [x2, 1]
38cff041	ldursb w1, [x2, 255]
38d00041	ldursb w1, [x2, -256]
38c00041	ldursb w1, [x2, 0]
38dff3e1	ldursb w1, [sp, -1]
38c013e1	ldursb w1, [sp, 1]
38cff3e1	ldursb w1, [sp, 255]
38d003e1	ldursb w1, [sp, -256]
38c003e1	ldursb w1, [sp, 0]
389ff05f	ldursb xzr, [x2, -1]
3880105f	ldursb xzr, [x2, 1]
388ff05f	ldursb xzr, [x2, 255]
3890005f	ldursb xzr, [x2, -256]
3880005f	ldursb xzr, [x2, 0]
389ff3ff	ldursb xzr, [sp, -1]
388013ff	ldursb xzr, [sp, 1]
388ff3ff	ldursb xzr, [sp, 255]
389003ff	ldursb xzr, [sp, -256]
388003ff	ldursb xzr, [sp, 0]
38dff05f	ldursb wzr, [x2, -1]
38c0105f	ldursb wzr, [x2, 1]
38cff05f	ldursb wzr, [x2, 255]
38d0005f	ldursb wzr, [x2, -256]
38c0005f	ldursb wzr, [x2, 0]
38dff3ff	ldursb wzr, [sp, -1]
38c013ff	ldursb wzr, [sp, 1]
38cff3ff	ldursb wzr, [sp, 255]
38d003ff	ldursb wzr, [sp, -256]
38c003ff	ldursb wzr, [sp, 0]
";

    test_cases! {
        LDURSB_DB, untested_ldursb_cases;
        test_ldursb_x1_x2_m1, ldursb(X1, (X2, -1)).unwrap(), "ldursb x1, [x2, -1]";
        test_ldursb_x1_x2_1, ldursb(X1, (X2, 1)).unwrap(), "ldursb x1, [x2, 1]";
        test_ldursb_x1_x2_255, ldursb(X1, (X2, 255)).unwrap(), "ldursb x1, [x2, 255]";
        test_ldursb_x1_x2_m256, ldursb(X1, (X2, -256)).unwrap(), "ldursb x1, [x2, -256]";
        test_ldursb_x1_x2_0, ldursb(X1, (X2, 0)).unwrap(), "ldursb x1, [x2, 0]";
        test_ldursb_x1_x2_simple, ldursb(X1, (X2,)), "ldursb x1, [x2]";
        test_ldursb_x1_sp_m1, ldursb(X1, (SP, -1)).unwrap(), "ldursb x1, [sp, -1]";
        test_ldursb_x1_sp_1, ldursb(X1, (SP, 1)).unwrap(), "ldursb x1, [sp, 1]";
        test_ldursb_x1_sp_255, ldursb(X1, (SP, 255)).unwrap(), "ldursb x1, [sp, 255]";
        test_ldursb_x1_sp_m256, ldursb(X1, (SP, -256)).unwrap(), "ldursb x1, [sp, -256]";
        test_ldursb_x1_sp_0, ldursb(X1, (SP, 0)).unwrap(), "ldursb x1, [sp, 0]";
        test_ldursb_w1_x2_m1, ldursb(W1, (X2, -1)).unwrap(), "ldursb w1, [x2, -1]";
        test_ldursb_w1_x2_1, ldursb(W1, (X2, 1)).unwrap(), "ldursb w1, [x2, 1]";
        test_ldursb_w1_x2_255, ldursb(W1, (X2, 255)).unwrap(), "ldursb w1, [x2, 255]";
        test_ldursb_w1_x2_m256, ldursb(W1, (X2, -256)).unwrap(), "ldursb w1, [x2, -256]";
        test_ldursb_w1_x2_0, ldursb(W1, (X2, 0)).unwrap(), "ldursb w1, [x2, 0]";
        test_ldursb_w1_sp_m1, ldursb(W1, (SP, -1)).unwrap(), "ldursb w1, [sp, -1]";
        test_ldursb_w1_sp_1, ldursb(W1, (SP, 1)).unwrap(), "ldursb w1, [sp, 1]";
        test_ldursb_w1_sp_255, ldursb(W1, (SP, 255)).unwrap(), "ldursb w1, [sp, 255]";
        test_ldursb_w1_sp_m256, ldursb(W1, (SP, -256)).unwrap(), "ldursb w1, [sp, -256]";
        test_ldursb_w1_sp_0, ldursb(W1, (SP, 0)).unwrap(), "ldursb w1, [sp, 0]";
        test_ldursb_xzr_x2_m1, ldursb(XZR, (X2, -1)).unwrap(), "ldursb xzr, [x2, -1]";
        test_ldursb_xzr_x2_1, ldursb(XZR, (X2, 1)).unwrap(), "ldursb xzr, [x2, 1]";
        test_ldursb_xzr_x2_255, ldursb(XZR, (X2, 255)).unwrap(), "ldursb xzr, [x2, 255]";
        test_ldursb_xzr_x2_m256, ldursb(XZR, (X2, -256)).unwrap(), "ldursb xzr, [x2, -256]";
        test_ldursb_xzr_x2_0, ldursb(XZR, (X2, 0)).unwrap(), "ldursb xzr, [x2, 0]";
        test_ldursb_xzr_sp_m1, ldursb(XZR, (SP, -1)).unwrap(), "ldursb xzr, [sp, -1]";
        test_ldursb_xzr_sp_1, ldursb(XZR, (SP, 1)).unwrap(), "ldursb xzr, [sp, 1]";
        test_ldursb_xzr_sp_255, ldursb(XZR, (SP, 255)).unwrap(), "ldursb xzr, [sp, 255]";
        test_ldursb_xzr_sp_m256, ldursb(XZR, (SP, -256)).unwrap(), "ldursb xzr, [sp, -256]";
        test_ldursb_xzr_sp_0, ldursb(XZR, (SP, 0)).unwrap(), "ldursb xzr, [sp, 0]";
        test_ldursb_wzr_x2_m1, ldursb(WZR, (X2, -1)).unwrap(), "ldursb wzr, [x2, -1]";
        test_ldursb_wzr_x2_1, ldursb(WZR, (X2, 1)).unwrap(), "ldursb wzr, [x2, 1]";
        test_ldursb_wzr_x2_255, ldursb(WZR, (X2, 255)).unwrap(), "ldursb wzr, [x2, 255]";
        test_ldursb_wzr_x2_m256, ldursb(WZR, (X2, -256)).unwrap(), "ldursb wzr, [x2, -256]";
        test_ldursb_wzr_x2_0, ldursb(WZR, (X2, 0)).unwrap(), "ldursb wzr, [x2, 0]";
        test_ldursb_wzr_sp_m1, ldursb(WZR, (SP, -1)).unwrap(), "ldursb wzr, [sp, -1]";
        test_ldursb_wzr_sp_1, ldursb(WZR, (SP, 1)).unwrap(), "ldursb wzr, [sp, 1]";
        test_ldursb_wzr_sp_255, ldursb(WZR, (SP, 255)).unwrap(), "ldursb wzr, [sp, 255]";
        test_ldursb_wzr_sp_m256, ldursb(WZR, (SP, -256)).unwrap(), "ldursb wzr, [sp, -256]";
        test_ldursb_wzr_sp_0, ldursb(WZR, (SP, 0)).unwrap(), "ldursb wzr, [sp, 0]";
    }
}
