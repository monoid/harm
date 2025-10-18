/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use crate::{
    instructions::logical::{
        LogicalArgs, MakeSpLogicalArgs, MakeTstLogicalArgs, MakeZeroLogicalArgs,
    },
    outcome::Unfallible,
    register::{IntoReg, RegOrSp32, RegOrSp64, RegOrZero32, RegOrZero64},
};

use super::immediate::{LogicalImmError, LogicalImmFields};

impl<RdIn, RnIn> MakeSpLogicalArgs<RdIn, RnIn, LogicalImmFields>
    for LogicalArgs<RegOrSp32, RegOrZero32, LogicalImmFields>
where
    RdIn: IntoReg<RegOrSp32>,
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

impl<RdIn, RnIn> MakeSpLogicalArgs<RdIn, RnIn, u32>
    for LogicalArgs<RegOrSp32, RegOrZero32, LogicalImmFields>
where
    RdIn: IntoReg<RegOrSp32>,
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

impl<RdIn, RnIn> MakeSpLogicalArgs<RdIn, RnIn, LogicalImmFields>
    for LogicalArgs<RegOrSp64, RegOrZero64, LogicalImmFields>
where
    RdIn: IntoReg<RegOrSp64>,
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

impl<RdIn, RnIn> MakeSpLogicalArgs<RdIn, RnIn, u64>
    for LogicalArgs<RegOrSp64, RegOrZero64, LogicalImmFields>
where
    RdIn: IntoReg<RegOrSp64>,
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

impl<RdIn, RnIn> MakeZeroLogicalArgs<RdIn, RnIn, LogicalImmFields>
    for LogicalArgs<RegOrZero32, RegOrZero32, LogicalImmFields>
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

impl<RdIn, RnIn> MakeZeroLogicalArgs<RdIn, RnIn, u32>
    for LogicalArgs<RegOrZero32, RegOrZero32, LogicalImmFields>
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

impl<RdIn, RnIn> MakeZeroLogicalArgs<RdIn, RnIn, LogicalImmFields>
    for LogicalArgs<RegOrZero64, RegOrZero64, LogicalImmFields>
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

impl<RdIn, RnIn> MakeZeroLogicalArgs<RdIn, RnIn, u64>
    for LogicalArgs<RegOrZero64, RegOrZero64, LogicalImmFields>
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

impl<RnIn> MakeTstLogicalArgs<RnIn, LogicalImmFields>
    for LogicalArgs<RegOrZero32, RegOrZero32, LogicalImmFields>
where
    RnIn: IntoReg<RegOrZero32>,
{
    type Outcome = Unfallible<Self>;

    fn new(rn: RnIn, mask: LogicalImmFields) -> Self::Outcome {
        Unfallible(Self {
            rd: RegOrZero32::WZR,
            rn: rn.into_reg(),
            mask,
        })
    }
}

impl<RnIn> MakeTstLogicalArgs<RnIn, u32> for LogicalArgs<RegOrZero32, RegOrZero32, LogicalImmFields>
where
    RnIn: IntoReg<RegOrZero32>,
{
    type Outcome = Result<Self, LogicalImmError>;

    fn new(rn: RnIn, mask: u32) -> Self::Outcome {
        LogicalImmFields::try_new32(mask).map(|mask| Self {
            rd: RegOrZero32::WZR,
            rn: rn.into_reg(),
            mask,
        })
    }
}

impl<RnIn> MakeTstLogicalArgs<RnIn, LogicalImmFields>
    for LogicalArgs<RegOrZero64, RegOrZero64, LogicalImmFields>
where
    RnIn: IntoReg<RegOrZero64>,
{
    type Outcome = Unfallible<Self>;

    fn new(rn: RnIn, mask: LogicalImmFields) -> Self::Outcome {
        Unfallible(Self {
            rd: RegOrZero64::XZR,
            rn: rn.into_reg(),
            mask,
        })
    }
}

impl<RnIn> MakeTstLogicalArgs<RnIn, u64> for LogicalArgs<RegOrZero64, RegOrZero64, LogicalImmFields>
where
    RnIn: IntoReg<RegOrZero64>,
{
    type Outcome = Result<Self, LogicalImmError>;

    fn new(rn: RnIn, mask: u64) -> Self::Outcome {
        LogicalImmFields::try_new64(mask).map(|mask| Self {
            rd: RegOrZero64::XZR,
            rn: rn.into_reg(),
            mask,
        })
    }
}
