/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use aarchmrs_instructions::A64::ldst::ldst_unscaled::STURH_32_ldst_unscaled::STURH_32_ldst_unscaled;

use super::args::{LdStArgs, MakeUrhArgs};
use super::UnscaledOffset;
use crate::{
    instructions::RawInstruction,
    outcome::Outcome,
    register::{RegOrSp64, RegOrZero32, Register},
};

/// A `sturh` instruction with a destination and an address.
#[derive(Debug, Copy, Clone)]
pub struct Sturh<Args>(pub Args);

/// sturh construction function.  See examples in the module documentation.
pub fn sturh<RtIn, Rt, AddrIn, Addr>(
    dst: RtIn,
    addr: AddrIn,
) -> <<LdStArgs<Rt, Addr> as MakeUrhArgs<RtIn, AddrIn>>::Outcome as Outcome>::Output<
    Sturh<LdStArgs<Rt, Addr>>,
>
where
    LdStArgs<Rt, Addr>: MakeUrhArgs<RtIn, AddrIn>,
    <LdStArgs<Rt, Addr> as MakeUrhArgs<RtIn, AddrIn>>::Outcome:
        Outcome<Inner = LdStArgs<Rt, Addr>>,
{
    <LdStArgs<Rt, Addr> as MakeUrhArgs<RtIn, AddrIn>>::new(dst, addr).map(Sturh)
}

impl RawInstruction for Sturh<LdStArgs<RegOrZero32, (RegOrSp64, UnscaledOffset)>> {
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let (base, offset) = self.0.addr;
        STURH_32_ldst_unscaled(offset.into(), base.index(), self.0.rt.index())
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

    const STURH_DB: &str = "
781ff041	sturh w1, [x2, -1]
78001041	sturh w1, [x2, 1]
780ff041	sturh w1, [x2, 255]
78100041	sturh w1, [x2, -256]
78000041	sturh w1, [x2, 0]
78000041	sturh w1, [x2]
781ff3e1	sturh w1, [sp, -1]
780013e1	sturh w1, [sp, 1]
780ff3e1	sturh w1, [sp, 255]
781003e1	sturh w1, [sp, -256]
780003e1	sturh w1, [sp, 0]
781ff05f	sturh wzr, [x2, -1]
7800105f	sturh wzr, [x2, 1]
780ff05f	sturh wzr, [x2, 255]
7810005f	sturh wzr, [x2, -256]
7800005f	sturh wzr, [x2, 0]
781ff3ff	sturh wzr, [sp, -1]
780013ff	sturh wzr, [sp, 1]
780ff3ff	sturh wzr, [sp, 255]
781003ff	sturh wzr, [sp, -256]
780003ff	sturh wzr, [sp, 0]
";

    test_cases! {
        STURH_DB, untested_sturh_cases;
        test_sturh_w1_x2_m1, sturh(W1, (X2, -1)).unwrap(), "sturh w1, [x2, -1]";
        test_sturh_w1_x2_1, sturh(W1, (X2, 1)).unwrap(), "sturh w1, [x2, 1]";
        test_sturh_w1_x2_255, sturh(W1, (X2, 255)).unwrap(), "sturh w1, [x2, 255]";
        test_sturh_w1_x2_m256, sturh(W1, (X2, -256)).unwrap(), "sturh w1, [x2, -256]";
        test_sturh_w1_x2_0, sturh(W1, (X2, 0)).unwrap(), "sturh w1, [x2, 0]";
        test_sturh_w1_x2_simple, sturh(W1, (X2,)), "sturh w1, [x2]";
        test_sturh_w1_sp_m1, sturh(W1, (SP, -1)).unwrap(), "sturh w1, [sp, -1]";
        test_sturh_w1_sp_1, sturh(W1, (SP, 1)).unwrap(), "sturh w1, [sp, 1]";
        test_sturh_w1_sp_255, sturh(W1, (SP, 255)).unwrap(), "sturh w1, [sp, 255]";
        test_sturh_w1_sp_m256, sturh(W1, (SP, -256)).unwrap(), "sturh w1, [sp, -256]";
        test_sturh_w1_sp_0, sturh(W1, (SP, 0)).unwrap(), "sturh w1, [sp, 0]";
        test_sturh_wzr_x2_m1, sturh(WZR, (X2, -1)).unwrap(), "sturh wzr, [x2, -1]";
        test_sturh_wzr_x2_1, sturh(WZR, (X2, 1)).unwrap(), "sturh wzr, [x2, 1]";
        test_sturh_wzr_x2_255, sturh(WZR, (X2, 255)).unwrap(), "sturh wzr, [x2, 255]";
        test_sturh_wzr_x2_m256, sturh(WZR, (X2, -256)).unwrap(), "sturh wzr, [x2, -256]";
        test_sturh_wzr_x2_0, sturh(WZR, (X2, 0)).unwrap(), "sturh wzr, [x2, 0]";
        test_sturh_wzr_sp_m1, sturh(WZR, (SP, -1)).unwrap(), "sturh wzr, [sp, -1]";
        test_sturh_wzr_sp_1, sturh(WZR, (SP, 1)).unwrap(), "sturh wzr, [sp, 1]";
        test_sturh_wzr_sp_255, sturh(WZR, (SP, 255)).unwrap(), "sturh wzr, [sp, 255]";
        test_sturh_wzr_sp_m256, sturh(WZR, (SP, -256)).unwrap(), "sturh wzr, [sp, -256]";
        test_sturh_wzr_sp_0, sturh(WZR, (SP, 0)).unwrap(), "sturh wzr, [sp, 0]";
    }
}
