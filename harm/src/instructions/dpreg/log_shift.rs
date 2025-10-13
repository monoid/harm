/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use aarchmrs_instructions::A64::dpreg::log_shift::{
    AND_32_log_shift::AND_32_log_shift, AND_64_log_shift::AND_64_log_shift,
    ANDS_32_log_shift::ANDS_32_log_shift, ANDS_64_log_shift::ANDS_64_log_shift,
    EOR_32_log_shift::EOR_32_log_shift, EOR_64_log_shift::EOR_64_log_shift,
    ORR_32_log_shift::ORR_32_log_shift, ORR_64_log_shift::ORR_64_log_shift,
};

use crate::{
    bits::BitError,
    instructions::{
        RawInstruction,
        logical::{
            And, Ands, Eor, LogicalArgs, LogicalShift, LogicalShiftAmount32, LogicalShiftAmount64,
            MakeSpLogicalArgs, MakeTstLogicalArgs, MakeZeroLogicalArgs, Orr,
        },
    },
    outcome::Unfallible,
    register::{RegOrZero32, RegOrZero64, Register as _},
};

macro_rules! define_logical_args {
    ($struct:ident, $trait:ident, $rn:ty, $amount:ident) => {
        impl<RdIn, RnIn, RsIn> $trait<RdIn, RnIn, RsIn>
            for $struct<$rn, $rn, ($rn, LogicalShift, $amount)>
        where
            RdIn: Into<$rn>,
            RnIn: Into<$rn>,
            RsIn: Into<$rn>,
        {
            type Outcome = Unfallible<$struct<$rn, $rn, ($rn, LogicalShift, $amount)>>;

            fn new(rd: RdIn, rn: RnIn, mask: RsIn) -> Self::Outcome {
                Unfallible(Self {
                    rd: rd.into(),
                    rn: rn.into(),
                    mask: (mask.into(), LogicalShift::LSL, $amount::default()),
                })
            }
        }

        impl<RdIn, RnIn, RsIn> $trait<RdIn, RnIn, (RsIn, LogicalShift, u8)>
            for $struct<$rn, $rn, ($rn, LogicalShift, $amount)>
        where
            RdIn: Into<$rn>,
            RnIn: Into<$rn>,
            RsIn: Into<$rn>,
        {
            type Outcome = Result<LogicalArgs<$rn, $rn, ($rn, LogicalShift, $amount)>, BitError>;

            fn new(
                rd: RdIn,
                rn: RnIn,
                (mask, shift, amount): (RsIn, LogicalShift, u8),
            ) -> Self::Outcome {
                u32::from(amount).try_into().map(|amount| Self {
                    rd: rd.into(),
                    rn: rn.into(),
                    mask: (mask.into(), shift, amount),
                })
            }
        }
    };
}

macro_rules! define_logical_args_tst {
    ($struct:ident, $trait:ident, $rn:ty, $zero:expr, $amount:ident) => {
        impl<RnIn, RsIn> $trait<RnIn, RsIn> for $struct<$rn, $rn, ($rn, LogicalShift, $amount)>
        where
            RnIn: Into<$rn>,
            RsIn: Into<$rn>,
        {
            type Outcome = Unfallible<$struct<$rn, $rn, ($rn, LogicalShift, $amount)>>;

            fn new(rn: RnIn, mask: RsIn) -> Self::Outcome {
                Unfallible(Self {
                    rd: $zero,
                    rn: rn.into(),
                    mask: (mask.into(), LogicalShift::LSL, $amount::default()),
                })
            }
        }
        impl<RnIn, RsIn> $trait<RnIn, (RsIn, LogicalShift, u8)>
            for LogicalArgs<$rn, $rn, ($rn, LogicalShift, $amount)>
        where
            RnIn: Into<$rn>,
            RsIn: Into<$rn>,
        {
            type Outcome = Result<$struct<$rn, $rn, ($rn, LogicalShift, $amount)>, BitError>;

            fn new(rn: RnIn, (mask, shift, amount): (RsIn, LogicalShift, u8)) -> Self::Outcome {
                u32::from(amount).try_into().map(|amount| Self {
                    rd: $zero,
                    rn: rn.into(),
                    mask: (mask.into(), shift, amount),
                })
            }
        }
    };
}

define_logical_args!(
    LogicalArgs,
    MakeZeroLogicalArgs,
    RegOrZero32,
    LogicalShiftAmount32
);
define_logical_args!(
    LogicalArgs,
    MakeZeroLogicalArgs,
    RegOrZero64,
    LogicalShiftAmount64
);
define_logical_args!(
    LogicalArgs,
    MakeSpLogicalArgs,
    RegOrZero32,
    LogicalShiftAmount32
);
define_logical_args!(
    LogicalArgs,
    MakeSpLogicalArgs,
    RegOrZero64,
    LogicalShiftAmount64
);

define_logical_args_tst!(
    LogicalArgs,
    MakeTstLogicalArgs,
    RegOrZero32,
    RegOrZero32::WZR,
    LogicalShiftAmount32
);
define_logical_args_tst!(
    LogicalArgs,
    MakeTstLogicalArgs,
    RegOrZero64,
    RegOrZero64::XZR,
    LogicalShiftAmount64
);

impl RawInstruction
    for And<
        LogicalArgs<RegOrZero32, RegOrZero32, (RegOrZero32, LogicalShift, LogicalShiftAmount32)>,
    >
{
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let args = &self.0;
        let (mask, shift, amount) = args.mask;
        AND_32_log_shift(
            (shift as u32).into(),
            mask.code(),
            amount.bits().into(),
            args.rn.code(),
            args.rd.code(),
        )
    }
}

impl RawInstruction
    for And<
        LogicalArgs<RegOrZero64, RegOrZero64, (RegOrZero64, LogicalShift, LogicalShiftAmount64)>,
    >
{
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let args = &self.0;
        let (mask, shift, amount) = args.mask;
        AND_64_log_shift(
            (shift as u32).into(),
            mask.code(),
            amount.bits().into(),
            args.rn.code(),
            args.rd.code(),
        )
    }
}

impl RawInstruction
    for Ands<
        LogicalArgs<RegOrZero32, RegOrZero32, (RegOrZero32, LogicalShift, LogicalShiftAmount32)>,
    >
{
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let args = &self.0;
        let (mask, shift, amount) = args.mask;
        ANDS_32_log_shift(
            (shift as u32).into(),
            mask.code(),
            amount.bits().into(),
            args.rn.code(),
            args.rd.code(),
        )
    }
}

impl RawInstruction
    for Ands<
        LogicalArgs<RegOrZero64, RegOrZero64, (RegOrZero64, LogicalShift, LogicalShiftAmount64)>,
    >
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

impl RawInstruction
    for Eor<
        LogicalArgs<RegOrZero32, RegOrZero32, (RegOrZero32, LogicalShift, LogicalShiftAmount32)>,
    >
{
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let args = &self.0;
        let (mask, shift, amount) = args.mask;
        EOR_32_log_shift(
            (shift as u32).into(),
            mask.code(),
            amount.bits().into(),
            args.rn.code(),
            args.rd.code(),
        )
    }
}

impl RawInstruction
    for Eor<
        LogicalArgs<RegOrZero64, RegOrZero64, (RegOrZero64, LogicalShift, LogicalShiftAmount64)>,
    >
{
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let args = &self.0;
        let (mask, shift, amount) = args.mask;
        EOR_64_log_shift(
            (shift as u32).into(),
            mask.code(),
            amount.into(),
            args.rn.code(),
            args.rd.code(),
        )
    }
}

impl RawInstruction
    for Orr<
        LogicalArgs<RegOrZero32, RegOrZero32, (RegOrZero32, LogicalShift, LogicalShiftAmount32)>,
    >
{
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let args = &self.0;
        let (mask, shift, amount) = args.mask;
        ORR_32_log_shift(
            (shift as u32).into(),
            mask.code(),
            amount.bits().into(),
            args.rn.code(),
            args.rd.code(),
        )
    }
}

impl RawInstruction
    for Orr<
        LogicalArgs<RegOrZero64, RegOrZero64, (RegOrZero64, LogicalShift, LogicalShiftAmount64)>,
    >
{
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let args = &self.0;
        let (mask, shift, amount) = args.mask;
        ORR_64_log_shift(
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
    use harm_test_utils::test_cases;

    use super::LogicalShift::*;
    use crate::instructions::InstructionSeq as _;
    use crate::instructions::logical::{and, ands, eor, orr, tst};
    use crate::register::Reg32::*;
    use crate::register::Reg64::*;
    use crate::register::RegOrZero32::WZR;
    use crate::register::RegOrZero64::XZR;

    const LOG_SHIFT_DB: &str = "
8a0a0303	and x3, x24, x10
8a0a3b03	and x3, x24, x10, lsl 14
8a4adb03	and x3, x24, x10, lsr 54
8a8a8b03	and x3, x24, x10, asr 34
8aca9b03	and x3, x24, x10, ror 38
8a1f03ff	and xzr, xzr, xzr
8a1fafff	and xzr, xzr, xzr, lsl 43
0a1d029a	and w26, w20, w29
0a1d069a	and w26, w20, w29, lsl 1
0a5d4a9a	and w26, w20, w29, lsr 18
0a9d129a	and w26, w20, w29, asr 4
0add2a9a	and w26, w20, w29, ror 10
0a1f03ff	and wzr, wzr, wzr
0a5f57ff	and wzr, wzr, wzr, lsr 21
ea020068	ands x8, x3, x2
ea020468	ands x8, x3, x2, lsl 1
ea427868	ands x8, x3, x2, lsr 30
ea821068	ands x8, x3, x2, asr 4
eac22468	ands x8, x3, x2, ror 9
ea1f03ff	ands xzr, xzr, xzr
ea9fe7ff	ands xzr, xzr, xzr, asr 57
6a1901aa	ands w10, w13, w25
6a1979aa	ands w10, w13, w25, lsl 30
6a5925aa	ands w10, w13, w25, lsr 9
6a991daa	ands w10, w13, w25, asr 7
6ad90daa	ands w10, w13, w25, ror 3
6a1f03ff	ands wzr, wzr, wzr
6adf1bff	ands wzr, wzr, wzr, ror 6
ca0f01c0	eor x0, x14, x15
ca0ff5c0	eor x0, x14, x15, lsl 61
ca4f39c0	eor x0, x14, x15, lsr 14
ca8fcdc0	eor x0, x14, x15, asr 51
cacfd5c0	eor x0, x14, x15, ror 53
ca1f03ff	eor xzr, xzr, xzr
ca5f9fff	eor xzr, xzr, xzr, lsr 39
4a14038e	eor w14, w28, w20
4a140f8e	eor w14, w28, w20, lsl 3
4a540b8e	eor w14, w28, w20, lsr 2
4a942b8e	eor w14, w28, w20, asr 10
4ad4478e	eor w14, w28, w20, ror 17
4a1f03ff	eor wzr, wzr, wzr
4a9f2fff	eor wzr, wzr, wzr, asr 11
aa07018c	orr x12, x12, x7
aa078d8c	orr x12, x12, x7, lsl 35
aa47758c	orr x12, x12, x7, lsr 29
aa87d98c	orr x12, x12, x7, asr 54
aac76d8c	orr x12, x12, x7, ror 27
aa1f03ff	orr xzr, xzr, xzr
aadf4fff	orr xzr, xzr, xzr, ror 19
2a0a0088	orr w8, w4, w10
2a0a0088	orr w8, w4, w10, lsl 0
2a4a3488	orr w8, w4, w10, lsr 13
2a8a2c88	orr w8, w4, w10, asr 11
2aca4888	orr w8, w4, w10, ror 18
2a1f03ff	orr wzr, wzr, wzr
2a1f3bff	orr wzr, wzr, wzr, lsl 14
ea1b031f	tst x24, x27
ea1b531f	tst x24, x27, lsl 20
ea5b1f1f	tst x24, x27, lsr 7
ea9b871f	tst x24, x27, asr 33
eadba71f	tst x24, x27, ror 41
ea1f03ff	tst xzr, xzr
ea5f17ff	tst xzr, xzr, lsr 5
eadf97ff	tst xzr, xzr, ror 37
6a0f001f	tst w0, w15
6a0f7c1f	tst w0, w15, lsl 31
6a4f781f	tst w0, w15, lsr 30
6a8f401f	tst w0, w15, asr 16
6acf341f	tst w0, w15, ror 13
6a1f03ff	tst wzr, wzr
6a1f67ff	tst wzr, wzr, lsl 25
6a9f6fff	tst wzr, wzr, asr 27
";

    test_cases! {
        LOG_SHIFT_DB, unchecked_log_shift_db;
        test_and_64, and(X3, X24, X10), "and x3, x24, x10";
        test_and_64_lsl, and(X3, X24, (X10, LSL, 14)).unwrap(), "and x3, x24, x10, lsl 14";
        test_and_64_lsr, and(X3, X24, (X10, LSR, 54)).unwrap(), "and x3, x24, x10, lsr 54";
        test_and_64_asr, and(X3, X24, (X10, ASR, 34)).unwrap(), "and x3, x24, x10, asr 34";
        test_and_64_ror, and(X3, X24, (X10, ROR, 38)).unwrap(), "and x3, x24, x10, ror 38";
        test_and_xzr, and(XZR, XZR, XZR), "and xzr, xzr, xzr";
        test_and_xzr_lsl, and(XZR, XZR, (XZR, LSL, 43)).unwrap(), "and xzr, xzr, xzr, lsl 43";
        test_and_32, and(W26, W20, W29), "and w26, w20, w29";
        test_and_32_lsl, and(W26, W20, (W29, LSL, 1)).unwrap(), "and w26, w20, w29, lsl 1";
        test_and_32_lsr, and(W26, W20, (W29, LSR, 18)).unwrap(), "and w26, w20, w29, lsr 18";
        test_and_32_asr, and(W26, W20, (W29, ASR, 4)).unwrap(), "and w26, w20, w29, asr 4";
        test_and_32_ror, and(W26, W20, (W29, ROR, 10)).unwrap(), "and w26, w20, w29, ror 10";
        test_and_wzr, and(WZR, WZR, WZR), "and wzr, wzr, wzr";
        test_and_wzr_lsr, and(WZR, WZR, (WZR, LSR, 21)).unwrap(), "and wzr, wzr, wzr, lsr 21";
        test_ands_64, ands(X8, X3, X2), "ands x8, x3, x2";
        test_ands_64_lsl, ands(X8, X3, (X2, LSL, 1)).unwrap(), "ands x8, x3, x2, lsl 1";
        test_ands_64_lsr, ands(X8, X3, (X2, LSR, 30)).unwrap(), "ands x8, x3, x2, lsr 30";
        test_ands_64_asr, ands(X8, X3, (X2, ASR, 4)).unwrap(), "ands x8, x3, x2, asr 4";
        test_ands_64_ror, ands(X8, X3, (X2, ROR, 9)).unwrap(), "ands x8, x3, x2, ror 9";
        test_ands_xzr, ands(XZR, XZR, XZR), "ands xzr, xzr, xzr";
        test_ands_xzr_asr, ands(XZR, XZR, (XZR, ASR, 57)).unwrap(), "ands xzr, xzr, xzr, asr 57";
        test_ands_32, ands(W10, W13, W25), "ands w10, w13, w25";
        test_ands_32_lsl, ands(W10, W13, (W25, LSL, 30)).unwrap(), "ands w10, w13, w25, lsl 30";
        test_ands_32_lsr, ands(W10, W13, (W25, LSR, 9)).unwrap(), "ands w10, w13, w25, lsr 9";
        test_ands_32_asr, ands(W10, W13, (W25, ASR, 7)).unwrap(), "ands w10, w13, w25, asr 7";
        test_ands_32_ror, ands(W10, W13, (W25, ROR, 3)).unwrap(), "ands w10, w13, w25, ror 3";
        test_ands_wzr, ands(WZR, WZR, WZR), "ands wzr, wzr, wzr";
        test_ands_wzr_ror, ands(WZR, WZR, (WZR, ROR, 6)).unwrap(), "ands wzr, wzr, wzr, ror 6";
        test_eor_64, eor(X0, X14, X15), "eor x0, x14, x15";
        test_eor_64_lsl, eor(X0, X14, (X15, LSL, 61)).unwrap(), "eor x0, x14, x15, lsl 61";
        test_eor_64_lsr, eor(X0, X14, (X15, LSR, 14)).unwrap(), "eor x0, x14, x15, lsr 14";
        test_eor_64_asr, eor(X0, X14, (X15, ASR, 51)).unwrap(), "eor x0, x14, x15, asr 51";
        test_eor_64_ror, eor(X0, X14, (X15, ROR, 53)).unwrap(), "eor x0, x14, x15, ror 53";
        test_eor_xzr, eor(XZR, XZR, XZR), "eor xzr, xzr, xzr";
        test_eor_xzr_lsr, eor(XZR, XZR, (XZR, LSR, 39)).unwrap(), "eor xzr, xzr, xzr, lsr 39";
        test_eor_32, eor(W14, W28, W20), "eor w14, w28, w20";
        test_eor_32_lsl, eor(W14, W28, (W20, LSL, 3)).unwrap(), "eor w14, w28, w20, lsl 3";
        test_eor_32_lsr, eor(W14, W28, (W20, LSR, 2)).unwrap(), "eor w14, w28, w20, lsr 2";
        test_eor_32_asr, eor(W14, W28, (W20, ASR, 10)).unwrap(), "eor w14, w28, w20, asr 10";
        test_eor_32_ror, eor(W14, W28, (W20, ROR, 17)).unwrap(), "eor w14, w28, w20, ror 17";
        test_eor_wzr, eor(WZR, WZR, WZR), "eor wzr, wzr, wzr";
        test_eor_wzr_asr, eor(WZR, WZR, (WZR, ASR, 11)).unwrap(), "eor wzr, wzr, wzr, asr 11";
        test_orr_64, orr(X12, X12, X7), "orr x12, x12, x7";
        test_orr_64_lsl, orr(X12, X12, (X7, LSL, 35)).unwrap(), "orr x12, x12, x7, lsl 35";
        test_orr_64_lsr, orr(X12, X12, (X7, LSR, 29)).unwrap(), "orr x12, x12, x7, lsr 29";
        test_orr_64_asr, orr(X12, X12, (X7, ASR, 54)).unwrap(), "orr x12, x12, x7, asr 54";
        test_orr_64_ror, orr(X12, X12, (X7, ROR, 27)).unwrap(), "orr x12, x12, x7, ror 27";
        test_orr_xzr, orr(XZR, XZR, XZR), "orr xzr, xzr, xzr";
        test_orr_xzr_ror, orr(XZR, XZR, (XZR, ROR, 19)).unwrap(), "orr xzr, xzr, xzr, ror 19";
        test_orr_32, orr(W8, W4, W10), "orr w8, w4, w10";
        test_orr_32_lsl, orr(W8, W4, (W10, LSL, 0)).unwrap(), "orr w8, w4, w10, lsl 0";
        test_orr_32_lsr, orr(W8, W4, (W10, LSR, 13)).unwrap(), "orr w8, w4, w10, lsr 13";
        test_orr_32_asr, orr(W8, W4, (W10, ASR, 11)).unwrap(), "orr w8, w4, w10, asr 11";
        test_orr_32_ror, orr(W8, W4, (W10, ROR, 18)).unwrap(), "orr w8, w4, w10, ror 18";
        test_orr_wzr, orr(WZR, WZR, WZR), "orr wzr, wzr, wzr";
        test_orr_wzr_lsl, orr(WZR, WZR, (WZR, LSL, 14)).unwrap(), "orr wzr, wzr, wzr, lsl 14";
        test_tst_64, tst(X24, X27), "tst x24, x27";
        test_tst_64_lsl, tst(X24, (X27, LSL, 20)).unwrap(), "tst x24, x27, lsl 20";
        test_tst_64_lsr, tst(X24, (X27, LSR, 7)).unwrap(), "tst x24, x27, lsr 7";
        test_tst_64_asr, tst(X24, (X27, ASR, 33)).unwrap(), "tst x24, x27, asr 33";
        test_tst_64_ror, tst(X24, (X27, ROR, 41)).unwrap(), "tst x24, x27, ror 41";
        test_tst_xzr, tst(XZR, XZR), "tst xzr, xzr";
        test_tst_xzr_lsr, tst(XZR, (XZR, LSR, 5)).unwrap(), "tst xzr, xzr, lsr 5";
        test_tst_xzr_ror, tst(XZR, (XZR, ROR, 37)).unwrap(), "tst xzr, xzr, ror 37";
        test_tst_32, tst(W0, W15), "tst w0, w15";
        test_tst_32_lsl, tst(W0, (W15, LSL, 31)).unwrap(), "tst w0, w15, lsl 31";
        test_tst_32_lsr, tst(W0, (W15, LSR, 30)).unwrap(), "tst w0, w15, lsr 30";
        test_tst_32_asr, tst(W0, (W15, ASR, 16)).unwrap(), "tst w0, w15, asr 16";
        test_tst_32_ror, tst(W0, (W15, ROR, 13)).unwrap(), "tst w0, w15, ror 13";
        test_tst_wzr, tst(WZR, WZR), "tst wzr, wzr";
        test_tst_wzr_lsl, tst(WZR, (WZR, LSL, 25)).unwrap(), "tst wzr, wzr, lsl 25";
        test_tst_wzr_asr, tst(WZR, (WZR, ASR, 27)).unwrap(), "tst wzr, wzr, asr 27";
    }
}
