/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use aarchmrs_instructions::A64::control::branch_reg::RET_64R_branch_reg::RET_64R_branch_reg;
use aarchmrs_types::InstructionCode;

use super::RawInstruction;
use crate::register::{IntoCode as _, Reg64, RegOrZero64};

#[inline]
pub fn ret() -> Ret {
    Ret::new()
}

#[derive(Debug, Copy, Clone)]
pub struct Ret(RegOrZero64);

impl Ret {
    #[allow(clippy::new_without_default)]
    #[inline]
    pub fn new() -> Self {
        Ret(RegOrZero64::Reg(Reg64::LR))
    }

    #[inline]
    pub fn reg(mut self, reg: impl Into<RegOrZero64>) -> Self {
        self.0 = reg.into();
        self
    }
}

impl RawInstruction for Ret {
    #[inline]
    fn to_code(&self) -> InstructionCode {
        let reg = self.0;

        RET_64R_branch_reg(reg.code())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::InstructionSeq;
    use harm_test_utils::test_cases;

    const RET_DB: &str = "
d65f03c0	ret
d65f0060	ret x3
";

    test_cases!(
        RET_DB, untested_ret_db;
        test_ret, ret(), "ret";
        test_ret_reg, ret().reg(Reg64::X3), "ret x3";
    );
}
