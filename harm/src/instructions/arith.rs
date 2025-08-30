/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use aarchmrs_types::BitValue;

use crate::{
    bits::{BitError, UBitValue},
    register::{Reg32, Reg64, RegOrZero32, RegOrZero64},
};

#[macro_use]
pub(crate) mod macros;

pub mod add;
pub mod sub;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ShiftedReg<T> {
    reg: T,
    shift: Shift,
}

impl<T> ShiftedReg<T> {
    pub fn new(reg: T) -> Self {
        Self {
            reg,
            shift: <_>::default(),
        }
    }

    pub fn shift(mut self, mode: ShiftMode, amount: ShiftAmount) -> Self {
        self.shift = Shift { mode, amount };
        self
    }

    pub fn try_shift(mut self, mode: ShiftMode, amount: u32) -> Result<Self, BitError> {
        let amount = amount.try_into()?;
        self.shift = Shift { mode, amount };
        Ok(self)
    }
}

impl From<Reg64> for ShiftedReg<RegOrZero64> {
    #[inline]
    fn from(value: Reg64) -> Self {
        Self::new(value.into())
    }
}

impl From<Reg32> for ShiftedReg<RegOrZero32> {
    #[inline]
    fn from(value: Reg32) -> Self {
        Self::new(value.into())
    }
}

pub type ShiftAmount = UBitValue<6>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct Shift {
    mode: ShiftMode,
    amount: ShiftAmount,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum ShiftMode {
    #[default]
    LSL = 0b00,
    LSR = 0b01,
    ASR = 0b10,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ExtendedReg<T> {
    reg: T,
    extend: Extend,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ExtendError {
    OutOfRange,
}

use core::fmt;
impl fmt::Display for ExtendError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExtendError::OutOfRange => write!(f, "value out of range"),
        }
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Hash)]
pub struct ExtendShiftAmount(UBitValue<3>);

impl ExtendShiftAmount {
    #[inline]
    pub fn try_new(value: u8) -> Result<Self, ExtendError> {
        let val = UBitValue::new(value.into()).map_err(|_| ExtendError::OutOfRange)?;
        if value <= 4 {
            Ok(ExtendShiftAmount(val))
        } else {
            Err(ExtendError::OutOfRange)
        }
    }
}

impl From<ExtendShiftAmount> for BitValue<3> {
    #[inline]
    fn from(value: ExtendShiftAmount) -> Self {
        value.0.into()
    }
}

impl TryFrom<u8> for ExtendShiftAmount {
    type Error = ExtendError;

    #[inline]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

impl<T> ExtendedReg<T> {
    pub fn new(reg: T) -> Self {
        Self {
            reg,
            extend: <_>::default(),
        }
    }

    pub fn extend(mut self, mode: ExtendMode, amount: ExtendShiftAmount) -> Self {
        self.extend = Extend { mode, amount };
        self
    }
}

impl From<Reg64> for ExtendedReg<RegOrZero64> {
    #[inline]
    fn from(value: Reg64) -> Self {
        Self::new(value.into())
    }
}

impl From<Reg32> for ExtendedReg<RegOrZero32> {
    #[inline]
    fn from(value: Reg32) -> Self {
        Self::new(value.into())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum ExtendMode {
    UXTB = 0b000,
    UXTH = 0b001,
    UXTW = 0b010,
    #[default]
    UXTX = 0b011, // it is essentially a noop for any register, so it is the default
    SXTB = 0b100,
    SXTH = 0b101,
    SXTW = 0b110,
    SXTX = 0b111,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct Extend {
    mode: ExtendMode,
    amount: ExtendShiftAmount,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AddSubImm12 {
    Unshifted(UBitValue<12, 0>),
    Shifted(UBitValue<12, 12>),
}

impl From<UBitValue<12, 0>> for AddSubImm12 {
    fn from(value: UBitValue<12, 0>) -> Self {
        Self::Unshifted(value)
    }
}

impl From<UBitValue<12, 12>> for AddSubImm12 {
    fn from(value: UBitValue<12, 12>) -> Self {
        Self::Shifted(value)
    }
}

impl TryFrom<u32> for AddSubImm12 {
    type Error = (BitError, BitError);

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let unshifted_err = match UBitValue::<12, 0>::try_from(value) {
            Ok(unshifted_value) => return Ok(unshifted_value.into()),
            Err(unshifted_err) => unshifted_err,
        };
        match UBitValue::<12, 12>::try_from(value) {
            Ok(shifted_value) => Ok(shifted_value.into()),
            Err(shifted_err) => Err((unshifted_err, shifted_err)),
        }
    }
}
