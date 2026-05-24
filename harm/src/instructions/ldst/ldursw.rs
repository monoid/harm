/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use aarchmrs_instructions::A64::ldst::ldst_unscaled::LDURSW_64_ldst_unscaled::LDURSW_64_ldst_unscaled;

use super::args::{LdStArgs, MakeUrswArgs};
use super::UnscaledOffset;
use crate::{
    instructions::RawInstruction,
    outcome::Outcome,
    register::{RegOrSp64, RegOrZero64, Register},
};

/// A `ldursw` instruction with a destination and an address.
#[derive(Debug, Copy, Clone)]
pub struct Ldursw<Args>(pub Args);

/// ldursw construction function.  See examples in the module documentation.
pub fn ldursw<RtIn, Rt, AddrIn, Addr>(
    dst: RtIn,
    addr: AddrIn,
) -> <<LdStArgs<Rt, Addr> as MakeUrswArgs<RtIn, AddrIn>>::Outcome as Outcome>::Output<
    Ldursw<LdStArgs<Rt, Addr>>,
>
where
    LdStArgs<Rt, Addr>: MakeUrswArgs<RtIn, AddrIn>,
    <LdStArgs<Rt, Addr> as MakeUrswArgs<RtIn, AddrIn>>::Outcome:
        Outcome<Inner = LdStArgs<Rt, Addr>>,
{
    <LdStArgs<Rt, Addr> as MakeUrswArgs<RtIn, AddrIn>>::new(dst, addr).map(Ldursw)
}

impl RawInstruction for Ldursw<LdStArgs<RegOrZero64, (RegOrSp64, UnscaledOffset)>> {
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let (base, offset) = self.0.addr;
        LDURSW_64_ldst_unscaled(offset.into(), base.index(), self.0.rt.index())
    }
}

#[cfg(test)]
mod tests {
    use harm_test_utils::test_cases;

    use super::*;
    use crate::instructions::InstructionSeq;
    use crate::register::Reg64::*;
    use RegOrSp64::SP;
    use RegOrZero64::XZR;

    const LDURSW_DB: &str = "
b89ff041	ldursw x1, [x2, -1]
b8801041	ldursw x1, [x2, 1]
b88ff041	ldursw x1, [x2, 255]
b8900041	ldursw x1, [x2, -256]
b8800041	ldursw x1, [x2, 0]
b8800041	ldursw x1, [x2]
b89ff3e1	ldursw x1, [sp, -1]
b88013e1	ldursw x1, [sp, 1]
b88ff3e1	ldursw x1, [sp, 255]
b89003e1	ldursw x1, [sp, -256]
b88003e1	ldursw x1, [sp, 0]
b89ff05f	ldursw xzr, [x2, -1]
b880105f	ldursw xzr, [x2, 1]
b88ff05f	ldursw xzr, [x2, 255]
b890005f	ldursw xzr, [x2, -256]
b880005f	ldursw xzr, [x2, 0]
b89ff3ff	ldursw xzr, [sp, -1]
b88013ff	ldursw xzr, [sp, 1]
b88ff3ff	ldursw xzr, [sp, 255]
b89003ff	ldursw xzr, [sp, -256]
b88003ff	ldursw xzr, [sp, 0]
";

    test_cases! {
        LDURSW_DB, untested_ldursw_cases;
        test_ldursw_x1_x2_m1, ldursw(X1, (X2, -1)).unwrap(), "ldursw x1, [x2, -1]";
        test_ldursw_x1_x2_1, ldursw(X1, (X2, 1)).unwrap(), "ldursw x1, [x2, 1]";
        test_ldursw_x1_x2_255, ldursw(X1, (X2, 255)).unwrap(), "ldursw x1, [x2, 255]";
        test_ldursw_x1_x2_m256, ldursw(X1, (X2, -256)).unwrap(), "ldursw x1, [x2, -256]";
        test_ldursw_x1_x2_0, ldursw(X1, (X2, 0)).unwrap(), "ldursw x1, [x2, 0]";
        test_ldursw_x1_x2_simple, ldursw(X1, (X2,)), "ldursw x1, [x2]";
        test_ldursw_x1_sp_m1, ldursw(X1, (SP, -1)).unwrap(), "ldursw x1, [sp, -1]";
        test_ldursw_x1_sp_1, ldursw(X1, (SP, 1)).unwrap(), "ldursw x1, [sp, 1]";
        test_ldursw_x1_sp_255, ldursw(X1, (SP, 255)).unwrap(), "ldursw x1, [sp, 255]";
        test_ldursw_x1_sp_m256, ldursw(X1, (SP, -256)).unwrap(), "ldursw x1, [sp, -256]";
        test_ldursw_x1_sp_0, ldursw(X1, (SP, 0)).unwrap(), "ldursw x1, [sp, 0]";
        test_ldursw_xzr_x2_m1, ldursw(XZR, (X2, -1)).unwrap(), "ldursw xzr, [x2, -1]";
        test_ldursw_xzr_x2_1, ldursw(XZR, (X2, 1)).unwrap(), "ldursw xzr, [x2, 1]";
        test_ldursw_xzr_x2_255, ldursw(XZR, (X2, 255)).unwrap(), "ldursw xzr, [x2, 255]";
        test_ldursw_xzr_x2_m256, ldursw(XZR, (X2, -256)).unwrap(), "ldursw xzr, [x2, -256]";
        test_ldursw_xzr_x2_0, ldursw(XZR, (X2, 0)).unwrap(), "ldursw xzr, [x2, 0]";
        test_ldursw_xzr_sp_m1, ldursw(XZR, (SP, -1)).unwrap(), "ldursw xzr, [sp, -1]";
        test_ldursw_xzr_sp_1, ldursw(XZR, (SP, 1)).unwrap(), "ldursw xzr, [sp, 1]";
        test_ldursw_xzr_sp_255, ldursw(XZR, (SP, 255)).unwrap(), "ldursw xzr, [sp, 255]";
        test_ldursw_xzr_sp_m256, ldursw(XZR, (SP, -256)).unwrap(), "ldursw xzr, [sp, -256]";
        test_ldursw_xzr_sp_0, ldursw(XZR, (SP, 0)).unwrap(), "ldursw xzr, [sp, 0]";
    }
}
