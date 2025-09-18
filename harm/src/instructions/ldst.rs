/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

#[macro_use]
pub(crate) mod macros;

mod increment;
mod shift_extend;

mod ldp;
mod ldpsw;
mod ldr;
mod ldrb;
mod ldrh;
mod ldrsb;
mod ldrsh;
mod ldrsw;
mod ldur;
mod ldurb;
mod ldurh;
mod ldursb;
mod ldursh;
mod ldursw;
mod stp;
mod str;
mod strb;
mod strh;
mod stur;
mod sturb;
mod sturh;

pub use self::increment::*;
pub use self::ldp::*;
pub use self::ldpsw::*;
pub use self::ldr::*;
pub use self::ldrb::*;
pub use self::ldrh::*;
pub use self::ldrsb::*;
pub use self::ldrsh::*;
pub use self::ldrsw::*;
pub use self::ldur::*;
pub use self::ldurb::*;
pub use self::ldurh::*;
pub use self::ldursb::*;
pub use self::ldursh::*;
pub use self::ldursw::*;
pub use self::shift_extend::*;
pub use self::stp::*;
pub use self::str::*;
pub use self::strb::*;
pub use self::strh::*;
pub use self::stur::*;
pub use self::sturb::*;
pub use self::sturh::*;
use crate::bits::{SBitValue, UBitValue};
use crate::sealed::Sealed;

pub type LdStPcOffset = SBitValue<19, 2>;

/// PC register as a base register with an immediate offset.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Pc;

pub type ScaledOffset8 = UBitValue<12, 0>;
pub type ScaledOffset16 = UBitValue<12, 1>;
pub type ScaledOffset32 = UBitValue<12, 2>;
pub type ScaledOffset64 = UBitValue<12, 3>;

pub type UnscaledOffset = SBitValue<9>;

pub type LdpStpOffset32 = SBitValue<7, 2>;
pub type LdpStpOffset64 = SBitValue<7, 3>;

#[derive(Copy, Clone, Debug)]
pub struct ByteShift;

impl Sealed for ByteShift {}

impl LdStDestShiftOption for ByteShift {
    const SHIFT_SIZE: u32 = 0;
}

#[derive(Copy, Clone, Debug)]
pub struct HalfShift;

impl Sealed for HalfShift {}

impl LdStDestShiftOption for HalfShift {
    const SHIFT_SIZE: u32 = 1;
}
