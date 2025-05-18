/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

#![allow(clippy::upper_case_acronyms)]

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
    fn from(value: Reg64) -> Self {
        Self::Reg(value)
    }
}

impl From<Reg64> for RegOrZero64 {
    fn from(value: Reg64) -> Self {
        Self::Reg(value)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

pub trait IntoCode {
    fn code(&self) -> u8;
}

impl IntoCode for RegOrSp64 {
    #[inline]
    fn code(&self) -> u8 {
        match self {
            Self::Reg(general_register64) => *general_register64 as _,
            Self::SP => 31,
        }
    }
}

impl IntoCode for RegOrZero64 {
    #[inline]
    fn code(&self) -> u8 {
        match self {
            Self::Reg(general_register64) => *general_register64 as _,
            Self::XZR => 31,
        }
    }
}

impl IntoCode for RegOrSp32 {
    #[inline]
    fn code(&self) -> u8 {
        match self {
            Self::Reg(general_register32) => *general_register32 as _,
            Self::WSP => 31,
        }
    }
}

impl IntoCode for RegOrZero32 {
    #[inline]
    fn code(&self) -> u8 {
        match self {
            Self::Reg(general_register32) => *general_register32 as _,
            Self::WZR => 31,
        }
    }
}
