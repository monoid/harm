/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */
use aarchmrs_instructions::A64::{
    dpimm::addsub_imm::{
        SUB_32_addsub_imm::SUB_32_addsub_imm, SUB_64_addsub_imm::SUB_64_addsub_imm,
    },
    dpreg::{
        addsub_ext::{SUB_32_addsub_ext::SUB_32_addsub_ext, SUB_64_addsub_ext::SUB_64_addsub_ext},
        addsub_shift::{
            SUB_32_addsub_shift::SUB_32_addsub_shift, SUB_64_addsub_shift::SUB_64_addsub_shift,
        },
    },
};
use aarchmrs_types::InstructionCode;

use super::args::{ArithArgs, MakeArithArgs};
use super::{
    AddSubImm12, ExtendMode, ExtendShiftAmount, ExtendedReg, ShiftAmount, ShiftMode, ShiftedReg,
};
use crate::{
    bits::BitError,
    instructions::RawInstruction,
    outcome::Outcome,
    register::{Reg32, Reg64, RegOrSp32, RegOrSp64, RegOrZero32, RegOrZero64, Register as _},
    sealed::Sealed,
};

#[derive(Debug, Copy, Clone)]
pub struct Sub<Args>(pub Args);

impl<Args> Sealed for Sub<Args> {}

pub fn sub<DstIn, T, Src1In, Src2In, S1, S2>(
    dst: DstIn,
    src1: Src1In,
    src2: Src2In,
) -> <<ArithArgs<T, S1, S2> as MakeArithArgs<DstIn, Src1In, Src2In>>::Outcome as Outcome>::Output<
    Sub<ArithArgs<T, S1, S2>>,
>
where
    ArithArgs<T, S1, S2>: MakeArithArgs<DstIn, Src1In, Src2In>,
    <ArithArgs<T, S1, S2> as MakeArithArgs<DstIn, Src1In, Src2In>>::Outcome:
        Outcome<Inner = ArithArgs<T, S1, S2>>,
{
    <ArithArgs<T, S1, S2> as MakeArithArgs<DstIn, Src1In, Src2In>>::new(dst, src1, src2).map(Sub)
}

// --- Shift/extend method forwarding ---

impl Sub<ArithArgs<Reg64, Reg64, Reg64>> {
    #[inline]
    pub fn shift(
        self,
        mode: ShiftMode,
        amount: ShiftAmount,
    ) -> Sub<ArithArgs<RegOrZero64, RegOrZero64, ShiftedReg<RegOrZero64>>> {
        Sub(self.0.shift(mode, amount))
    }

    #[inline]
    pub fn try_shift(
        self,
        mode: ShiftMode,
        amount: u32,
    ) -> Result<Sub<ArithArgs<RegOrZero64, RegOrZero64, ShiftedReg<RegOrZero64>>>, BitError> {
        self.0.try_shift(mode, amount).map(Sub)
    }

    #[inline]
    pub fn extend(
        self,
        mode: ExtendMode,
        amount: ExtendShiftAmount,
    ) -> Sub<ArithArgs<RegOrSp64, RegOrSp64, ExtendedReg<RegOrZero64>>> {
        Sub(self.0.extend(mode, amount))
    }
}

impl Sub<ArithArgs<Reg32, Reg32, Reg32>> {
    #[inline]
    pub fn shift(
        self,
        mode: ShiftMode,
        amount: ShiftAmount,
    ) -> Sub<ArithArgs<RegOrZero32, RegOrZero32, ShiftedReg<RegOrZero32>>> {
        Sub(self.0.shift(mode, amount))
    }

    #[inline]
    pub fn try_shift(
        self,
        mode: ShiftMode,
        amount: u32,
    ) -> Result<Sub<ArithArgs<RegOrZero32, RegOrZero32, ShiftedReg<RegOrZero32>>>, BitError> {
        self.0.try_shift(mode, amount).map(Sub)
    }

    #[inline]
    pub fn extend(
        self,
        mode: ExtendMode,
        amount: ExtendShiftAmount,
    ) -> Sub<ArithArgs<RegOrSp32, RegOrSp32, ExtendedReg<RegOrZero32>>> {
        Sub(self.0.extend(mode, amount))
    }
}

impl Sub<ArithArgs<RegOrZero64, RegOrZero64, RegOrZero64>> {
    #[inline]
    pub fn shift(
        self,
        mode: ShiftMode,
        amount: ShiftAmount,
    ) -> Sub<ArithArgs<RegOrZero64, RegOrZero64, ShiftedReg<RegOrZero64>>> {
        Sub(self.0.shift(mode, amount))
    }

    #[inline]
    pub fn try_shift(
        self,
        mode: ShiftMode,
        amount: u32,
    ) -> Result<Sub<ArithArgs<RegOrZero64, RegOrZero64, ShiftedReg<RegOrZero64>>>, BitError> {
        self.0.try_shift(mode, amount).map(Sub)
    }
}

impl Sub<ArithArgs<RegOrZero32, RegOrZero32, RegOrZero32>> {
    #[inline]
    pub fn shift(
        self,
        mode: ShiftMode,
        amount: ShiftAmount,
    ) -> Sub<ArithArgs<RegOrZero32, RegOrZero32, ShiftedReg<RegOrZero32>>> {
        Sub(self.0.shift(mode, amount))
    }

    #[inline]
    pub fn try_shift(
        self,
        mode: ShiftMode,
        amount: u32,
    ) -> Result<Sub<ArithArgs<RegOrZero32, RegOrZero32, ShiftedReg<RegOrZero32>>>, BitError> {
        self.0.try_shift(mode, amount).map(Sub)
    }
}

impl Sub<ArithArgs<RegOrZero64, RegOrZero64, ShiftedReg<RegOrZero64>>> {
    #[inline]
    pub fn shift(self, mode: ShiftMode, amount: ShiftAmount) -> Self {
        Sub(self.0.shift(mode, amount))
    }

    #[inline]
    pub fn try_shift(self, mode: ShiftMode, amount: u32) -> Result<Self, BitError> {
        self.0.try_shift(mode, amount).map(Sub)
    }
}

impl Sub<ArithArgs<RegOrZero32, RegOrZero32, ShiftedReg<RegOrZero32>>> {
    #[inline]
    pub fn shift(self, mode: ShiftMode, amount: ShiftAmount) -> Self {
        Sub(self.0.shift(mode, amount))
    }

    #[inline]
    pub fn try_shift(self, mode: ShiftMode, amount: u32) -> Result<Self, BitError> {
        self.0.try_shift(mode, amount).map(Sub)
    }
}

impl Sub<ArithArgs<RegOrSp64, RegOrSp64, RegOrZero64>> {
    #[inline]
    pub fn extend(
        self,
        mode: ExtendMode,
        amount: ExtendShiftAmount,
    ) -> Sub<ArithArgs<RegOrSp64, RegOrSp64, ExtendedReg<RegOrZero64>>> {
        Sub(self.0.extend(mode, amount))
    }
}

impl Sub<ArithArgs<RegOrSp32, RegOrSp32, RegOrZero32>> {
    #[inline]
    pub fn extend(
        self,
        mode: ExtendMode,
        amount: ExtendShiftAmount,
    ) -> Sub<ArithArgs<RegOrSp32, RegOrSp32, ExtendedReg<RegOrZero32>>> {
        Sub(self.0.extend(mode, amount))
    }
}

impl Sub<ArithArgs<RegOrSp64, RegOrSp64, ExtendedReg<RegOrZero64>>> {
    #[inline]
    pub fn extend(self, mode: ExtendMode, amount: ExtendShiftAmount) -> Self {
        Sub(self.0.extend(mode, amount))
    }
}

impl Sub<ArithArgs<RegOrSp32, RegOrSp32, ExtendedReg<RegOrZero32>>> {
    #[inline]
    pub fn extend(self, mode: ExtendMode, amount: ExtendShiftAmount) -> Self {
        Sub(self.0.extend(mode, amount))
    }
}

// --- RawInstruction impls ---

impl RawInstruction for Sub<ArithArgs<Reg64, Reg64, Reg64>> {
    #[inline]
    fn to_code(&self) -> InstructionCode {
        Sub(ArithArgs {
            dst: RegOrZero64::Reg(self.0.dst),
            src1: RegOrZero64::Reg(self.0.src1),
            src2: ShiftedReg::new(RegOrZero64::Reg(self.0.src2)),
        })
        .to_code()
    }
}

impl RawInstruction for Sub<ArithArgs<RegOrZero64, RegOrZero64, ShiftedReg<RegOrZero64>>> {
    #[inline]
    fn to_code(&self) -> InstructionCode {
        let shift = self.0.src2.shift.mode as u8;
        let rm = self.0.src2.reg.index();
        let shift_amount = self.0.src2.shift.amount;
        let rn = self.0.src1.index();
        let rd = self.0.dst.index();
        SUB_64_addsub_shift(shift.into(), rm.into(), shift_amount.into(), rn.into(), rd.into())
    }
}

impl RawInstruction for Sub<ArithArgs<Reg32, Reg32, Reg32>> {
    #[inline]
    fn to_code(&self) -> InstructionCode {
        Sub(ArithArgs {
            dst: RegOrZero32::Reg(self.0.dst),
            src1: RegOrZero32::Reg(self.0.src1),
            src2: ShiftedReg::new(RegOrZero32::Reg(self.0.src2)),
        })
        .to_code()
    }
}

impl RawInstruction for Sub<ArithArgs<RegOrZero32, RegOrZero32, ShiftedReg<RegOrZero32>>> {
    #[inline]
    fn to_code(&self) -> InstructionCode {
        let shift = self.0.src2.shift.mode as u8;
        let rm = self.0.src2.reg.index();
        let shift_amount = self.0.src2.shift.amount;
        let rn = self.0.src1.index();
        let rd = self.0.dst.index();
        SUB_32_addsub_shift(shift.into(), rm.into(), shift_amount.into(), rn.into(), rd.into())
    }
}

impl RawInstruction for Sub<ArithArgs<RegOrSp64, RegOrSp64, ExtendedReg<RegOrZero64>>> {
    #[inline]
    fn to_code(&self) -> InstructionCode {
        let option = self.0.src2.extend.mode as u8;
        let rm = self.0.src2.reg.index();
        let imm3 = self.0.src2.extend.amount;
        let rn = self.0.src1.index();
        let rd = self.0.dst.index();
        SUB_64_addsub_ext(rm.into(), option.into(), imm3.into(), rn.into(), rd.into())
    }
}

impl RawInstruction for Sub<ArithArgs<RegOrSp32, RegOrSp32, ExtendedReg<RegOrZero32>>> {
    #[inline]
    fn to_code(&self) -> InstructionCode {
        let option = self.0.src2.extend.mode as u8;
        let rm = self.0.src2.reg.index();
        let imm3 = self.0.src2.extend.amount;
        let rn = self.0.src1.index();
        let rd = self.0.dst.index();
        SUB_32_addsub_ext(rm.into(), option.into(), imm3.into(), rn.into(), rd.into())
    }
}

impl RawInstruction for Sub<ArithArgs<RegOrSp64, RegOrSp64, AddSubImm12>> {
    #[inline]
    fn to_code(&self) -> InstructionCode {
        use AddSubImm12::*;
        let (shifted, imm12) = match self.0.src2 {
            Unshifted(value) => (false, value.into()),
            Shifted(value) => (true, value.into()),
        };
        let rn = self.0.src1.index();
        let rd = self.0.dst.index();
        SUB_64_addsub_imm(shifted.into(), imm12, rn.into(), rd.into())
    }
}

impl RawInstruction for Sub<ArithArgs<RegOrSp32, RegOrSp32, AddSubImm12>> {
    #[inline]
    fn to_code(&self) -> InstructionCode {
        use AddSubImm12::*;
        let (shifted, imm12) = match self.0.src2 {
            Unshifted(value) => (false, value.into()),
            Shifted(value) => (true, value.into()),
        };
        let rn = self.0.src1.index();
        let rd = self.0.dst.index();
        SUB_32_addsub_imm(shifted.into(), imm12, rn.into(), rd.into())
    }
}

#[cfg(test)]
mod tests {
    use harm_test_utils::test_cases;

    use super::*;
    use crate::instructions::InstructionSeq;
    use crate::instructions::arith::AddSubImm12;
    use Reg32::*;
    use Reg64::*;
    use RegOrSp32::Reg as Reg3S;
    use RegOrSp32::WSP;
    use RegOrSp64::Reg as RegS;
    use RegOrSp64::SP;
    use RegOrZero32::WZR;
    use RegOrZero64::XZR;

    const SUB_DB: &str = "
cb3f2c41	sub x1, x2, wzr, uxth #3
4b0c0041	sub w1, w2, w12
4b2c6c41	sub w1, w2, w12, uxtx #3
4b2c4c41	sub w1, w2, w12, uxtw #3
4b3f6c41	sub w1, w2, wzr, uxtx #3
4b3f4c41	sub w1, w2, wzr, uxtw #3
4b4c1041	sub w1, w2, w12, lsr #4
4b4c13e1	sub w1, wzr, w12, lsr #4
51048c41	sub w1, w2, #0x123
51048fff	sub wsp, wsp, #0x123
51448c41	sub w1, w2, #0x123000
cb0c0041	sub x1, x2, x12
cb2c4c41	sub x1, x2, w12, uxtw #3
cb2c6c41	sub x1, x2, x12, uxtx #3
cb2c4fff	sub sp, sp, w12, uxtw #3
cb2c6fff	sub sp, sp, x12, uxtx #3
cb2c7041	sub x1, x2, x12, uxtx #4
cb3f4c41	sub x1, x2, wzr, uxtw #3
cb3f4fff	sub sp, sp, wzr, uxtw #3
cb4c1041	sub x1, x2, x12, lsr #4
cb4c13e1	sub x1, xzr, x12, lsr #4
d1000441	sub x1, x2, #1
d10007ff	sub sp, sp, #1
d1400441	sub x1, x2, #0x1000
d14007ff	sub sp, sp, #0x1000
";

    test_cases! {
        SUB_DB, untested_sub_db;
        test_sub_64, sub(X1, X2, X12), "sub x1, x2, x12";
        test_sub_64_shift, sub(X1, X2, X12).try_shift(ShiftMode::LSR, 4).unwrap(), "sub x1, x2, x12, lsr #4";
        test_sub_64_zero,
            sub(X1, XZR, ShiftedReg::from(X12).try_shift(ShiftMode::LSR, 4).unwrap()),
            "sub x1, xzr, x12, lsr #4";
        test_sub_64_shift_2,
            sub(X1, X2, ShiftedReg::from(X12).try_shift(ShiftMode::LSR, 4)).unwrap(),
            "sub x1, x2, x12, lsr #4";
        test_sub_64_shift_3,
            sub(X1, X2, (X12, ShiftMode::LSR, 4)).unwrap(),
            "sub x1, x2, x12, lsr #4";
        test_sub_64_extend_uxtx, sub(RegS(X1), X2, X12).extend(ExtendMode::UXTX, ExtendShiftAmount::try_new(3).unwrap()),
            "sub x1, x2, x12, uxtx #3";
        test_sub_64_extend_uxtx_2, sub(RegS(X1), X2, (X12, ExtendMode::UXTX, 3)).unwrap(),
            "sub x1, x2, x12, uxtx #3";
        // KLUDGE: Using Reg64 instead of Reg32 at the last argument.
        // To be reimplemented akin `ldr` family.
        test_sub_64_extend_uxtw,
            sub(X1, X2, X12).extend(ExtendMode::UXTW, ExtendShiftAmount::try_new(3).unwrap()),
            "sub x1, x2, w12, uxtw #3";
        test_sub_64_wzr_extend_uxtw,
            sub(RegS(X1), X2, XZR).extend(ExtendMode::UXTW, ExtendShiftAmount::try_new(3).unwrap()),
            "sub x1, x2, wzr, uxtw #3";
        test_sub_64_extend_uxtx_4, sub(X1, X2, X12).extend(ExtendMode::UXTX, ExtendShiftAmount::try_new(4).unwrap()), "sub x1, x2, x12, uxtx #4";
        test_sub_64_extend_uxth_xzr,
            sub(RegS(X1), RegS(X2), XZR).extend(ExtendMode::UXTH, ExtendShiftAmount::try_new(3).unwrap()),
            "sub x1, x2, wzr, uxth #3";
        test_sub_64_const_1, sub(X1, X2, 1u32).unwrap(), "sub x1, x2, #1";
        test_sub_64_const_1_1, sub(X1, X2, AddSubImm12::try_from(1).unwrap()), "sub x1, x2, #1";
        test_sub_64_const_0x1000, sub(X1, X2, 0x1000u32).unwrap(), "sub x1, x2, #0x1000";
        test_sub_sp_64_const_1, sub(SP, SP, 1u32).unwrap(), "sub sp, sp, #1";
        test_sub_sp_64_const_0x1000, sub(SP, SP, 0x1000u32).unwrap(), "sub sp, sp, #0x1000";
        test_sub_32, sub(W1, W2, W12), "sub w1, w2, w12";
        test_sub_32_shift, sub(W1, W2, W12).try_shift(ShiftMode::LSR, 4).unwrap(), "sub w1, w2, w12, lsr #4";
        test_sub_32_zero,
            sub(W1, WZR, ShiftedReg::from(W12).try_shift(ShiftMode::LSR, 4)).unwrap(),
            "sub w1, wzr, w12, lsr #4";
        test_sub_32_extend_uxtx,
            sub(W1, W2, W12).extend(ExtendMode::UXTX, ExtendShiftAmount::try_new(3).unwrap()),
            "sub w1, w2, w12, uxtx #3";
        test_sub_32_extend_uxtw,
            sub(W1, W2, W12).extend(ExtendMode::UXTW, ExtendShiftAmount::try_new(3).unwrap()),
            "sub w1, w2, w12, uxtw #3";
        test_sub_32_extend_uxtx_wzr,
            sub(Reg3S(W1), W2, WZR).extend(ExtendMode::UXTX, ExtendShiftAmount::try_new(3).unwrap()),
            "sub w1, w2, wzr, uxtx #3";
        test_sub_32_extend_uxtw_wzr,
            sub(Reg3S(W1), W2, WZR).extend(ExtendMode::UXTW, ExtendShiftAmount::try_new(3).unwrap()),
            "sub w1, w2, wzr, uxtw #3";
        test_sub_32_const_0x123, sub(W1, W2, 0x123u32).unwrap(), "sub w1, w2, #0x123";
        test_sub_32_const_0x123_1, sub(W1, W2, AddSubImm12::try_from(0x123).unwrap()), "sub w1, w2, #0x123";
        test_sub_wsp_32_const_0x123, sub(WSP, WSP, 0x123u32).unwrap(), "sub wsp, wsp, #0x123";
        test_sub_32_const_0x123000, sub(W1, W2, 0x123000u32).unwrap(), "sub w1, w2, #0x123000";
        test_sub_32_const_0x123000_1, sub(W1, W2, AddSubImm12::try_from(0x123000).unwrap()), "sub w1, w2, #0x123000";
        test_sub_64_sp_extend_uxtx,
            sub(SP, SP, X12).extend(ExtendMode::UXTX, ExtendShiftAmount::try_new(3).unwrap()),
            "sub sp, sp, x12, uxtx #3";
        test_sub_64_sp_extend_uxtw,
            sub(SP, SP, X12).extend(ExtendMode::UXTW, ExtendShiftAmount::try_new(3).unwrap()),
            "sub sp, sp, w12, uxtw #3";
        test_sub_64_sp_extend_uxtw_xzr,
            sub(SP, SP, XZR).extend(ExtendMode::UXTW, ExtendShiftAmount::try_new(3).unwrap()),
            "sub sp, sp, wzr, uxtw #3";
    }

    #[test]
    fn test_sub_64_const_0x1001() {
        let a = sub(X1, X2, 0x1001u32);
        assert!(a.is_err());
    }

    #[test]
    fn test_sub_32_const_0x1001() {
        let a = sub(W1, W2, 0x1001u32);
        assert!(a.is_err());
    }
}
