/* Copyright (C) 2026 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use aarchmrs_instructions::A64::dpreg::addsub_carry::{
    ADC_32_addsub_carry::ADC_32_addsub_carry, ADC_64_addsub_carry::ADC_64_addsub_carry,
};
use aarchmrs_types::InstructionCode;

use crate::{
    instructions::RawInstruction,
    register::{IntoReg, Reg32, Reg64, RegOrZero32, RegOrZero64, Register as _},
    sealed::Sealed,
};

pub fn adc<T, RealT, S1, S2, RealS1, RealS2>(
    dst: T,
    src1: S1,
    src2: S2,
) -> <Adc<RealT, RealS1, RealS2> as MakeAdc<T, S1, S2>>::Output
where
    Adc<RealT, RealS1, RealS2>: MakeAdc<T, S1, S2>,
{
    Adc::<RealT, RealS1, RealS2>::new(dst, src1, src2)
}

pub trait MakeAdc<T, S1, S2>: Sealed {
    type Output;

    fn new(dst: T, src1: S1, src2: S2) -> Self::Output;
}

pub struct Adc<T, S1, S2> {
    pub dst: T,
    pub src1: S1,
    pub src2: S2,
}

impl<T, S1, S2> Sealed for Adc<T, S1, S2> {}

impl MakeAdc<Reg64, Reg64, Reg64> for Adc<Reg64, Reg64, Reg64> {
    type Output = Self;

    #[inline]
    fn new(dst: Reg64, src1: Reg64, src2: Reg64) -> Self {
        Self { dst, src1, src2 }
    }
}

impl MakeAdc<Reg32, Reg32, Reg32> for Adc<Reg32, Reg32, Reg32> {
    type Output = Self;

    #[inline]
    fn new(dst: Reg32, src1: Reg32, src2: Reg32) -> Self {
        Self { dst, src1, src2 }
    }
}

define_arith_carry!(Adc, 32, addsub, RegOrZero32, Reg32);
define_arith_carry!(Adc, 64, addsub, RegOrZero64, Reg64);

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

    const ADC_DB: &str = "
1a0c0041	adc w1, w2, w12
1a1f0041	adc w1, w2, wzr
1a0c03e1	adc w1, wzr, w12
9a0c0041	adc x1, x2, x12
9a1f0041	adc x1, x2, xzr
9a0c03e1	adc x1, xzr, x12
";

    test_cases! {
        ADC_DB, untested_adc_db;
        test_adc_32, adc(W1, W2, W12), "adc w1, w2, w12";
        test_adc_64, adc(X1, X2, X12), "adc x1, x2, x12";
        test_adc_64_zero, adc(RegZ(X1), XZR, X12), "adc x1, xzr, x12";
        test_adc_32_zero, adc(RegZ32(W1), WZR, W12), "adc w1, wzr, w12";
        test_adc_zero_64, adc(RegZ(X1), X2, XZR), "adc x1, x2, xzr";
        test_adc_zero_32, adc(RegZ32(W1), W2, WZR), "adc w1, w2, wzr";
    }
}
