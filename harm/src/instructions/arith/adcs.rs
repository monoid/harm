/* Copyright (C) 2026 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use aarchmrs_instructions::A64::dpreg::addsub_carry::{
    ADCS_32_addsub_carry::ADCS_32_addsub_carry, ADCS_64_addsub_carry::ADCS_64_addsub_carry,
};
use aarchmrs_types::InstructionCode;

use crate::{
    instructions::RawInstruction,
    register::{IntoReg, Reg32, Reg64, RegOrZero32, RegOrZero64, Register as _},
    sealed::Sealed,
};

pub fn adcs<T, RealT, S1, S2, RealS1, RealS2>(
    dst: T,
    src1: S1,
    src2: S2,
) -> <Adcs<RealT, RealS1, RealS2> as MakeAdcs<T, S1, S2>>::Output
where
    Adcs<RealT, RealS1, RealS2>: MakeAdcs<T, S1, S2>,
{
    Adcs::<RealT, RealS1, RealS2>::new(dst, src1, src2)
}

pub trait MakeAdcs<T, S1, S2>: Sealed {
    type Output;

    fn new(dst: T, src1: S1, src2: S2) -> Self::Output;
}

pub struct Adcs<T, S1, S2> {
    pub dst: T,
    pub src1: S1,
    pub src2: S2,
}

impl<T, S1, S2> Sealed for Adcs<T, S1, S2> {}

impl MakeAdcs<Reg64, Reg64, Reg64> for Adcs<Reg64, Reg64, Reg64> {
    type Output = Self;

    #[inline]
    fn new(dst: Reg64, src1: Reg64, src2: Reg64) -> Self {
        Self { dst, src1, src2 }
    }
}

impl MakeAdcs<Reg32, Reg32, Reg32> for Adcs<Reg32, Reg32, Reg32> {
    type Output = Self;

    #[inline]
    fn new(dst: Reg32, src1: Reg32, src2: Reg32) -> Self {
        Self { dst, src1, src2 }
    }
}

define_arith_carry!(Adcs, 32, addsub, RegOrZero32, Reg32);
define_arith_carry!(Adcs, 64, addsub, RegOrZero64, Reg64);

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

    const ADCS_DB: &str = "
3a0c0041	adcs w1, w2, w12
3a1f0041	adcs w1, w2, wzr
3a0c03e1	adcs w1, wzr, w12
ba0c0041	adcs x1, x2, x12
ba1f0041	adcs x1, x2, xzr
ba0c03e1	adcs x1, xzr, x12
";

    test_cases! {
        ADCS_DB, untested_adcs_db;
        test_adcs_32, adcs(W1, W2, W12), "adcs w1, w2, w12";
        test_adcs_64, adcs(X1, X2, X12), "adcs x1, x2, x12";
        test_adcs_64_zero, adcs(RegZ(X1), XZR, X12), "adcs x1, xzr, x12";
        test_adcs_32_zero, adcs(RegZ32(W1), WZR, W12), "adcs w1, wzr, w12";
        test_adcs_zero_64, adcs(RegZ(X1), X2, XZR), "adcs x1, x2, xzr";
        test_adcs_zero_32, adcs(RegZ32(W1), W2, WZR), "adcs w1, w2, wzr";
    }
}
