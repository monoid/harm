/* Copyright (C) 2026 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use crate::register::{RegOrSp64, RegOrZero32, Register as _};
use aarchmrs_instructions::A64::ldst::ldstexclr::LDXRH_LR32_ldstexclr::LDXRH_LR32_ldstexclr;

use crate::instructions::RawInstruction;

use super::super::args::reg_addr::{MakeReg32Addr, Reg32Addr};

/// A `ldxrh` instruction with a destination and an address.
#[derive(Debug, Clone, Copy)]
pub struct Ldxrh {
    args: Reg32Addr,
}

impl Ldxrh {
    #[inline]
    pub fn rt(&self) -> RegOrZero32 {
        self.args.reg
    }

    #[inline]
    pub fn addr(&self) -> RegOrSp64 {
        self.args.addr
    }
}

pub fn ldxrh<TargetInp, AddrInp>(dst: TargetInp, addr: AddrInp) -> Ldxrh
where
    Reg32Addr: MakeReg32Addr<TargetInp, AddrInp>,
{
    Ldxrh {
        args: <Reg32Addr as MakeReg32Addr<TargetInp, AddrInp>>::new(dst, addr),
    }
}

impl RawInstruction for Ldxrh {
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        LDXRH_LR32_ldstexclr(self.addr().index(), self.rt().index())
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

    const LDXRH_TEST_DB: &str = "
485f7d02	ldxrh w2, [x8]
485f7fff	ldxrh wzr, [sp]
";
    test_cases! {
        LDXRH_TEST_DB, untested_ldxrh_test_db;
        test_ldxrh_w2_x8, ldxrh(W2, X8), "ldxrh w2, [x8]";
        test_ldxrh_wzr_sp, ldxrh(WZR, SP), "ldxrh wzr, [sp]";
    }
}
