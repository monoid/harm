/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use super::*;
use crate::{
    instructions::{
        RawInstruction,
        logical::{LogicalArgs, LogicalShift, LogicalShiftable, MakeSpLogicalArgs, Orr, orr},
    },
    outcome::Unfallible,
    register::{IntoReg, RegOrSp32, RegOrSp64, RegOrZero32, RegOrZero64, Zero},
    sealed::Sealed,
};

pub struct MovReg<R> {
    pub dst: R,
    pub src: R,
}

impl<R> Sealed for MovReg<R> {}

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
