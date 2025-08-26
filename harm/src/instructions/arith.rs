/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use crate::{
    bits::{BitError, UBitValue},
    register::{Reg32, Reg64, RegOrZero32, RegOrZero64},
};

macro_rules! define_arith_faillible {
    ($name:ident) => {
        ::paste::paste! {
            impl<Dst, RealDst, Src1, RealSrc1, Src2, Err> [<Make $name>]<Dst, Src1, Result<Src2, Err>>
                for $name<RealDst, RealSrc1, Src2>
                where
                    $name<RealDst, RealSrc1, Src2>: [<Make $name>]<Dst, Src1, Src2>
            {
                type Output = Result<<$name<RealDst, RealSrc1, Src2> as [<Make $name>]<Dst, Src1, Src2>>::Output, Err>;

                fn new(dst: Dst, src1: Src1, src2: Result<Src2, Err>) -> Self::Output {
                    src2.map(
                        |src2| <$name<RealDst, RealSrc1, Src2> as [<Make $name>]<Dst, Src1, Src2>>::new(
                            dst, src1, src2
                        )
                    )
                }
            }
        }
    };
}

macro_rules! define_arith_shift {
    ($name:ident, $bits:expr, $cmd:ident, $reg:ty, $ztype:ty) => {
        ::paste::paste! {
            impl $name<$reg, $reg, $reg> {
                #[inline]
                pub fn shift(self, mode: ShiftMode, amount: ShiftAmount) -> $name<$ztype, $ztype, ShiftedReg<$ztype>> {
                    $name::new(
                        $ztype::Reg(self.dst),
                        $ztype::Reg(self.src1),
                        ShiftedReg::new($ztype::Reg(self.src2)),
                    )
                        .shift(mode, amount)
                }
                #[inline]
                pub fn try_shift(self, mode: ShiftMode, amount: u32)
                                 -> Result<$name<$ztype, $ztype, ShiftedReg<$ztype>>, BitError> {
                    $name::new(
                        $ztype::Reg(self.dst),
                        $ztype::Reg(self.src1),
                        ShiftedReg::new($ztype::Reg(self.src2)),
                    )
                        .try_shift(mode, amount)
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
                pub fn shift(self, mode: ShiftMode, amount: ShiftAmount) -> $name<$ztype, $ztype, ShiftedReg<$ztype>> {
                    $name::new(self.dst, self.src1, ShiftedReg::new(self.src2))
                        .shift(mode, amount)
                }
                pub fn try_shift(self, mode: ShiftMode, amount: u32)
                                 -> Result<$name<$ztype, $ztype, ShiftedReg<$ztype>>, BitError> {
                    $name::new(self.dst, self.src1, ShiftedReg::new(self.src2))
                        .try_shift(mode, amount)
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

            impl<Dst, Src1, Src2> [<Make $name>]<Dst, Src1, (Src2, ShiftMode, ShiftAmount)> for $name<$ztype, $ztype, ShiftedReg<$ztype>>
            where
                Dst: Into<$ztype>,
                Src1: Into<$ztype>,
                Src2: Into<$ztype>,
            {
                type Output = Self;

                #[inline]
                fn new(
                    dst: Dst,
                    src1: Src1,
                    (src2, shift_mode, shift_amount): (Src2, ShiftMode, ShiftAmount)
                ) -> Self {
                    let src2 = ShiftedReg::new(src2.into()).shift(shift_mode, shift_amount);
                    Self { dst: dst.into(), src1: src1.into(), src2 }
                }
            }

            impl<Dst, Src1, Src2> [<Make $name>]<Dst, Src1, (Src2, ShiftMode, u32)> for $name<$ztype, $ztype, ShiftedReg<$ztype>>
            where
                Dst: Into<$ztype>,
                Src1: Into<$ztype>,
                Src2: Into<$ztype>,
            {
                type Output = Result<Self, BitError>;

                #[inline]
                fn new(
                    dst: Dst,
                    src1: Src1,
                    (src2, shift_mode, shift_amount): (Src2, ShiftMode, u32)
                ) -> Result<Self, BitError> {
                    let shift_amount = shift_amount.try_into()?;
                    let src2 = ShiftedReg::new(src2.into()).shift(shift_mode, shift_amount);
                    Ok(Self { dst: dst.into(), src1: src1.into(), src2 })
                }
            }

            impl $name<$ztype, $ztype, ShiftedReg<$ztype>> {
                #[inline]
                pub fn shift(mut self, mode: ShiftMode, amount: ShiftAmount) -> Self {
                    self.src2.shift = Shift { mode, amount };
                    self
                }

                pub fn try_shift(mut self, mode: ShiftMode, amount: u32) -> Result<Self, BitError> {
                    let amount = amount.try_into()?;
                    self.src2.shift = Shift { mode, amount };
                    Ok(self)
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
            impl<Dst, Src> [<Make $name>]<Dst, Src, u32>
                for $name<$etype, $etype, $crate::instructions::arith::AddSubImm12>
            where
                Dst: Into<$etype>,
                Src: Into<$etype>,
            {
                type Output = Result<Self, (BitError, BitError)>;

                #[inline]
                fn new(dst: Dst, src1: Src, src2: u32) -> Self::Output {
                    let imm12 = $crate::instructions::arith::AddSubImm12::try_from(src2)?;
                    Ok(Self {
                        dst: dst.into(),
                        src1: src1.into(),
                        src2: imm12,
                    })
                }
            }

            impl<Dst, Src1, Src2> [<Make $name>]<Dst, Src1, Src2>
                for $name<$etype, $etype, $crate::instructions::arith::AddSubImm12>
            where
                Dst: Into<$etype>,
                Src1: Into<$etype>,
                Src2: Into<$crate::instructions::arith::AddSubImm12>,
            {
                type Output = Self;

                #[inline]
                fn new(dst: Dst, src1: Src1, src2: Src2) -> Self::Output {
                    Self {
                        dst: dst.into(),
                        src1: src1.into(),
                        src2: src2.into(),
                    }
                }
            }

            impl RawInstruction for $name<$etype, $etype, $crate::instructions::arith::AddSubImm12> {
                #[inline]
                fn to_code(&self) -> InstructionCode {
                    use $crate::instructions::arith::AddSubImm12::*;
                    let (shifted, imm12) = match self.src2 {
                        Unshifted(value) => (false, value.into()),
                        Shifted(value) => (true, value.into()),
                    };
                    let rn = self.src1.code();
                    let rd = self.dst.code();

                    [<$name:upper _ $bits _ $cmd _imm>](shifted.into(), imm12, rn.into(), rd.into())
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

            impl<Dst, Src1, Src2> [<Make $name>]<Dst, Src1, (Src2, ExtendMode)>
                for $name<$stype, $stype, ExtendedReg<$ztype>>
                where
                    Dst: Into<$stype>,
                    Src1: Into<$stype>,
                    Src2: Into<$ztype>,
            {
                type Output = Self;

                #[inline]
                fn new(
                    dst: Dst,
                    src1: Src1,
                    (src2, mode): (Src2, ExtendMode),
                ) -> Self {
                    let src2 = ExtendedReg::new(src2.into()).extend(mode, <_>::default());
                    Self { dst: dst.into(), src1: src1.into(), src2 }
                }
            }

            impl<Dst, Src1, Src2> [<Make $name>]<Dst, Src1, (Src2, ExtendMode, u8)>
                for $name<$stype, $stype, ExtendedReg<$ztype>>
                where
                    Dst: Into<$stype>,
                    Src1: Into<$stype>,
                    Src2: Into<$ztype>,
            {
                type Output = Self;

                #[inline]
                fn new(
                    dst: Dst,
                    src1: Src1,
                    (src2, mode, offset): (Src2, ExtendMode, u8),
                ) -> Self {
                    let src2 = ExtendedReg::new(src2.into()).extend(mode, offset);
                    Self { dst: dst.into(), src1: src1.into(), src2 }
                }
            }
        }
    };
}

// These modules have to be defined after the macros
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
