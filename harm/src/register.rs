/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

#![allow(clippy::upper_case_acronyms)]

const NICHE_REG: u8 = 31;

use aarchmrs_types::BitValue;
use num_enum::TryFromPrimitive;

use crate::sealed::Sealed;

pub trait Register: Sealed {
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

impl Reg64 {
    #[inline]
    pub fn narrow(self) -> Reg32 {
        match self {
            Reg64::X0 => Reg32::W0,
            Reg64::X1 => Reg32::W1,
            Reg64::X2 => Reg32::W2,
            Reg64::X3 => Reg32::W3,
            Reg64::X4 => Reg32::W4,
            Reg64::X5 => Reg32::W5,
            Reg64::X6 => Reg32::W6,
            Reg64::X7 => Reg32::W7,
            Reg64::X8 => Reg32::W8,
            Reg64::X9 => Reg32::W9,
            Reg64::X10 => Reg32::W10,
            Reg64::X11 => Reg32::W11,
            Reg64::X12 => Reg32::W12,
            Reg64::X13 => Reg32::W13,
            Reg64::X14 => Reg32::W14,
            Reg64::X15 => Reg32::W15,
            Reg64::X16 => Reg32::W16,
            Reg64::X17 => Reg32::W17,
            Reg64::X18 => Reg32::W18,
            Reg64::X19 => Reg32::W19,
            Reg64::X20 => Reg32::W20,
            Reg64::X21 => Reg32::W21,
            Reg64::X22 => Reg32::W22,
            Reg64::X23 => Reg32::W23,
            Reg64::X24 => Reg32::W24,
            Reg64::X25 => Reg32::W25,
            Reg64::X26 => Reg32::W26,
            Reg64::X27 => Reg32::W27,
            Reg64::X28 => Reg32::W28,
            Reg64::X29 => Reg32::W29,
            Reg64::LR => Reg32::WLR,
        }
    }
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

impl RegOrSp64 {
    #[inline]
    pub fn narrow(self) -> RegOrSp32 {
        match self {
            RegOrSp64::Reg(reg64) => RegOrSp32::Reg(reg64.narrow()),
            RegOrSp64::SP => RegOrSp32::WSP,
        }
    }
}

impl RegOrZero64 {
    #[inline]
    pub fn narrow(self) -> RegOrZero32 {
        match self {
            RegOrZero64::Reg(reg64) => RegOrZero32::Reg(reg64.narrow()),
            RegOrZero64::XZR => RegOrZero32::WZR,
        }
    }
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
        Reg64::try_from(value).map(Into::into).or({
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
        Reg64::try_from(value).map(Into::into).or({
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

impl Reg32 {
    #[inline]
    pub fn extend(self) -> Reg64 {
        match self {
            Reg32::W0 => Reg64::X0,
            Reg32::W1 => Reg64::X1,
            Reg32::W2 => Reg64::X2,
            Reg32::W3 => Reg64::X3,
            Reg32::W4 => Reg64::X4,
            Reg32::W5 => Reg64::X5,
            Reg32::W6 => Reg64::X6,
            Reg32::W7 => Reg64::X7,
            Reg32::W8 => Reg64::X8,
            Reg32::W9 => Reg64::X9,
            Reg32::W10 => Reg64::X10,
            Reg32::W11 => Reg64::X11,
            Reg32::W12 => Reg64::X12,
            Reg32::W13 => Reg64::X13,
            Reg32::W14 => Reg64::X14,
            Reg32::W15 => Reg64::X15,
            Reg32::W16 => Reg64::X16,
            Reg32::W17 => Reg64::X17,
            Reg32::W18 => Reg64::X18,
            Reg32::W19 => Reg64::X19,
            Reg32::W20 => Reg64::X20,
            Reg32::W21 => Reg64::X21,
            Reg32::W22 => Reg64::X22,
            Reg32::W23 => Reg64::X23,
            Reg32::W24 => Reg64::X24,
            Reg32::W25 => Reg64::X25,
            Reg32::W26 => Reg64::X26,
            Reg32::W27 => Reg64::X27,
            Reg32::W28 => Reg64::X28,
            Reg32::W29 => Reg64::X29,
            Reg32::WLR => Reg64::LR,
        }
    }
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

impl RegOrSp32 {
    #[inline]
    pub fn extend(self) -> RegOrSp64 {
        match self {
            RegOrSp32::Reg(reg32) => RegOrSp64::Reg(reg32.extend()),
            RegOrSp32::WSP => RegOrSp64::SP,
        }
    }
}

impl RegOrZero32 {
    #[inline]
    pub fn extend(self) -> RegOrZero64 {
        match self {
            RegOrZero32::Reg(reg32) => RegOrZero64::Reg(reg32.extend()),
            RegOrZero32::WZR => RegOrZero64::XZR,
        }
    }
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
        Reg32::try_from(value).map(Into::into).or({
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
        Reg32::try_from(value).map(Into::into).or({
            if value == NICHE_REG {
                Ok(Self::WZR)
            } else {
                Err(RegisterError::InvalidRegisterCode(value))
            }
        })
    }
}

impl Sealed for Reg64 {}
impl Sealed for RegOrSp64 {}
impl Sealed for RegOrZero64 {}
impl Sealed for Reg32 {}
impl Sealed for RegOrSp32 {}
impl Sealed for RegOrZero32 {}

impl Register for Reg64 {
    #[inline]
    fn code(&self) -> BitValue<5> {
        BitValue::new_u32(*self as u32)
    }
}

impl Register for Reg32 {
    #[inline]
    fn code(&self) -> BitValue<5> {
        BitValue::new_u32(*self as u32)
    }
}

impl Register for RegOrSp64 {
    #[inline]
    fn code(&self) -> BitValue<5> {
        BitValue::new_u32(match self {
            Self::Reg(general_register64) => *general_register64 as _,
            Self::SP => NICHE_REG.into(),
        })
    }
}

impl Register for RegOrZero64 {
    #[inline]
    fn code(&self) -> BitValue<5> {
        BitValue::new_u32(match self {
            Self::Reg(general_register64) => *general_register64 as _,
            Self::XZR => NICHE_REG.into(),
        })
    }
}

impl Register for RegOrSp32 {
    #[inline]
    fn code(&self) -> BitValue<5> {
        BitValue::new_u32(match self {
            Self::Reg(general_register32) => *general_register32 as _,
            Self::WSP => NICHE_REG.into(),
        })
    }
}

impl Register for RegOrZero32 {
    #[inline]
    fn code(&self) -> BitValue<5> {
        BitValue::new_u32(match self {
            Self::Reg(general_register32) => *general_register32 as _,
            Self::WZR => NICHE_REG.into(),
        })
    }
}

pub trait IntoReg<X>: Sealed {
    fn into_reg(self) -> X;
}

impl IntoReg<RegOrSp64> for Reg64 {
    #[inline]
    fn into_reg(self) -> RegOrSp64 {
        RegOrSp64::Reg(self)
    }
}

impl IntoReg<RegOrSp64> for RegOrSp64 {
    #[inline]
    fn into_reg(self) -> RegOrSp64 {
        self
    }
}

impl IntoReg<RegOrZero64> for Reg64 {
    #[inline]
    fn into_reg(self) -> RegOrZero64 {
        RegOrZero64::Reg(self)
    }
}

impl IntoReg<RegOrZero64> for RegOrZero64 {
    #[inline]
    fn into_reg(self) -> RegOrZero64 {
        self
    }
}

impl IntoReg<RegOrSp32> for Reg32 {
    #[inline]
    fn into_reg(self) -> RegOrSp32 {
        RegOrSp32::Reg(self)
    }
}

impl IntoReg<RegOrSp32> for RegOrSp32 {
    #[inline]
    fn into_reg(self) -> RegOrSp32 {
        self
    }
}

impl IntoReg<RegOrZero32> for Reg32 {
    #[inline]
    fn into_reg(self) -> RegOrZero32 {
        RegOrZero32::Reg(self)
    }
}

impl IntoReg<RegOrZero32> for RegOrZero32 {
    #[inline]
    fn into_reg(self) -> RegOrZero32 {
        self
    }
}

pub trait Zero: Sealed {
    const ZERO: Self;
}

impl Zero for RegOrZero32 {
    const ZERO: Self = Self::WZR;
}

impl Zero for RegOrZero64 {
    const ZERO: Self = Self::XZR;
}

#[cfg(test)]
mod tests {
    use super::*;
    use Reg32::*;
    use Reg64::*;
    use RegOrSp32::WSP;
    use RegOrSp64::SP;
    use RegOrZero32::WZR;
    use RegOrZero64::XZR;

    #[test]
    fn test_reg64_from_u8() {
        assert_eq!(Reg64::try_from(2), Ok(X2));
    }

    #[test]
    fn test_reg64_from_u8_invalid() {
        assert!(Reg64::try_from(NICHE_REG).is_err());
    }

    #[test]
    fn test_reg_or_sp64_from_u8() {
        assert_eq!(RegOrSp64::try_from(2), Ok(RegOrSp64::Reg(X2)));
    }

    #[test]
    fn test_reg_or_sp64_from_u8_sp() {
        assert_eq!(RegOrSp64::try_from(NICHE_REG), Ok(SP));
    }

    #[test]
    fn test_reg64_or_sp64_from_u8_invalid() {
        assert!(RegOrSp64::try_from(NICHE_REG + 1).is_err());
    }

    #[test]
    fn test_reg_or_zero64_from_u8() {
        assert_eq!(RegOrZero64::try_from(2), Ok(RegOrZero64::Reg(X2)));
    }

    #[test]
    fn test_reg_or_zero64_from_u8_zero() {
        assert_eq!(RegOrZero64::try_from(NICHE_REG), Ok(XZR));
    }

    #[test]
    fn test_reg64_or_zero64_from_u8_invalid() {
        assert!(RegOrZero64::try_from(NICHE_REG + 1).is_err());
    }

    #[test]
    fn test_reg32_from_u8() {
        assert_eq!(Reg32::try_from(2), Ok(W2));
    }

    #[test]
    fn test_reg32_from_u8_invalid() {
        assert!(Reg32::try_from(NICHE_REG).is_err());
    }

    #[test]
    fn test_reg_or_sp32_from_u8() {
        assert_eq!(RegOrSp32::try_from(2), Ok(RegOrSp32::Reg(W2)));
    }

    #[test]
    fn test_reg_or_sp32_from_u8_sp() {
        assert_eq!(RegOrSp32::try_from(NICHE_REG), Ok(WSP));
    }

    #[test]
    fn test_reg32_or_sp32_from_u8_invalid() {
        assert!(RegOrSp32::try_from(NICHE_REG + 1).is_err());
    }

    #[test]
    fn test_reg_or_zero32_from_u8() {
        assert_eq!(RegOrZero32::try_from(2), Ok(RegOrZero32::Reg(W2)));
    }

    #[test]
    fn test_reg_or_zero32_from_u8_zero() {
        assert_eq!(RegOrZero32::try_from(NICHE_REG), Ok(WZR));
    }

    #[test]
    fn test_reg32_or_zero32_from_u8_invalid() {
        assert!(RegOrZero32::try_from(NICHE_REG + 1).is_err());
    }

    #[test]
    fn test_narrow_reg64() {
        assert_eq!(X8.narrow(), W8);
    }

    #[test]
    fn test_narrow_reg64_s() {
        assert_eq!(RegOrSp64::Reg(X8).narrow(), RegOrSp32::Reg(W8));
    }

    #[test]
    fn test_narrow_reg64_z() {
        assert_eq!(RegOrZero64::Reg(X8).narrow(), RegOrZero32::Reg(W8));
    }

    #[test]
    fn test_narrow_sp() {
        assert_eq!(SP.narrow(), WSP);
    }

    #[test]
    fn test_narrow_zero() {
        assert_eq!(XZR.narrow(), WZR);
    }

    #[test]
    fn test_extend_reg32() {
        assert_eq!(W8.extend(), X8);
    }

    #[test]
    fn test_extend_reg32_s() {
        assert_eq!(RegOrSp32::Reg(W8).extend(), RegOrSp64::Reg(X8));
    }

    //
    #[test]
    fn test_extend_reg64_z() {
        assert_eq!(RegOrZero32::Reg(W8).extend(), RegOrZero64::Reg(X8));
    }

    #[test]
    fn test_extend_sp() {
        assert_eq!(WSP.extend(), SP);
    }

    #[test]
    fn test_extend_zero() {
        assert_eq!(WZR.extend(), XZR);
    }
}
