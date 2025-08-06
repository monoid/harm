/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

#[macro_use]
pub(crate) mod macros;

mod increment;
mod shift_extend;

mod ldr;
mod ldrb;
mod ldrh;
mod ldrsb;
mod ldrsh;
mod ldrsw;
mod ldur;
mod str;
mod strb;
mod strh;

pub use self::increment::*;
pub use self::ldr::*;
pub use self::ldrb::*;
pub use self::ldrh::*;
pub use self::ldrsb::*;
pub use self::ldrsh::*;
pub use self::ldrsw::*;
pub use self::ldur::*;
pub use self::shift_extend::*;
pub use self::str::*;
pub use self::strb::*;
pub use self::strh::*;
use crate::bits::{SBitValue, UBitValue};

pub type LdStPcOffset = SBitValue<19, 2>;

/// PC register as a base register with an immediate offset.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Pc;

pub type ScaledOffset8 = UBitValue<12, 0>;
pub type ScaledOffset16 = UBitValue<12, 1>;
pub type ScaledOffset32 = UBitValue<12, 2>;
pub type ScaledOffset64 = UBitValue<12, 3>;

pub type UnscaledOffset = SBitValue<9>;

pub struct ByteShift;

impl LdStDestShiftOption for ByteShift {
    const SHIFT_SIZE: u32 = 0;
}

pub struct HalfShift;

impl LdStDestShiftOption for HalfShift {
    const SHIFT_SIZE: u32 = 1;
}
