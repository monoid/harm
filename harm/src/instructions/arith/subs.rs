/* Copyright (C) 2026 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use aarchmrs_instructions::A64::{
    dpimm::addsub_imm::{
        SUBS_32S_addsub_imm::SUBS_32S_addsub_imm, SUBS_64S_addsub_imm::SUBS_64S_addsub_imm,
    },
    dpreg::{
        addsub_ext::{
            SUBS_32S_addsub_ext::SUBS_32S_addsub_ext, SUBS_64S_addsub_ext::SUBS_64S_addsub_ext,
        },
        addsub_shift::{
            SUBS_32_addsub_shift::SUBS_32_addsub_shift, SUBS_64_addsub_shift::SUBS_64_addsub_shift,
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

pub fn subs<T, RealT, S1, S2, RealS1, RealS2>(
    dst: T,
    src1: S1,
    src2: S2,
) -> <Subs<RealT, RealS1, RealS2> as MakeSubs<T, S1, S2>>::Output
where
    Subs<RealT, RealS1, RealS2>: MakeSubs<T, S1, S2>,
{
    Subs::<RealT, RealS1, RealS2>::new(dst, src1, src2)
}

pub trait MakeSubs<T, S1, S2>: Sealed {
    type Output;

    fn new(dst: T, src1: S1, src2: S2) -> Self::Output;
}

pub struct Subs<T, S1, S2> {
    pub dst: T,
    pub src1: S1,
    pub src2: S2,
}

impl<T, S1, S2> Sealed for Subs<T, S1, S2> {}

impl MakeSubs<Reg64, Reg64, Reg64> for Subs<Reg64, Reg64, Reg64> {
    type Output = Self;

    #[inline]
    fn new(dst: Reg64, src1: Reg64, src2: Reg64) -> Self {
        Self { dst, src1, src2 }
    }
}

impl MakeSubs<Reg32, Reg32, Reg32> for Subs<Reg32, Reg32, Reg32> {
    type Output = Self;

    #[inline]
    fn new(dst: Reg32, src1: Reg32, src2: Reg32) -> Self {
        Self { dst, src1, src2 }
    }
}

define_arith_faillible!(Subs);

define_arith_shift!(Subs, 32, addsub, RegOrZero32, Reg32);
define_arith_shift!(Subs, 64, addsub, RegOrZero64, Reg64);

// N.B.: `add`/`sub` use `RegOrSp{N}, RegOrSp{N}, RegOrZero{N}`
define_arith_extend!(
    Subs,
    32S,
    addsub,
    RegOrZero32,
    RegOrSp32,
    RegOrZero32,
    Reg32
);
define_arith_extend!(
    Subs,
    64S,
    addsub,
    RegOrZero64,
    RegOrSp64,
    RegOrZero64,
    Reg64
);

// N.B.: `add`/`sub` use `RegOrSp{N}, RegOrSp{N}`
define_arith_imm12!(Subs, 32S, addsub, RegOrZero32, RegOrSp32);
define_arith_imm12!(Subs, 64S, addsub, RegOrZero64, RegOrSp64);

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

    const SUBS_DB: &str = "
eb3f2c41	subs x1, x2, wzr, uxth #3
6b0c0041	subs w1, w2, w12
6b2c6c41	subs w1, w2, w12, uxtx #3
6b2c4c41	subs w1, w2, w12, uxtw #3
6b3f6c41	subs w1, w2, wzr, uxtx #3
6b3f4c41	subs w1, w2, wzr, uxtw #3
6b4c1041	subs w1, w2, w12, lsr #4
6b4c13e1	subs w1, wzr, w12, lsr #4
71048c41	subs w1, w2, #0x123
71448c41	subs w1, w2, #0x123000
eb0c0041	subs x1, x2, x12
eb2c4c41	subs x1, x2, w12, uxtw #3
eb2c6c41	subs x1, x2, x12, uxtx #3
eb2c7041	subs x1, x2, x12, uxtx #4
eb3f4c41	subs x1, x2, wzr, uxtw #3
eb4c1041	subs x1, x2, x12, lsr #4
eb4c13e1	subs x1, xzr, x12, lsr #4
f1000441	subs x1, x2, #1
f1400441	subs x1, x2, #0x1000
";

    test_cases! {
        SUBS_DB, untested_subs_db;
        test_subs_64, subs(X1, X2, X12), "subs x1, x2, x12";
        test_subs_64_shift, subs(X1, X2, X12).try_shift(ShiftMode::LSR, 4).unwrap(), "subs x1, x2, x12, lsr #4";
        test_subs_64_zero,
            subs(X1, XZR, ShiftedReg::from(X12).try_shift(ShiftMode::LSR, 4).unwrap()),
            "subs x1, xzr, x12, lsr #4";
        test_subs_64_shift_2,
            subs(X1, X2, ShiftedReg::from(X12).try_shift(ShiftMode::LSR, 4)).unwrap(),
            "subs x1, x2, x12, lsr #4";
        test_subs_64_shift_3,
            subs(X1, X2, (X12, ShiftMode::LSR, 4)).unwrap(),
            "subs x1, x2, x12, lsr #4";
        test_subs_64_extend_uxtx, subs(RegZ(X1), RegS(X2), X12).extend(ExtendMode::UXTX, ExtendShiftAmount::try_new(3).unwrap()),
            "subs x1, x2, x12, uxtx #3";
        test_subs_64_extend_uxtx_2, subs(RegZ(X1), X2, (X12, ExtendMode::UXTX, 3)).unwrap(),
            "subs x1, x2, x12, uxtx #3";
        // KLUDGE: Using Reg64 instead of Reg32 at the last argument.
        // To be reimplemented akin `ldr` family.
        test_subs_64_extend_uxtw,
            subs(X1, X2, X12).extend(ExtendMode::UXTW, ExtendShiftAmount::try_new(3).unwrap()),
            "subs x1, x2, w12, uxtw #3";
        test_subs_64_wzr_extend_uxtw,
            subs(RegZ(X1), RegS(X2), XZR).extend(ExtendMode::UXTW, ExtendShiftAmount::try_new(3).unwrap()),
            "subs x1, x2, wzr, uxtw #3";
        test_subs_64_extend_uxtx_4,
            subs(X1, X2, X12).extend(ExtendMode::UXTX, ExtendShiftAmount::try_new(4).unwrap()),
            "subs x1, x2, x12, uxtx #4";
        test_subs_64_extend_uxth_xzr,
            subs(RegZ(X1), RegS(X2), XZR).extend(ExtendMode::UXTH, ExtendShiftAmount::try_new(3).unwrap()),
            "subs x1, x2, wzr, uxth #3";
        test_subs_64_const_1, subs(X1, X2, 1u32).unwrap(), "subs x1, x2, #1";
        test_subs_64_const_1_1, subs(X1, X2, AddSubImm12::try_from(1).unwrap()), "subs x1, x2, #1";
        test_subs_64_const_0x1000, subs(X1, X2, 0x1000).unwrap(), "subs x1, x2, #0x1000";
        test_subs_32, subs(W1, W2, W12), "subs w1, w2, w12";
        test_subs_32_shift, subs(W1, W2, W12).try_shift(ShiftMode::LSR, 4).unwrap(), "subs w1, w2, w12, lsr #4";
        test_subs_32_zero,
            subs(W1, WZR, ShiftedReg::from(W12).try_shift(ShiftMode::LSR, 4).unwrap()),
            "subs w1, wzr, w12, lsr #4";
        test_subs_32_extend_uxtx,
            subs(W1, W2, W12).extend(ExtendMode::UXTX, ExtendShiftAmount::try_new(3).unwrap()),
            "subs w1, w2, w12, uxtx #3";
        test_subs_32_extend_uxtw,
            subs(W1, W2, W12).extend(ExtendMode::UXTW, ExtendShiftAmount::try_new(3).unwrap()),
            "subs w1, w2, w12, uxtw #3";
        test_subs_32_extend_uxtx_wzr,  // that's really strange it works
            subs(Reg3Z(W1), Reg3S(W2), WZR).extend(ExtendMode::UXTX, ExtendShiftAmount::try_new(3).unwrap()),
            "subs w1, w2, wzr, uxtx #3";
        test_subs_32_extend_uxtw_wzr,
            subs(Reg3Z(W1), Reg3S(W2), WZR).extend(ExtendMode::UXTW, ExtendShiftAmount::try_new(3).unwrap()),
            "subs w1, w2, wzr, uxtw #3";
        test_subs_32_const_0x123, subs(W1, W2, 0x123).unwrap(), "subs w1, w2, #0x123";
        test_subs_32_const_0x123_1, subs(W1, W2, AddSubImm12::try_from(0x123)).unwrap(), "subs w1, w2, #0x123";
        test_subs_32_const_0x123000, subs(W1, W2, 0x123000).unwrap(), "subs w1, w2, #0x123000";
    }

    #[test]
    fn test_subs_64_const_0x1001() {
        let a = subs(X1, X2, 0x1001);
        assert!(a.is_err());
    }

    #[test]
    fn test_subs_32_const_0x1001() {
        let a = subs(W1, W2, 0x1001);
        assert!(a.is_err());
    }
}
