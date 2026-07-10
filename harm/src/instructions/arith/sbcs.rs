/* Copyright (C) 2026 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use aarchmrs_instructions::A64::dpreg::addsub_carry::{
    SBCS_32_addsub_carry::SBCS_32_addsub_carry, SBCS_64_addsub_carry::SBCS_64_addsub_carry,
};
use aarchmrs_types::InstructionCode;

use crate::{
    instructions::RawInstruction,
    register::{IntoReg, Reg32, Reg64, RegOrZero32, RegOrZero64, Register as _},
    sealed::Sealed,
};

pub fn sbcs<T, RealT, S1, S2, RealS1, RealS2>(
    dst: T,
    src1: S1,
    src2: S2,
) -> <Sbcs<RealT, RealS1, RealS2> as MakeSbcs<T, S1, S2>>::Output
where
    Sbcs<RealT, RealS1, RealS2>: MakeSbcs<T, S1, S2>,
{
    Sbcs::<RealT, RealS1, RealS2>::new(dst, src1, src2)
}

pub trait MakeSbcs<T, S1, S2>: Sealed {
    type Output;

    fn new(dst: T, src1: S1, src2: S2) -> Self::Output;
}

pub struct Sbcs<T, S1, S2> {
    pub dst: T,
    pub src1: S1,
    pub src2: S2,
}

impl<T, S1, S2> Sealed for Sbcs<T, S1, S2> {}

impl MakeSbcs<Reg64, Reg64, Reg64> for Sbcs<Reg64, Reg64, Reg64> {
    type Output = Self;

    #[inline]
    fn new(dst: Reg64, src1: Reg64, src2: Reg64) -> Self {
        Self { dst, src1, src2 }
    }
}

impl MakeSbcs<Reg32, Reg32, Reg32> for Sbcs<Reg32, Reg32, Reg32> {
    type Output = Self;

    #[inline]
    fn new(dst: Reg32, src1: Reg32, src2: Reg32) -> Self {
        Self { dst, src1, src2 }
    }
}

define_arith_carry!(Sbcs, 32, addsub, RegOrZero32, Reg32);
define_arith_carry!(Sbcs, 64, addsub, RegOrZero64, Reg64);

#[cfg(test)]
mod tests {
    use harm_test_utils::test_cases;

    use super::*;
    use crate::instructions::InstructionSeq;
    use Reg32::*;
    use Reg64::*;
    use RegOrZero32::Reg as RegZ32;
    use RegOrZero32::WZR;
    use RegOrZero64::Reg as RegZ;
    use RegOrZero64::XZR;

    const SBCS_DB: &str = "
7a0c0041	sbcs w1, w2, w12
7a1f0041	sbcs w1, w2, wzr
7a0c03e1	sbcs w1, wzr, w12
fa0c0041	sbcs x1, x2, x12
fa1f0041	sbcs x1, x2, xzr
fa0c03e1	sbcs x1, xzr, x12
";

    test_cases! {
        SBCS_DB, untested_sbcs_db;
        test_sbcs_32, sbcs(W1, W2, W12), "sbcs w1, w2, w12";
        test_sbcs_64, sbcs(X1, X2, X12), "sbcs x1, x2, x12";
        test_sbcs_64_zero, sbcs(RegZ(X1), XZR, X12), "sbcs x1, xzr, x12";
        test_sbcs_32_zero, sbcs(RegZ32(W1), WZR, W12), "sbcs w1, wzr, w12";
        test_sbcs_zero_64, sbcs(RegZ(X1), X2, XZR), "sbcs x1, x2, xzr";
        test_sbcs_zero_32, sbcs(RegZ32(W1), W2, WZR), "sbcs w1, w2, wzr";
    }
}
