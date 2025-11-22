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
    bits::{BitError, UBitValue},
    instructions::RawInstruction,
    outcome::{Outcome, Unfallible},
    register::{IntoReg, RegOrZero32, RegOrZero64, Register},
    sealed::Sealed,
};

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
pub struct MovImmArgs<Reg: MoveShift> {
    pub rd: Reg,
    pub imm16: MoveImm16,
    pub shift: Reg::Shift,
}

impl<Reg: MoveShift> Sealed for MovImmArgs<Reg> {}

impl<RIn: IntoReg<RegOrZero32>, Imm16: Into<MoveImm16>> MakeMovArgs<RIn, (Imm16, Shift32)>
    for MovImmArgs<RegOrZero32>
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

impl<RIn, Imm16> MakeMovArgs<RIn, (Imm16, u8)> for MovImmArgs<RegOrZero32>
where
    RIn: IntoReg<RegOrZero32>,
    Imm16: Into<MoveImm16>,
{
    type Outcome = Result<Self, BitError>;

    fn new(rd: RIn, (imm16, shift): (Imm16, u8)) -> Self::Outcome {
        (shift as u32).try_into().map(|shift| Self {
            rd: rd.into_reg(),
            imm16: imm16.into(),
            shift,
        })
    }
}

impl<RIn, Imm16> MakeMovArgs<RIn, Imm16> for MovImmArgs<RegOrZero32>
where
    RIn: IntoReg<RegOrZero32>,
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
    MakeMovArgs<RIn, (Imm16, <RegOrZero64 as MoveShift>::Shift)> for MovImmArgs<RegOrZero64>
{
    type Outcome = Unfallible<Self>;

    fn new(rd: RIn, (imm16, shift): (Imm16, Shift64)) -> Self::Outcome {
        Unfallible(Self {
            rd: rd.into_reg(),
            imm16: imm16.into(),
            shift,
        })
    }
}

impl<RIn, Imm16> MakeMovArgs<RIn, Imm16> for MovImmArgs<RegOrZero64>
where
    RIn: IntoReg<RegOrZero64>,
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

impl<RIn, Imm16> MakeMovArgs<RIn, (Imm16, u8)> for MovImmArgs<RegOrZero64>
where
    RIn: IntoReg<RegOrZero64>,
    Imm16: Into<MoveImm16>,
{
    type Outcome = Result<Self, BitError>;

    fn new(rd: RIn, (imm16, shift): (Imm16, u8)) -> Self::Outcome {
        (shift as u32).try_into().map(|shift| Self {
            rd: rd.into_reg(),
            imm16: imm16.into(),
            shift,
        })
    }
}

pub struct MovK<Args>(Args);

pub fn movk<RIn, Val, ROut>(
    rd: RIn,
    val: Val,
) -> <<MovImmArgs<ROut> as MakeMovArgs<RIn, Val>>::Outcome as Outcome>::Output<MovK<MovImmArgs<ROut>>>
where
    ROut: MoveShift,
    MovImmArgs<ROut>: MakeMovArgs<RIn, Val>,
    <MovImmArgs<ROut> as MakeMovArgs<RIn, Val>>::Outcome: Outcome<Inner = MovImmArgs<ROut>>,
{
    (<MovImmArgs<ROut> as MakeMovArgs<RIn, Val>>::new(rd, val)).map(MovK)
}

impl RawInstruction for MovK<MovImmArgs<RegOrZero32>> {
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        MOVK_32_movewide(self.0.shift.into(), self.0.imm16.into(), self.0.rd.index())
    }
}

impl RawInstruction for MovK<MovImmArgs<RegOrZero64>> {
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        MOVK_64_movewide(self.0.shift.into(), self.0.imm16.into(), self.0.rd.index())
    }
}

pub struct MovN<Args>(Args);

pub fn movn<RIn, Val, ROut>(
    rd: RIn,
    val: Val,
) -> <<MovImmArgs<ROut> as MakeMovArgs<RIn, Val>>::Outcome as Outcome>::Output<MovN<MovImmArgs<ROut>>>
where
    ROut: MoveShift,
    MovImmArgs<ROut>: MakeMovArgs<RIn, Val>,
    <MovImmArgs<ROut> as MakeMovArgs<RIn, Val>>::Outcome: Outcome<Inner = MovImmArgs<ROut>>,
{
    <MovImmArgs<ROut> as MakeMovArgs<RIn, Val>>::new(rd, val).map(MovN)
}

impl RawInstruction for MovN<MovImmArgs<RegOrZero32>> {
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        MOVN_32_movewide(self.0.shift.into(), self.0.imm16.into(), self.0.rd.index())
    }
}

impl RawInstruction for MovN<MovImmArgs<RegOrZero64>> {
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        MOVN_64_movewide(self.0.shift.into(), self.0.imm16.into(), self.0.rd.index())
    }
}

pub struct MovZ<Args>(Args);

pub fn movz<RIn, Val, ROut>(
    rd: RIn,
    val: Val,
) -> <<MovImmArgs<ROut> as MakeMovArgs<RIn, Val>>::Outcome as Outcome>::Output<MovZ<MovImmArgs<ROut>>>
where
    ROut: MoveShift,
    MovImmArgs<ROut>: MakeMovArgs<RIn, Val>,
    <MovImmArgs<ROut> as MakeMovArgs<RIn, Val>>::Outcome: Outcome<Inner = MovImmArgs<ROut>>,
{
    <MovImmArgs<ROut> as MakeMovArgs<RIn, Val>>::new(rd, val).map(MovZ)
}

impl RawInstruction for MovZ<MovImmArgs<RegOrZero32>> {
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        MOVZ_32_movewide(self.0.shift.into(), self.0.imm16.into(), self.0.rd.index())
    }
}

impl RawInstruction for MovZ<MovImmArgs<RegOrZero64>> {
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        MOVZ_64_movewide(self.0.shift.into(), self.0.imm16.into(), self.0.rd.index())
    }
}

#[cfg(test)]
mod tests {
    use crate::instructions::InstructionSeq;
    use crate::register::Reg32::*;
    use crate::register::Reg64::*;
    use crate::register::RegOrZero32::WZR;
    use crate::register::RegOrZero64::XZR;
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
5280055f	movz wzr, #42
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
d280055f	movz xzr, #42
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
        test_movk_32_42_lsl_0, movk(W1, (42, UBitValue::new(0).unwrap())), "movk w1, #42, lsl #0";
        test_movk_32_42_lsl_16, movk(W1, (42, UBitValue::new(16).unwrap())), "movk w1, #42, lsl #16";
        test_movk_32_0x8421, movk(W1, 0x8421), "movk w1, #0x8421";
        test_movk_32_0x8421_lsl_0, movk(W1, (0x8421, UBitValue::new(0).unwrap())), "movk w1, #0x8421, lsl #0";
        test_movk_32_0x8421_lsl_16, movk(W1, (0x8421, UBitValue::new(16).unwrap())), "movk w1, #0x8421, lsl #16";
        test_movk_32_0x8421_lsl_16_fallible, movk(W1, (0x8421, 16)).unwrap(), "movk w1, #0x8421, lsl #16";

        test_movk_64_42, movk(X1, 42), "movk x1, #42";
        test_movk_64_42_lsl_0, movk(X1, (42, UBitValue::new(0).unwrap())), "movk x1, #42, lsl #0";
        test_movk_64_42_lsl_16, movk(X1, (42, UBitValue::new(16).unwrap())), "movk x1, #42, lsl #16";
        test_movk_64_42_lsl_32, movk(X1, (42, UBitValue::new(32).unwrap())), "movk x1, #42, lsl #32";
        test_movk_64_42_lsl_48, movk(X1, (42, UBitValue::new(48).unwrap())), "movk x1, #42, lsl #48";
        test_movk_64_0x8421, movk(X1, 0x8421), "movk x1, #0x8421";
        test_movk_64_0x8421_lsl_0, movk(X1, (0x8421, UBitValue::new(0).unwrap())), "movk x1, #0x8421, lsl #0";
        test_movk_64_0x8421_lsl_16, movk(X1, (0x8421, UBitValue::new(16).unwrap())), "movk x1, #0x8421, lsl #16";
        test_movk_64_0x8421_lsl_32, movk(X1, (0x8421, UBitValue::new(32).unwrap())), "movk x1, #0x8421, lsl #32";
        test_movk_64_0x8421_lsl_48, movk(X1, (0x8421, UBitValue::new(48).unwrap())), "movk x1, #0x8421, lsl #48";
        test_movk_64_0x8421_lsl_48_fallible, movk(X1, (0x8421, 48)).unwrap(), "movk x1, #0x8421, lsl #48";

        test_movn_32_42, movn(W1, 42), "movn w1, #42";
        test_movn_32_42_lsl_0, movn(W1, (42, UBitValue::new(0).unwrap())), "movn w1, #42, lsl #0";
        test_movn_32_42_lsl_16, movn(W1, (42, UBitValue::new(16).unwrap())), "movn w1, #42, lsl #16";
        test_movn_32_0x8421, movn(W1, 0x8421), "movn w1, #0x8421";
        test_movn_32_0x8421_lsl_0, movn(W1, (0x8421, UBitValue::new(0).unwrap())), "movn w1, #0x8421, lsl #0";
        test_movn_32_0x8421_lsl_16, movn(W1, (0x8421, UBitValue::new(16).unwrap())), "movn w1, #0x8421, lsl #16";
        test_movn_32_0x8421_lsl_16_fallible, movn(W1, (0x8421, 16)).unwrap(), "movn w1, #0x8421, lsl #16";

        test_movn_64_42, movn(X1, 42), "movn x1, #42";
        test_movn_64_42_lsl_0, movn(X1, (42, UBitValue::new(0).unwrap())), "movn x1, #42, lsl #0";
        test_movn_64_42_lsl_16, movn(X1, (42, UBitValue::new(16).unwrap())), "movn x1, #42, lsl #16";
        test_movn_64_42_lsl_32, movn(X1, (42, UBitValue::new(32).unwrap())), "movn x1, #42, lsl #32";
        test_movn_64_42_lsl_48, movn(X1, (42, UBitValue::new(48).unwrap())), "movn x1, #42, lsl #48";
        test_movn_64_0x8421, movn(X1, 0x8421), "movn x1, #0x8421";
        test_movn_64_0x8421_lsl_0, movn(X1, (0x8421, UBitValue::new(0).unwrap())), "movn x1, #0x8421, lsl #0";
        test_movn_64_0x8421_lsl_16, movn(X1, (0x8421, UBitValue::new(16).unwrap())), "movn x1, #0x8421, lsl #16";
        test_movn_64_0x8421_lsl_32, movn(X1, (0x8421, UBitValue::new(32).unwrap())), "movn x1, #0x8421, lsl #32";
        test_movn_64_0x8421_lsl_48, movn(X1, (0x8421, UBitValue::new(48).unwrap())), "movn x1, #0x8421, lsl #48";
        test_movn_64_0x8421_lsl_48_fallible, movn(X1, (0x8421, 48)).unwrap(), "movn x1, #0x8421, lsl #48";

        test_movz_32_42, movz(W1, 42), "movz w1, #42";
        test_movz_wzr_42, movz(WZR, 42), "movz wzr, #42";
        test_movz_32_42_lsl_0, movz(W1, (42, UBitValue::new(0).unwrap())), "movz w1, #42, lsl #0";
        test_movz_32_42_lsl_16, movz(W1, (42, UBitValue::new(16).unwrap())), "movz w1, #42, lsl #16";
        test_movz_32_0x8421, movz(W1, 0x8421), "movz w1, #0x8421";
        test_movz_32_0x8421_lsl_0, movz(W1, (0x8421, UBitValue::new(0).unwrap())), "movz w1, #0x8421, lsl #0";
        test_movz_32_0x8421_lsl_16, movz(W1, (0x8421, UBitValue::new(16).unwrap())), "movz w1, #0x8421, lsl #16";
        test_movz_32_0x8421_lsl_16_fallible, movz(W1, (0x8421, 16)).unwrap(), "movz w1, #0x8421, lsl #16";

        test_movz_64_42, movz(X1, 42), "movz x1, #42";
        test_movz_xzr_42, movz(XZR, 42), "movz xzr, #42";
        test_movz_64_42_lsl_0, movz(X1, (42, UBitValue::new(0).unwrap())), "movz x1, #42, lsl #0";
        test_movz_64_42_lsl_16, movz(X1, (42, UBitValue::new(16).unwrap())), "movz x1, #42, lsl #16";
        test_movz_64_42_lsl_32, movz(X1, (42, UBitValue::new(32).unwrap())), "movz x1, #42, lsl #32";
        test_movz_64_42_lsl_48, movz(X1, (42, UBitValue::new(48).unwrap())), "movz x1, #42, lsl #48";
        test_movz_64_0x8421, movz(X1, 0x8421), "movz x1, #0x8421";
        test_movz_64_0x8421_lsl_0, movz(X1, (0x8421, UBitValue::new(0).unwrap())), "movz x1, #0x8421, lsl #0";
        test_movz_64_0x8421_lsl_16, movz(X1, (0x8421, UBitValue::new(16).unwrap())), "movz x1, #0x8421, lsl #16";
        test_movz_64_0x8421_lsl_32, movz(X1, (0x8421, UBitValue::new(32).unwrap())), "movz x1, #0x8421, lsl #32";
        test_movz_64_0x8421_lsl_48, movz(X1, (0x8421, UBitValue::new(48).unwrap())), "movz x1, #0x8421, lsl #48";
        test_movz_64_0x8421_lsl_48_unwrap, movz(X1, (0x8421, 48)).unwrap(), "movz x1, #0x8421, lsl #48";
    }
}
