/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use super::*;
use crate::instructions::RawInstruction;
use crate::instructions::dpimm::immediate::LogicalImmFields;
use crate::instructions::dpimm::{
    MakeMovArgs, MovImmArgs, MoveImm16, MoveShift, Shift32, Shift64, movn, movz,
};
use crate::instructions::logical::{LogicalArgs, Orr, orr};
use crate::outcome::Outcome;
use crate::register::{IntoReg, RegOrSp32, RegOrSp64, RegOrZero32, RegOrZero64};

#[derive(Debug, Clone)]
pub struct InvalidMovImm;

use core::fmt;
impl fmt::Display for InvalidMovImm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "the immediate value doesn't match any possible pattern for `mov`"
        )
    }
}

impl core::error::Error for InvalidMovImm {}

pub enum MovImm<RZ: MoveShift, RSp> {
    MovNOrZ(MovNOrZImm<RZ>),
    Orr(Orr<LogicalArgs<RSp, RZ, LogicalImmFields>>),
}

pub struct MovNOrZImm<R: MoveShift> {
    pub neg: bool,
    pub args: MovImmArgs<R>,
}

impl<R: MoveShift> Sealed for MovNOrZImm<R> {}

impl<RZ: MoveShift + Sealed, RSp: Sealed> Sealed for MovImm<RZ, RSp> {}

impl<Dst> MakeMov<Dst, u32> for MovImpls<MovImm<RegOrZero32, RegOrSp32>>
where
    Dst: IntoReg<RegOrZero32>,
{
    type Output = Result<MovImm<RegOrZero32, RegOrSp32>, InvalidMovImm>;

    fn make(dst: Dst, src: u32) -> Self::Output {
        let dst = dst.into_reg();
        try_into_mov_z_or_k_32(dst, src)
            .map(MovImm::MovNOrZ)
            .or_else(|_| match dst {
                RegOrZero32::Reg(reg32) => imm_orr32(reg32, src),
                RegOrZero32::WZR => Err(InvalidMovImm),
            })
    }
}

impl MakeMov<RegOrSp32, u32> for MovImpls<MovImm<RegOrZero32, RegOrSp32>> {
    type Output = Result<MovImm<RegOrZero32, RegOrSp32>, InvalidMovImm>;

    fn make(dst: RegOrSp32, src: u32) -> Self::Output {
        match dst {
            RegOrSp32::Reg(reg32) => try_into_mov_z_or_k_32(RegOrZero32::Reg(reg32), src)
                .map(MovImm::MovNOrZ)
                .or_else(|_| imm_orr32(reg32, src)),
            RegOrSp32::WSP => imm_orr32(dst, src),
        }
    }
}

impl<Dst> MakeMov<Dst, i32> for MovImpls<MovNOrZImm<RegOrZero32>>
where
    Dst: IntoReg<RegOrZero32>,
{
    type Output = Result<MovImm<RegOrZero32, RegOrSp32>, InvalidMovImm>;

    fn make(dst: Dst, src: i32) -> Self::Output {
        <MovImpls<MovImm<RegOrZero32, RegOrSp32>> as MakeMov<Dst, u32>>::make(dst, src as u32)
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

fn imm_orr32(
    dst: impl Into<RegOrSp32>,
    src: u32,
) -> Result<MovImm<RegOrZero32, RegOrSp32>, InvalidMovImm> {
    match orr(dst.into(), RegOrZero32::WZR, src) {
        Ok(orr) => Ok(MovImm::Orr(orr)),
        Err(_) => Err(InvalidMovImm),
    }
}

const MOVZ_HALFWORD_SIZE: u32 = 16;

fn matches_halfword(v: u64, shift: u32) -> Option<MoveImm16> {
    let mask = (1 << MOVZ_HALFWORD_SIZE) - 1;
    if (mask << shift) & v == v {
        MoveImm16::new((v >> shift) as _).ok()
    } else {
        None
    }
}

fn halfword_pos(v: u64) -> u32 {
    if v == 0 {
        0
    } else {
        v.trailing_zeros() / MOVZ_HALFWORD_SIZE
    }
}

fn try_into_mov_z_or_k_32(
    reg: RegOrZero32,
    v: u32,
) -> Result<MovNOrZImm<RegOrZero32>, InvalidMovImm> {
    let movz = {
        let min_hw = halfword_pos(v.into());
        let shift = MOVZ_HALFWORD_SIZE * min_hw;
        let shift32 = Shift32::new(shift).expect("invalid shift generated");
        matches_halfword(v.into(), shift).map(|val| {
            MovImmArgs::new(reg, (val, shift32)).map(|args| MovNOrZImm { neg: false, args })
        })
    };

    let neg_v = !v;
    let movk = || {
        let min_hw = halfword_pos(neg_v.into());
        let shift = MOVZ_HALFWORD_SIZE * min_hw;
        let shift32 = Shift32::new(shift).expect("invalid shift generated");
        matches_halfword(neg_v.into(), shift).map(|val| {
            MovImmArgs::new(reg, (val, shift32)).map(|args| MovNOrZImm { neg: true, args })
        })
    };

    // movz is tried first!
    movz.or_else(movk).ok_or(InvalidMovImm)
}

impl<Dst> MakeMov<Dst, u64> for MovImpls<MovImm<RegOrZero64, RegOrSp64>>
where
    Dst: IntoReg<RegOrZero64>,
{
    type Output = Result<MovImm<RegOrZero64, RegOrSp64>, InvalidMovImm>;

    fn make(dst: Dst, src: u64) -> Self::Output {
        let dst = dst.into_reg();
        try_into_mov_z_or_k_64(dst, src)
            .map(MovImm::MovNOrZ)
            .or_else(|_| match dst {
                RegOrZero64::Reg(reg64) => imm_orr64(reg64, src),
                RegOrZero64::XZR => Err(InvalidMovImm),
            })
    }
}

impl MakeMov<RegOrSp64, u64> for MovImpls<MovImm<RegOrZero64, RegOrSp64>> {
    type Output = Result<MovImm<RegOrZero64, RegOrSp64>, InvalidMovImm>;

    fn make(dst: RegOrSp64, src: u64) -> Self::Output {
        match dst {
            RegOrSp64::Reg(reg64) => try_into_mov_z_or_k_64(RegOrZero64::Reg(reg64), src)
                .map(MovImm::MovNOrZ)
                .or_else(|_| imm_orr64(reg64, src)),
            RegOrSp64::SP => imm_orr64(dst, src),
        }
    }
}

impl<Dst> MakeMov<Dst, i64> for MovImpls<MovNOrZImm<RegOrZero64>>
where
    Dst: IntoReg<RegOrZero64>,
{
    type Output = Result<MovImm<RegOrZero64, RegOrSp64>, InvalidMovImm>;

    fn make(dst: Dst, src: i64) -> Self::Output {
        <MovImpls<MovImm<RegOrZero64, RegOrSp64>> as MakeMov<Dst, u64>>::make(dst, src as u64)
    }
}

fn imm_orr64(
    dst: impl Into<RegOrSp64>,
    src: u64,
) -> Result<MovImm<RegOrZero64, RegOrSp64>, InvalidMovImm> {
    match orr(dst.into(), RegOrZero64::XZR, src) {
        Ok(orr) => Ok(MovImm::Orr(orr)),
        Err(_) => Err(InvalidMovImm),
    }
}

fn try_into_mov_z_or_k_64(
    reg: RegOrZero64,
    v: u64,
) -> Result<MovNOrZImm<RegOrZero64>, InvalidMovImm> {
    let movz = {
        let min_hw = halfword_pos(v);
        let shift = MOVZ_HALFWORD_SIZE * min_hw;
        let shift64 = Shift64::new(shift).expect("invalid shift generated");
        matches_halfword(v, shift).map(|val| {
            MovImmArgs::new(reg, (val, shift64)).map(|args| MovNOrZImm { neg: false, args })
        })
    };

    let neg_v = !v;
    let movk = || {
        let min_hw = halfword_pos(neg_v);
        let shift = MOVZ_HALFWORD_SIZE * min_hw;
        let shift64 = Shift64::new(shift).expect("invalid shift generated");
        matches_halfword(neg_v, shift).map(|val| {
            MovImmArgs::new(reg, (val, shift64)).map(|args| MovNOrZImm { neg: true, args })
        })
    };

    // movz is tried first!
    movz.or_else(movk).ok_or(InvalidMovImm)
}

impl RawInstruction for MovNOrZImm<RegOrZero64> {
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        if self.neg {
            movn(self.args.rd, (self.args.imm16, self.args.shift)).to_code()
        } else {
            movz(self.args.rd, (self.args.imm16, self.args.shift)).to_code()
        }
    }
}

impl RawInstruction for MovImm<RegOrZero64, RegOrSp64> {
    #[inline]
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        match self {
            MovImm::MovNOrZ(mov_nzimm) => mov_nzimm.to_code(),
            MovImm::Orr(orr) => orr.to_code(),
        }
    }
}
