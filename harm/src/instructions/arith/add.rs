/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */
use aarchmrs_instructions::A64::{
    dpimm::addsub_imm::{
        ADD_32_addsub_imm::ADD_32_addsub_imm, ADD_64_addsub_imm::ADD_64_addsub_imm,
    },
    dpreg::{
        addsub_ext::{ADD_32_addsub_ext::ADD_32_addsub_ext, ADD_64_addsub_ext::ADD_64_addsub_ext},
        addsub_shift::{
            ADD_32_addsub_shift::ADD_32_addsub_shift, ADD_64_addsub_shift::ADD_64_addsub_shift,
        },
    },
};
use aarchmrs_types::InstructionCode;

use super::{Error, Extend, ExtendMode, ExtendedReg, Shift, ShiftMode, ShiftedReg};
use crate::{
    instructions::Instruction,
    register::{IntoCode as _, Reg32, Reg64, RegOrSp32, RegOrSp64, RegOrZero32, RegOrZero64},
};

pub fn add<T, U>(dst: T, src1: T, src2: U) -> Result<Add<T, U>, Error>
where
    Add<T, U>: MakeAdd<T, U>,
{
    Add::<T, U>::new(dst, src1, src2)
}

pub trait MakeAdd<T, U>: Sized {
    fn new(dst: T, src1: T, src2: U) -> Result<Self, Error>;
}

pub struct Add<T, U> {
    pub dst: T,
    pub src1: T,
    pub src2: U,
}

impl MakeAdd<Reg64, Reg64> for Add<Reg64, Reg64> {
    #[inline]
    fn new(dst: Reg64, src1: Reg64, src2: Reg64) -> Result<Self, &'static str> {
        Ok(Self { dst, src1, src2 })
    }
}

impl MakeAdd<Reg32, Reg32> for Add<Reg32, Reg32> {
    #[inline]
    fn new(dst: Reg32, src1: Reg32, src2: Reg32) -> Result<Self, Error> {
        Ok(Self { dst, src1, src2 })
    }
}

define_arith_shift!(Add, 32, addsub, Reg32, RegOrZero32);
define_arith_shift!(Add, 64, addsub, Reg64, RegOrZero64);

define_arith_extend!(Add, 32, addsub, Reg32, RegOrSp32, RegOrZero32);
define_arith_extend!(Add, 64, addsub, Reg64, RegOrSp64, RegOrZero64);

define_arith_imm12!(Add, 32, addsub, Reg32, RegOrSp32);
define_arith_imm12!(Add, 64, addsub, Reg64, RegOrSp64);

#[cfg(test)]
mod tests {
    use harm_test_utils::test_cases;

    use super::*;
    use Reg32::*;
    use Reg64::*;
    use RegOrSp32::Reg as Reg3S;
    use RegOrSp32::WSP;
    use RegOrSp64::Reg as RegS;
    use RegOrSp64::SP;
    use RegOrZero32::WZR;
    use RegOrZero64::Reg as RegZ;
    use RegOrZero64::XZR;

    const ADD_DB: &str = "
8b3f2c41	add x1, x2, wzr, uxth #3
0b0c0041	add w1, w2, w12
0b2c6c41	add w1, w2, w12, uxtx #3
0b2c4c41	add w1, w2, w12, uxtw #3
0b3f6c41	add w1, w2, wzr, uxtx #3
0b3f4c41	add w1, w2, wzr, uxtw #3
0b4c1041	add w1, w2, w12, lsr #4
0b4c13e1	add w1, wzr, w12, lsr #4
11048c41	add w1, w2, #0x123
11048fff	add wsp, wsp, #0x123
11448c41	add w1, w2, #0x123000
8b0c0041	add x1, x2, x12
8b2c4c41	add x1, x2, w12, uxtw #3
8b2c6c41	add x1, x2, x12, uxtx #3
8b2c4fff	add sp, sp, w12, uxtw #3
8b2c6fff	add sp, sp, x12, uxtx #3
8b2c7041	add x1, x2, x12, uxtx #4
8b3f4c41	add x1, x2, wzr, uxtw #3
8b3f4fff	add sp, sp, wzr, uxtw #3
8b4c1041	add x1, x2, x12, lsr #4
8b4c13e1	add x1, xzr, x12, lsr #4
91000441	add x1, x2, #1
910007ff	add sp, sp, #1
91400441	add x1, x2, #0x1000
914007ff	add sp, sp, #0x1000
";

    test_cases! {
        ADD_DB, untested_add_db;
        test_add_64, add(X1, X2, X12).unwrap(), "add x1, x2, x12";
        test_add_64_shift, add(X1, X2, X12).unwrap().shift(ShiftMode::LSR, 4), "add x1, x2, x12, lsr #4";
        test_add_64_zero,
            add(X1.into(), XZR, ShiftedReg::from(X12).shift(ShiftMode::LSR, 4)).unwrap(),
            "add x1, xzr, x12, lsr #4";
        test_add_64_extend_uxtx, add(RegS(X1), RegS(X2), RegZ(X12)).unwrap().extend(ExtendMode::UXTX, 3),
            "add x1, x2, x12, uxtx #3";
        // KLUDGE: Using Reg64 instead of Reg32 at the last argument.
        // To be reimplemented akin `ldr` family.
        test_add_64_extend_uxtw, add(X1, X2, X12).unwrap().extend(ExtendMode::UXTW, 3), "add x1, x2, w12, uxtw #3";
        test_add_64_wzr_extend_uxtw, add(RegS(X1), RegS(X2), XZR).unwrap().extend(ExtendMode::UXTW, 3), "add x1, x2, wzr, uxtw #3";
        test_add_64_extend_uxtx_4, add(X1, X2, X12).unwrap().extend(ExtendMode::UXTX, 4), "add x1, x2, x12, uxtx #4";
        test_add_64_extend_uxth_xzr,
            add(RegS(X1), RegS(X2), XZR)
                .unwrap()
                .extend(ExtendMode::UXTH, 3),
            "add x1, x2, wzr, uxth #3";
        test_add_64_const_1, add(X1, X2, 1).unwrap(), "add x1, x2, #1";
        test_add_64_const_0x1000, add(X1, X2, 0x1000).unwrap(), "add x1, x2, #0x1000";
        test_add_sp_64_const_1, add(SP, SP, 1).unwrap(), "add sp, sp, #1";
        test_add_sp_64_const_0x1000, add(SP, SP, 0x1000).unwrap(), "add sp, sp, #0x1000";
        test_add_32, add(W1, W2, W12).unwrap(), "add w1, w2, w12";
        test_add_32_shift, add(W1, W2, W12).unwrap().shift(ShiftMode::LSR, 4), "add w1, w2, w12, lsr #4";
        test_add_32_zero,
            add(W1.into(), WZR, ShiftedReg::from(W12).shift(ShiftMode::LSR, 4)).unwrap(),
            "add w1, wzr, w12, lsr #4";
        test_add_32_extend_uxtx, add(W1, W2, W12).unwrap().extend(ExtendMode::UXTX, 3), "add w1, w2, w12, uxtx #3";
        test_add_32_extend_uxtw, add(W1, W2, W12).unwrap().extend(ExtendMode::UXTW, 3), "add w1, w2, w12, uxtw #3";
        test_add_32_extend_uxtx_wzr,
            add(Reg3S(W1), Reg3S(W2), WZR)
                .unwrap()
                .extend(ExtendMode::UXTX, 3),
            "add w1, w2, wzr, uxtx #3";
        test_add_32_extend_uxtw_wzr,
            add(Reg3S(W1), Reg3S(W2), WZR)
                .unwrap()
                .extend(ExtendMode::UXTW, 3),
            "add w1, w2, wzr, uxtw #3";
        test_add_32_const_0x123, add(W1, W2, 0x123).unwrap(), "add w1, w2, #0x123";
        test_add_wsp_32_const_0x123, add(WSP, WSP, 0x123).unwrap(), "add wsp, wsp, #0x123";
        test_add_32_const_0x123000, add(W1, W2, 0x123000).unwrap(), "add w1, w2, #0x123000";
        test_add_64_sp_extend_uxtx,
            add(SP, SP, X12).unwrap().extend(ExtendMode::UXTX, 3),
            "add sp, sp, x12, uxtx #3";
        test_add_64_sp_extend_uxtw,
            add(SP, SP, X12).unwrap().extend(ExtendMode::UXTW, 3),
            "add sp, sp, w12, uxtw #3";
        test_add_64_sp_extend_uxtw_xzr,
            add(SP, SP, XZR).unwrap().extend(ExtendMode::UXTW, 3),
            "add sp, sp, wzr, uxtw #3";
    }

    #[test]
    fn test_add_64_const_0x1001() {
        let a = add(X1, X2, 0x1001);
        assert!(a.is_err());
    }

    #[test]
    fn test_add_32_const_0x1001() {
        let a = add(W1, W2, 0x1001);
        assert!(a.is_err());
    }
}
