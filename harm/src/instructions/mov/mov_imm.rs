/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use super::*;
use crate::instructions::RawInstruction;
use crate::instructions::dpimm::{
    MakeMovArgs, MovImmArgs, MoveImm16, Shift32, Shift64, movn, movz,
};
use crate::instructions::logical::orr;
use crate::outcome::Outcome;
use crate::register::{IntoReg, RegOrSp32, RegOrZero32};

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

impl<Dst> MakeMov<Dst, u32> for MovImpls<MovImm<RegOrZero32, RegOrSp32>>
where
    Dst: IntoReg<RegOrZero32>,
{
    type Output = Result<MovImm<RegOrZero32, RegOrSp32>, InvalidMovImm>;

    fn make(dst: Dst, src: u32) -> Self::Output {
        use RegOrZero32::WZR;

        let dst = dst.into_reg();
        try_into_mov_z_or_k_32(dst, src)
            .map(|(neg, args)| MovImm::MovNOrZ(MovNOrZImm { neg, args }))
            .or_else(|_| match dst {
                RegOrZero32::Reg(reg32) => orr(reg32, WZR, src)
                    .map(MovImm::Orr)
                    .map_err(|_e| InvalidMovImm),
                WZR => Err(InvalidMovImm),
            })
    }
}

impl MakeMov<RegOrSp32, u32> for MovImpls<MovImm<RegOrZero32, RegOrSp32>> {
    type Output = Result<MovImm<RegOrZero32, RegOrSp32>, InvalidMovImm>;

    fn make(dst: RegOrSp32, src: u32) -> Self::Output {
        use RegOrZero32::WZR;

        match dst {
            RegOrSp32::Reg(reg32) => try_into_mov_z_or_k_32(RegOrZero32::Reg(reg32), src)
                .map(|(neg, args)| MovImm::MovNOrZ(MovNOrZImm { neg, args }))
                .or_else(|_| {
                    orr(reg32, WZR, src)
                        .map(MovImm::Orr)
                        .map_err(|_e| InvalidMovImm)
                }),
            RegOrSp32::WSP => orr(dst, WZR, src)
                .map(MovImm::Orr)
                .map_err(|_e| InvalidMovImm),
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

fn matches_u16(v: u64, shift: u32) -> Option<MoveImm16> {
    if (0xFFFF << shift) & v == v {
        MoveImm16::new((v >> shift) as _).ok()
    } else {
        None
    }
}

fn try_into_mov_z_or_k_32(
    reg: RegOrZero32,
    v: u32,
) -> Result<(bool, MovImmArgs<RegOrZero32>), InvalidMovImm> {
    const MAX_R32_SHIFT: u32 = 1;
    const R32_SHIFT_SIZE: u32 = 16;
    let movz = (0..=MAX_R32_SHIFT).filter_map(|idx| {
        let shift = R32_SHIFT_SIZE * idx;
        let shift32 = Shift32::new(shift).expect("invalid shift generated");
        matches_u16(v.into(), shift)
            .map(|val| MovImmArgs::new(reg, (val, shift32)).map(|x| (false, x)))
    });

    let neg_v = !v;
    let movk = (0..=MAX_R32_SHIFT).filter_map(|idx| {
        let shift = R32_SHIFT_SIZE * idx;
        let shift32 = Shift32::new(shift).expect("invalid shift generated");
        matches_u16(neg_v.into(), shift)
            .map(|val| MovImmArgs::new(reg, (val, shift32)).map(|x| (true, x)))
    });

    // movz is tried first!
    movz.chain(movk).next().ok_or(InvalidMovImm)
}

impl<Dst> MakeMov<Dst, u64> for MovImpls<MovImm<RegOrZero64, RegOrSp64>>
where
    Dst: IntoReg<RegOrZero64>,
{
    type Output = Result<MovImm<RegOrZero64, RegOrSp64>, InvalidMovImm>;

    fn make(dst: Dst, src: u64) -> Self::Output {
        use RegOrZero64::XZR;

        let dst = dst.into_reg();
        try_into_mov_z_or_k_64(dst, src)
            .map(|(neg, args)| MovImm::MovNOrZ(MovNOrZImm { neg, args }))
            .or_else(|_| match dst {
                RegOrZero64::Reg(reg64) => orr(reg64, XZR, src)
                    .map(MovImm::Orr)
                    .map_err(|_e| InvalidMovImm),
                XZR => Err(InvalidMovImm),
            })
    }
}

impl MakeMov<RegOrSp64, u64> for MovImpls<MovImm<RegOrZero64, RegOrSp64>> {
    type Output = Result<MovImm<RegOrZero64, RegOrSp64>, InvalidMovImm>;

    fn make(dst: RegOrSp64, src: u64) -> Self::Output {
        use RegOrZero64::XZR;

        match dst {
            RegOrSp64::Reg(reg64) => try_into_mov_z_or_k_64(RegOrZero64::Reg(reg64), src)
                .map(|(neg, args)| MovImm::MovNOrZ(MovNOrZImm { neg, args }))
                .or_else(|_| {
                    orr(reg64, XZR, src)
                        .map(MovImm::Orr)
                        .map_err(|_e| InvalidMovImm)
                }),
            RegOrSp64::SP => orr(dst, XZR, src)
                .map(MovImm::Orr)
                .map_err(|_e| InvalidMovImm),
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

fn try_into_mov_z_or_k_64(
    reg: RegOrZero64,
    v: u64,
) -> Result<(bool, MovImmArgs<RegOrZero64>), InvalidMovImm> {
    const MAX_R64_SHIFT: u32 = 3;
    const R64_SHIFT_SIZE: u32 = 16;
    let movz = (0..=MAX_R64_SHIFT).filter_map(|idx| {
        let shift = R64_SHIFT_SIZE * idx;
        let shift64 = Shift64::new(shift).expect("invalid shift generated");
        matches_u16(v, shift).map(|val| MovImmArgs::new(reg, (val, shift64)).map(|x| (false, x)))
    });

    let neg_v = !v;
    let movk = (0..=MAX_R64_SHIFT).filter_map(|idx| {
        let shift = R64_SHIFT_SIZE * idx;
        let shift64 = Shift64::new(shift).expect("invalid shift generated");
        matches_u16(neg_v, shift).map(|val| MovImmArgs::new(reg, (val, shift64)).map(|x| (true, x)))
    });

    // movz is tried first!
    movz.chain(movk).next().ok_or(InvalidMovImm)
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
