/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use aarchmrs_instructions::A64::control::branch_reg::{
    BLR_64_branch_reg::BLR_64_branch_reg, BR_64_branch_reg::BR_64_branch_reg,
    RET_64R_branch_reg::RET_64R_branch_reg,
};
use aarchmrs_types::InstructionCode;

use crate::instructions::RawInstruction;
use crate::register::{IntoReg, Reg64, RegOrZero64, Register as _};

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
    pub fn reg(mut self, reg: impl IntoReg<RegOrZero64>) -> Self {
        self.0 = reg.into_reg();
        self
    }
}

impl RawInstruction for Ret {
    #[inline]
    fn to_code(&self) -> InstructionCode {
        RET_64R_branch_reg(self.0.index())
    }
}

pub struct Br(RegOrZero64);

impl RawInstruction for Br {
    #[inline]
    fn to_code(&self) -> InstructionCode {
        BR_64_branch_reg(self.0.index())
    }
}

#[inline]
pub fn br(reg: impl IntoReg<RegOrZero64>) -> Br {
    Br(reg.into_reg())
}

pub struct Blr(RegOrZero64);

impl RawInstruction for Blr {
    #[inline]
    fn to_code(&self) -> InstructionCode {
        BLR_64_branch_reg(self.0.index())
    }
}

#[inline]
pub fn blr(reg: impl Into<RegOrZero64>) -> Blr {
    Blr(reg.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::InstructionSeq;
    use Reg64::*;
    use RegOrZero64::XZR;
    use harm_test_utils::test_cases;

    const BRANCH_REG_DB: &str = "
d61f0020	br x1
d61f03e0	br xzr
d63f0020	blr x1
d63f03e0	blr xzr
d65f0060	ret x3
d65f03c0	ret
d65f03e0	ret xzr
";

    test_cases!(
        BRANCH_REG_DB, untested_ret_db;
        test_br_reg, br(X1), "br x1";
        test_br_xzr, br(XZR), "br xzr";
        test_blr_reg, blr(X1), "blr x1";
        test_blr_xzr, blr(XZR), "blr xzr";
        test_ret, ret(), "ret";
        test_ret_reg, ret().reg(X3), "ret x3";
        test_ret_xzr, ret().reg(XZR), "ret xzr";
    );
}
