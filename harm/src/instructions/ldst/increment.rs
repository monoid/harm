/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use crate::{
    bits::{BitError, SBitValue},
    register::{RegOrSp64, RegOrZero64},
};

pub type LdStIncOffset = SBitValue<9>;

pub struct PreIncrement<Offset> {
    pub base: RegOrSp64,
    pub offset: Offset,
}

pub trait MakePreIncrement<Base, Offset> {
    type Output;
    fn new(base: Base, offset: Offset) -> Self::Output;
}

impl<const WIDTH: u32, Base: Into<RegOrSp64>>
    MakePreIncrement<Base, SBitValue<WIDTH>> for PreIncrement<SBitValue<WIDTH>>
{
    type Output = Self;

    fn new(base: Base, offset: SBitValue<WIDTH>) -> Self::Output {
        PreIncrement {
            base: base.into(),
            offset,
        }
    }
}

impl<const WIDTH: u32, Base: Into<RegOrSp64>> MakePreIncrement<Base, i32>
    for PreIncrement<SBitValue<WIDTH>>
{
    type Output = Result<Self, BitError>;

    fn new(base: Base, offset: i32) -> Self::Output {
        SBitValue::new(offset).map(|offset| Self::new(base, offset))
    }
}

pub struct PostIncrement<Offset> {
    pub base: RegOrSp64,
    pub offset: Offset,
}

pub trait MakePostIncrement<Base, Offset> {
    type Output;
    fn new(base: Base, offset: Offset) -> Self::Output;
}

impl<const WIDTH: u32, Base: Into<RegOrSp64>>
    MakePostIncrement<Base, SBitValue<WIDTH>> for PostIncrement<SBitValue<WIDTH>>
{
    type Output = Self;

    fn new(base: Base, offset: SBitValue<WIDTH>) -> Self::Output {
        PostIncrement {
            base: base.into(),
            offset,
        }
    }
}

impl<const WIDTH: u32, Base: Into<RegOrSp64>> MakePostIncrement<Base, i32>
    for PostIncrement<SBitValue<WIDTH>>
{
    type Output = Result<Self, BitError>;

    fn new(base: Base, offset: i32) -> Self::Output {
        SBitValue::new(offset).map(|offset| Self::new(base, offset))
    }
}

impl<Base: Into<RegOrSp64>, Offset: Into<RegOrZero64>> MakePostIncrement<Base, Offset>
    for PostIncrement<RegOrZero64>
{
    type Output = Self;

    fn new(base: Base, offset: Offset) -> Self::Output {
        PostIncrement {
            base: base.into(),
            offset: offset.into(),
        }
    }
}

pub fn preinc<Base, OffsetInp, OffsetOut>(
    base: Base,
    offset: OffsetInp,
) -> <PreIncrement<OffsetOut> as MakePreIncrement<Base, OffsetInp>>::Output
where
    PreIncrement<OffsetOut>: MakePreIncrement<Base, OffsetInp>,
{
    PreIncrement::new(base, offset)
}

pub fn postinc<Base, OffsetInp, OffsetOut>(
    base: Base,
    offset: OffsetInp,
) -> <PostIncrement<OffsetOut> as MakePostIncrement<Base, OffsetInp>>::Output
where
    PostIncrement<OffsetOut>: MakePostIncrement<Base, OffsetInp>,
{
    PostIncrement::new(base, offset)
}
