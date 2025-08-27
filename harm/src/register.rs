/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

#![allow(clippy::upper_case_acronyms)]

const NICHE_REG: u8 = 31;

use aarchmrs_types::BitValue;
use num_enum::TryFromPrimitive;

pub trait IntoCode {
    fn code(&self) -> BitValue<5>;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RegisterError {
    InvalidRegisterCode(u8),
}

impl ::core::error::Error for RegisterError {}

impl ::core::fmt::Display for RegisterError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            RegisterError::InvalidRegisterCode(val) => write!(f, "invalid register code: {}", val),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, TryFromPrimitive)]
#[num_enum(error_type(name = RegisterError, constructor = RegisterError::InvalidRegisterCode))]
#[repr(u8)]
pub enum Reg64 {
    X0 = 0,
    X1 = 1,
    X2 = 2,
    X3 = 3,
    X4 = 4,
    X5 = 5,
    X6 = 6,
    X7 = 7,
    X8 = 8,
    X9 = 9,
    X10 = 10,
    X11 = 11,
    X12 = 12,
    X13 = 13,
    X14 = 14,
    X15 = 15,
    X16 = 16,
    X17 = 17,
    X18 = 18,
    X19 = 19,
    X20 = 20,
    X21 = 21,
    X22 = 22,
    X23 = 23,
    X24 = 24,
    X25 = 25,
    X26 = 26,
    X27 = 27,
    X28 = 28,
    X29 = 29,
    LR = 30,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RegOrSp64 {
    Reg(Reg64),
    SP,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RegOrZero64 {
    Reg(Reg64),
    XZR,
}

impl From<Reg64> for RegOrSp64 {
    #[inline]
    fn from(value: Reg64) -> Self {
        Self::Reg(value)
    }
}

impl From<Reg64> for RegOrZero64 {
    #[inline]
    fn from(value: Reg64) -> Self {
        Self::Reg(value)
    }
}

impl TryFrom<u8> for RegOrSp64 {
    type Error = RegisterError;

    #[inline]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Reg64::try_from(value).map(Into::into).or_else(|_| {
            if value == NICHE_REG {
                Ok(Self::SP)
            } else {
                Err(RegisterError::InvalidRegisterCode(value))
            }
        })
    }
}

impl TryFrom<u8> for RegOrZero64 {
    type Error = RegisterError;

    #[inline]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Reg64::try_from(value).map(Into::into).or_else(|_| {
            if value == NICHE_REG {
                Ok(Self::XZR)
            } else {
                Err(RegisterError::InvalidRegisterCode(value))
            }
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, TryFromPrimitive)]
#[num_enum(error_type(name = RegisterError, constructor = RegisterError::InvalidRegisterCode))]
#[repr(u8)]
pub enum Reg32 {
    W0 = 0,
    W1 = 1,
    W2 = 2,
    W3 = 3,
    W4 = 4,
    W5 = 5,
    W6 = 6,
    W7 = 7,
    W8 = 8,
    W9 = 9,
    W10 = 10,
    W11 = 11,
    W12 = 12,
    W13 = 13,
    W14 = 14,
    W15 = 15,
    W16 = 16,
    W17 = 17,
    W18 = 18,
    W19 = 19,
    W20 = 20,
    W21 = 21,
    W22 = 22,
    W23 = 23,
    W24 = 24,
    W25 = 25,
    W26 = 26,
    W27 = 27,
    W28 = 28,
    W29 = 29,
    WLR = 30,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RegOrSp32 {
    Reg(Reg32),
    WSP,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RegOrZero32 {
    Reg(Reg32),
    WZR,
}

impl From<Reg32> for RegOrSp32 {
    #[inline]
    fn from(value: Reg32) -> Self {
        Self::Reg(value)
    }
}

impl From<Reg32> for RegOrZero32 {
    #[inline]
    fn from(value: Reg32) -> Self {
        Self::Reg(value)
    }
}

impl TryFrom<u8> for RegOrSp32 {
    type Error = RegisterError;

    #[inline]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Reg32::try_from(value).map(Into::into).or_else(|_| {
            if value == NICHE_REG {
                Ok(Self::WSP)
            } else {
                Err(RegisterError::InvalidRegisterCode(value))
            }
        })
    }
}

impl TryFrom<u8> for RegOrZero32 {
    type Error = RegisterError;

    #[inline]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Reg32::try_from(value).map(Into::into).or_else(|_| {
            if value == NICHE_REG {
                Ok(Self::WZR)
            } else {
                Err(RegisterError::InvalidRegisterCode(value))
            }
        })
    }
}

impl IntoCode for Reg64 {
    #[inline]
    fn code(&self) -> BitValue<5> {
        BitValue::new_u32(*self as u32)
    }
}

impl IntoCode for Reg32 {
    #[inline]
    fn code(&self) -> BitValue<5> {
        BitValue::new_u32(*self as u32)
    }
}

impl IntoCode for RegOrSp64 {
    #[inline]
    fn code(&self) -> BitValue<5> {
        BitValue::new_u32(match self {
            Self::Reg(general_register64) => *general_register64 as _,
            Self::SP => NICHE_REG.into(),
        })
    }
}

impl IntoCode for RegOrZero64 {
    #[inline]
    fn code(&self) -> BitValue<5> {
        BitValue::new_u32(match self {
            Self::Reg(general_register64) => *general_register64 as _,
            Self::XZR => NICHE_REG.into(),
        })
    }
}

impl IntoCode for RegOrSp32 {
    #[inline]
    fn code(&self) -> BitValue<5> {
        BitValue::new_u32(match self {
            Self::Reg(general_register32) => *general_register32 as _,
            Self::WSP => NICHE_REG.into(),
        })
    }
}

impl IntoCode for RegOrZero32 {
    #[inline]
    fn code(&self) -> BitValue<5> {
        BitValue::new_u32(match self {
            Self::Reg(general_register32) => *general_register32 as _,
            Self::WZR => NICHE_REG.into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reg64_from_u8() {
        assert_eq!(Reg64::try_from(2), Ok(Reg64::X2));
    }

    #[test]
    fn test_reg64_from_u8_invalid() {
        assert!(Reg64::try_from(NICHE_REG).is_err());
    }

    #[test]
    fn test_reg_or_sp64_from_u8() {
        assert_eq!(RegOrSp64::try_from(2), Ok(RegOrSp64::Reg(Reg64::X2)));
    }

    #[test]
    fn test_reg_or_sp64_from_u8_sp() {
        assert_eq!(RegOrSp64::try_from(NICHE_REG), Ok(RegOrSp64::SP));
    }

    #[test]
    fn test_reg64_or_sp64_from_u8_invalid() {
        assert!(RegOrSp64::try_from(NICHE_REG + 1).is_err());
    }

    #[test]
    fn test_reg_or_zero64_from_u8() {
        assert_eq!(RegOrZero64::try_from(2), Ok(RegOrZero64::Reg(Reg64::X2)));
    }

    #[test]
    fn test_reg_or_zero64_from_u8_zero() {
        assert_eq!(RegOrZero64::try_from(NICHE_REG), Ok(RegOrZero64::XZR));
    }

    #[test]
    fn test_reg64_or_zero64_from_u8_invalid() {
        assert!(RegOrZero64::try_from(NICHE_REG + 1).is_err());
    }

    #[test]
    fn test_reg32_from_u8() {
        assert_eq!(Reg32::try_from(2), Ok(Reg32::W2));
    }

    #[test]
    fn test_reg32_from_u8_invalid() {
        assert!(Reg32::try_from(NICHE_REG).is_err());
    }

    #[test]
    fn test_reg_or_sp32_from_u8() {
        assert_eq!(RegOrSp32::try_from(2), Ok(RegOrSp32::Reg(Reg32::W2)));
    }

    #[test]
    fn test_reg_or_sp32_from_u8_sp() {
        assert_eq!(RegOrSp32::try_from(NICHE_REG), Ok(RegOrSp32::WSP));
    }

    #[test]
    fn test_reg32_or_sp32_from_u8_invalid() {
        assert!(RegOrSp32::try_from(NICHE_REG + 1).is_err());
    }

    #[test]
    fn test_reg_or_zero32_from_u8() {
        assert_eq!(RegOrZero32::try_from(2), Ok(RegOrZero32::Reg(Reg32::W2)));
    }

    #[test]
    fn test_reg_or_zero32_from_u8_zero() {
        assert_eq!(RegOrZero32::try_from(NICHE_REG), Ok(RegOrZero32::WZR));
    }

    #[test]
    fn test_reg32_or_zero32_from_u8_invalid() {
        assert!(RegOrZero32::try_from(NICHE_REG + 1).is_err());
    }
}
