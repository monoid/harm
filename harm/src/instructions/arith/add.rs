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
use paste::paste;

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
    use super::*;

    #[test]
    fn test_add_64() {
        use Reg64::*;

        let codes: Vec<_> = add(X1, X2, X12).unwrap().represent().collect();

        assert_eq!(codes.len(), 1);
        assert_eq!(codes[0].0, [0x41, 0x00, 0x0c, 0x8b]); // 0x8b0c0041
    }

    #[test]
    fn test_add_64_shift() {
        use Reg64::*;

        let codes: Vec<_> = add(X1, X2, X12)
            .unwrap()
            .shift(ShiftMode::LSR, 4)
            .represent()
            .collect();

        assert_eq!(codes.len(), 1);
        assert_eq!(codes[0].0, [0x41, 0x10, 0x4c, 0x8b]); // 0x8b4c1041
    }

    #[test]
    fn test_add_64_zero() {
        use Reg64::*;
        use RegOrZero64::*;

        let codes: Vec<_> = add(
            X1.into(),
            XZR,
            ShiftedReg::from(X12).shift(ShiftMode::LSR, 4),
        )
        .unwrap()
        .represent()
        .collect();

        assert_eq!(codes.len(), 1);
        assert_eq!(codes[0].0, [0xe1, 0x13, 0x4c, 0x8b]); // 0x8b4c13e1
    }

    #[test]
    fn test_add_64_extend_uxtx() {
        use Reg64::*;
        use RegOrSp64::Reg as RegS;
        use RegOrZero64::Reg as RegZ;

        let codes: Vec<_> = add(RegS(X1), RegS(X2), RegZ(X12))
            .unwrap()
            .extend(ExtendMode::UXTX, 3)
            .represent()
            .collect();

        assert_eq!(codes.len(), 1);
        // add x1, x2, x12, uxtx #3
        assert_eq!(codes[0].0, [0x41, 0x6c, 0x2c, 0x8b]); // 0x8b2c6c41
    }

    #[test]
    fn test_add_64_extend_uxtw() {
        use Reg64::*;

        let codes: Vec<_> = add(X1, X2, X12)
            .unwrap()
            .extend(ExtendMode::UXTW, 3)
            .represent()
            .collect();

        assert_eq!(codes.len(), 1);
        // add x1, x2, w12, uxtw #3
        assert_eq!(codes[0].0, [0x41, 0x4c, 0x2c, 0x8b]); // 0x8b2c4c41
    }

    #[test]
    fn test_add_64_extend_uxth_xzr() {
        use Reg64::*;
        use RegOrSp64::Reg as RegS;
        use RegOrZero64::XZR;

        let codes: Vec<_> = add(RegS(X1), RegS(X2), XZR)
            .unwrap()
            .extend(ExtendMode::UXTH, 3)
            .represent()
            .collect();

        assert_eq!(codes.len(), 1);
        // add x1, x2, wzr, uxtw #3
        assert_eq!(codes[0].0, [0x41, 0x2c, 0x3f, 0x8b]); // 0x8b3f2c41
    }

    #[test]
    fn test_add_64_const_1() {
        use Reg64::*;

        let codes: Vec<_> = add(X1, X2, 1).unwrap().represent().collect();

        assert_eq!(codes.len(), 1);
        assert_eq!(codes[0].0, [0x41, 0x04, 0x00, 0x91]); // 0x91000441
    }

    #[test]
    fn test_add_64_const_0x1000() {
        use Reg64::*;

        let codes: Vec<_> = add(X1, X2, 0x1000).unwrap().represent().collect();

        assert_eq!(codes.len(), 1);
        assert_eq!(codes[0].0, [0x41, 0x04, 0x40, 0x91]); // 0x91400441
    }

    #[test]
    fn test_add_64_const_0x1001() {
        use Reg64::*;

        let a = add(X1, X2, 0x1001);
        assert!(a.is_err());
    }

    #[test]
    fn test_add_sp_64_const_1() {
        use RegOrSp64::SP;

        let codes: Vec<_> = add(SP, SP, 1).unwrap().represent().collect();

        assert_eq!(codes.len(), 1);
        assert_eq!(codes[0].0, [0xFF, 0x07, 0x00, 0x91]); // 0x910007FF
    }

    #[test]
    fn test_add_sp_64_const_0x1000() {
        use RegOrSp64::SP;

        let codes: Vec<_> = add(SP, SP, 1).unwrap().represent().collect();

        assert_eq!(codes.len(), 1);
        assert_eq!(codes[0].0, [0xFF, 0x07, 0x00, 0x91]); // 0x910007FF
    }

    #[test]
    fn test_add_32() {
        use Reg32::*;

        let codes: Vec<_> = add(W1, W2, W12).unwrap().represent().collect();

        assert_eq!(codes.len(), 1);
        assert_eq!(codes[0].0, [0x41, 0x00, 0x0c, 0x0b]); // 0x0b0c0041
    }

    #[test]
    fn test_add_32_shift() {
        use Reg32::*;

        let codes: Vec<_> = add(W1, W2, W12)
            .unwrap()
            .shift(ShiftMode::LSR, 4)
            .represent()
            .collect();

        assert_eq!(codes.len(), 1);
        assert_eq!(codes[0].0, [0x41, 0x10, 0x4c, 0x0b]); // 0x0b4c1041
    }

    #[test]
    fn test_add_32_zero() {
        use Reg32::*;
        use RegOrZero32::*;

        let codes: Vec<_> = add(
            W1.into(),
            WZR,
            ShiftedReg::from(W12).shift(ShiftMode::LSR, 4),
        )
        .unwrap()
        .represent()
        .collect();

        assert_eq!(codes.len(), 1);
        assert_eq!(codes[0].0, [0xe1, 0x13, 0x4c, 0x0b]); // 0x0b4c13e1
    }

    #[test]
    fn test_add_32_extend_uxtx() {
        use Reg32::*;

        let codes: Vec<_> = add(W1, W2, W12)
            .unwrap()
            .extend(ExtendMode::UXTX, 3)
            .represent()
            .collect();

        assert_eq!(codes.len(), 1);
        // add x1, x2, x12, uxtx #3
        assert_eq!(codes[0].0, [0x41, 0x6c, 0x2c, 0x0b]); // 0x0b2c6c41
    }

    #[test]
    fn test_add_32_extend_uxtw() {
        use Reg32::*;

        let codes: Vec<_> = add(W1, W2, W12)
            .unwrap()
            .extend(ExtendMode::UXTW, 3)
            .represent()
            .collect();

        assert_eq!(codes.len(), 1);
        // add w1, w2, w12, uxtw #3
        assert_eq!(codes[0].0, [0x41, 0x4c, 0x2c, 0x0b]); // 0x0b2c4c41
    }

    #[test]
    fn test_add_32_extend_uxtx_wzr() {
        use Reg32::*;
        use RegOrSp32::Reg as RegS;
        use RegOrZero32::WZR;

        let codes: Vec<_> = add(RegS(W1), RegS(W2), WZR)
            .unwrap()
            .extend(ExtendMode::UXTX, 3)
            .represent()
            .collect();

        assert_eq!(codes.len(), 1);
        // add w1, w2, wzr, uxtx #3
        assert_eq!(codes[0].0, [0x41, 0x6c, 0x3f, 0x0b]); // 0x0b3f6c41
    }

    #[test]
    fn test_add_32_extend_uxtw_wzr() {
        use Reg32::*;
        use RegOrSp32::Reg as RegS;
        use RegOrZero32::WZR;

        let codes: Vec<_> = add(RegS(W1), RegS(W2), WZR)
            .unwrap()
            .extend(ExtendMode::UXTW, 3)
            .represent()
            .collect();

        assert_eq!(codes.len(), 1);
        // add w1, w2, wzr, uxtw #3
        assert_eq!(codes[0].0, [0x41, 0x4c, 0x3f, 0x0b]); // 0x0b3f4c41
    }

    #[test]
    fn test_add_32_const_0x123() {
        use Reg32::*;

        let codes: Vec<_> = add(W1, W2, 0x123).unwrap().represent().collect();

        assert_eq!(codes.len(), 1);
        // add w1, w2, 0x123
        assert_eq!(codes[0].0, [0x41, 0x8c, 0x04, 0x11]); // 0x11048c41
    }

    #[test]
    fn test_add_wsp_32_const_0x123() {
        use RegOrSp32::WSP;

        let codes: Vec<_> = add(WSP, WSP, 0x123).unwrap().represent().collect();

        assert_eq!(codes.len(), 1);
        // add w1, w2, 0x123
        assert_eq!(codes[0].0, [0xFF, 0x8F, 0x04, 0x11]); // 0x11048fff
    }

    #[test]
    fn test_add_32_const_0x123000() {
        use Reg32::*;

        let codes: Vec<_> = add(W1, W2, 0x123000).unwrap().represent().collect();

        assert_eq!(codes.len(), 1);
        // add w1, w2, 0x123000
        assert_eq!(codes[0].0, [0x41, 0x8c, 0x44, 0x11]); // 0x11448c41
    }

    #[test]
    fn test_add_32_const_0x1001() {
        use Reg32::*;

        let a = add(W1, W2, 0x1001);
        assert!(a.is_err());
    }

    #[test]
    fn test_add_64_sp_extend_uxtx() {
        use Reg64::*;
        use RegOrSp64::SP;

        let codes: Vec<_> = add(SP, SP, X12)
            .unwrap()
            .extend(ExtendMode::UXTX, 3)
            .represent()
            .collect();

        assert_eq!(codes.len(), 1);
        // add sp, sp, x12, uxtx 3
        assert_eq!(codes[0].0, [0xff, 0x6f, 0x2c, 0x8b]); // 0x8b2c6fff
    }

    #[test]
    fn test_add_64_sp_extend_uxtw() {
        use Reg64::*;
        use RegOrSp64::SP;

        let codes: Vec<_> = add(SP, SP, X12)
            .unwrap()
            .extend(ExtendMode::UXTW, 3)
            .represent()
            .collect();

        assert_eq!(codes.len(), 1);
        // add sp, sp, w12, uxtw 3
        assert_eq!(codes[0].0, [0xff, 0x4f, 0x2c, 0x8b]); // 0x8b2c4fff
    }

    #[test]
    fn test_add_64_sp_extend_uxtw_xzr() {
        use RegOrSp64::SP;
        use RegOrZero64::XZR;

        let codes: Vec<_> = add(SP, SP, XZR)
            .unwrap()
            .extend(ExtendMode::UXTW, 3)
            .represent()
            .collect();

        assert_eq!(codes.len(), 1);
        // add sp, sp, wzr, uxtw 3
        assert_eq!(codes[0].0, [0xff, 0x4f, 0x3f, 0x8b]); // 0x8b3f4fff
    }
}
