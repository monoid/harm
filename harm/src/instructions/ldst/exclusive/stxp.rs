/* Copyright (C) 2026 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use crate::register::{RegOrSp64, RegOrZero32, RegOrZero64, Register};
use aarchmrs_instructions::A64::ldst::ldstexclp::{
    STXP_SP32_ldstexclp::STXP_SP32_ldstexclp, STXP_SP64_ldstexclp::STXP_SP64_ldstexclp,
};

use crate::instructions::RawInstruction;

use super::exclusive_args::{ExclusivePairStoreArgs, MakeExclusivePairStoreArgs};

/// A `stxp` instruction with a destination and an address.
pub struct Stxp<Rt> {
    args: ExclusivePairStoreArgs<Rt>,
}

impl<Rt: Copy> Stxp<Rt> {
    pub fn status(&self) -> RegOrZero32 {
        self.args.status
    }

    pub fn rt1(&self) -> Rt {
        self.args.reg1
    }

    pub fn rt2(&self) -> Rt {
        self.args.reg2
    }

    pub fn addr(&self) -> RegOrSp64 {
        self.args.addr
    }
}

pub fn stxp<TargetOut, StatusInp, Reg1Inp, Reg2Inp, AddrRegInp>(
    status: StatusInp,
    r1: Reg1Inp,
    r2: Reg2Inp,
    addr: AddrRegInp,
) -> Stxp<TargetOut>
where
    ExclusivePairStoreArgs<TargetOut>:
        MakeExclusivePairStoreArgs<StatusInp, Reg1Inp, Reg2Inp, AddrRegInp>,
{
    Stxp {
        args: <ExclusivePairStoreArgs<TargetOut> as MakeExclusivePairStoreArgs<
            StatusInp,
            Reg1Inp,
            Reg2Inp,
            AddrRegInp,
        >>::new(status, r1, r2, addr),
    }
}

impl RawInstruction for Stxp<RegOrZero32> {
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        STXP_SP32_ldstexclp(
            self.status().index(),
            self.rt2().index(),
            self.addr().index(),
            self.rt1().index(),
        )
    }
}

impl RawInstruction for Stxp<RegOrZero64> {
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        STXP_SP64_ldstexclp(
            self.status().index(),
            self.rt2().index(),
            self.addr().index(),
            self.rt1().index(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::InstructionSeq;
    use crate::register::Reg32::*;
    use crate::register::Reg64::*;
    use crate::register::RegOrSp64::SP;
    use crate::register::RegOrZero32::WZR;
    use crate::register::RegOrZero64::XZR;
    use harm_test_utils::test_cases;

    const STXP_TEST_DB: &str = "
88250903	stxp w5, w3, w2, [x8]
c8250903	stxp w5, x3, x2, [x8]
882513ff	stxp w5, wzr, w4, [sp]
c82513ff	stxp w5, xzr, x4, [sp]
";
    test_cases! {
        STXP_TEST_DB, untested_stxp_test_db;
        test_stxp_w2_x8, stxp(W5, W3, W2, X8), "stxp w5, w3, w2, [x8]";
        test_stxp_x2_x8, stxp(W5, X3, X2, X8), "stxp w5, x3, x2, [x8]";
        test_stxp_wzr_sp, stxp(W5, WZR, W4, SP), "stxp w5, wzr, w4, [sp]";
        test_stxp_xzr_sp, stxp(W5, XZR, X4, SP), "stxp w5, xzr, x4, [sp]";
    }
}
