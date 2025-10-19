/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

/*!
 * Module for `MOV` virtual instruction.
 */

/*

mov x1, x2 -- or x1, xzr, x2  // verified with spec, xzr is the second.
mov sp, x2 -- add sp, x2, #0
mov x2, sp -- add x2, sp, #0
mov w1, wzr -- or w1, wzr, wzr
mov x3, N -- either
      movz
      movn
      or `or x3, xzr, N`

It seems re-using underlying traits (OR/AND/MOVX) wouldn't be possible, and we need a trait. And we may use different
Self types for implementation of these traits.
 */

use crate::{
    instructions::{
        RawInstruction,
        arith::add::Add,
        dpimm::{MovImmArgs, MovN, MovZ, MoveShift, immediate::LogicalImmFields},
        logical::{LogicalArgs, MakeSpLogicalArgs, Orr, orr},
    },
    outcome::Unfallible,
    register::{IntoReg, RegOrZero32, RegOrZero64, Zero},
    sealed::Sealed,
};

pub trait MakeMov<Dst, Src>: Sealed {
    type Output;

    fn make(dst: Dst, src: Src) -> Self::Output;
}

pub struct MovReg<R> {
    pub dst: R,
    pub src: R,
}

impl<R> Sealed for MovReg<R> {}

pub enum MovRegSp<R, RSp> {
    Ordinary(MovReg<R>),
    // actually, it is not u32, it is something equal to zero (not Zero)
    Sp(Add<RSp, RSp, u32>),
}

impl<Dst, Src> MakeMov<Dst, Src> for MovReg<RegOrZero32>
where
    Dst: IntoReg<RegOrZero32>,
    Src: IntoReg<RegOrZero32>,
{
    type Output = Self;

    fn make(dst: Dst, src: Src) -> Self::Output {
        Self {
            dst: dst.into_reg(),
            src: src.into_reg(),
        }
    }
}

impl<Dst, Src> MakeMov<Dst, Src> for MovReg<RegOrZero64>
where
    Dst: IntoReg<RegOrZero64>,
    Src: IntoReg<RegOrZero64>,
{
    type Output = Self;

    fn make(dst: Dst, src: Src) -> Self::Output {
        Self {
            dst: dst.into_reg(),
            src: src.into_reg(),
        }
    }
}

pub enum MovConst<R: MoveShift + Zero> {
    MovZ(MovZ<MovImmArgs<R>>),
    MovN(MovN<MovImmArgs<R>>),
    Orr(Orr<LogicalArgs<R, R, LogicalImmFields>>),
}

impl<Rs: Zero + Copy> RawInstruction for MovReg<Rs>
where
    LogicalArgs<Rs, Rs, Rs>:
        MakeSpLogicalArgs<Rs, Rs, Rs, Outcome = Unfallible<LogicalArgs<Rs, Rs, Rs>>>,
    Orr<LogicalArgs<Rs, Rs, Rs>>: RawInstruction,
{
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        orr(self.dst, Rs::ZERO, self.src).to_code()
    }
}

pub fn mov<Rd, Rs, X>(dst: Rd, src: Rs) -> <X as MakeMov<Rd, Rs>>::Output
where
    X: MakeMov<Rd, Rs>,
{
    <X as MakeMov<Rd, Rs>>::make(dst, src)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::register::Reg32::*;
    use crate::register::Reg64::*;
    use harm_test_utils::test_cases;

    const MOV_DB: &str = "
aa0203e1	mov x1, x2
aa1f03e3	mov x3, xzr
9100009f	mov sp, x4
910003e5	mov x5, sp
aa0a03ff	mov xzr, x10

d2800026	mov x6, 1
92800027	mov x7, -2
b204cfe8	mov x8, 0xF0F0F0F0F0F0F0F0
b2408fe9	orr x9, xzr, 0xFFFFFFFFF
b2408fe9	mov x9, 0xFFFFFFFFF

2a0203e1	mov w1, w2
2a1f03e3	mov w3, wzr
1100009f	mov wsp, w4
110003e5	mov w5, wsp
2a0a03ff	mov wzr, w10

52800026	mov w6, 1
12800027	mov w7, -2
3204cfe8	mov w8, 0xF0F0F0F0

32003fe9	orr w9, wzr, 0x0000FFFF
529fffe9	mov w9, 0x00000FFFF
32103fe9	orr w9, wzr, 0xFFFF0000
52bfffe9	mov w9, 0xFFFF0000
32143fe9	orr w9, wzr, 0x0FFFF000
32143fe9	mov w9, 0x0FFFF000

";

    test_cases! {
        MOV_DB, untested_mov_cases;
        test_mov_64, mov(X1, X2), "mov x1, x2";
        test_mov_32, mov(W1, W2), "mov w1, w2";
    }
}
