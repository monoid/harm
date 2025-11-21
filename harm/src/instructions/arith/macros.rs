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
                Src1: IntoReg<$ztype>,
                Src2: IntoReg<$ztype>,
            {
                type Output = Self;

                #[inline]
                fn new(dst: $ztype, src1: Src1, src2: Src2) -> Self {
                    Self { dst, src1: src1.into_reg(), src2: src2.into_reg() }
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
                Dst: IntoReg<$ztype>,
                Src1: IntoReg<$ztype>,
            {
                type Output = Self;

                #[inline]
                fn new(
                    dst: Dst,
                    src1: Src1,
                    src2: ShiftedReg<$ztype>,
                ) -> Self {
                    Self { dst: dst.into_reg(), src1: src1.into_reg(), src2 }
                }
            }

            impl<Dst, Src1, Src2> [<Make $name>]<Dst, Src1, (Src2, ShiftMode, ShiftAmount)> for $name<$ztype, $ztype, ShiftedReg<$ztype>>
            where
                Dst: IntoReg<$ztype>,
                Src1: IntoReg<$ztype>,
                Src2: IntoReg<$ztype>,
            {
                type Output = Self;

                #[inline]
                fn new(
                    dst: Dst,
                    src1: Src1,
                    (src2, shift_mode, shift_amount): (Src2, ShiftMode, ShiftAmount)
                ) -> Self {
                    let src2 = ShiftedReg::new(src2.into_reg()).shift(shift_mode, shift_amount);
                    Self { dst: dst.into_reg(), src1: src1.into_reg(), src2 }
                }
            }

            impl<Dst, Src1, Src2> [<Make $name>]<Dst, Src1, (Src2, ShiftMode, u32)> for $name<$ztype, $ztype, ShiftedReg<$ztype>>
            where
                Dst: IntoReg<$ztype>,
                Src1: IntoReg<$ztype>,
                Src2: IntoReg<$ztype>,
            {
                type Output = Result<Self, BitError>;

                #[inline]
                fn new(
                    dst: Dst,
                    src1: Src1,
                    (src2, shift_mode, shift_amount): (Src2, ShiftMode, u32)
                ) -> Result<Self, BitError> {
                    let shift_amount = shift_amount.try_into()?;
                    let src2 = ShiftedReg::new(src2.into_reg()).shift(shift_mode, shift_amount);
                    Ok(Self { dst: dst.into_reg(), src1: src1.into_reg(), src2 })
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
                    let rm = self.src2.reg.index();
                    let shift_amount_imm6 = self.src2.shift.amount;
                    let rn = self.src1.index();
                    let rd = self.dst.index();

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
                Dst: IntoReg<$etype>,
                Src: IntoReg<$etype>,
            {
                type Output = Result<Self, (BitError, BitError)>;

                #[inline]
                fn new(dst: Dst, src1: Src, src2: u32) -> Self::Output {
                    let imm12 = $crate::instructions::arith::AddSubImm12::try_from(src2)?;
                    Ok(Self {
                        dst: dst.into_reg(),
                        src1: src1.into_reg(),
                        src2: imm12,
                    })
                }
            }

            impl<Dst, Src1, Src2> [<Make $name>]<Dst, Src1, Src2>
                for $name<$etype, $etype, $crate::instructions::arith::AddSubImm12>
            where
                Dst: IntoReg<$etype>,
                Src1: IntoReg<$etype>,
                Src2: Into<$crate::instructions::arith::AddSubImm12>,
            {
                type Output = Self;

                #[inline]
                fn new(dst: Dst, src1: Src1, src2: Src2) -> Self::Output {
                    Self {
                        dst: dst.into_reg(),
                        src1: src1.into_reg(),
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
                    let rn = self.src1.index();
                    let rd = self.dst.index();

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
                    amount: ExtendShiftAmount,
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
            where Src1: IntoReg<$stype>,
                  Src2: IntoReg<$ztype>
            {
                type Output = Self;

                #[inline]
                fn new(dst: $stype, src1: Src1, src2: Src2) -> Self {
                    Self { dst, src1: src1.into_reg(), src2: src2.into_reg() }
                }
            }

            impl $name<$stype, $stype, $reg> {
                #[inline]
                pub fn extend(
                    self,
                    mode: ExtendMode,
                    amount: ExtendShiftAmount,
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
                pub fn extend(mut self, mode: ExtendMode, amount: ExtendShiftAmount) -> Self {
                    self.src2.extend = Extend { mode, amount };
                    self
                }
            }

            impl $name<$stype, $stype, $ztype> {
                #[inline]
                pub fn extend(self, mode: ExtendMode, amount: ExtendShiftAmount) -> $name<$stype, $stype, ExtendedReg<$ztype>> {
                    $name::new(self.dst, self.src1, ExtendedReg::new(self.src2))
                        .extend(mode, amount)
                }
            }

            impl RawInstruction for $name<$stype, $stype, ExtendedReg<$ztype>> {
                #[inline]
                fn to_code(&self) -> InstructionCode {
                    let option = self.src2.extend.mode as u8;
                    let rm = self.src2.reg.index();
                    let imm3 = self.src2.extend.amount;
                    let rn = self.src1.index();
                    let rd = self.dst.index();

                    [<$name:upper _ $bits _ $cmd _ext>](
                        rm.into(), option.into(), imm3.into(), rn.into(), rd.into()
                    )
                }
            }

            impl<Dst, Src1, Src2> [<Make $name>]<Dst, Src1, (Src2, ExtendMode)>
                for $name<$stype, $stype, ExtendedReg<$ztype>>
                where
                    Dst: IntoReg<$stype>,
                    Src1: IntoReg<$stype>,
                    Src2: IntoReg<$ztype>,
            {
                type Output = Self;

                #[inline]
                fn new(
                    dst: Dst,
                    src1: Src1,
                    (src2, mode): (Src2, ExtendMode),
                ) -> Self {
                    let src2 = ExtendedReg::new(src2.into_reg()).extend(mode, <_>::default());
                    Self { dst: dst.into_reg(), src1: src1.into_reg(), src2 }
                }
            }

            impl<Dst, Src1, Src2> [<Make $name>]<Dst, Src1, (Src2, ExtendMode, ExtendShiftAmount)>
                for $name<$stype, $stype, ExtendedReg<$ztype>>
                where
                    Dst: IntoReg<$stype>,
                    Src1: IntoReg<$stype>,
                    Src2: IntoReg<$ztype>,
            {
                type Output = Self;

                #[inline]
                fn new(
                    dst: Dst,
                    src1: Src1,
                    (src2, mode, offset): (Src2, ExtendMode, ExtendShiftAmount),
                ) -> Self::Output {
                    let src2 = ExtendedReg::new(src2.into_reg()).extend(mode, offset);
                    Self { dst: dst.into_reg(), src1: src1.into_reg(), src2 }
                }
            }

            impl<Dst, Src1, Src2> [<Make $name>]<Dst, Src1, (Src2, ExtendMode, u8)>
                for $name<$stype, $stype, ExtendedReg<$ztype>>
                where
                    Dst: IntoReg<$stype>,
                    Src1: IntoReg<$stype>,
                    Src2: IntoReg<$ztype>,
            {
                type Output = Result<Self, ExtendError>;

                #[inline]
                fn new(
                    dst: Dst,
                    src1: Src1,
                    (src2, mode, shift): (Src2, ExtendMode, u8),
                ) -> Self::Output {
                    ExtendShiftAmount::try_new(shift).map(|shift| Self::new(dst, src1, (src2, mode, shift)))
                }
            }
        }
    };
}
