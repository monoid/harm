/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use crate::sealed::Sealed;
pub use harm_types::A64::register::*;

impl Sealed for Reg64 {}
impl Sealed for RegOrSp64 {}
impl Sealed for RegOrZero64 {}
impl Sealed for Reg32 {}
impl Sealed for RegOrSp32 {}
impl Sealed for RegOrZero32 {}

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
