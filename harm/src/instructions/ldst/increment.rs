/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use crate::{
    bits::{BitError, SBitValue},
    register::RegOrZero64,
};

pub type LdStIncOffset = SBitValue<9>;

#[derive(Copy, Clone, Debug)]
pub struct Inc<Offset> {
    pub offset: Offset,
}

pub trait MakeInc<Offset> {
    type Output;
    fn new(offset: Offset) -> Self::Output;
}

impl<const WIDTH: u32> MakeInc<SBitValue<WIDTH>> for Inc<SBitValue<WIDTH>> {
    type Output = Self;

    fn new(offset: SBitValue<WIDTH>) -> Self::Output {
        Inc { offset }
    }
}

impl<const WIDTH: u32> MakeInc<i32> for Inc<SBitValue<WIDTH>> {
    type Output = Result<Self, BitError>;

    fn new(offset: i32) -> Self::Output {
        SBitValue::new(offset).map(Self::new)
    }
}

impl<Offset: Into<RegOrZero64>> MakeInc<Offset> for Inc<RegOrZero64> {
    type Output = Self;

    fn new(offset: Offset) -> Self::Output {
        Inc {
            offset: offset.into(),
        }
    }
}

pub fn preinc<Base, OffsetInp, OffsetOut>(
    base: Base,
    offset: OffsetInp,
) -> (<Inc<OffsetOut> as MakeInc<OffsetInp>>::Output, Base)
where
    Inc<OffsetOut>: MakeInc<OffsetInp>,
{
    (Inc::new(offset), base)
}

pub fn postinc<Base, OffsetInp, OffsetOut>(
    base: Base,
    offset: OffsetInp,
) -> (Base, <Inc<OffsetOut> as MakeInc<OffsetInp>>::Output)
where
    Inc<OffsetOut>: MakeInc<OffsetInp>,
{
    (base, Inc::new(offset))
}

pub fn inc<OffsetInp, OffsetOut>(
    offset: OffsetInp,
) -> <Inc<OffsetOut> as MakeInc<OffsetInp>>::Output
where
    Inc<OffsetOut>: MakeInc<OffsetInp>,
{
    Inc::new(offset)
}
