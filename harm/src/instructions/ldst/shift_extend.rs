/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use aarchmrs_types::BitValue;

use crate::register::{Reg32, Reg64, RegOrZero32, RegOrZero64};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
// It seems that these variants are actually noop: you cannot actually extend r64 to r64.
// It is only useful if you want to produce a specific bit pattern.
pub enum LdrExtendOption64 {
    #[default]
    LSL = 0b011,
    SXTX = 0b111,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum LdrExtendOption32 {
    #[default]
    UXTW = 0b010,
    SXTW = 0b110,
}

pub trait LdrDestShiftOption {
    const SHIFT_SIZE: u32;
}
pub trait LdrOffsetExtendOption {
    type ExtendOption;
}

impl LdrDestShiftOption for Reg64 {
    const SHIFT_SIZE: u32 = 3;
}

impl LdrOffsetExtendOption for Reg64 {
    type ExtendOption = LdrExtendOption64;
}

impl LdrDestShiftOption for RegOrZero64 {
    const SHIFT_SIZE: u32 = 3;
}

impl LdrOffsetExtendOption for RegOrZero64 {
    type ExtendOption = LdrExtendOption64;
}

impl LdrDestShiftOption for Reg32 {
    const SHIFT_SIZE: u32 = 2;
}

impl LdrOffsetExtendOption for Reg32 {
    type ExtendOption = LdrExtendOption32;
}

impl LdrDestShiftOption for RegOrZero32 {
    const SHIFT_SIZE: u32 = 2;
}

impl LdrOffsetExtendOption for RegOrZero32 {
    type ExtendOption = LdrExtendOption32;
}

pub struct Extended<Dest: LdrDestShiftOption, Offset: LdrOffsetExtendOption> {
    pub shifted: Shifted<Dest, Offset>,
    pub extend: <Offset as LdrOffsetExtendOption>::ExtendOption,
}

pub fn shifted<Dest: LdrDestShiftOption, Offset: LdrOffsetExtendOption>(
    offset: Offset,
    shifted: LdrShift,
) -> Shifted<Dest, Offset> {
    Shifted {
        offset,
        shifted,
        phantom: std::marker::PhantomData,
    }
}

impl<Dest, Offset: LdrOffsetExtendOption> From<Offset> for Shifted<Dest, Offset> {
    fn from(reg: Offset) -> Self {
        Shifted {
            offset: reg,
            shifted: LdrShift::Unshifted,
            phantom: std::marker::PhantomData,
        }
    }
}

// TODO
// ext((W1, UXTW))
// ext((W1, UXTW, 3))
// ldr(W1, (X2, (W3, UXTW, 3))) // !!!
pub fn extended<Dest, Offset>(
    shifted: impl Into<Shifted<Dest, Offset>>,
    extend: <Offset as LdrOffsetExtendOption>::ExtendOption,
) -> Extended<Dest, Offset>
where
    Dest: LdrDestShiftOption,
    Offset: LdrOffsetExtendOption,
{
    Extended {
        shifted: shifted.into(),
        extend,
    }
}

pub fn shifted_by<Dest: LdrDestShiftOption, Offset: LdrOffsetExtendOption>(
    reg: Offset,
    shift: u32,
) -> Result<Shifted<Dest, Offset>, ShiftedError> {
    let accepted = Dest::SHIFT_SIZE;
    let shifted = if shift == 0 {
        LdrShift::Unshifted
    } else if shift == accepted {
        LdrShift::Shifted
    } else {
        return Err(ShiftedError::InvalidShiftSize { shift, accepted });
    };
    Ok(Shifted {
        offset: reg,
        shifted,
        phantom: std::marker::PhantomData,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShiftedError {
    InvalidShiftSize { shift: u32, accepted: u32 },
}

use std::fmt;
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

#[repr(u8)]
pub enum LdrShift {
    Unshifted = 0,
    Shifted = 1,
}

impl From<LdrShift> for BitValue<1> {
    fn from(value: LdrShift) -> Self {
        (value as u8).into()
    }
}

pub struct Shifted<Dest, Reg> {
    pub offset: Reg,
    pub shifted: LdrShift,
    phantom: std::marker::PhantomData<Dest>,
}
