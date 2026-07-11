/* Copyright (C) 2026 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use aarchmrs_instructions::A64::{
    dpimm::addsub_imm::{
        ADDS_32S_addsub_imm::ADDS_32S_addsub_imm, ADDS_64S_addsub_imm::ADDS_64S_addsub_imm,
    },
    dpreg::{
        addsub_ext::{
            ADDS_32S_addsub_ext::ADDS_32S_addsub_ext, ADDS_64S_addsub_ext::ADDS_64S_addsub_ext,
        },
        addsub_shift::{
            ADDS_32_addsub_shift::ADDS_32_addsub_shift, ADDS_64_addsub_shift::ADDS_64_addsub_shift,
        },
    },
};
use aarchmrs_types::InstructionCode;

use super::*;
use crate::{
    bits::BitError,
    instructions::RawInstruction,
    register::{
        IntoReg, Reg32, Reg64, RegOrSp32, RegOrSp64, RegOrZero32, RegOrZero64, Register as _,
    },
    sealed::Sealed,
};

pub fn adds<T, RealT, S1, S2, RealS1, RealS2>(
    dst: T,
    src1: S1,
    src2: S2,
) -> <Adds<RealT, RealS1, RealS2> as MakeAdds<T, S1, S2>>::Output
where
    Adds<RealT, RealS1, RealS2>: MakeAdds<T, S1, S2>,
{
    Adds::<RealT, RealS1, RealS2>::new(dst, src1, src2)
}

pub trait MakeAdds<T, S1, S2>: Sealed {
    type Output;

    fn new(dst: T, src1: S1, src2: S2) -> Self::Output;
}

pub struct Adds<T, S1, S2> {
    pub dst: T,
    pub src1: S1,
    pub src2: S2,
}

impl<T, S1, S2> Sealed for Adds<T, S1, S2> {}

impl MakeAdds<Reg64, Reg64, Reg64> for Adds<Reg64, Reg64, Reg64> {
    type Output = Self;

    #[inline]
    fn new(dst: Reg64, src1: Reg64, src2: Reg64) -> Self {
        Self { dst, src1, src2 }
    }
}

impl MakeAdds<Reg32, Reg32, Reg32> for Adds<Reg32, Reg32, Reg32> {
    type Output = Self;

    #[inline]
    fn new(dst: Reg32, src1: Reg32, src2: Reg32) -> Self {
        Self { dst, src1, src2 }
    }
}

define_arith_fallible!(Adds);

define_arith_shift!(Adds, 32, addsub, RegOrZero32, Reg32);
define_arith_shift!(Adds, 64, addsub, RegOrZero64, Reg64);

// N.B.: `add`/`sub` use `RegOrSp{N}, RegOrSp{N}, RegOrZero{N}`
define_arith_extend!(
    Adds,
    32S,
    addsub,
    RegOrZero32,
    RegOrSp32,
    RegOrZero32,
    Reg32
);
define_arith_extend!(
    Adds,
    64S,
    addsub,
    RegOrZero64,
    RegOrSp64,
    RegOrZero64,
    Reg64
);

// N.B.: `add`/`sub` use `RegOrSp{N}, RegOrSp{N}`
define_arith_imm12!(Adds, 32S, addsub, RegOrZero32, RegOrSp32);
define_arith_imm12!(Adds, 64S, addsub, RegOrZero64, RegOrSp64);

#[cfg(test)]
mod tests {
    use harm_test_utils::test_cases;

    use super::*;
    use crate::instructions::InstructionSeq;
    use crate::instructions::arith::AddSubImm12;
    use Reg32::*;
    use Reg64::*;
    use RegOrSp32::Reg as Reg3S;
    use RegOrSp64::Reg as RegS;
    use RegOrZero32::Reg as Reg3Z;
    use RegOrZero32::WZR;
    use RegOrZero64::Reg as RegZ;
    use RegOrZero64::XZR;

    const ADDS_DB: &str = "
ab3f2c41	adds x1, x2, wzr, uxth #3
2b0c0041	adds w1, w2, w12
2b2c6c41	adds w1, w2, w12, uxtx #3
2b2c4c41	adds w1, w2, w12, uxtw #3
2b3f6c41	adds w1, w2, wzr, uxtx #3
2b3f4c41	adds w1, w2, wzr, uxtw #3
2b4c1041	adds w1, w2, w12, lsr #4
2b4c13e1	adds w1, wzr, w12, lsr #4
31048c41	adds w1, w2, #0x123
31448c41	adds w1, w2, #0x123000
ab0c0041	adds x1, x2, x12
ab2c4c41	adds x1, x2, w12, uxtw #3
ab2c6c41	adds x1, x2, x12, uxtx #3
ab2c7041	adds x1, x2, x12, uxtx #4
ab3f4c41	adds x1, x2, wzr, uxtw #3
ab4c1041	adds x1, x2, x12, lsr #4
ab4c13e1	adds x1, xzr, x12, lsr #4
b1000441	adds x1, x2, #1
b1400441	adds x1, x2, #0x1000
";

    test_cases! {
        ADDS_DB, untested_adds_db;
        test_adds_64, adds(X1, X2, X12), "adds x1, x2, x12";
        test_adds_64_shift, adds(X1, X2, X12).try_shift(ShiftMode::LSR, 4).unwrap(), "adds x1, x2, x12, lsr #4";
        test_adds_64_zero,
            adds(X1, XZR, ShiftedReg::from(X12).try_shift(ShiftMode::LSR, 4).unwrap()),
            "adds x1, xzr, x12, lsr #4";
        test_adds_64_shift_2,
            adds(X1, X2, ShiftedReg::from(X12).try_shift(ShiftMode::LSR, 4)).unwrap(),
            "adds x1, x2, x12, lsr #4";
        test_adds_64_shift_3,
            adds(X1, X2, (X12, ShiftMode::LSR, 4)).unwrap(),
            "adds x1, x2, x12, lsr #4";
        test_adds_64_extend_uxtx, adds(RegZ(X1), RegS(X2), X12).extend(ExtendMode::UXTX, ExtendShiftAmount::try_new(3).unwrap()),
            "adds x1, x2, x12, uxtx #3";
        test_adds_64_extend_uxtx_2, adds(RegZ(X1), X2, (X12, ExtendMode::UXTX, 3)).unwrap(),
            "adds x1, x2, x12, uxtx #3";
        // KLUDGE: Using Reg64 instead of Reg32 at the last argument.
        // To be reimplemented akin `ldr` family.
        test_adds_64_extend_uxtw,
            adds(X1, X2, X12).extend(ExtendMode::UXTW, ExtendShiftAmount::try_new(3).unwrap()),
            "adds x1, x2, w12, uxtw #3";
        test_adds_64_wzr_extend_uxtw,
            adds(RegZ(X1), RegS(X2), XZR).extend(ExtendMode::UXTW, ExtendShiftAmount::try_new(3).unwrap()),
            "adds x1, x2, wzr, uxtw #3";
        test_adds_64_extend_uxtx_4,
            adds(X1, X2, X12).extend(ExtendMode::UXTX, ExtendShiftAmount::try_new(4).unwrap()),
            "adds x1, x2, x12, uxtx #4";
        test_adds_64_extend_uxth_xzr,
            adds(RegZ(X1), RegS(X2), XZR).extend(ExtendMode::UXTH, ExtendShiftAmount::try_new(3).unwrap()),
            "adds x1, x2, wzr, uxth #3";
        test_adds_64_const_1, adds(X1, X2, 1u32).unwrap(), "adds x1, x2, #1";
        test_adds_64_const_1_1, adds(X1, X2, AddSubImm12::try_from(1).unwrap()), "adds x1, x2, #1";
        test_adds_64_const_0x1000, adds(X1, X2, 0x1000).unwrap(), "adds x1, x2, #0x1000";
        test_adds_32, adds(W1, W2, W12), "adds w1, w2, w12";
        test_adds_32_shift, adds(W1, W2, W12).try_shift(ShiftMode::LSR, 4).unwrap(), "adds w1, w2, w12, lsr #4";
        test_adds_32_zero,
            adds(W1, WZR, ShiftedReg::from(W12).try_shift(ShiftMode::LSR, 4).unwrap()),
            "adds w1, wzr, w12, lsr #4";
        test_adds_32_extend_uxtx,
            adds(W1, W2, W12).extend(ExtendMode::UXTX, ExtendShiftAmount::try_new(3).unwrap()),
            "adds w1, w2, w12, uxtx #3";
        test_adds_32_extend_uxtw,
            adds(W1, W2, W12).extend(ExtendMode::UXTW, ExtendShiftAmount::try_new(3).unwrap()),
            "adds w1, w2, w12, uxtw #3";
        test_adds_32_extend_uxtx_wzr,  // that's really strange it works
            adds(Reg3Z(W1), Reg3S(W2), WZR).extend(ExtendMode::UXTX, ExtendShiftAmount::try_new(3).unwrap()),
            "adds w1, w2, wzr, uxtx #3";
        test_adds_32_extend_uxtw_wzr,
            adds(Reg3Z(W1), Reg3S(W2), WZR).extend(ExtendMode::UXTW, ExtendShiftAmount::try_new(3).unwrap()),
            "adds w1, w2, wzr, uxtw #3";
        test_adds_32_const_0x123, adds(W1, W2, 0x123).unwrap(), "adds w1, w2, #0x123";
        test_adds_32_const_0x123_1, adds(W1, W2, AddSubImm12::try_from(0x123)).unwrap(), "adds w1, w2, #0x123";
        test_adds_32_const_0x123000, adds(W1, W2, 0x123000).unwrap(), "adds w1, w2, #0x123000";
    }

    #[test]
    fn test_adds_64_const_0x1001() {
        let a = adds(X1, X2, 0x1001);
        assert!(a.is_err());
    }

    #[test]
    fn test_adds_32_const_0x1001() {
        let a = adds(W1, W2, 0x1001);
        assert!(a.is_err());
    }
}
