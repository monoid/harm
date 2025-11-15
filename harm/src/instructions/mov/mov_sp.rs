/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use super::mov_reg::MovReg;
use super::*;
use crate::instructions::RawInstruction;
use crate::instructions::arith::AddSubImm12;
use crate::instructions::arith::add::{Add, add};
use crate::register::{IntoReg, Reg32, Reg64, RegOrSp32, RegOrSp64, RegOrZero32, RegOrZero64};

pub enum MovRegSp<RZ, RSp> {
    Reg(MovReg<RZ>),
    Sp(Add<RSp, RSp, AddSubImm12>),
}

impl<RZ: Sealed, RSp: Sealed> Sealed for MovRegSp<RZ, RSp> {}

// mov wsp, wN is implemented as add wsp, wN, 0
impl<Src> MakeMov<RegOrSp32, Src> for MovImpls<MovRegSp<RegOrZero32, RegOrSp32>>
where
    Src: IntoReg<RegOrSp32>,
{
    type Output = MovRegSp<RegOrZero32, RegOrSp32>;

    fn make(dst: RegOrSp32, src: Src) -> Self::Output {
        use RegOrSp32::*;

        let src = src.into_reg();
        match (dst, src) {
            (Reg(dst32), Reg(src32)) => MovRegSp::Reg(mov(dst32, src32)),
            (dst, src) => MovRegSp::Sp(add(dst, src, AddSubImm12::Unshifted(<_>::default()))),
        }
    }
}

// mov wN, wsp is implemented as add wN, wsp, 0
impl MakeMov<Reg32, RegOrSp32> for MovImpls<MovRegSp<RegOrZero32, RegOrSp32>> {
    type Output = MovRegSp<RegOrZero32, RegOrSp32>;

    #[inline]
    fn make(dst: Reg32, src: RegOrSp32) -> Self::Output {
        <Self as MakeMov<RegOrSp32, RegOrSp32>>::make(RegOrSp32::Reg(dst), src)
    }
}

// mov sp, xN is implemented as add sp, xN, 0
impl<Src> MakeMov<RegOrSp64, Src> for MovImpls<MovRegSp<RegOrZero64, RegOrSp64>>
where
    Src: IntoReg<RegOrSp64>,
{
    type Output = MovRegSp<RegOrZero64, RegOrSp64>;

    fn make(dst: RegOrSp64, src: Src) -> Self::Output {
        use RegOrSp64::*;

        let src = src.into_reg();
        match (dst, src) {
            (Reg(dst64), Reg(src64)) => MovRegSp::Reg(mov(dst64, src64)),
            (dst, src) => MovRegSp::Sp(add(dst, src, AddSubImm12::Unshifted(<_>::default()))),
        }
    }
}

// mov xN, sp is implemented as add xN, sp, 0
impl MakeMov<Reg64, RegOrSp64> for MovImpls<MovRegSp<RegOrZero64, RegOrSp64>> {
    type Output = MovRegSp<RegOrZero64, RegOrSp64>;

    #[inline]
    fn make(dst: Reg64, src: RegOrSp64) -> Self::Output {
        <Self as MakeMov<RegOrSp64, RegOrSp64>>::make(RegOrSp64::Reg(dst), src)
    }
}

impl RawInstruction for MovRegSp<RegOrZero32, RegOrSp32> {
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        match self {
            MovRegSp::Reg(mov_reg) => mov_reg.to_code(),
            MovRegSp::Sp(add) => add.to_code(),
        }
    }
}

impl RawInstruction for MovRegSp<RegOrZero64, RegOrSp64> {
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        match self {
            MovRegSp::Reg(mov_reg) => mov_reg.to_code(),
            MovRegSp::Sp(add) => add.to_code(),
        }
    }
}
