/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

mod increment;
pub mod ldr;
mod shift_extend;

pub use self::increment::*;
pub use self::ldr::*;
pub use self::shift_extend::*;
use crate::{
    bits::{SBitValue, UBitValue},
    register::RegOrSp64,
};

pub type LdStPcOffset = SBitValue<19, 2>;

/// PC register as a base register with an immediate offset.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Pc;

pub type ScaledOffset32 = UBitValue<12, 2>;
pub type ScaledOffset64 = UBitValue<12, 3>;

pub type UnscaledOffset = SBitValue<9>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Unscaled(pub RegOrSp64, pub UnscaledOffset);

pub const fn unscaled(base: RegOrSp64, offset: UnscaledOffset) -> Unscaled {
    Unscaled(base, offset)
}

/// A `LDR` instruction with a destination and an address.
pub struct Load<Dst, Addr> {
    dst: Dst,
    addr: Addr,
}

impl<Dst, Addr> Load<Dst, Addr> {
    pub fn dst(&self) -> &Dst {
        &self.dst
    }

    pub fn addr(&self) -> &Addr {
        &self.addr
    }
}

/// Defines possible was to construct a `Load` instruction.
// TODO sealed trait?
pub trait MakeLoad<Dst, Addr> {
    /// Allows defining both faillible and infallible constructors.
    type Output;
    fn new(dst: Dst, addr: Addr) -> Self::Output;
}
