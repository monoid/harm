/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */
use aarchmrs_instructions::A64::{
    dpimm::addsub_imm::{
        ADD_32_addsub_imm::ADD_32_addsub_imm, ADD_64_addsub_imm::ADD_64_addsub_imm,
    },
    dpreg::{
        addsub_ext::{ADD_32_addsub_ext::ADD_32_addsub_ext, ADD_64_addsub_ext::ADD_64_addsub_ext},
        addsub_shift::{
            ADD_32_addsub_shift::ADD_32_addsub_shift, ADD_64_addsub_shift::ADD_64_addsub_shift,
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
pub struct Add<Args>(pub Args);

impl<Args> Sealed for Add<Args> {}

pub fn add<DstIn, T, Src1In, Src2In, S1, S2>(
    dst: DstIn,
    src1: Src1In,
    src2: Src2In,
) -> <<ArithArgs<T, S1, S2> as MakeArithArgs<DstIn, Src1In, Src2In>>::Outcome as Outcome>::Output<
    Add<ArithArgs<T, S1, S2>>,
>
where
    ArithArgs<T, S1, S2>: MakeArithArgs<DstIn, Src1In, Src2In>,
    <ArithArgs<T, S1, S2> as MakeArithArgs<DstIn, Src1In, Src2In>>::Outcome:
        Outcome<Inner = ArithArgs<T, S1, S2>>,
{
    <ArithArgs<T, S1, S2> as MakeArithArgs<DstIn, Src1In, Src2In>>::new(dst, src1, src2).map(Add)
}

// --- Shift/extend method forwarding ---

impl Add<ArithArgs<Reg64, Reg64, Reg64>> {
    #[inline]
    pub fn shift(
        self,
        mode: ShiftMode,
        amount: ShiftAmount,
    ) -> Add<ArithArgs<RegOrZero64, RegOrZero64, ShiftedReg<RegOrZero64>>> {
        Add(self.0.shift(mode, amount))
    }

    #[inline]
    pub fn try_shift(
        self,
        mode: ShiftMode,
        amount: u32,
    ) -> Result<Add<ArithArgs<RegOrZero64, RegOrZero64, ShiftedReg<RegOrZero64>>>, BitError> {
        self.0.try_shift(mode, amount).map(Add)
    }

    #[inline]
    pub fn extend(
        self,
        mode: ExtendMode,
        amount: ExtendShiftAmount,
    ) -> Add<ArithArgs<RegOrSp64, RegOrSp64, ExtendedReg<RegOrZero64>>> {
        Add(self.0.extend(mode, amount))
    }
}

impl Add<ArithArgs<Reg32, Reg32, Reg32>> {
    #[inline]
    pub fn shift(
        self,
        mode: ShiftMode,
        amount: ShiftAmount,
    ) -> Add<ArithArgs<RegOrZero32, RegOrZero32, ShiftedReg<RegOrZero32>>> {
        Add(self.0.shift(mode, amount))
    }

    #[inline]
    pub fn try_shift(
        self,
        mode: ShiftMode,
        amount: u32,
    ) -> Result<Add<ArithArgs<RegOrZero32, RegOrZero32, ShiftedReg<RegOrZero32>>>, BitError> {
        self.0.try_shift(mode, amount).map(Add)
    }

    #[inline]
    pub fn extend(
        self,
        mode: ExtendMode,
        amount: ExtendShiftAmount,
    ) -> Add<ArithArgs<RegOrSp32, RegOrSp32, ExtendedReg<RegOrZero32>>> {
        Add(self.0.extend(mode, amount))
    }
}

impl Add<ArithArgs<RegOrZero64, RegOrZero64, RegOrZero64>> {
    #[inline]
    pub fn shift(
        self,
        mode: ShiftMode,
        amount: ShiftAmount,
    ) -> Add<ArithArgs<RegOrZero64, RegOrZero64, ShiftedReg<RegOrZero64>>> {
        Add(self.0.shift(mode, amount))
    }

    #[inline]
    pub fn try_shift(
        self,
        mode: ShiftMode,
        amount: u32,
    ) -> Result<Add<ArithArgs<RegOrZero64, RegOrZero64, ShiftedReg<RegOrZero64>>>, BitError> {
        self.0.try_shift(mode, amount).map(Add)
    }
}

impl Add<ArithArgs<RegOrZero32, RegOrZero32, RegOrZero32>> {
    #[inline]
    pub fn shift(
        self,
        mode: ShiftMode,
        amount: ShiftAmount,
    ) -> Add<ArithArgs<RegOrZero32, RegOrZero32, ShiftedReg<RegOrZero32>>> {
        Add(self.0.shift(mode, amount))
    }

    #[inline]
    pub fn try_shift(
        self,
        mode: ShiftMode,
        amount: u32,
    ) -> Result<Add<ArithArgs<RegOrZero32, RegOrZero32, ShiftedReg<RegOrZero32>>>, BitError> {
        self.0.try_shift(mode, amount).map(Add)
    }
}

impl Add<ArithArgs<RegOrZero64, RegOrZero64, ShiftedReg<RegOrZero64>>> {
    #[inline]
    pub fn shift(self, mode: ShiftMode, amount: ShiftAmount) -> Self {
        Add(self.0.shift(mode, amount))
    }

    #[inline]
    pub fn try_shift(self, mode: ShiftMode, amount: u32) -> Result<Self, BitError> {
        self.0.try_shift(mode, amount).map(Add)
    }
}

impl Add<ArithArgs<RegOrZero32, RegOrZero32, ShiftedReg<RegOrZero32>>> {
    #[inline]
    pub fn shift(self, mode: ShiftMode, amount: ShiftAmount) -> Self {
        Add(self.0.shift(mode, amount))
    }

    #[inline]
    pub fn try_shift(self, mode: ShiftMode, amount: u32) -> Result<Self, BitError> {
        self.0.try_shift(mode, amount).map(Add)
    }
}

impl Add<ArithArgs<RegOrSp64, RegOrSp64, RegOrZero64>> {
    #[inline]
    pub fn extend(
        self,
        mode: ExtendMode,
        amount: ExtendShiftAmount,
    ) -> Add<ArithArgs<RegOrSp64, RegOrSp64, ExtendedReg<RegOrZero64>>> {
        Add(self.0.extend(mode, amount))
    }
}

impl Add<ArithArgs<RegOrSp32, RegOrSp32, RegOrZero32>> {
    #[inline]
    pub fn extend(
        self,
        mode: ExtendMode,
        amount: ExtendShiftAmount,
    ) -> Add<ArithArgs<RegOrSp32, RegOrSp32, ExtendedReg<RegOrZero32>>> {
        Add(self.0.extend(mode, amount))
    }
}

impl Add<ArithArgs<RegOrSp64, RegOrSp64, ExtendedReg<RegOrZero64>>> {
    #[inline]
    pub fn extend(self, mode: ExtendMode, amount: ExtendShiftAmount) -> Self {
        Add(self.0.extend(mode, amount))
    }
}

impl Add<ArithArgs<RegOrSp32, RegOrSp32, ExtendedReg<RegOrZero32>>> {
    #[inline]
    pub fn extend(self, mode: ExtendMode, amount: ExtendShiftAmount) -> Self {
        Add(self.0.extend(mode, amount))
    }
}

// --- RawInstruction impls ---

impl RawInstruction for Add<ArithArgs<Reg64, Reg64, Reg64>> {
    #[inline]
    fn to_code(&self) -> InstructionCode {
        Add(ArithArgs {
            dst: RegOrZero64::Reg(self.0.dst),
            src1: RegOrZero64::Reg(self.0.src1),
            src2: ShiftedReg::new(RegOrZero64::Reg(self.0.src2)),
        })
        .to_code()
    }
}

impl RawInstruction for Add<ArithArgs<RegOrZero64, RegOrZero64, ShiftedReg<RegOrZero64>>> {
    #[inline]
    fn to_code(&self) -> InstructionCode {
        let shift = self.0.src2.shift.mode as u8;
        let rm = self.0.src2.reg.index();
        let shift_amount = self.0.src2.shift.amount;
        let rn = self.0.src1.index();
        let rd = self.0.dst.index();
        ADD_64_addsub_shift(shift.into(), rm.into(), shift_amount.into(), rn.into(), rd.into())
    }
}

impl RawInstruction for Add<ArithArgs<Reg32, Reg32, Reg32>> {
    #[inline]
    fn to_code(&self) -> InstructionCode {
        Add(ArithArgs {
            dst: RegOrZero32::Reg(self.0.dst),
            src1: RegOrZero32::Reg(self.0.src1),
            src2: ShiftedReg::new(RegOrZero32::Reg(self.0.src2)),
        })
        .to_code()
    }
}

impl RawInstruction for Add<ArithArgs<RegOrZero32, RegOrZero32, ShiftedReg<RegOrZero32>>> {
    #[inline]
    fn to_code(&self) -> InstructionCode {
        let shift = self.0.src2.shift.mode as u8;
        let rm = self.0.src2.reg.index();
        let shift_amount = self.0.src2.shift.amount;
        let rn = self.0.src1.index();
        let rd = self.0.dst.index();
        ADD_32_addsub_shift(shift.into(), rm.into(), shift_amount.into(), rn.into(), rd.into())
    }
}

impl RawInstruction for Add<ArithArgs<RegOrSp64, RegOrSp64, ExtendedReg<RegOrZero64>>> {
    #[inline]
    fn to_code(&self) -> InstructionCode {
        let option = self.0.src2.extend.mode as u8;
        let rm = self.0.src2.reg.index();
        let imm3 = self.0.src2.extend.amount;
        let rn = self.0.src1.index();
        let rd = self.0.dst.index();
        ADD_64_addsub_ext(rm.into(), option.into(), imm3.into(), rn.into(), rd.into())
    }
}

impl RawInstruction for Add<ArithArgs<RegOrSp32, RegOrSp32, ExtendedReg<RegOrZero32>>> {
    #[inline]
    fn to_code(&self) -> InstructionCode {
        let option = self.0.src2.extend.mode as u8;
        let rm = self.0.src2.reg.index();
        let imm3 = self.0.src2.extend.amount;
        let rn = self.0.src1.index();
        let rd = self.0.dst.index();
        ADD_32_addsub_ext(rm.into(), option.into(), imm3.into(), rn.into(), rd.into())
    }
}

impl RawInstruction for Add<ArithArgs<RegOrSp64, RegOrSp64, AddSubImm12>> {
    #[inline]
    fn to_code(&self) -> InstructionCode {
        use AddSubImm12::*;
        let (shifted, imm12) = match self.0.src2 {
            Unshifted(value) => (false, value.into()),
            Shifted(value) => (true, value.into()),
        };
        let rn = self.0.src1.index();
        let rd = self.0.dst.index();
        ADD_64_addsub_imm(shifted.into(), imm12, rn.into(), rd.into())
    }
}

impl RawInstruction for Add<ArithArgs<RegOrSp32, RegOrSp32, AddSubImm12>> {
    #[inline]
    fn to_code(&self) -> InstructionCode {
        use AddSubImm12::*;
        let (shifted, imm12) = match self.0.src2 {
            Unshifted(value) => (false, value.into()),
            Shifted(value) => (true, value.into()),
        };
        let rn = self.0.src1.index();
        let rd = self.0.dst.index();
        ADD_32_addsub_imm(shifted.into(), imm12, rn.into(), rd.into())
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

    const ADD_DB: &str = "
8b3f2c41	add x1, x2, wzr, uxth #3
0b0c0041	add w1, w2, w12
0b2c6c41	add w1, w2, w12, uxtx #3
0b2c4c41	add w1, w2, w12, uxtw #3
0b3f6c41	add w1, w2, wzr, uxtx #3
0b3f4c41	add w1, w2, wzr, uxtw #3
0b4c1041	add w1, w2, w12, lsr #4
0b4c13e1	add w1, wzr, w12, lsr #4
11048c41	add w1, w2, #0x123
11048fff	add wsp, wsp, #0x123
11448c41	add w1, w2, #0x123000
8b0c0041	add x1, x2, x12
8b2c4c41	add x1, x2, w12, uxtw #3
8b2c6c41	add x1, x2, x12, uxtx #3
8b2c4fff	add sp, sp, w12, uxtw #3
8b2c6fff	add sp, sp, x12, uxtx #3
8b2c7041	add x1, x2, x12, uxtx #4
8b3f4c41	add x1, x2, wzr, uxtw #3
8b3f4fff	add sp, sp, wzr, uxtw #3
8b4c1041	add x1, x2, x12, lsr #4
8b4c13e1	add x1, xzr, x12, lsr #4
91000441	add x1, x2, #1
910007ff	add sp, sp, #1
91400441	add x1, x2, #0x1000
914007ff	add sp, sp, #0x1000
";

    test_cases! {
        ADD_DB, untested_add_db;
        test_add_64, add(X1, X2, X12), "add x1, x2, x12";
        test_add_64_shift, add(X1, X2, X12).try_shift(ShiftMode::LSR, 4).unwrap(), "add x1, x2, x12, lsr #4";
        test_add_64_zero,
            add(X1, XZR, ShiftedReg::from(X12).try_shift(ShiftMode::LSR, 4).unwrap()),
            "add x1, xzr, x12, lsr #4";
        test_add_64_shift_2,
            add(X1, X2, ShiftedReg::from(X12).try_shift(ShiftMode::LSR, 4)).unwrap(),
            "add x1, x2, x12, lsr #4";
        test_add_64_shift_3,
            add(X1, X2, (X12, ShiftMode::LSR, 4)).unwrap(),
            "add x1, x2, x12, lsr #4";
        test_add_64_extend_uxtx, add(RegS(X1), X2, X12).extend(ExtendMode::UXTX, ExtendShiftAmount::try_new(3).unwrap()),
            "add x1, x2, x12, uxtx #3";
        test_add_64_extend_uxtx_2, add(RegS(X1), X2, (X12, ExtendMode::UXTX, 3)).unwrap(),
            "add x1, x2, x12, uxtx #3";
        // KLUDGE: Using Reg64 instead of Reg32 at the last argument.
        // To be reimplemented akin `ldr` family.
        test_add_64_extend_uxtw,
            add(X1, X2, X12).extend(ExtendMode::UXTW, ExtendShiftAmount::try_new(3).unwrap()),
            "add x1, x2, w12, uxtw #3";
        test_add_64_wzr_extend_uxtw,
            add(RegS(X1), X2, XZR).extend(ExtendMode::UXTW, ExtendShiftAmount::try_new(3).unwrap()),
            "add x1, x2, wzr, uxtw #3";
        test_add_64_extend_uxtx_4,
            add(X1, X2, X12).extend(ExtendMode::UXTX, ExtendShiftAmount::try_new(4).unwrap()),
            "add x1, x2, x12, uxtx #4";
        test_add_64_extend_uxth_xzr,
            add(RegS(X1), RegS(X2), XZR).extend(ExtendMode::UXTH, ExtendShiftAmount::try_new(3).unwrap()),
            "add x1, x2, wzr, uxth #3";
        test_add_64_const_1, add(X1, X2, 1u32).unwrap(), "add x1, x2, #1";
        test_add_64_const_1_1, add(X1, X2, AddSubImm12::try_from(1).unwrap()), "add x1, x2, #1";
        test_add_64_const_0x1000, add(X1, X2, 0x1000u32).unwrap(), "add x1, x2, #0x1000";
        test_add_sp_64_const_1, add(SP, SP, 1u32).unwrap(), "add sp, sp, #1";
        test_add_sp_64_const_0x1000, add(SP, SP, 0x1000u32).unwrap(), "add sp, sp, #0x1000";
        test_add_32, add(W1, W2, W12), "add w1, w2, w12";
        test_add_32_shift, add(W1, W2, W12).try_shift(ShiftMode::LSR, 4).unwrap(), "add w1, w2, w12, lsr #4";
        test_add_32_zero,
            add(W1, WZR, ShiftedReg::from(W12).try_shift(ShiftMode::LSR, 4).unwrap()),
            "add w1, wzr, w12, lsr #4";
        test_add_32_extend_uxtx,
            add(W1, W2, W12).extend(ExtendMode::UXTX, ExtendShiftAmount::try_new(3).unwrap()),
            "add w1, w2, w12, uxtx #3";
        test_add_32_extend_uxtw,
            add(W1, W2, W12).extend(ExtendMode::UXTW, ExtendShiftAmount::try_new(3).unwrap()),
            "add w1, w2, w12, uxtw #3";
        test_add_32_extend_uxtx_wzr,  // that's really strange it works
            add(Reg3S(W1), W2, WZR).extend(ExtendMode::UXTX, ExtendShiftAmount::try_new(3).unwrap()),
            "add w1, w2, wzr, uxtx #3";
        test_add_32_extend_uxtw_wzr,
            add(Reg3S(W1), W2, WZR).extend(ExtendMode::UXTW, ExtendShiftAmount::try_new(3).unwrap()),
            "add w1, w2, wzr, uxtw #3";
        test_add_32_const_0x123, add(W1, W2, 0x123u32).unwrap(), "add w1, w2, #0x123";
        test_add_32_const_0x123_1, add(W1, W2, AddSubImm12::try_from(0x123)).unwrap(), "add w1, w2, #0x123";
        test_add_wsp_32_const_0x123, add(WSP, WSP, 0x123u32).unwrap(), "add wsp, wsp, #0x123";
        test_add_32_const_0x123000, add(W1, W2, 0x123000u32).unwrap(), "add w1, w2, #0x123000";
        test_add_32_const_0x123000_1, add(W1, W2, AddSubImm12::try_from(0x123000).unwrap()), "add w1, w2, #0x123000";
        test_add_64_sp_extend_uxtx,
            add(SP, SP, X12).extend(ExtendMode::UXTX, ExtendShiftAmount::try_new(3).unwrap()),
            "add sp, sp, x12, uxtx #3";
        test_add_64_sp_extend_uxtw,
            add(SP, SP, X12).extend(ExtendMode::UXTW, ExtendShiftAmount::try_new(3).unwrap()),
            "add sp, sp, w12, uxtw #3";
        test_add_64_sp_extend_uxtw_xzr,
            add(SP, SP, XZR).extend(ExtendMode::UXTW, ExtendShiftAmount::try_new(3).unwrap()),
            "add sp, sp, wzr, uxtw #3";
    }

    #[test]
    fn test_add_64_const_0x1001() {
        let a = add(X1, X2, 0x1001u32);
        assert!(a.is_err());
    }

    #[test]
    fn test_add_32_const_0x1001() {
        let a = add(W1, W2, 0x1001u32);
        assert!(a.is_err());
    }
}
