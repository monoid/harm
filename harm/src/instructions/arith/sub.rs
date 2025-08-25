use aarchmrs_instructions::A64::{
    dpimm::addsub_imm::{
        SUB_32_addsub_imm::SUB_32_addsub_imm, SUB_64_addsub_imm::SUB_64_addsub_imm,
    },
    dpreg::addsub_ext::{
        SUB_32_addsub_ext::SUB_32_addsub_ext, SUB_64_addsub_ext::SUB_64_addsub_ext,
    },
    dpreg::addsub_shift::{
        SUB_32_addsub_shift::SUB_32_addsub_shift, SUB_64_addsub_shift::SUB_64_addsub_shift,
    },
};
use aarchmrs_types::InstructionCode;

use super::Error;
use crate::{
    instructions::{
        RawInstruction,
        arith::{Extend, ExtendMode, ExtendedReg, Shift, ShiftMode, ShiftedReg},
    },
    register::{IntoCode as _, Reg32, Reg64, RegOrSp32, RegOrSp64, RegOrZero32, RegOrZero64},
};

pub fn sub<T, RealT, S1, S2, RealS1, RealS2>(
    dst: T,
    src1: S1,
    src2: S2,
) -> <Sub<RealT, RealS1, RealS2> as MakeSub<T, S1, S2>>::Output
where
    Sub<RealT, RealS1, RealS2>: MakeSub<T, S1, S2>,
{
    Sub::<RealT, RealS1, RealS2>::new(dst, src1, src2)
}

pub trait MakeSub<T, Src1, Src2>: Sized {
    type Output;

    fn new(dst: T, src1: Src1, src2: Src2) -> Self::Output;
}

pub struct Sub<T, Src1, Src2> {
    pub dst: T,
    pub src1: Src1,
    pub src2: Src2,
}

impl MakeSub<Reg64, Reg64, Reg64> for Sub<Reg64, Reg64, Reg64> {
    type Output = Self;

    #[inline]
    fn new(dst: Reg64, src1: Reg64, src2: Reg64) -> Self {
        Self { dst, src1, src2 }
    }
}

impl MakeSub<Reg32, Reg32, Reg32> for Sub<Reg32, Reg32, Reg32> {
    type Output = Self;

    #[inline]
    fn new(dst: Reg32, src1: Reg32, src2: Reg32) -> Self::Output {
        Self { dst, src1, src2 }
    }
}

define_arith_faillible!(Sub);

define_arith_shift!(Sub, 32, addsub, Reg32, RegOrZero32);
define_arith_shift!(Sub, 64, addsub, Reg64, RegOrZero64);

define_arith_extend!(Sub, 32, addsub, Reg32, RegOrSp32, RegOrZero32);
define_arith_extend!(Sub, 64, addsub, Reg64, RegOrSp64, RegOrZero64);

define_arith_imm12!(Sub, 32, addsub, Reg32, RegOrSp32);
define_arith_imm12!(Sub, 64, addsub, Reg64, RegOrSp64);

#[cfg(test)]
mod tests {
    use harm_test_utils::test_cases;

    use super::*;
    use crate::instructions::InstructionSeq;
    use Reg32::*;
    use Reg64::*;
    use RegOrSp32::Reg as Reg3S;
    use RegOrSp32::WSP;
    use RegOrSp64::Reg as RegS;
    use RegOrSp64::SP;
    use RegOrZero32::WZR;
    use RegOrZero64::XZR;

    const SUB_DB: &str = "
cb3f2c41	sub x1, x2, wzr, uxth #3
4b0c0041	sub w1, w2, w12
4b2c6c41	sub w1, w2, w12, uxtx #3
4b2c4c41	sub w1, w2, w12, uxtw #3
4b3f6c41	sub w1, w2, wzr, uxtx #3
4b3f4c41	sub w1, w2, wzr, uxtw #3
4b4c1041	sub w1, w2, w12, lsr #4
4b4c13e1	sub w1, wzr, w12, lsr #4
51048c41	sub w1, w2, #0x123
51048fff	sub wsp, wsp, #0x123
51448c41	sub w1, w2, #0x123000
cb0c0041	sub x1, x2, x12
cb2c4c41	sub x1, x2, w12, uxtw #3
cb2c6c41	sub x1, x2, x12, uxtx #3
cb2c4fff	sub sp, sp, w12, uxtw #3
cb2c6fff	sub sp, sp, x12, uxtx #3
cb2c7041	sub x1, x2, x12, uxtx #4
cb3f4c41	sub x1, x2, wzr, uxtw #3
cb3f4fff	sub sp, sp, wzr, uxtw #3
cb4c1041	sub x1, x2, x12, lsr #4
cb4c13e1	sub x1, xzr, x12, lsr #4
d1000441	sub x1, x2, #1
d10007ff	sub sp, sp, #1
d1400441	sub x1, x2, #0x1000
d14007ff	sub sp, sp, #0x1000
";

    test_cases! {
        SUB_DB, untested_sub_db;
        test_sub_64, sub(X1, X2, X12), "sub x1, x2, x12";
        test_sub_64_shift, sub(X1, X2, X12).shift(ShiftMode::LSR, 4), "sub x1, x2, x12, lsr #4";
        test_sub_64_zero,
            sub(X1, XZR, ShiftedReg::from(X12).shift(ShiftMode::LSR, 4)),
            "sub x1, xzr, x12, lsr #4";
        test_sub_64_extend_uxtx, sub(RegS(X1), X2, X12).extend(ExtendMode::UXTX, 3),
            "sub x1, x2, x12, uxtx #3";
        // KLUDGE: Using Reg64 instead of Reg32 at the last argument.
        // To be reimplemented akin `ldr` family.
        test_sub_64_extend_uxtw, sub(X1, X2, X12).extend(ExtendMode::UXTW, 3), "sub x1, x2, w12, uxtw #3";
        test_sub_64_wzr_extend_uxtw, sub(RegS(X1), X2, XZR).extend(ExtendMode::UXTW, 3), "sub x1, x2, wzr, uxtw #3";
        test_sub_64_extend_uxtx_4, sub(X1, X2, X12).extend(ExtendMode::UXTX, 4), "sub x1, x2, x12, uxtx #4";
        test_sub_64_extend_uxth_xzr,
            sub(RegS(X1), RegS(X2), XZR).extend(ExtendMode::UXTH, 3),
            "sub x1, x2, wzr, uxth #3";
        test_sub_64_const_1, sub(X1, X2, 1).unwrap(), "sub x1, x2, #1";
        test_sub_64_const_0x1000, sub(X1, X2, 0x1000).unwrap(), "sub x1, x2, #0x1000";
        test_sub_sp_64_const_1, sub(SP, SP, 1).unwrap(), "sub sp, sp, #1";
        test_sub_sp_64_const_0x1000, sub(SP, SP, 0x1000).unwrap(), "sub sp, sp, #0x1000";
        test_sub_32, sub(W1, W2, W12), "sub w1, w2, w12";
        test_sub_32_shift, sub(W1, W2, W12).shift(ShiftMode::LSR, 4), "sub w1, w2, w12, lsr #4";
        test_sub_32_zero,
            sub(W1, WZR, ShiftedReg::from(W12).shift(ShiftMode::LSR, 4)),
            "sub w1, wzr, w12, lsr #4";
        test_sub_32_extend_uxtx, sub(W1, W2, W12).extend(ExtendMode::UXTX, 3), "sub w1, w2, w12, uxtx #3";
        test_sub_32_extend_uxtw, sub(W1, W2, W12).extend(ExtendMode::UXTW, 3), "sub w1, w2, w12, uxtw #3";
        test_sub_32_extend_uxtx_wzr,
            sub(Reg3S(W1), W2, WZR).extend(ExtendMode::UXTX, 3),
            "sub w1, w2, wzr, uxtx #3";
        test_sub_32_extend_uxtw_wzr,
            sub(Reg3S(W1), W2, WZR).extend(ExtendMode::UXTW, 3),
            "sub w1, w2, wzr, uxtw #3";
        test_sub_32_const_0x123, sub(W1, W2, 0x123).unwrap(), "sub w1, w2, #0x123";
        test_sub_wsp_32_const_0x123, sub(WSP, WSP, 0x123).unwrap(), "sub wsp, wsp, #0x123";
        test_sub_32_const_0x123000, sub(W1, W2, 0x123000).unwrap(), "sub w1, w2, #0x123000";
        test_sub_64_sp_extend_uxtx,
            sub(SP, SP, X12).extend(ExtendMode::UXTX, 3),
            "sub sp, sp, x12, uxtx #3";
        test_sub_64_sp_extend_uxtw,
            sub(SP, SP, X12).extend(ExtendMode::UXTW, 3),
            "sub sp, sp, w12, uxtw #3";
        test_sub_64_sp_extend_uxtw_xzr,
            sub(SP, SP, XZR).extend(ExtendMode::UXTW, 3),
            "sub sp, sp, wzr, uxtw #3";
    }

    #[test]
    fn test_sub_64_const_0x1001() {
        let a = sub(X1, X2, 0x1001);
        assert!(a.is_err());
    }

    #[test]
    fn test_sub_32_const_0x1001() {
        let a = sub(W1, W2, 0x1001);
        assert!(a.is_err());
    }
}
