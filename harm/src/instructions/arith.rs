/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use crate::{
    bits::UBitValue,
    register::{Reg32, Reg64, RegOrZero32, RegOrZero64},
};

macro_rules! define_arith_shift {
    ($name:ident, $bits:expr, $cmd:ident, $reg:ty, $ztype:ty) => {
        ::paste::paste! {
            impl $name<$reg, $reg, $reg> {
                #[inline]
                pub fn shift(self, mode: ShiftMode, amount: u8) -> $name<$ztype, $ztype, ShiftedReg<$ztype>> {
                    $name::new(
                        $ztype::Reg(self.dst),
                        $ztype::Reg(self.src1),
                        ShiftedReg::new($ztype::Reg(self.src2)),
                    )
                        .shift(mode, amount)
                }
            }

            impl RawInstruction for $name<$reg, $reg, $reg> {
                #[inline]
                fn to_code(&self) -> InstructionCode {
                    $name {
                        dst: $ztype::Reg(self.dst),
                        src1: $ztype::Reg(self.src1),
                        src2: ShiftedReg::new($ztype::Reg(self.src2)),
                    }
                    .to_code()
                }
            }

            impl<Src1, Src2> [<Make $name>]<$ztype, Src1, Src2> for $name<$ztype, $ztype, $ztype>
            where
                Src1: Into<$ztype>,
                Src2: Into<$ztype>,
            {
                type Output = Self;

                #[inline]
                fn new(dst: $ztype, src1: Src1, src2: Src2) -> Self {
                    Self { dst, src1: src1.into(), src2: src2.into() }
                }
            }

            impl $name<$ztype, $ztype, $ztype> {
                #[inline]
                pub fn shift(self, mode: ShiftMode, amount: u8) -> $name<$ztype, $ztype, ShiftedReg<$ztype>> {
                    $name::new(self.dst, self.src1, ShiftedReg::new(self.src2))
                        .shift(mode, amount)
                }
            }

            impl<Dst, Src1> [<Make $name>]<Dst, Src1, ShiftedReg<$ztype>> for $name<$ztype, $ztype, ShiftedReg<$ztype>>
            where
                Dst: Into<$ztype>,
                Src1: Into<$ztype>,
            {
                type Output = Self;

                #[inline]
                fn new(
                    dst: Dst,
                    src1: Src1,
                    src2: ShiftedReg<$ztype>,
                ) -> Self {
                    Self { dst: dst.into(), src1: src1.into(), src2 }
                }
            }

            impl $name<$ztype, $ztype, ShiftedReg<$ztype>> {
                #[inline]
                pub fn shift(mut self, mode: ShiftMode, amount: u8) -> Self {
                    self.src2.shift = Shift { mode, amount };
                    self
                }
            }

            impl RawInstruction for $name<$ztype, $ztype, ShiftedReg<$ztype>> {
                #[inline]
                fn to_code(&self) -> InstructionCode {
                    let shift = self.src2.shift.mode as u8;
                    let rm = self.src2.reg.code();
                    let shift_amount_imm6 = self.src2.shift.amount;
                    let rn = self.src1.code();
                    let rd = self.dst.code();

                    [<$name:upper _ $bits _ $cmd _shift>](
                        shift.into(),
                        rm.into(),
                        shift_amount_imm6.into(),
                        rn.into(),
                        rd.into(),
                    )
                }
            }
        }
    }
}

// TODO instead of u32, use Or<UBitValue<12>, UBitValue<12, 12>>.
macro_rules! define_arith_imm12 {
    ($name:ident, $bits:expr, $cmd:ident, $reg:ty, $etype:ty) => {
        ::paste::paste! {
            impl<Src> [<Make $name>]<$reg, Src, u32> for $name<$etype, $etype, u32>
            where
                Src: Into<$etype>
            {
                type Output = Result<Self, Error>;

                #[inline]
                fn new(dst: $reg, src1: Src, src2: u32) -> Result<Self, Error> {
                    let imm12 = $crate::instructions::arith::validate_imm12(src2)?;
                    Ok(Self {
                        dst: dst.into(),
                        src1: src1.into(),
                        src2: imm12,
                    })
                }
            }

            impl<Src> [<Make $name>]<$etype, Src, u32> for $name<$etype, $etype, u32>
            where
                Src: Into<$etype>,
            {
                type Output = Result<Self, Error>;

                #[inline]
                fn new(dst: $etype, src1: Src, src2: u32) -> Result<Self, Error> {
                    let imm12 = $crate::instructions::arith::validate_imm12(src2)?;
                    Ok(Self {
                        dst,
                        src1: src1.into(),
                        src2: imm12,
                    })
                }
            }

            impl RawInstruction for $name<$etype, $etype, u32> {
                #[inline]
                fn to_code(&self) -> InstructionCode {
                    let shift = self.src2 & ((1 << 12) - 1) == 0;
                    let imm12 = if shift { self.src2 >> 12 } else { self.src2 };
                    assert!(imm12 < (1 << 12));
                    let rn = self.src1.code();
                    let rd = self.dst.code();

                    [<$name:upper _ $bits _ $cmd _imm>](shift.into(), imm12.into(), rn.into(), rd.into())
                }
            }
        }
    }
}

macro_rules! define_arith_extend {
    ($name:ident, $bits:expr, $cmd:ident, $reg:ty, $stype:ty, $ztype:ty) => {
        ::paste::paste! {
            impl $name<$reg, $reg, $reg> {
                #[inline]
                pub fn extend(
                    self,
                    mode: ExtendMode,
                    amount: u8,
                ) -> $name<$stype, $stype, ExtendedReg<$ztype>> {
                    $name::new(
                        <$stype>::Reg(self.dst),
                        <$stype>::Reg(self.src1),
                        ExtendedReg::new(<$ztype>::Reg(self.src2)),
                    )
                    .extend(mode, amount)
                }
            }

            impl<Src1, Src2> [<Make $name>]<$stype, Src1, Src2> for $name<$stype, $stype, $ztype>
            where Src1: Into<$stype>,
                  Src2: Into<$ztype>
            {
                type Output = Self;

                #[inline]
                fn new(dst: $stype, src1: Src1, src2: Src2) -> Self {
                    Self { dst, src1: src1.into(), src2: src2.into() }
                }
            }

            impl $name<$stype, $stype, $reg> {
                #[inline]
                pub fn extend(
                    self,
                    mode: ExtendMode,
                    amount: u8,
                ) -> $name<$stype, $stype, ExtendedReg<$ztype>> {
                    $name::new(self.dst, self.src1, ExtendedReg::new(self.src2.into()))
                        .extend(mode, amount)
                }
            }

            impl [<Make $name>]<$stype, $stype, ExtendedReg<$ztype>>
                for $name<$stype, $stype, ExtendedReg<$ztype>>
            {
                type Output = Self;

                #[inline]
                fn new(
                    dst: $stype,
                    src1: $stype,
                    src2: ExtendedReg<$ztype>,
                ) -> Self {
                    Self { dst, src1, src2 }
                }
            }

            impl $name<$stype, $stype, ExtendedReg<$ztype>> {
                #[inline]
                pub fn extend(mut self, mode: ExtendMode, amount: u8) -> Self {
                    self.src2.extend = Extend { mode, amount };
                    self
                }
            }

            impl $name<$stype, $stype, $ztype> {
                #[inline]
                pub fn extend(self, mode: ExtendMode, amount: u8) -> $name<$stype, $stype, ExtendedReg<$ztype>> {
                    $name::new(self.dst, self.src1, ExtendedReg::new(self.src2))
                        .extend(mode, amount)
                }
            }

            impl RawInstruction for $name<$stype, $stype, ExtendedReg<$ztype>> {
                #[inline]
                fn to_code(&self) -> InstructionCode {
                    let option = self.src2.extend.mode as u8;
                    let rm = self.src2.reg.code();
                    let imm3 = self.src2.extend.amount;
                    let rn = self.src1.code();
                    let rd = self.dst.code();

                    [<$name:upper _ $bits _ $cmd _ext>](
                        rm.into(), option.into(), imm3.into(), rn.into(), rd.into()
                    )
                }
            }
        }
    };
}

// These modules have to be defined after the macros
pub mod add;
pub mod sub;

// TODO: proper error type
pub type Error = &'static str;

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

    pub fn shift(mut self, mode: ShiftMode, amount: u8) -> Self {
        self.shift = Shift { mode, amount };
        self
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct Shift {
    mode: ShiftMode,
    amount: u8,
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

impl<T> ExtendedReg<T> {
    pub fn new(reg: T) -> Self {
        Self {
            reg,
            extend: <_>::default(),
        }
    }

    pub fn extend(mut self, mode: ExtendMode, amount: u8) -> Self {
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
    amount: u8,
}

// TODO: remove in favor of type-specific impls.
pub(crate) const fn validate_imm12(imm12: u32) -> Result<u32, Error> {
    let shifted = UBitValue::<12, 12>::new(imm12);
    let unshifted = UBitValue::<12, 0>::new(imm12);
    if shifted.is_ok() || unshifted.is_ok() {
        Ok(imm12)
    } else {
        Err("Immediate value out of range for an arithmetic instruction")
    }
}
