/* Copyright (C) 2026 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use aarchmrs_instructions::A64::ldst::ldstexclr::STXRB_SR32_ldstexclr::STXRB_SR32_ldstexclr;

use super::exclusive_args::{ExclusiveStore32Args, MakeExclusiveStore32Args};
use crate::instructions::RawInstruction;
use crate::register::{RegOrSp64, RegOrZero32, Register as _};

/// A `stxrb` instruction with a destination and an address.
pub struct Stxrb {
    args: ExclusiveStore32Args,
}

impl Stxrb {
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

pub fn stxrb<StatusInp, TargetInp, AddrInp>(
    status: StatusInp,
    dst: TargetInp,
    addr: AddrInp,
) -> Stxrb
where
    ExclusiveStore32Args: MakeExclusiveStore32Args<StatusInp, TargetInp, AddrInp>,
{
    Stxrb {
        args:
            <ExclusiveStore32Args as MakeExclusiveStore32Args<StatusInp, TargetInp, AddrInp>>::new(
                status, dst, addr,
            ),
    }
}

impl RawInstruction for Stxrb {
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        STXRB_SR32_ldstexclr(
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

    const STXRB_TEST_DB: &str = "
08017d02	stxrb w1, w2, [x8]
08017fff	stxrb w1, wzr, [sp]
";
    test_cases! {
        STXRB_TEST_DB, untested_stxrb_test_db;
        test_stxrb_w2_x8, stxrb(W1, W2, X8), "stxrb w1, w2, [x8]";
        test_stxrb_wzr_sp, stxrb(W1, WZR, SP), "stxrb w1, wzr, [sp]";
    }
}
