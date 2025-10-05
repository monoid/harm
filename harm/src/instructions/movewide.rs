/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

/*!
 * Module for move instructions, like `MOVZ`, `MOVN` (not to be confused with `MVN`), `MOVK` in AArch64.
 */

use aarchmrs_instructions::A64::dpimm::movewide::{
    MOVK_32_movewide::MOVK_32_movewide, MOVK_64_movewide::MOVK_64_movewide,
    MOVN_32_movewide::MOVN_32_movewide, MOVN_64_movewide::MOVN_64_movewide,
    MOVZ_32_movewide::MOVZ_32_movewide, MOVZ_64_movewide::MOVZ_64_movewide,
};

use crate::{
    bits::UBitValue,
    outcome::{Outcome, Unfallible},
    register::{IntoReg, RegOrZero32, RegOrZero64, Register},
    sealed::Sealed,
};

use super::RawInstruction;

// Either 0 or 16 = 1 << 4.
pub type Shift32 = UBitValue<1, 4>;
// Either 0, 16, 32 or 48.
pub type Shift64 = UBitValue<2, 4>;

pub type MoveImm16 = UBitValue<16>;

pub trait MoveShift: Register {
    type Shift;
}

impl MoveShift for RegOrZero32 {
    type Shift = Shift32;
}

impl MoveShift for RegOrZero64 {
    type Shift = Shift64;
}

pub trait MakeMovArgs<R, Val>: Sealed {
    type Outcome: Outcome;

    fn new(rd: R, val: Val) -> Self::Outcome;
}

#[derive(Debug, Clone, Copy)]
pub struct MovArgs<Reg: MoveShift> {
    pub rd: Reg,
    pub imm16: MoveImm16,
    pub shift: Reg::Shift,
}

impl<Reg: MoveShift> Sealed for MovArgs<Reg> {}

impl<RIn: IntoReg<RegOrZero32>, Imm16: Into<MoveImm16>>
    MakeMovArgs<RIn, (Imm16, <RegOrZero32 as MoveShift>::Shift)> for MovArgs<RegOrZero32>
{
    type Outcome = Unfallible<Self>;

    fn new(rd: RIn, (imm16, shift): (Imm16, <RegOrZero32 as MoveShift>::Shift)) -> Self::Outcome {
        Unfallible(Self {
            rd: rd.into_reg(),
            imm16: imm16.into(),
            shift,
        })
    }
}

impl<RIn, Imm16> MakeMovArgs<RIn, Imm16> for MovArgs<RegOrZero32>
where
    RIn: IntoReg<RegOrZero32>,
    RegOrZero32: MoveShift,
    <RegOrZero32 as MoveShift>::Shift: Default,
    Imm16: Into<MoveImm16>,
{
    type Outcome = Unfallible<Self>;

    fn new(rd: RIn, imm16: Imm16) -> Self::Outcome {
        Unfallible(Self {
            rd: rd.into_reg(),
            imm16: imm16.into(),
            shift: <_>::default(),
        })
    }
}

impl<RIn: IntoReg<RegOrZero64>, Imm16: Into<MoveImm16>>
    MakeMovArgs<RIn, (Imm16, <RegOrZero64 as MoveShift>::Shift)> for MovArgs<RegOrZero64>
{
    type Outcome = Unfallible<Self>;

    fn new(rd: RIn, (imm16, shift): (Imm16, <RegOrZero64 as MoveShift>::Shift)) -> Self::Outcome {
        Unfallible(Self {
            rd: rd.into_reg(),
            imm16: imm16.into(),
            shift,
        })
    }
}

impl<RIn, Imm16> MakeMovArgs<RIn, Imm16> for MovArgs<RegOrZero64>
where
    RIn: IntoReg<RegOrZero64>,
    RegOrZero64: MoveShift,
    <RegOrZero64 as MoveShift>::Shift: Default,
    Imm16: Into<MoveImm16>,
{
    type Outcome = Unfallible<Self>;

    fn new(rd: RIn, imm16: Imm16) -> Self::Outcome {
        Unfallible(Self {
            rd: rd.into_reg(),
            imm16: imm16.into(),
            shift: <_>::default(),
        })
    }
}

pub struct MovK<Args>(Args);

pub fn movk<RIn, Val, ROut>(
    rd: RIn,
    val: Val,
) -> <<MovArgs<ROut> as MakeMovArgs<RIn, Val>>::Outcome as Outcome>::Output<
    MovK<<<MovArgs<ROut> as MakeMovArgs<RIn, Val>>::Outcome as Outcome>::Inner>,
>
where
    ROut: MoveShift,
    MovArgs<ROut>: MakeMovArgs<RIn, Val>,
{
    (<MovArgs<ROut> as MakeMovArgs<RIn, Val>>::new(rd, val)).map(MovK)
}

impl RawInstruction for MovK<MovArgs<RegOrZero32>> {
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        MOVK_32_movewide(self.0.shift.into(), self.0.imm16.into(), self.0.rd.code())
    }
}

impl RawInstruction for MovK<MovArgs<RegOrZero64>> {
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        MOVK_64_movewide(self.0.shift.into(), self.0.imm16.into(), self.0.rd.code())
    }
}

pub struct MovN<Args>(Args);

pub fn movn<RIn, Val, ROut>(
    rd: RIn,
    val: Val,
) -> <<MovArgs<ROut> as MakeMovArgs<RIn, Val>>::Outcome as Outcome>::Output<
    MovN<<<MovArgs<ROut> as MakeMovArgs<RIn, Val>>::Outcome as Outcome>::Inner>,
>
where
    ROut: MoveShift,
    MovArgs<ROut>: MakeMovArgs<RIn, Val>,
{
    <MovArgs<ROut> as MakeMovArgs<RIn, Val>>::new(rd, val).map(MovN)
}

impl RawInstruction for MovN<MovArgs<RegOrZero32>> {
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        MOVN_32_movewide(self.0.shift.into(), self.0.imm16.into(), self.0.rd.code())
    }
}

impl RawInstruction for MovN<MovArgs<RegOrZero64>> {
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        MOVN_64_movewide(self.0.shift.into(), self.0.imm16.into(), self.0.rd.code())
    }
}

pub struct MovZ<Args>(Args);

pub fn movz<RIn, Val, ROut>(
    rd: RIn,
    val: Val,
) -> <<MovArgs<ROut> as MakeMovArgs<RIn, Val>>::Outcome as Outcome>::Output<
    MovZ<<<MovArgs<ROut> as MakeMovArgs<RIn, Val>>::Outcome as Outcome>::Inner>,
>
where
    ROut: MoveShift,
    MovArgs<ROut>: MakeMovArgs<RIn, Val>,
{
    <MovArgs<ROut> as MakeMovArgs<RIn, Val>>::new(rd, val).map(MovZ)
}

// TODO variants fallible on the value should be implemented in the virtual `MOV` instruction; however,
// we should implemenet a variant fallible on shift.

impl RawInstruction for MovZ<MovArgs<RegOrZero32>> {
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        MOVZ_32_movewide(self.0.shift.into(), self.0.imm16.into(), self.0.rd.code())
    }
}

impl RawInstruction for MovZ<MovArgs<RegOrZero64>> {
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        MOVZ_64_movewide(self.0.shift.into(), self.0.imm16.into(), self.0.rd.code())
    }
}

#[cfg(test)]
mod tests {
    use crate::instructions::InstructionSeq;
    use crate::register::Reg32::*;
    use crate::register::Reg64::*;
    use harm_test_utils::test_cases;

    use super::*;

    const MOVX_DB: &str = "
12800541	movn w1, #42
12800541	movn w1, #42, lsl #0
12908421	movn w1, #0x8421
12908421	movn w1, #0x8421, lsl #0
12a00541	movn w1, #42, lsl #16
12b08421	movn w1, #0x8421, lsl #16
52800541	movz w1, #42
52800541	movz w1, #42, lsl #0
52908421	movz w1, #0x8421
52908421	movz w1, #0x8421, lsl #0
52a00541	movz w1, #42, lsl #16
52b08421	movz w1, #0x8421, lsl #16
72800541	movk w1, #42
72800541	movk w1, #42, lsl #0
72908421	movk w1, #0x8421
72908421	movk w1, #0x8421, lsl #0
72a00541	movk w1, #42, lsl #16
72b08421	movk w1, #0x8421, lsl #16
92800541	movn x1, #42
92800541	movn x1, #42, lsl #0
92908421	movn x1, #0x8421
92908421	movn x1, #0x8421, lsl #0
92a00541	movn x1, #42, lsl #16
92b08421	movn x1, #0x8421, lsl #16
92c00541	movn x1, #42, lsl #32
92d08421	movn x1, #0x8421, lsl #32
92e00541	movn x1, #42, lsl #48
92f08421	movn x1, #0x8421, lsl #48
d2800541	movz x1, #42
d2800541	movz x1, #42, lsl #0
d2908421	movz x1, #0x8421
d2908421	movz x1, #0x8421, lsl #0
d2a00541	movz x1, #42, lsl #16
d2b08421	movz x1, #0x8421, lsl #16
d2c00541	movz x1, #42, lsl #32
d2d08421	movz x1, #0x8421, lsl #32
d2e00541	movz x1, #42, lsl #48
d2f08421	movz x1, #0x8421, lsl #48
f2800541	movk x1, #42
f2800541	movk x1, #42, lsl #0
f2908421	movk x1, #0x8421
f2908421	movk x1, #0x8421, lsl #0
f2a00541	movk x1, #42, lsl #16
f2b08421	movk x1, #0x8421, lsl #16
f2c00541	movk x1, #42, lsl #32
f2d08421	movk x1, #0x8421, lsl #32
f2e00541	movk x1, #42, lsl #48
f2f08421	movk x1, #0x8421, lsl #48
";

    test_cases! {
        MOVX_DB, untested_movx_db;
        test_movk_32_42, movk(W1, 42), "movk w1, #42";
        test_movk_32_42_lsl_0, movk(W1, (42, 0.try_into().unwrap())), "movk w1, #42, lsl #0";
        test_movk_32_42_lsl_16, movk(W1, (42, 16.try_into().unwrap())), "movk w1, #42, lsl #16";
        test_movk_32_0x8421, movk(W1, 0x8421), "movk w1, #0x8421";
        test_movk_32_0x8421_lsl_0, movk(W1, (0x8421, 0.try_into().unwrap())), "movk w1, #0x8421, lsl #0";
        test_movk_32_0x8421_lsl_16, movk(W1, (0x8421, 16.try_into().unwrap())), "movk w1, #0x8421, lsl #16";

        test_movk_64_42, movk(X1, 42), "movk x1, #42";
        test_movk_64_42_lsl_0, movk(X1, (42, 0.try_into().unwrap())), "movk x1, #42, lsl #0";
        test_movk_64_42_lsl_16, movk(X1, (42, 16.try_into().unwrap())), "movk x1, #42, lsl #16";
        test_movk_64_42_lsl_32, movk(X1, (42, 32.try_into().unwrap())), "movk x1, #42, lsl #32";
        test_movk_64_42_lsl_48, movk(X1, (42, 48.try_into().unwrap())), "movk x1, #42, lsl #48";
        test_movk_64_0x8421, movk(X1, 0x8421), "movk x1, #0x8421";
        test_movk_64_0x8421_lsl_0, movk(X1, (0x8421, 0.try_into().unwrap())), "movk x1, #0x8421, lsl #0";
        test_movk_64_0x8421_lsl_16, movk(X1, (0x8421, 16.try_into().unwrap())), "movk x1, #0x8421, lsl #16";
        test_movk_64_0x8421_lsl_32, movk(X1, (0x8421, 32.try_into().unwrap())), "movk x1, #0x8421, lsl #32";
        test_movk_64_0x8421_lsl_48, movk(X1, (0x8421, 48.try_into().unwrap())), "movk x1, #0x8421, lsl #48";

        test_movn_32_42, movn(W1, 42), "movn w1, #42";
        test_movn_32_42_lsl_0, movn(W1, (42, 0.try_into().unwrap())), "movn w1, #42, lsl #0";
        test_movn_32_42_lsl_16, movn(W1, (42, 16.try_into().unwrap())), "movn w1, #42, lsl #16";
        test_movn_32_0x8421, movn(W1, 0x8421), "movn w1, #0x8421";
        test_movn_32_0x8421_lsl_0, movn(W1, (0x8421, 0.try_into().unwrap())), "movn w1, #0x8421, lsl #0";
        test_movn_32_0x8421_lsl_16, movn(W1, (0x8421, 16.try_into().unwrap())), "movn w1, #0x8421, lsl #16";

        test_movn_64_42, movn(X1, 42), "movn x1, #42";
        test_movn_64_42_lsl_0, movn(X1, (42, 0.try_into().unwrap())), "movn x1, #42, lsl #0";
        test_movn_64_42_lsl_16, movn(X1, (42, 16.try_into().unwrap())), "movn x1, #42, lsl #16";
        test_movn_64_42_lsl_32, movn(X1, (42, 32.try_into().unwrap())), "movn x1, #42, lsl #32";
        test_movn_64_42_lsl_48, movn(X1, (42, 48.try_into().unwrap())), "movn x1, #42, lsl #48";
        test_movn_64_0x8421, movn(X1, 0x8421), "movn x1, #0x8421";
        test_movn_64_0x8421_lsl_0, movn(X1, (0x8421, 0.try_into().unwrap())), "movn x1, #0x8421, lsl #0";
        test_movn_64_0x8421_lsl_16, movn(X1, (0x8421, 16.try_into().unwrap())), "movn x1, #0x8421, lsl #16";
        test_movn_64_0x8421_lsl_32, movn(X1, (0x8421, 32.try_into().unwrap())), "movn x1, #0x8421, lsl #32";
        test_movn_64_0x8421_lsl_48, movn(X1, (0x8421, 48.try_into().unwrap())), "movn x1, #0x8421, lsl #48";

        test_movz_32_42, movz(W1, 42), "movz w1, #42";
        test_movz_32_42_lsl_0, movz(W1, (42, 0.try_into().unwrap())), "movz w1, #42, lsl #0";
        test_movz_32_42_lsl_16, movz(W1, (42, 16.try_into().unwrap())), "movz w1, #42, lsl #16";
        test_movz_32_0x8421, movz(W1, 0x8421), "movz w1, #0x8421";
        test_movz_32_0x8421_lsl_0, movz(W1, (0x8421, 0.try_into().unwrap())), "movz w1, #0x8421, lsl #0";
        test_movz_32_0x8421_lsl_16, movz(W1, (0x8421, 16.try_into().unwrap())), "movz w1, #0x8421, lsl #16";

        test_movz_64_42, movz(X1, 42), "movz x1, #42";
        test_movz_64_42_lsl_0, movz(X1, (42, 0.try_into().unwrap())), "movz x1, #42, lsl #0";
        test_movz_64_42_lsl_16, movz(X1, (42, 16.try_into().unwrap())), "movz x1, #42, lsl #16";
        test_movz_64_42_lsl_32, movz(X1, (42, 32.try_into().unwrap())), "movz x1, #42, lsl #32";
        test_movz_64_42_lsl_48, movz(X1, (42, 48.try_into().unwrap())), "movz x1, #42, lsl #48";
        test_movz_64_0x8421, movz(X1, 0x8421), "movz x1, #0x8421";
        test_movz_64_0x8421_lsl_0, movz(X1, (0x8421, 0.try_into().unwrap())), "movz x1, #0x8421, lsl #0";
        test_movz_64_0x8421_lsl_16, movz(X1, (0x8421, 16.try_into().unwrap())), "movz x1, #0x8421, lsl #16";
        test_movz_64_0x8421_lsl_32, movz(X1, (0x8421, 32.try_into().unwrap())), "movz x1, #0x8421, lsl #32";
        test_movz_64_0x8421_lsl_48, movz(X1, (0x8421, 48.try_into().unwrap())), "movz x1, #0x8421, lsl #48";
    }
}
