/* Copyright (C) 2026 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use aarchmrs_instructions::A64::ldst::ldstexclr::STXRH_SR32_ldstexclr::STXRH_SR32_ldstexclr;

use super::exclusive_args::{ExclusiveStore32Args, MakeExclusiveStore32Args};
use crate::instructions::RawInstruction;
use crate::register::{RegOrSp64, RegOrZero32, Register as _};

/// A `stxrh` instruction with a destination and an address.
pub struct Stxrh {
    args: ExclusiveStore32Args,
}

impl Stxrh {
    #[inline]
    pub fn rt(&self) -> RegOrZero32 {
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

pub fn stxrh<StatusInp, TargetInp, AddrInp>(
    status: StatusInp,
    dst: TargetInp,
    addr: AddrInp,
) -> Stxrh
where
    ExclusiveStore32Args: MakeExclusiveStore32Args<StatusInp, TargetInp, AddrInp>,
{
    Stxrh {
        args:
            <ExclusiveStore32Args as MakeExclusiveStore32Args<StatusInp, TargetInp, AddrInp>>::new(
                status, dst, addr,
            ),
    }
}

impl RawInstruction for Stxrh {
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        STXRH_SR32_ldstexclr(
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
    use harm_test_utils::test_cases;

    const STXRH_TEST_DB: &str = "
48017d02	stxrh w1, w2, [x8]
48017fff	stxrh w1, wzr, [sp]
";
    test_cases! {
        STXRH_TEST_DB, untested_stxrh_test_db;
        test_stxrh_w2_x8, stxrh(W1, W2, X8), "stxrh w1, w2, [x8]";
        test_stxrh_wzr_sp, stxrh(W1, WZR, SP), "stxrh w1, wzr, [sp]";
    }
}
