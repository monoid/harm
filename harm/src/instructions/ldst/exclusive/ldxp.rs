/* Copyright (C) 2026 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use crate::register::{RegOrSp64, RegOrZero32, RegOrZero64, Register};
use aarchmrs_instructions::A64::ldst::ldstexclp::{
    LDXP_LP32_ldstexclp::LDXP_LP32_ldstexclp, LDXP_LP64_ldstexclp::LDXP_LP64_ldstexclp,
};

use crate::instructions::RawInstruction;

use super::exclusive_args::{ExclusivePairLoadArgs, MakeExclusivePairLoadArgs};

/// A `ldxp` instruction with a destination and an address.
pub struct Ldxp<Rt> {
    args: ExclusivePairLoadArgs<Rt>,
}

impl<Rt: Copy> Ldxp<Rt> {
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

pub fn ldxp<TargetOut, Reg1Inp, Reg2Inp, AddrRegInp>(
    r1: Reg1Inp,
    r2: Reg2Inp,
    addr: AddrRegInp,
) -> Ldxp<TargetOut>
where
    ExclusivePairLoadArgs<TargetOut>: MakeExclusivePairLoadArgs<Reg1Inp, Reg2Inp, AddrRegInp>,
{
    Ldxp {
        args: <ExclusivePairLoadArgs<TargetOut> as MakeExclusivePairLoadArgs<
            Reg1Inp,
            Reg2Inp,
            AddrRegInp,
        >>::new(r1, r2, addr),
    }
}

impl RawInstruction for Ldxp<RegOrZero32> {
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        LDXP_LP32_ldstexclp(self.rt2().index(), self.addr().index(), self.rt1().index())
    }
}

impl RawInstruction for Ldxp<RegOrZero64> {
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        LDXP_LP64_ldstexclp(self.rt2().index(), self.addr().index(), self.rt1().index())
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

    const LDXP_TEST_DB: &str = "
887f0903	ldxp w3, w2, [x8]
c87f0903	ldxp x3, x2, [x8]
887f13ff	ldxp wzr, w4, [sp]
c87f13ff	ldxp xzr, x4, [sp]
";
    test_cases! {
        LDXP_TEST_DB, untested_ldxp_test_db;
        test_ldxp_w2_x8, ldxp(W3, W2, X8), "ldxp w3, w2, [x8]";
        test_ldxp_x2_x8, ldxp(X3, X2, X8), "ldxp x3, x2, [x8]";
        test_ldxp_wzr_sp, ldxp(WZR, W4, SP), "ldxp wzr, w4, [sp]";
        test_ldxp_xzr_sp, ldxp(XZR, X4, SP), "ldxp xzr, x4, [sp]";
    }
}
