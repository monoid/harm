/* Copyright (C) 2026 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use crate::register::{RegOrSp64, RegOrZero32, RegOrZero64, Register};
use aarchmrs_instructions::A64::ldst::ldstexclr::{
    STXR_SR32_ldstexclr::STXR_SR32_ldstexclr, STXR_SR64_ldstexclr::STXR_SR64_ldstexclr,
};

use crate::instructions::RawInstruction;

use super::exclusive_args::{ExclusiveStoreArgs, MakeExclusiveStoreArgs};

/// A `stxr` instruction with a destination and an address.
pub struct Stxr<Rt> {
    args: ExclusiveStoreArgs<Rt>,
}

impl<Rt: Copy> Stxr<Rt> {
    #[inline]
    pub fn rt(&self) -> Rt {
        self.args.reg
    }

    #[inline]
    pub fn addr(&self) -> RegOrSp64 {
        self.args.addr
    }

    #[inline]
    pub fn status(&self) -> RegOrZero32 {
        self.args.status
    }
}

pub fn stxr<StatusInp, TargetInp, TargetOut, AddrInp>(
    status: StatusInp,
    dst: TargetInp,
    addr: AddrInp,
) -> Stxr<TargetOut>
where
    ExclusiveStoreArgs<TargetOut>: MakeExclusiveStoreArgs<StatusInp, TargetInp, AddrInp>,
{
    Stxr {
        args: <ExclusiveStoreArgs<TargetOut> as MakeExclusiveStoreArgs<
            StatusInp,
            TargetInp,
            AddrInp,
        >>::new(status, dst, addr),
    }
}

impl RawInstruction for Stxr<RegOrZero32> {
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        STXR_SR32_ldstexclr(
            self.status().index(),
            self.addr().index(),
            self.rt().index(),
        )
    }
}

impl RawInstruction for Stxr<RegOrZero64> {
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        STXR_SR64_ldstexclr(
            self.status().index(),
            self.addr().index(),
            self.rt().index(),
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

    const STXR_TEST_DB: &str = "
88017d02	stxr w1, w2, [x8]
c8017d02	stxr w1, x2, [x8]
88017fff	stxr w1, wzr, [sp]
c8017fff	stxr w1, xzr, [sp]
";
    test_cases! {
        STXR_TEST_DB, untested_stxr_test_db;
        test_stxr_w2_x8, stxr(W1, W2, X8), "stxr w1, w2, [x8]";
        test_stxr_x2_x8, stxr(W1, X2, X8), "stxr w1, x2, [x8]";
        test_stxr_wzr_sp, stxr(W1, WZR, SP), "stxr w1, wzr, [sp]";
        test_stxr_xzr_sp, stxr(W1, XZR, SP), "stxr w1, xzr, [sp]";
    }
}
