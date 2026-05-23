/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use crate::{
    bits::BitError,
    outcome::{Outcome, Unfallible},
    register::{IntoReg, Reg32, Reg64, RegOrSp32, RegOrSp64, RegOrZero32, RegOrZero64},
    sealed::Sealed,
};

use super::{
    AddSubImm12, ExtendError, ExtendMode, ExtendShiftAmount, ExtendedReg, ShiftAmount, ShiftMode,
    ShiftedReg,
};

#[derive(Debug, Copy, Clone)]
pub struct ArithArgs<T, S1, S2> {
    pub dst: T,
    pub src1: S1,
    pub src2: S2,
}

impl<T, S1, S2> Sealed for ArithArgs<T, S1, S2> {}

pub trait MakeArithArgs<DstIn, Src1In, Src2In>: Sealed {
    type Outcome: Outcome<Inner = Self>;
    fn new(dst: DstIn, src1: Src1In, src2: Src2In) -> Self::Outcome;
}

// --- Basic register-register-register impls ---

impl MakeArithArgs<Reg64, Reg64, Reg64> for ArithArgs<Reg64, Reg64, Reg64> {
    type Outcome = Unfallible<Self>;

    #[inline]
    fn new(dst: Reg64, src1: Reg64, src2: Reg64) -> Self::Outcome {
        Unfallible(Self { dst, src1, src2 })
    }
}

impl MakeArithArgs<Reg32, Reg32, Reg32> for ArithArgs<Reg32, Reg32, Reg32> {
    type Outcome = Unfallible<Self>;

    #[inline]
    fn new(dst: Reg32, src1: Reg32, src2: Reg32) -> Self::Outcome {
        Unfallible(Self { dst, src1, src2 })
    }
}

// --- Fallible wrappers: lift MakeArithArgs<D, S1, ShiftedReg<_>> to handle Result<ShiftedReg<_>, BitError> ---

impl<Dst, Src1> MakeArithArgs<Dst, Src1, Result<ShiftedReg<RegOrZero64>, BitError>>
    for ArithArgs<RegOrZero64, RegOrZero64, ShiftedReg<RegOrZero64>>
where
    Dst: IntoReg<RegOrZero64>,
    Src1: IntoReg<RegOrZero64>,
{
    type Outcome = Result<Self, BitError>;

    #[inline]
    fn new(dst: Dst, src1: Src1, src2: Result<ShiftedReg<RegOrZero64>, BitError>) -> Self::Outcome {
        src2.map(|s2| Self {
            dst: dst.into_reg(),
            src1: src1.into_reg(),
            src2: s2,
        })
    }
}

impl<Dst, Src1> MakeArithArgs<Dst, Src1, Result<ShiftedReg<RegOrZero32>, BitError>>
    for ArithArgs<RegOrZero32, RegOrZero32, ShiftedReg<RegOrZero32>>
where
    Dst: IntoReg<RegOrZero32>,
    Src1: IntoReg<RegOrZero32>,
{
    type Outcome = Result<Self, BitError>;

    #[inline]
    fn new(dst: Dst, src1: Src1, src2: Result<ShiftedReg<RegOrZero32>, BitError>) -> Self::Outcome {
        src2.map(|s2| Self {
            dst: dst.into_reg(),
            src1: src1.into_reg(),
            src2: s2,
        })
    }
}

// --- Fallible wrappers: lift MakeArithArgs<D, S1, AddSubImm12> to handle Result<AddSubImm12, _> ---

impl<Dst, Src1> MakeArithArgs<Dst, Src1, Result<AddSubImm12, (BitError, BitError)>>
    for ArithArgs<RegOrSp64, RegOrSp64, AddSubImm12>
where
    Dst: IntoReg<RegOrSp64>,
    Src1: IntoReg<RegOrSp64>,
{
    type Outcome = Result<Self, (BitError, BitError)>;

    #[inline]
    fn new(
        dst: Dst,
        src1: Src1,
        src2: Result<AddSubImm12, (BitError, BitError)>,
    ) -> Self::Outcome {
        src2.map(|s2| Self {
            dst: dst.into_reg(),
            src1: src1.into_reg(),
            src2: s2,
        })
    }
}

impl<Dst, Src1> MakeArithArgs<Dst, Src1, Result<AddSubImm12, (BitError, BitError)>>
    for ArithArgs<RegOrSp32, RegOrSp32, AddSubImm12>
where
    Dst: IntoReg<RegOrSp32>,
    Src1: IntoReg<RegOrSp32>,
{
    type Outcome = Result<Self, (BitError, BitError)>;

    #[inline]
    fn new(
        dst: Dst,
        src1: Src1,
        src2: Result<AddSubImm12, (BitError, BitError)>,
    ) -> Self::Outcome {
        src2.map(|s2| Self {
            dst: dst.into_reg(),
            src1: src1.into_reg(),
            src2: s2,
        })
    }
}

// --- Shift impls (64-bit) ---

impl<Src1, Src2> MakeArithArgs<RegOrZero64, Src1, Src2>
    for ArithArgs<RegOrZero64, RegOrZero64, RegOrZero64>
where
    Src1: IntoReg<RegOrZero64>,
    Src2: IntoReg<RegOrZero64>,
{
    type Outcome = Unfallible<Self>;

    #[inline]
    fn new(dst: RegOrZero64, src1: Src1, src2: Src2) -> Self::Outcome {
        Unfallible(Self {
            dst,
            src1: src1.into_reg(),
            src2: src2.into_reg(),
        })
    }
}

impl<Dst, Src1> MakeArithArgs<Dst, Src1, ShiftedReg<RegOrZero64>>
    for ArithArgs<RegOrZero64, RegOrZero64, ShiftedReg<RegOrZero64>>
where
    Dst: IntoReg<RegOrZero64>,
    Src1: IntoReg<RegOrZero64>,
{
    type Outcome = Unfallible<Self>;

    #[inline]
    fn new(dst: Dst, src1: Src1, src2: ShiftedReg<RegOrZero64>) -> Self::Outcome {
        Unfallible(Self {
            dst: dst.into_reg(),
            src1: src1.into_reg(),
            src2,
        })
    }
}

impl<Dst, Src1, Src2> MakeArithArgs<Dst, Src1, (Src2, ShiftMode, ShiftAmount)>
    for ArithArgs<RegOrZero64, RegOrZero64, ShiftedReg<RegOrZero64>>
where
    Dst: IntoReg<RegOrZero64>,
    Src1: IntoReg<RegOrZero64>,
    Src2: IntoReg<RegOrZero64>,
{
    type Outcome = Unfallible<Self>;

    #[inline]
    fn new(dst: Dst, src1: Src1, (src2, mode, amount): (Src2, ShiftMode, ShiftAmount)) -> Self::Outcome {
        let src2 = ShiftedReg::new(src2.into_reg()).shift(mode, amount);
        Unfallible(Self {
            dst: dst.into_reg(),
            src1: src1.into_reg(),
            src2,
        })
    }
}

impl<Dst, Src1, Src2> MakeArithArgs<Dst, Src1, (Src2, ShiftMode, u32)>
    for ArithArgs<RegOrZero64, RegOrZero64, ShiftedReg<RegOrZero64>>
where
    Dst: IntoReg<RegOrZero64>,
    Src1: IntoReg<RegOrZero64>,
    Src2: IntoReg<RegOrZero64>,
{
    type Outcome = Result<Self, BitError>;

    #[inline]
    fn new(dst: Dst, src1: Src1, (src2, mode, amount): (Src2, ShiftMode, u32)) -> Result<Self, BitError> {
        let amount = amount.try_into()?;
        let src2 = ShiftedReg::new(src2.into_reg()).shift(mode, amount);
        Ok(Self {
            dst: dst.into_reg(),
            src1: src1.into_reg(),
            src2,
        })
    }
}

// --- Shift impls (32-bit) ---

impl<Src1, Src2> MakeArithArgs<RegOrZero32, Src1, Src2>
    for ArithArgs<RegOrZero32, RegOrZero32, RegOrZero32>
where
    Src1: IntoReg<RegOrZero32>,
    Src2: IntoReg<RegOrZero32>,
{
    type Outcome = Unfallible<Self>;

    #[inline]
    fn new(dst: RegOrZero32, src1: Src1, src2: Src2) -> Self::Outcome {
        Unfallible(Self {
            dst,
            src1: src1.into_reg(),
            src2: src2.into_reg(),
        })
    }
}

impl<Dst, Src1> MakeArithArgs<Dst, Src1, ShiftedReg<RegOrZero32>>
    for ArithArgs<RegOrZero32, RegOrZero32, ShiftedReg<RegOrZero32>>
where
    Dst: IntoReg<RegOrZero32>,
    Src1: IntoReg<RegOrZero32>,
{
    type Outcome = Unfallible<Self>;

    #[inline]
    fn new(dst: Dst, src1: Src1, src2: ShiftedReg<RegOrZero32>) -> Self::Outcome {
        Unfallible(Self {
            dst: dst.into_reg(),
            src1: src1.into_reg(),
            src2,
        })
    }
}

impl<Dst, Src1, Src2> MakeArithArgs<Dst, Src1, (Src2, ShiftMode, ShiftAmount)>
    for ArithArgs<RegOrZero32, RegOrZero32, ShiftedReg<RegOrZero32>>
where
    Dst: IntoReg<RegOrZero32>,
    Src1: IntoReg<RegOrZero32>,
    Src2: IntoReg<RegOrZero32>,
{
    type Outcome = Unfallible<Self>;

    #[inline]
    fn new(dst: Dst, src1: Src1, (src2, mode, amount): (Src2, ShiftMode, ShiftAmount)) -> Self::Outcome {
        let src2 = ShiftedReg::new(src2.into_reg()).shift(mode, amount);
        Unfallible(Self {
            dst: dst.into_reg(),
            src1: src1.into_reg(),
            src2,
        })
    }
}

impl<Dst, Src1, Src2> MakeArithArgs<Dst, Src1, (Src2, ShiftMode, u32)>
    for ArithArgs<RegOrZero32, RegOrZero32, ShiftedReg<RegOrZero32>>
where
    Dst: IntoReg<RegOrZero32>,
    Src1: IntoReg<RegOrZero32>,
    Src2: IntoReg<RegOrZero32>,
{
    type Outcome = Result<Self, BitError>;

    #[inline]
    fn new(dst: Dst, src1: Src1, (src2, mode, amount): (Src2, ShiftMode, u32)) -> Result<Self, BitError> {
        let amount = amount.try_into()?;
        let src2 = ShiftedReg::new(src2.into_reg()).shift(mode, amount);
        Ok(Self {
            dst: dst.into_reg(),
            src1: src1.into_reg(),
            src2,
        })
    }
}

// --- Extend impls (64-bit) ---

impl<Src1, Src2> MakeArithArgs<RegOrSp64, Src1, Src2>
    for ArithArgs<RegOrSp64, RegOrSp64, RegOrZero64>
where
    Src1: IntoReg<RegOrSp64>,
    Src2: IntoReg<RegOrZero64>,
{
    type Outcome = Unfallible<Self>;

    #[inline]
    fn new(dst: RegOrSp64, src1: Src1, src2: Src2) -> Self::Outcome {
        Unfallible(Self {
            dst,
            src1: src1.into_reg(),
            src2: src2.into_reg(),
        })
    }
}

impl MakeArithArgs<RegOrSp64, RegOrSp64, ExtendedReg<RegOrZero64>>
    for ArithArgs<RegOrSp64, RegOrSp64, ExtendedReg<RegOrZero64>>
{
    type Outcome = Unfallible<Self>;

    #[inline]
    fn new(dst: RegOrSp64, src1: RegOrSp64, src2: ExtendedReg<RegOrZero64>) -> Self::Outcome {
        Unfallible(Self { dst, src1, src2 })
    }
}

impl<Dst, Src1, Src2> MakeArithArgs<Dst, Src1, (Src2, ExtendMode)>
    for ArithArgs<RegOrSp64, RegOrSp64, ExtendedReg<RegOrZero64>>
where
    Dst: IntoReg<RegOrSp64>,
    Src1: IntoReg<RegOrSp64>,
    Src2: IntoReg<RegOrZero64>,
{
    type Outcome = Unfallible<Self>;

    #[inline]
    fn new(dst: Dst, src1: Src1, (src2, mode): (Src2, ExtendMode)) -> Self::Outcome {
        let src2 = ExtendedReg::new(src2.into_reg()).extend(mode, <_>::default());
        Unfallible(Self {
            dst: dst.into_reg(),
            src1: src1.into_reg(),
            src2,
        })
    }
}

impl<Dst, Src1, Src2> MakeArithArgs<Dst, Src1, (Src2, ExtendMode, ExtendShiftAmount)>
    for ArithArgs<RegOrSp64, RegOrSp64, ExtendedReg<RegOrZero64>>
where
    Dst: IntoReg<RegOrSp64>,
    Src1: IntoReg<RegOrSp64>,
    Src2: IntoReg<RegOrZero64>,
{
    type Outcome = Unfallible<Self>;

    #[inline]
    fn new(
        dst: Dst,
        src1: Src1,
        (src2, mode, amount): (Src2, ExtendMode, ExtendShiftAmount),
    ) -> Self::Outcome {
        let src2 = ExtendedReg::new(src2.into_reg()).extend(mode, amount);
        Unfallible(Self {
            dst: dst.into_reg(),
            src1: src1.into_reg(),
            src2,
        })
    }
}

impl<Dst, Src1, Src2> MakeArithArgs<Dst, Src1, (Src2, ExtendMode, u8)>
    for ArithArgs<RegOrSp64, RegOrSp64, ExtendedReg<RegOrZero64>>
where
    Dst: IntoReg<RegOrSp64>,
    Src1: IntoReg<RegOrSp64>,
    Src2: IntoReg<RegOrZero64>,
{
    type Outcome = Result<Self, ExtendError>;

    #[inline]
    fn new(dst: Dst, src1: Src1, (src2, mode, shift): (Src2, ExtendMode, u8)) -> Result<Self, ExtendError> {
        ExtendShiftAmount::try_new(shift).map(|amount| Self {
            dst: dst.into_reg(),
            src1: src1.into_reg(),
            src2: ExtendedReg::new(src2.into_reg()).extend(mode, amount),
        })
    }
}

// --- Extend impls (32-bit) ---

impl<Src1, Src2> MakeArithArgs<RegOrSp32, Src1, Src2>
    for ArithArgs<RegOrSp32, RegOrSp32, RegOrZero32>
where
    Src1: IntoReg<RegOrSp32>,
    Src2: IntoReg<RegOrZero32>,
{
    type Outcome = Unfallible<Self>;

    #[inline]
    fn new(dst: RegOrSp32, src1: Src1, src2: Src2) -> Self::Outcome {
        Unfallible(Self {
            dst,
            src1: src1.into_reg(),
            src2: src2.into_reg(),
        })
    }
}

impl MakeArithArgs<RegOrSp32, RegOrSp32, ExtendedReg<RegOrZero32>>
    for ArithArgs<RegOrSp32, RegOrSp32, ExtendedReg<RegOrZero32>>
{
    type Outcome = Unfallible<Self>;

    #[inline]
    fn new(dst: RegOrSp32, src1: RegOrSp32, src2: ExtendedReg<RegOrZero32>) -> Self::Outcome {
        Unfallible(Self { dst, src1, src2 })
    }
}

impl<Dst, Src1, Src2> MakeArithArgs<Dst, Src1, (Src2, ExtendMode)>
    for ArithArgs<RegOrSp32, RegOrSp32, ExtendedReg<RegOrZero32>>
where
    Dst: IntoReg<RegOrSp32>,
    Src1: IntoReg<RegOrSp32>,
    Src2: IntoReg<RegOrZero32>,
{
    type Outcome = Unfallible<Self>;

    #[inline]
    fn new(dst: Dst, src1: Src1, (src2, mode): (Src2, ExtendMode)) -> Self::Outcome {
        let src2 = ExtendedReg::new(src2.into_reg()).extend(mode, <_>::default());
        Unfallible(Self {
            dst: dst.into_reg(),
            src1: src1.into_reg(),
            src2,
        })
    }
}

impl<Dst, Src1, Src2> MakeArithArgs<Dst, Src1, (Src2, ExtendMode, ExtendShiftAmount)>
    for ArithArgs<RegOrSp32, RegOrSp32, ExtendedReg<RegOrZero32>>
where
    Dst: IntoReg<RegOrSp32>,
    Src1: IntoReg<RegOrSp32>,
    Src2: IntoReg<RegOrZero32>,
{
    type Outcome = Unfallible<Self>;

    #[inline]
    fn new(
        dst: Dst,
        src1: Src1,
        (src2, mode, amount): (Src2, ExtendMode, ExtendShiftAmount),
    ) -> Self::Outcome {
        let src2 = ExtendedReg::new(src2.into_reg()).extend(mode, amount);
        Unfallible(Self {
            dst: dst.into_reg(),
            src1: src1.into_reg(),
            src2,
        })
    }
}

impl<Dst, Src1, Src2> MakeArithArgs<Dst, Src1, (Src2, ExtendMode, u8)>
    for ArithArgs<RegOrSp32, RegOrSp32, ExtendedReg<RegOrZero32>>
where
    Dst: IntoReg<RegOrSp32>,
    Src1: IntoReg<RegOrSp32>,
    Src2: IntoReg<RegOrZero32>,
{
    type Outcome = Result<Self, ExtendError>;

    #[inline]
    fn new(dst: Dst, src1: Src1, (src2, mode, shift): (Src2, ExtendMode, u8)) -> Result<Self, ExtendError> {
        ExtendShiftAmount::try_new(shift).map(|amount| Self {
            dst: dst.into_reg(),
            src1: src1.into_reg(),
            src2: ExtendedReg::new(src2.into_reg()).extend(mode, amount),
        })
    }
}

// --- Immediate imm12 impls (64-bit) ---

impl<Dst, Src> MakeArithArgs<Dst, Src, u32>
    for ArithArgs<RegOrSp64, RegOrSp64, AddSubImm12>
where
    Dst: IntoReg<RegOrSp64>,
    Src: IntoReg<RegOrSp64>,
{
    type Outcome = Result<Self, (BitError, BitError)>;

    #[inline]
    fn new(dst: Dst, src1: Src, src2: u32) -> Result<Self, (BitError, BitError)> {
        let imm12 = AddSubImm12::try_from(src2)?;
        Ok(Self {
            dst: dst.into_reg(),
            src1: src1.into_reg(),
            src2: imm12,
        })
    }
}

impl<Dst, Src1, Src2> MakeArithArgs<Dst, Src1, Src2>
    for ArithArgs<RegOrSp64, RegOrSp64, AddSubImm12>
where
    Dst: IntoReg<RegOrSp64>,
    Src1: IntoReg<RegOrSp64>,
    Src2: Into<AddSubImm12>,
{
    type Outcome = Unfallible<Self>;

    #[inline]
    fn new(dst: Dst, src1: Src1, src2: Src2) -> Self::Outcome {
        Unfallible(Self {
            dst: dst.into_reg(),
            src1: src1.into_reg(),
            src2: src2.into(),
        })
    }
}

// --- Immediate imm12 impls (32-bit) ---

impl<Dst, Src> MakeArithArgs<Dst, Src, u32>
    for ArithArgs<RegOrSp32, RegOrSp32, AddSubImm12>
where
    Dst: IntoReg<RegOrSp32>,
    Src: IntoReg<RegOrSp32>,
{
    type Outcome = Result<Self, (BitError, BitError)>;

    #[inline]
    fn new(dst: Dst, src1: Src, src2: u32) -> Result<Self, (BitError, BitError)> {
        let imm12 = AddSubImm12::try_from(src2)?;
        Ok(Self {
            dst: dst.into_reg(),
            src1: src1.into_reg(),
            src2: imm12,
        })
    }
}

impl<Dst, Src1, Src2> MakeArithArgs<Dst, Src1, Src2>
    for ArithArgs<RegOrSp32, RegOrSp32, AddSubImm12>
where
    Dst: IntoReg<RegOrSp32>,
    Src1: IntoReg<RegOrSp32>,
    Src2: Into<AddSubImm12>,
{
    type Outcome = Unfallible<Self>;

    #[inline]
    fn new(dst: Dst, src1: Src1, src2: Src2) -> Self::Outcome {
        Unfallible(Self {
            dst: dst.into_reg(),
            src1: src1.into_reg(),
            src2: src2.into(),
        })
    }
}

// --- Methods on ArithArgs for shift/extend chaining ---

impl ArithArgs<Reg64, Reg64, Reg64> {
    #[inline]
    pub fn shift(
        self,
        mode: ShiftMode,
        amount: ShiftAmount,
    ) -> ArithArgs<RegOrZero64, RegOrZero64, ShiftedReg<RegOrZero64>> {
        ArithArgs {
            dst: RegOrZero64::Reg(self.dst),
            src1: RegOrZero64::Reg(self.src1),
            src2: ShiftedReg::new(RegOrZero64::Reg(self.src2)).shift(mode, amount),
        }
    }

    #[inline]
    pub fn try_shift(
        self,
        mode: ShiftMode,
        amount: u32,
    ) -> Result<ArithArgs<RegOrZero64, RegOrZero64, ShiftedReg<RegOrZero64>>, BitError> {
        let amount = amount.try_into()?;
        Ok(ArithArgs {
            dst: RegOrZero64::Reg(self.dst),
            src1: RegOrZero64::Reg(self.src1),
            src2: ShiftedReg::new(RegOrZero64::Reg(self.src2)).shift(mode, amount),
        })
    }

    #[inline]
    pub fn extend(
        self,
        mode: ExtendMode,
        amount: ExtendShiftAmount,
    ) -> ArithArgs<RegOrSp64, RegOrSp64, ExtendedReg<RegOrZero64>> {
        ArithArgs {
            dst: RegOrSp64::Reg(self.dst),
            src1: RegOrSp64::Reg(self.src1),
            src2: ExtendedReg::new(RegOrZero64::Reg(self.src2)).extend(mode, amount),
        }
    }
}

impl ArithArgs<Reg32, Reg32, Reg32> {
    #[inline]
    pub fn shift(
        self,
        mode: ShiftMode,
        amount: ShiftAmount,
    ) -> ArithArgs<RegOrZero32, RegOrZero32, ShiftedReg<RegOrZero32>> {
        ArithArgs {
            dst: RegOrZero32::Reg(self.dst),
            src1: RegOrZero32::Reg(self.src1),
            src2: ShiftedReg::new(RegOrZero32::Reg(self.src2)).shift(mode, amount),
        }
    }

    #[inline]
    pub fn try_shift(
        self,
        mode: ShiftMode,
        amount: u32,
    ) -> Result<ArithArgs<RegOrZero32, RegOrZero32, ShiftedReg<RegOrZero32>>, BitError> {
        let amount = amount.try_into()?;
        Ok(ArithArgs {
            dst: RegOrZero32::Reg(self.dst),
            src1: RegOrZero32::Reg(self.src1),
            src2: ShiftedReg::new(RegOrZero32::Reg(self.src2)).shift(mode, amount),
        })
    }

    #[inline]
    pub fn extend(
        self,
        mode: ExtendMode,
        amount: ExtendShiftAmount,
    ) -> ArithArgs<RegOrSp32, RegOrSp32, ExtendedReg<RegOrZero32>> {
        ArithArgs {
            dst: RegOrSp32::Reg(self.dst),
            src1: RegOrSp32::Reg(self.src1),
            src2: ExtendedReg::new(RegOrZero32::Reg(self.src2)).extend(mode, amount),
        }
    }
}

impl ArithArgs<RegOrZero64, RegOrZero64, RegOrZero64> {
    #[inline]
    pub fn shift(
        self,
        mode: ShiftMode,
        amount: ShiftAmount,
    ) -> ArithArgs<RegOrZero64, RegOrZero64, ShiftedReg<RegOrZero64>> {
        ArithArgs {
            dst: self.dst,
            src1: self.src1,
            src2: ShiftedReg::new(self.src2).shift(mode, amount),
        }
    }

    #[inline]
    pub fn try_shift(
        self,
        mode: ShiftMode,
        amount: u32,
    ) -> Result<ArithArgs<RegOrZero64, RegOrZero64, ShiftedReg<RegOrZero64>>, BitError> {
        let amount = amount.try_into()?;
        Ok(ArithArgs {
            dst: self.dst,
            src1: self.src1,
            src2: ShiftedReg::new(self.src2).shift(mode, amount),
        })
    }
}

impl ArithArgs<RegOrZero32, RegOrZero32, RegOrZero32> {
    #[inline]
    pub fn shift(
        self,
        mode: ShiftMode,
        amount: ShiftAmount,
    ) -> ArithArgs<RegOrZero32, RegOrZero32, ShiftedReg<RegOrZero32>> {
        ArithArgs {
            dst: self.dst,
            src1: self.src1,
            src2: ShiftedReg::new(self.src2).shift(mode, amount),
        }
    }

    #[inline]
    pub fn try_shift(
        self,
        mode: ShiftMode,
        amount: u32,
    ) -> Result<ArithArgs<RegOrZero32, RegOrZero32, ShiftedReg<RegOrZero32>>, BitError> {
        let amount = amount.try_into()?;
        Ok(ArithArgs {
            dst: self.dst,
            src1: self.src1,
            src2: ShiftedReg::new(self.src2).shift(mode, amount),
        })
    }
}

impl ArithArgs<RegOrZero64, RegOrZero64, ShiftedReg<RegOrZero64>> {
    #[inline]
    pub fn shift(mut self, mode: ShiftMode, amount: ShiftAmount) -> Self {
        self.src2 = self.src2.shift(mode, amount);
        self
    }

    #[inline]
    pub fn try_shift(mut self, mode: ShiftMode, amount: u32) -> Result<Self, BitError> {
        self.src2 = self.src2.try_shift(mode, amount)?;
        Ok(self)
    }
}

impl ArithArgs<RegOrZero32, RegOrZero32, ShiftedReg<RegOrZero32>> {
    #[inline]
    pub fn shift(mut self, mode: ShiftMode, amount: ShiftAmount) -> Self {
        self.src2 = self.src2.shift(mode, amount);
        self
    }

    #[inline]
    pub fn try_shift(mut self, mode: ShiftMode, amount: u32) -> Result<Self, BitError> {
        self.src2 = self.src2.try_shift(mode, amount)?;
        Ok(self)
    }
}

impl ArithArgs<RegOrSp64, RegOrSp64, RegOrZero64> {
    #[inline]
    pub fn extend(
        self,
        mode: ExtendMode,
        amount: ExtendShiftAmount,
    ) -> ArithArgs<RegOrSp64, RegOrSp64, ExtendedReg<RegOrZero64>> {
        ArithArgs {
            dst: self.dst,
            src1: self.src1,
            src2: ExtendedReg::new(self.src2).extend(mode, amount),
        }
    }
}

impl ArithArgs<RegOrSp32, RegOrSp32, RegOrZero32> {
    #[inline]
    pub fn extend(
        self,
        mode: ExtendMode,
        amount: ExtendShiftAmount,
    ) -> ArithArgs<RegOrSp32, RegOrSp32, ExtendedReg<RegOrZero32>> {
        ArithArgs {
            dst: self.dst,
            src1: self.src1,
            src2: ExtendedReg::new(self.src2).extend(mode, amount),
        }
    }
}

impl ArithArgs<RegOrSp64, RegOrSp64, ExtendedReg<RegOrZero64>> {
    #[inline]
    pub fn extend(mut self, mode: ExtendMode, amount: ExtendShiftAmount) -> Self {
        self.src2 = self.src2.extend(mode, amount);
        self
    }
}

impl ArithArgs<RegOrSp32, RegOrSp32, ExtendedReg<RegOrZero32>> {
    #[inline]
    pub fn extend(mut self, mode: ExtendMode, amount: ExtendShiftAmount) -> Self {
        self.src2 = self.src2.extend(mode, amount);
        self
    }
}
