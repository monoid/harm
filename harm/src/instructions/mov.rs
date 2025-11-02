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
      movz (xzr)
      movn (xzr)
      or `or x3, xzr, N` (sp)

It seems re-using underlying traits (OR/AND/MOVX) wouldn't be possible, and we need a trait. And we may use different
Self types for implementation of these traits.
 */

use core::marker::PhantomData;

use super::logical::{LogicalArgs, LogicalShift, LogicalShiftable, MakeSpLogicalArgs, Orr, orr};
use crate::{
    instructions::{
        RawInstruction,
        arith::add::Add,
        dpimm::{MovImmArgs, MovN, MovZ, MoveShift, immediate::LogicalImmFields},
    },
    outcome::Unfallible,
    register::{IntoReg, RegOrSp32, RegOrSp64, RegOrZero32, RegOrZero64, Zero},
    sealed::Sealed,
};

pub struct MovImpls<X>(PhantomData<X>);

pub trait MakeMov<Dst, Src>: Sealed {
    type Output;

    fn make(dst: Dst, src: Src) -> Self::Output;
}

pub struct MovReg<R> {
    pub dst: R,
    pub src: R,
}

impl<R> Sealed for MovReg<R> {}
impl<X: Sealed> Sealed for MovImpls<X> {}

pub enum MovRegSp<R, RSp> {
    Ordinary(MovReg<R>),
    // actually, it is not u32, it is something equal to zero (not Zero)
    Sp(Add<RSp, RSp, u32>),
}

impl<Dst, Src> MakeMov<Dst, Src> for MovImpls<MovReg<RegOrSp32>>
where
    Dst: IntoReg<RegOrZero32>,
    Src: IntoReg<RegOrZero32>,
{
    type Output = MovReg<RegOrZero32>;

    fn make(dst: Dst, src: Src) -> Self::Output {
        MovReg {
            dst: dst.into_reg(),
            src: src.into_reg(),
        }
    }
}

impl<Dst, Src> MakeMov<Dst, Src> for MovImpls<MovReg<RegOrSp64>>
where
    Dst: IntoReg<RegOrZero64>,
    Src: IntoReg<RegOrZero64>,
{
    type Output = MovReg<RegOrZero64>;

    fn make(dst: Dst, src: Src) -> Self::Output {
        MovReg {
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

impl<Rs: Zero + Copy + LogicalShiftable> RawInstruction for MovReg<Rs>
where
    LogicalArgs<Rs, Rs, (Rs, LogicalShift, <Rs as LogicalShiftable>::ShiftAmount)>:
        MakeSpLogicalArgs<
                Rs,
                Rs,
                Rs,
                Outcome = Unfallible<
                    LogicalArgs<Rs, Rs, (Rs, LogicalShift, <Rs as LogicalShiftable>::ShiftAmount)>,
                >,
            >,
    <Rs as LogicalShiftable>::ShiftAmount: Default,
    Orr<LogicalArgs<Rs, Rs, (Rs, LogicalShift, <Rs as LogicalShiftable>::ShiftAmount)>>:
        RawInstruction,
{
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        orr(self.dst, Rs::ZERO, self.src).to_code()
    }
}

pub fn mov<Rd, Rs, X>(dst: Rd, src: Rs) -> <MovImpls<X> as MakeMov<Rd, Rs>>::Output
where
    MovImpls<X>: MakeMov<Rd, Rs>,
{
    <MovImpls<X> as MakeMov<Rd, Rs>>::make(dst, src)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::InstructionSeq;
    use crate::register::Reg32::*;
    use crate::register::Reg64::*;
    use crate::register::RegOrZero32::WZR;
    use crate::register::RegOrZero64::XZR;
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
d280003f    mov xzr, 1
b204cfff	mov sp, 0xF0F0F0F0F0F0F0F0
b2408fff	orr sp, xzr, 0xFFFFFFFFF
b2408fff	mov sp, 0xFFFFFFFFF

2a0203e1	mov w1, w2
2a1f03e3	mov w3, wzr
1100009f	mov wsp, w4
110003e5	mov w5, wsp
2a0a03ff	mov wzr, w10

52800026	mov w6, 1
12800027	mov w7, -2
3204cfe8	mov w8, 0xF0F0F0F0
5280003f	mov wzr, 1
320003ff	mov wsp, 1
3204cfff	mov wsp, 0xF0F0F0F0

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
        test_mov_32_wzr, mov(W3, WZR), "mov w3, wzr";
        test_mov_64_xzr, mov(X3, XZR), "mov x3, xzr";
        test_mov_wzr_32, mov(WZR, W10), "mov wzr, w10";

        // these implementations compare visually that implementation of certiain MOV are implemented
        // or not implemented as ORR.
        // TODO: implement as match
        test_or_w9_wzr_0x0000ffff, orr(W9, WZR, 0x0000FFFF).unwrap(), "orr w9, wzr, 0x0000FFFF";
        test_or_w9_wzr_0x0ffff000, orr(W9, WZR, 0x0FFFF000).unwrap(), "orr w9, wzr, 0x0FFFF000";
        test_or_w9_wzr_0xffff0000, orr(W9, WZR, 0xFFFF0000).unwrap(), "orr w9, wzr, 0xFFFF0000";
        test_or_x9_xzr_0xfffffffff, orr(X9, XZR, 0xFFFFFFFFF).unwrap(), "orr x9, xzr, 0xFFFFFFFFF";
    }
}
