/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

#[macro_use]
pub(crate) mod macros;

mod increment;
mod ldr;
mod ldrb;
mod shift_extend;
mod str;

pub use self::increment::*;
pub use self::ldr::*;
pub use self::ldrb::*;
pub use self::shift_extend::*;
pub use self::str::*;
use crate::bits::{SBitValue, UBitValue};

pub type LdStPcOffset = SBitValue<19, 2>;

/// PC register as a base register with an immediate offset.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Pc;

pub type ScaledOffset8 = UBitValue<12, 0>;
pub type ScaledOffset32 = UBitValue<12, 2>;
pub type ScaledOffset64 = UBitValue<12, 3>;

pub type UnscaledOffset = SBitValue<9>;
