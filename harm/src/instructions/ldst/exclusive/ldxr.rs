/* Copyright (C) 2026 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use crate::register::{RegOrSp64, RegOrZero32, RegOrZero64, Register};
use aarchmrs_instructions::A64::ldst::ldstexclr::{
    LDXR_LR32_ldstexclr::LDXR_LR32_ldstexclr, LDXR_LR64_ldstexclr::LDXR_LR64_ldstexclr,
};

use crate::instructions::RawInstruction;

use super::super::args::reg_addr::{MakeRegAddr, RegAddr};

/// A `ldxr` instruction with a destination and an address.
pub struct Ldxr<Rt> {
    args: RegAddr<Rt>,
}

impl<Rt: Copy> Ldxr<Rt> {
    pub fn rt(&self) -> Rt {
        self.args.reg
    }

    pub fn addr(&self) -> RegOrSp64 {
        self.args.addr
    }
}

pub fn ldxr<TargetInp, TargetOut, AddrInp>(dst: TargetInp, addr: AddrInp) -> Ldxr<TargetOut>
where
    RegAddr<TargetOut>: MakeRegAddr<TargetInp, AddrInp>,
{
    Ldxr {
        args: <RegAddr<TargetOut> as MakeRegAddr<TargetInp, AddrInp>>::new(dst, addr),
    }
}

impl RawInstruction for Ldxr<RegOrZero32> {
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        LDXR_LR32_ldstexclr(self.addr().index(), self.rt().index())
    }
}

impl RawInstruction for Ldxr<RegOrZero64> {
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        LDXR_LR64_ldstexclr(self.addr().index(), self.rt().index())
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

    const LDXR_TEST_DB: &str = "
885f7d02	ldxr w2, [x8]
c85f7d02	ldxr x2, [x8]
885f7fff	ldxr wzr, [sp]
c85f7fff	ldxr xzr, [sp]
";
    test_cases! {
        LDXR_TEST_DB, untested_ldxr_test_db;
        test_ldxr_w2_x8, ldxr(W2, X8), "ldxr w2, [x8]";
        test_ldxr_x2_x8, ldxr(X2, X8), "ldxr x2, [x8]";
        test_ldxr_wzr_sp, ldxr(WZR, SP), "ldxr wzr, [sp]";
        test_ldxr_xzr_sp, ldxr(XZR, SP), "ldxr xzr, [sp]";
    }
}
