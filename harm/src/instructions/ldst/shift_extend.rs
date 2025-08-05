/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use aarchmrs_types::BitValue;

use crate::register::{Reg32, Reg64, RegOrZero32, RegOrZero64};

/// Represents whether the register is shifted by the destination register size or not.
#[repr(u8)]
pub enum LdStShift {
    Unshifted = 0,
    Shifted = 1,
}

impl From<LdStShift> for BitValue<1> {
    fn from(value: LdStShift) -> Self {
        (value as u8).into()
    }
}
/// `LDR` extend options for 64-bit registers.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
// It seems that these variants are actually noop: you cannot actually extend r64 to r64.
// It is only useful if you want to produce a specific bit pattern.
pub enum LdStExtendOption64 {
    #[default]
    LSL = 0b011,
    SXTX = 0b111,
}

/// `LDR` extend options for 32-bit registers.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum LdStExtendOption32 {
    #[default]
    UXTW = 0b010,
    SXTW = 0b110,
}

// TODO sealed traits
pub trait LdStDestShiftOption {
    const SHIFT_SIZE: u32;
}
pub trait LdStOffsetExtendOption {
    type ExtendOption;
}

impl LdStDestShiftOption for Reg64 {
    const SHIFT_SIZE: u32 = 3;
}

impl LdStOffsetExtendOption for Reg64 {
    type ExtendOption = LdStExtendOption64;
}

impl LdStDestShiftOption for RegOrZero64 {
    const SHIFT_SIZE: u32 = 3;
}

impl LdStOffsetExtendOption for RegOrZero64 {
    type ExtendOption = LdStExtendOption64;
}

impl LdStDestShiftOption for Reg32 {
    const SHIFT_SIZE: u32 = 2;
}

impl LdStOffsetExtendOption for Reg32 {
    type ExtendOption = LdStExtendOption32;
}

impl LdStDestShiftOption for RegOrZero32 {
    const SHIFT_SIZE: u32 = 2;
}

impl LdStOffsetExtendOption for RegOrZero32 {
    type ExtendOption = LdStExtendOption32;
}

pub trait MakeExtended<Dest> {
    type Output;

    fn new(args: Self) -> Self::Output;
}

pub struct Extended<Dest, Offset: LdStOffsetExtendOption> {
    pub offset: Offset,
    pub extend: <Offset as LdStOffsetExtendOption>::ExtendOption,
    pub shifted: LdStShift,
    phantom: PhantomData<Dest>,
}

impl<Dest, Offset: LdStOffsetExtendOption> Extended<Dest, Offset> {
    pub fn new(
        offset: Offset,
        extend: <Offset as LdStOffsetExtendOption>::ExtendOption,
        shifted: LdStShift,
    ) -> Self {
        Self {
            offset,
            extend,
            shifted,
            phantom: PhantomData,
        }
    }
}

// TODO
// ext((W1, UXTW))
// ext((W1, UXTW, 3))
// ldr(W1, (X2, (W3, UXTW, 3))) // !!!
pub fn ext<Dest, Args>(args: Args) -> <Args as MakeExtended<Dest>>::Output
where
    Args: MakeExtended<Dest>,
    Dest: LdStDestShiftOption,
{
    <_>::new(args)
}

impl<Dest, R64> MakeExtended<Dest> for (R64, LdStExtendOption64)
where
    RegOrZero64: From<R64>,
{
    type Output = Extended<Dest, RegOrZero64>;

    fn new((offset, extend): Self) -> Self::Output {
        Extended::new(offset.into(), extend, LdStShift::Unshifted)
    }
}

impl<Dest, R32> MakeExtended<Dest> for (R32, LdStExtendOption32)
where
    RegOrZero32: From<R32>,
{
    type Output = Extended<Dest, RegOrZero32>;

    fn new((offset, extend): Self) -> Self::Output {
        Extended::new(offset.into(), extend, LdStShift::Unshifted)
    }
}
impl<Dest, R64> MakeExtended<Dest> for (R64, LdStExtendOption64, LdStShift)
where
    RegOrZero64: From<R64>,
{
    type Output = Extended<Dest, RegOrZero64>;

    fn new((offset, extend, shifted): Self) -> Self::Output {
        Extended::new(offset.into(), extend, shifted)
    }
}

impl<Dest, R32> MakeExtended<Dest> for (R32, LdStExtendOption32, LdStShift)
where
    RegOrZero32: From<R32>,
{
    type Output = Extended<Dest, RegOrZero32>;

    fn new((offset, extend, shifted): Self) -> Self::Output {
        Extended::new(offset.into(), extend, shifted)
    }
}

impl<Dest, R64> MakeExtended<Dest> for (R64, LdStExtendOption64, u32)
where
    RegOrZero64: From<R64>,
    Dest: LdStDestShiftOption,
{
    type Output = Result<Extended<Dest, RegOrZero64>, ShiftedError>;

    fn new((offset, extend, shift): Self) -> Self::Output {
        shifted_by::<Dest>(shift).map(|shifted| Extended::new(offset.into(), extend, shifted))
    }
}

impl<Dest, R32> MakeExtended<Dest> for (R32, LdStExtendOption32, u32)
where
    RegOrZero32: From<R32>,
    Dest: LdStDestShiftOption,
{
    type Output = Result<Extended<Dest, RegOrZero32>, ShiftedError>;

    fn new((offset, extend, shift): Self) -> Self::Output {
        shifted_by::<Dest>(shift).map(|shifted| Extended::new(offset.into(), extend, shifted))
    }
}

/// Creates a shifted register with a specific shift size. Please note that allowed the shift size depeds on the
/// destination register size. For 64-bit registers, the shift is either 0 or 3, and for 32-bit registers, the shift is
/// either 0 or 2.
fn shifted_by<Dest: LdStDestShiftOption>(shift: u32) -> Result<LdStShift, ShiftedError> {
    let accepted = Dest::SHIFT_SIZE;
    // N.B.: for LDRB/LDRSB shift is 0, and it has to be Shifted
    if shift == accepted {
        Ok(LdStShift::Shifted)
    } else if shift == 0 {
        Ok(LdStShift::Unshifted)
    } else {
        Err(ShiftedError::InvalidShiftSize { shift, accepted })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShiftedError {
    InvalidShiftSize { shift: u32, accepted: u32 },
}

use std::{fmt, marker::PhantomData};
impl fmt::Display for ShiftedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidShiftSize { shift, accepted } => {
                write!(
                    f,
                    "Invalid shift {shift} for a register of shift size {accepted}"
                )
            }
        }
    }
}
