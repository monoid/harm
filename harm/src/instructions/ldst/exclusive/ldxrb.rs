/* Copyright (C) 2026 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use crate::register::{RegOrSp64, RegOrZero32, Register as _};
use aarchmrs_instructions::A64::ldst::ldstexclr::LDXRB_LR32_ldstexclr::LDXRB_LR32_ldstexclr;

use crate::instructions::RawInstruction;

use super::super::args::reg_addr::{MakeReg32Addr, Reg32Addr};

/// A `ldxrb` instruction with a destination and an address.
#[derive(Debug, Clone, Copy)]
pub struct Ldxrb {
    args: Reg32Addr,
}

impl Ldxrb {
    #[inline]
    pub fn rt(&self) -> RegOrZero32 {
        self.args.reg
    }

    #[inline]
    pub fn addr(&self) -> RegOrSp64 {
        self.args.addr
    }
}

pub fn ldxrb<TargetInp, AddrInp>(dst: TargetInp, addr: AddrInp) -> Ldxrb
where
    Reg32Addr: MakeReg32Addr<TargetInp, AddrInp>,
{
    Ldxrb {
        args: <Reg32Addr as MakeReg32Addr<TargetInp, AddrInp>>::new(dst, addr),
    }
}

impl RawInstruction for Ldxrb {
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        LDXRB_LR32_ldstexclr(self.addr().index(), self.rt().index())
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

    const LDXRB_TEST_DB: &str = "
085f7d02	ldxrb w2, [x8]
085f7fff	ldxrb wzr, [sp]
";
    test_cases! {
        LDXRB_TEST_DB, untested_ldxrb_test_db;
        test_ldxrb_w2_x8, ldxrb(W2, X8), "ldxrb w2, [x8]";
        test_ldxrb_wzr_sp, ldxrb(WZR, SP), "ldxrb wzr, [sp]";
    }
}
