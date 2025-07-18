/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use crate::bits::SBitValue;

pub type PcOffset = SBitValue<19>;
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Pc(pub PcOffset);

pub const fn pc(offset: PcOffset) -> Pc {
    Pc(offset)
}

pub type ScaledOffset = SBitValue<9>;
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Scaled(pub ScaledOffset);

pub const fn scaled(offset: ScaledOffset) -> Scaled {
    Scaled(offset)
}

pub type UnscaledOffset = SBitValue<9>;
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Unscaled(pub UnscaledOffset);

pub const fn unscaled(offset: UnscaledOffset) -> Unscaled {
    Unscaled(offset)
}

// TODO register shifted

pub struct Load<Dst, Base, Offset> {
    pub dst: Dst,
    pub base: Base,
    pub offset: Offset,
}

pub trait MakeLoad<Dst, Base, Offset> {
    fn new(dst: Dst, base: Base, offset: Offset) -> Self;
}
