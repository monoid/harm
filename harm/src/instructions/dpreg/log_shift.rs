/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use aarchmrs_instructions::A64::dpreg::log_shift::{
    AND_32_log_shift::AND_32_log_shift, AND_64_log_shift::AND_64_log_shift,
    ANDS_32_log_shift::ANDS_32_log_shift, ANDS_64_log_shift::ANDS_64_log_shift,
};

use crate::{
    bits::BitError,
    instructions::{
        RawInstruction,
        logical::{And, Ands, LogicalArgs, LogicalShift, LogicalShiftAmount, MakeZeroLogicalArgs},
    },
    outcome::Unfallible,
    register::{RegOrZero32, RegOrZero64, Register as _},
};

// TODO implement a mixin here, and use an autoimpl for MakeZeroLogicalArgs/MakeSpLogicalArgs. The mixin should be
// sealed?

impl<RdIn, RnIn, RsIn> MakeZeroLogicalArgs<RdIn, RnIn, RsIn>
    for LogicalArgs<RegOrZero32, RegOrZero32, (RegOrZero32, LogicalShift, LogicalShiftAmount)>
where
    RdIn: Into<RegOrZero32>,
    RnIn: Into<RegOrZero32>,
    RsIn: Into<RegOrZero32>,
{
    type Outcome = Unfallible<
        LogicalArgs<RegOrZero32, RegOrZero32, (RegOrZero32, LogicalShift, LogicalShiftAmount)>,
    >;

    fn new(rd: RdIn, rn: RnIn, mask: RsIn) -> Self::Outcome {
        Unfallible(Self {
            rd: rd.into(),
            rn: rn.into(),
            mask: (
                mask.into(),
                LogicalShift::LSL,
                LogicalShiftAmount::default(),
            ),
        })
    }
}

impl<RdIn, RnIn, RsIn> MakeZeroLogicalArgs<RdIn, RnIn, (RsIn, LogicalShift, u8)>
    for LogicalArgs<RegOrZero32, RegOrZero32, (RegOrZero32, LogicalShift, LogicalShiftAmount)>
where
    RdIn: Into<RegOrZero32>,
    RnIn: Into<RegOrZero32>,
    RsIn: Into<RegOrZero32>,
{
    type Outcome = Result<
        LogicalArgs<RegOrZero32, RegOrZero32, (RegOrZero32, LogicalShift, LogicalShiftAmount)>,
        BitError,
    >;

    fn new(rd: RdIn, rn: RnIn, (mask, shift, amount): (RsIn, LogicalShift, u8)) -> Self::Outcome {
        u32::from(amount).try_into().map(|amount| Self {
            rd: rd.into(),
            rn: rn.into(),
            mask: (mask.into(), shift, amount),
        })
    }
}

impl<RdIn, RnIn, RsIn> MakeZeroLogicalArgs<RdIn, RnIn, RsIn>
    for LogicalArgs<RegOrZero64, RegOrZero64, (RegOrZero64, LogicalShift, LogicalShiftAmount)>
where
    RdIn: Into<RegOrZero64>,
    RnIn: Into<RegOrZero64>,
    RsIn: Into<RegOrZero64>,
{
    type Outcome = Unfallible<
        LogicalArgs<RegOrZero64, RegOrZero64, (RegOrZero64, LogicalShift, LogicalShiftAmount)>,
    >;

    fn new(rd: RdIn, rn: RnIn, mask: RsIn) -> Self::Outcome {
        Unfallible(Self {
            rd: rd.into(),
            rn: rn.into(),
            mask: (
                mask.into(),
                LogicalShift::LSL,
                LogicalShiftAmount::default(),
            ),
        })
    }
}

impl<RdIn, RnIn, RsIn> MakeZeroLogicalArgs<RdIn, RnIn, (RsIn, LogicalShift, u8)>
    for LogicalArgs<RegOrZero64, RegOrZero64, (RegOrZero64, LogicalShift, LogicalShiftAmount)>
where
    RdIn: Into<RegOrZero64>,
    RnIn: Into<RegOrZero64>,
    RsIn: Into<RegOrZero64>,
{
    type Outcome = Result<
        LogicalArgs<RegOrZero64, RegOrZero64, (RegOrZero64, LogicalShift, LogicalShiftAmount)>,
        BitError,
    >;

    fn new(rd: RdIn, rn: RnIn, (mask, shift, amount): (RsIn, LogicalShift, u8)) -> Self::Outcome {
        u32::from(amount).try_into().map(|amount| Self {
            rd: rd.into(),
            rn: rn.into(),
            mask: (mask.into(), shift, amount),
        })
    }
}

impl RawInstruction
    for And<LogicalArgs<RegOrZero32, RegOrZero32, (RegOrZero32, LogicalShift, LogicalShiftAmount)>>
{
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let args = &self.0;
        let (mask, shift, amount) = args.mask;
        AND_32_log_shift(
            (shift as u32).into(),
            mask.code(),
            amount.into(),
            args.rn.code(),
            args.rd.code(),
        )
    }
}

impl RawInstruction
    for And<LogicalArgs<RegOrZero64, RegOrZero64, (RegOrZero64, LogicalShift, LogicalShiftAmount)>>
{
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let args = &self.0;
        let (mask, shift, amount) = args.mask;
        AND_64_log_shift(
            (shift as u32).into(),
            mask.code(),
            amount.into(),
            args.rn.code(),
            args.rd.code(),
        )
    }
}

impl RawInstruction
    for Ands<LogicalArgs<RegOrZero32, RegOrZero32, (RegOrZero32, LogicalShift, LogicalShiftAmount)>>
{
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let args = &self.0;
        let (mask, shift, amount) = args.mask;
        ANDS_32_log_shift(
            (shift as u32).into(),
            mask.code(),
            amount.into(),
            args.rn.code(),
            args.rd.code(),
        )
    }
}

impl RawInstruction
    for Ands<LogicalArgs<RegOrZero64, RegOrZero64, (RegOrZero64, LogicalShift, LogicalShiftAmount)>>
{
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let args = &self.0;
        let (mask, shift, amount) = args.mask;
        ANDS_64_log_shift(
            (shift as u32).into(),
            mask.code(),
            amount.into(),
            args.rn.code(),
            args.rd.code(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::instructions::InstructionSeq as _;
    use harm_test_utils::test_cases;

    use crate::instructions::logical::ands;
    use crate::register::Reg32::*;
    use crate::register::Reg64::*;

    const LOG_SHIFT_DB: &str = "
";

    test_cases! {
        LOG_SHIFT_DB, unchecked_log_shift_db;
        test_and_32, ands(W1, W2, W3), "ands w1, w2, w3";
        test_and_64, ands(X1, X2, X3), "ands x1, x2, x3";
    }
}
