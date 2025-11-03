/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use super::*;

use crate::instructions::RawInstruction;
use crate::instructions::dpimm::{MakeMovArgs, MovImmArgs, MoveImm16, Shift32, movn, movz};
use crate::instructions::logical::{
    LogicalArgs, LogicalShift, LogicalShiftable, MakeSpLogicalArgs, orr,
};
use crate::outcome::{Outcome, Unfallible};
use crate::register::Zero;
use crate::register::{IntoReg, RegOrSp32, RegOrZero32};

impl<Dst> MakeMov<Dst, u32> for MovImpls<MovImm<RegOrZero32, RegOrSp32>>
where
    Dst: IntoReg<RegOrZero32>,
{
    type Output = Result<MovImm<RegOrZero32, RegOrSp32>, &'static str>;

    fn make(dst: Dst, src: u32) -> Self::Output {
        use RegOrZero32::WZR;

        let dst = dst.into_reg();
        try_into_mov_z_or_k_32(dst, src)
            .map(|(neg, args)| MovImm::MovNOrZ(MovNOrZImm { neg, args }))
            .or_else(|_| match dst {
                RegOrZero32::Reg(reg32) => orr(reg32, WZR, src)
                    .map(MovImm::Orr)
                    .map_err(|_e| "TODO can't"),
                WZR => Err("TODO can't"),
            })
    }
}

impl RawInstruction for MovNOrZImm<RegOrZero32> {
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        if self.neg {
            movn(self.args.rd, (self.args.imm16, self.args.shift)).to_code()
        } else {
            movz(self.args.rd, (self.args.imm16, self.args.shift)).to_code()
        }
    }
}

impl RawInstruction for MovImm<RegOrZero32, RegOrSp32> {
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        match self {
            MovImm::MovNOrZ(mov_nzimm) => mov_nzimm.to_code(),
            MovImm::Orr(orr) => orr.to_code(),
        }
    }
}

impl<Dst> MakeMov<Dst, i32> for MovImpls<MovNOrZImm<RegOrZero32>>
where
    Dst: IntoReg<RegOrZero32>,
{
    type Output = Result<MovImm<RegOrZero32, RegOrSp32>, &'static str>;

    fn make(dst: Dst, src: i32) -> Self::Output {
        <MovImpls<MovImm<RegOrZero32, RegOrSp32>> as MakeMov<Dst, u32>>::make(dst, src as u32)
    }
}

pub(crate) fn matches_u16(v: u32, shift: u32) -> Option<MoveImm16> {
    if (0xFFFF << shift) & v == v {
        MoveImm16::new(v >> shift).ok()
    } else {
        None
    }
}

pub(crate) fn try_into_mov_z_or_k_32(
    reg: RegOrZero32,
    v: u32,
) -> Result<(bool, MovImmArgs<RegOrZero32>), &'static str> {
    const MAX_R32_SHIFT: u32 = 1;
    const R32_SHIFT_SIZE: u32 = 16;
    let movz = (0..=MAX_R32_SHIFT).filter_map(|idx| {
        let shift = R32_SHIFT_SIZE * idx;
        let shift32 = Shift32::new(shift).expect("invalid shift generated");
        matches_u16(v, shift).map(|val| MovImmArgs::new(reg, (val, shift32)).map(|x| (false, x)))
    });

    let neg_v = !v;
    let movk = (0..=MAX_R32_SHIFT).filter_map(|idx| {
        let shift = R32_SHIFT_SIZE * idx;
        let shift32 = Shift32::new(shift).expect("invalid shift generated");
        matches_u16(neg_v, shift).map(|val| MovImmArgs::new(reg, (val, shift32)).map(|x| (true, x)))
    });

    // movz is tried first!
    movz.chain(movk).next().ok_or("TODO")
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
