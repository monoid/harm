/* Copyright (C) 2026 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use aarchmrs_instructions::A64::dpreg::addsub_carry::{
    SBC_32_addsub_carry::SBC_32_addsub_carry, SBC_64_addsub_carry::SBC_64_addsub_carry,
};
use aarchmrs_types::InstructionCode;

use crate::{
    instructions::RawInstruction,
    register::{IntoReg, Reg32, Reg64, RegOrZero32, RegOrZero64, Register as _},
    sealed::Sealed,
};

pub fn sbc<T, RealT, S1, S2, RealS1, RealS2>(
    dst: T,
    src1: S1,
    src2: S2,
) -> <Sbc<RealT, RealS1, RealS2> as MakeSbc<T, S1, S2>>::Output
where
    Sbc<RealT, RealS1, RealS2>: MakeSbc<T, S1, S2>,
{
    Sbc::<RealT, RealS1, RealS2>::new(dst, src1, src2)
}

pub trait MakeSbc<T, S1, S2>: Sealed {
    type Output;

    fn new(dst: T, src1: S1, src2: S2) -> Self::Output;
}

pub struct Sbc<T, S1, S2> {
    pub dst: T,
    pub src1: S1,
    pub src2: S2,
}

impl<T, S1, S2> Sealed for Sbc<T, S1, S2> {}

impl MakeSbc<Reg64, Reg64, Reg64> for Sbc<Reg64, Reg64, Reg64> {
    type Output = Self;

    #[inline]
    fn new(dst: Reg64, src1: Reg64, src2: Reg64) -> Self {
        Self { dst, src1, src2 }
    }
}

impl MakeSbc<Reg32, Reg32, Reg32> for Sbc<Reg32, Reg32, Reg32> {
    type Output = Self;

    #[inline]
    fn new(dst: Reg32, src1: Reg32, src2: Reg32) -> Self {
        Self { dst, src1, src2 }
    }
}

define_arith_carry!(Sbc, 32, addsub, RegOrZero32, Reg32);
define_arith_carry!(Sbc, 64, addsub, RegOrZero64, Reg64);

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

    const SBC_DB: &str = "
5a0c0041	sbc w1, w2, w12
5a1f0041	sbc w1, w2, wzr
5a0c03e1	sbc w1, wzr, w12
da0c0041	sbc x1, x2, x12
da1f0041	sbc x1, x2, xzr
da0c03e1	sbc x1, xzr, x12
";

    test_cases! {
        SBC_DB, untested_sbc_db;
        test_sbc_32, sbc(W1, W2, W12), "sbc w1, w2, w12";
        test_sbc_64, sbc(X1, X2, X12), "sbc x1, x2, x12";
        test_sbc_64_zero, sbc(RegZ(X1), XZR, X12), "sbc x1, xzr, x12";
        test_sbc_32_zero, sbc(RegZ32(W1), WZR, W12), "sbc w1, wzr, w12";
        test_sbc_zero_64, sbc(RegZ(X1), X2, XZR), "sbc x1, x2, xzr";
        test_sbc_zero_32, sbc(RegZ32(W1), W2, WZR), "sbc w1, w2, wzr";
    }
}
