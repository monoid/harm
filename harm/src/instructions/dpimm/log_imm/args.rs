/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use crate::{
    outcome::{Outcome, Unfallible},
    register::{IntoReg, RegOrZero32, RegOrZero64},
};

use super::immediate::{LogicalImmError, LogicalImmFields};

#[derive(Debug, Copy, Clone)]
pub struct LogicalArgs<Reg, Mask> {
    pub rd: Reg,
    pub rn: Reg,
    pub mask: Mask,
}

pub trait MakeLogicalArgs<Rd, Rn, Mask> {
    type Outcome: Outcome;

    fn new(rd: Rd, rn: Rn, mask: Mask) -> Self::Outcome;
}

impl<RdIn, RnIn> MakeLogicalArgs<RdIn, RnIn, LogicalImmFields>
    for LogicalArgs<RegOrZero32, LogicalImmFields>
where
    RdIn: IntoReg<RegOrZero32>,
    RnIn: IntoReg<RegOrZero32>,
{
    type Outcome = Unfallible<Self>;

    fn new(rd: RdIn, rn: RnIn, mask: LogicalImmFields) -> Self::Outcome {
        Unfallible(Self {
            rd: rd.into_reg(),
            rn: rn.into_reg(),
            mask,
        })
    }
}

impl<RdIn, RnIn> MakeLogicalArgs<RdIn, RnIn, u32> for LogicalArgs<RegOrZero32, LogicalImmFields>
where
    RdIn: IntoReg<RegOrZero32>,
    RnIn: IntoReg<RegOrZero32>,
{
    type Outcome = Result<Self, LogicalImmError>;

    fn new(rd: RdIn, rn: RnIn, mask: u32) -> Self::Outcome {
        LogicalImmFields::try_new32(mask).map(|mask| Self {
            rd: rd.into_reg(),
            rn: rn.into_reg(),
            mask,
        })
    }
}

impl<RdIn, RnIn> MakeLogicalArgs<RdIn, RnIn, LogicalImmFields>
    for LogicalArgs<RegOrZero64, LogicalImmFields>
where
    RdIn: IntoReg<RegOrZero64>,
    RnIn: IntoReg<RegOrZero64>,
{
    type Outcome = Unfallible<Self>;

    fn new(rd: RdIn, rn: RnIn, mask: LogicalImmFields) -> Self::Outcome {
        Unfallible(Self {
            rd: rd.into_reg(),
            rn: rn.into_reg(),
            mask,
        })
    }
}

impl<RdIn, RnIn> MakeLogicalArgs<RdIn, RnIn, u64> for LogicalArgs<RegOrZero64, LogicalImmFields>
where
    RdIn: IntoReg<RegOrZero64>,
    RnIn: IntoReg<RegOrZero64>,
{
    type Outcome = Result<Self, LogicalImmError>;

    fn new(rd: RdIn, rn: RnIn, mask: u64) -> Self::Outcome {
        LogicalImmFields::try_new64(mask).map(|mask| Self {
            rd: rd.into_reg(),
            rn: rn.into_reg(),
            mask,
        })
    }
}
