/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

mod args;
pub mod immediate;

pub use self::args::{LogicalArgs, MakeSpLogicalArgs, MakeTstLogicalArgs, MakeZeroLogicalArgs};
use self::immediate::LogicalImmFields;
use crate::{
    instructions::RawInstruction,
    outcome::Outcome,
    register::{RegOrSp32, RegOrSp64, RegOrZero32, RegOrZero64, Register},
};
use aarchmrs_instructions::A64::dpimm::log_imm::{
    AND_32_log_imm::AND_32_log_imm, AND_64_log_imm::AND_64_log_imm,
    ANDS_32S_log_imm::ANDS_32S_log_imm, ANDS_64S_log_imm::ANDS_64S_log_imm,
    EOR_32_log_imm::EOR_32_log_imm, EOR_64_log_imm::EOR_64_log_imm, ORR_32_log_imm::ORR_32_log_imm,
    ORR_64_log_imm::ORR_64_log_imm,
};

pub struct And<Args>(pub Args);

impl RawInstruction for And<LogicalArgs<RegOrSp32, RegOrZero32, LogicalImmFields>> {
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let args = &self.0;
        AND_32_log_imm(
            args.mask.immr.into(),
            args.mask.imms.into(),
            args.rn.code(),
            args.rd.code(),
        )
    }
}
impl RawInstruction for And<LogicalArgs<RegOrSp64, RegOrZero64, LogicalImmFields>> {
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let args = &self.0;
        AND_64_log_imm(
            args.mask.n.into(),
            args.mask.immr.into(),
            args.mask.imms.into(),
            args.rn.code(),
            args.rd.code(),
        )
    }
}

pub fn and<RdIn, RnIn, MaskIn, RdOut, RnOut, MaskOut>(rd: RdIn, rn: RnIn, mask: MaskIn) ->
    <<LogicalArgs<RdOut, RnOut, MaskOut> as MakeSpLogicalArgs<RdIn, RnIn, MaskIn>>::Outcome as Outcome>::Output<And<LogicalArgs<RdOut, RnOut, MaskOut>>>
where
    LogicalArgs<RdOut, RnOut, MaskOut>: MakeSpLogicalArgs<RdIn, RnIn, MaskIn>,
    <LogicalArgs<RdOut, RnOut, MaskOut> as MakeSpLogicalArgs<RdIn, RnIn, MaskIn>>::Outcome:
        Outcome<Inner = LogicalArgs<RdOut, RnOut, MaskOut>>,
{
    <LogicalArgs<RdOut, RnOut, MaskOut> as MakeSpLogicalArgs<RdIn, RnIn, MaskIn>>::new(rd, rn, mask)
        .map(And)
}

pub struct Ands<Args>(pub Args);

impl RawInstruction for Ands<LogicalArgs<RegOrZero32, RegOrZero32, LogicalImmFields>> {
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let args = &self.0;
        ANDS_32S_log_imm(
            args.mask.immr.into(),
            args.mask.imms.into(),
            args.rn.code(),
            args.rd.code(),
        )
    }
}

impl RawInstruction for Ands<LogicalArgs<RegOrZero64, RegOrZero64, LogicalImmFields>> {
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let args = &self.0;
        ANDS_64S_log_imm(
            args.mask.n.into(),
            args.mask.immr.into(),
            args.mask.imms.into(),
            args.rn.code(),
            args.rd.code(),
        )
    }
}

pub fn ands<RdIn, RnIn, MaskIn, RdOut, RnOut, MaskOut>(rd: RdIn, rn: RnIn, mask: MaskIn) ->
    <<LogicalArgs<RdOut, RnOut, MaskOut> as MakeZeroLogicalArgs<RdIn, RnIn, MaskIn>>::Outcome as Outcome>::Output<Ands<LogicalArgs<RdOut, RnOut, MaskOut>>>
where
    LogicalArgs<RdOut, RnOut, MaskOut>: MakeZeroLogicalArgs<RdIn, RnIn, MaskIn>,
    <LogicalArgs<RdOut, RnOut, MaskOut> as MakeZeroLogicalArgs<RdIn, RnIn, MaskIn>>::Outcome:
        Outcome<Inner = LogicalArgs<RdOut, RnOut, MaskOut>>,
{
    <LogicalArgs<RdOut, RnOut, MaskOut> as MakeZeroLogicalArgs<RdIn, RnIn, MaskIn>>::new(
        rd, rn, mask,
    )
    .map(Ands)
}

pub fn tst<RnIn, MaskIn, RdOut, RnOut, MaskOut>(rn: RnIn, mask: MaskIn) ->
    <<LogicalArgs<RdOut, RnOut, MaskOut> as MakeTstLogicalArgs<RnIn, MaskIn>>::Outcome as Outcome>::Output<Ands<LogicalArgs<RdOut, RnOut, MaskOut>>>
where
    LogicalArgs<RdOut, RnOut, MaskOut>: MakeTstLogicalArgs<RnIn, MaskIn>,
    <LogicalArgs<RdOut, RnOut, MaskOut> as MakeTstLogicalArgs<RnIn, MaskIn>>::Outcome:
        Outcome<Inner = LogicalArgs<RdOut, RnOut, MaskOut>>,
{
    <LogicalArgs<RdOut, RnOut, MaskOut> as MakeTstLogicalArgs<RnIn, MaskIn>>::new(rn, mask)
        .map(Ands)
}

pub struct Eor<Args>(pub Args);

pub fn eor<RdIn, RnIn, MaskIn, RdOut, RnOut, MaskOut>(rd: RdIn, rn: RnIn, mask: MaskIn) ->
    <<LogicalArgs<RdOut, RnOut, MaskOut> as MakeSpLogicalArgs<RdIn, RnIn, MaskIn>>::Outcome as Outcome>::Output<Eor<LogicalArgs<RdOut, RnOut, MaskOut>>>
where
    LogicalArgs<RdOut, RnOut, MaskOut>: MakeSpLogicalArgs<RdIn, RnIn, MaskIn>,
    <LogicalArgs<RdOut, RnOut, MaskOut> as MakeSpLogicalArgs<RdIn, RnIn, MaskIn>>::Outcome:
        Outcome<Inner = LogicalArgs<RdOut, RnOut, MaskOut>>,
{
    <LogicalArgs<RdOut, RnOut, MaskOut> as MakeSpLogicalArgs<RdIn, RnIn, MaskIn>>::new(rd, rn, mask)
        .map(Eor)
}

impl RawInstruction for Eor<LogicalArgs<RegOrSp32, RegOrZero32, LogicalImmFields>> {
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let args = &self.0;
        EOR_32_log_imm(
            args.mask.immr.into(),
            args.mask.imms.into(),
            args.rn.code(),
            args.rd.code(),
        )
    }
}

impl RawInstruction for Eor<LogicalArgs<RegOrSp64, RegOrZero64, LogicalImmFields>> {
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let args = &self.0;
        EOR_64_log_imm(
            args.mask.n.into(),
            args.mask.immr.into(),
            args.mask.imms.into(),
            args.rn.code(),
            args.rd.code(),
        )
    }
}

pub struct Orr<Args>(pub Args);

pub fn orr<RdIn, RnIn, MaskIn, RdOut, RnOut, MaskOut>(rd: RdIn, rn: RnIn, mask: MaskIn) ->
    <<LogicalArgs<RdOut, RnOut, MaskOut> as MakeSpLogicalArgs<RdIn, RnIn, MaskIn>>::Outcome as Outcome>::Output<Orr<LogicalArgs<RdOut, RnOut, MaskOut>>>
where
    LogicalArgs<RdOut, RnOut, MaskOut>: MakeSpLogicalArgs<RdIn, RnIn, MaskIn>,
    <LogicalArgs<RdOut, RnOut, MaskOut> as MakeSpLogicalArgs<RdIn, RnIn, MaskIn>>::Outcome:
        Outcome<Inner = LogicalArgs<RdOut, RnOut, MaskOut>>,
{
    <LogicalArgs<RdOut, RnOut, MaskOut> as MakeSpLogicalArgs<RdIn, RnIn, MaskIn>>::new(rd, rn, mask)
        .map(Orr)
}

impl RawInstruction for Orr<LogicalArgs<RegOrSp32, RegOrZero32, LogicalImmFields>> {
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let args = &self.0;
        ORR_32_log_imm(
            args.mask.immr.into(),
            args.mask.imms.into(),
            args.rn.code(),
            args.rd.code(),
        )
    }
}

impl RawInstruction for Orr<LogicalArgs<RegOrSp64, RegOrZero64, LogicalImmFields>> {
    fn to_code(&self) -> aarchmrs_types::InstructionCode {
        let args = &self.0;
        ORR_64_log_imm(
            args.mask.n.into(),
            args.mask.immr.into(),
            args.mask.imms.into(),
            args.rn.code(),
            args.rd.code(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::instructions::InstructionSeq as _;
    use crate::register::{Reg32, Reg64, RegOrSp32, RegOrSp64};
    use harm_test_utils::test_cases;

    use super::*;

    use Reg32::*;
    use Reg64::*;
    use RegOrSp32::WSP;
    use RegOrSp64::SP;
    use RegOrZero32::WZR;
    use RegOrZero64::XZR;

    const LOG_IMM_DB: &str = "
120a4041	and w1, w2, #0xffc0007f
1206c45f	and wsp, w2, #0x0c0c0c0c
121243e1	and w1, wzr, #0x7fffc000
3202ac41	orr w1, w2, #0xc3ffc3ff
3200985f	orr wsp, w2, #0x007f007f
321f0be1	orr w1, wzr, #0x0000000e
52132841	eor w1, w2, #0x00ffe000
5209545f	eor wsp, w2, #0xff801fff
520a83e1	eor w1, wzr, #0x00400040
72163041	ands w1, w2, #0x007ffc00
721f405f	ands wzr, w2, #0x0003fffe
72063fe1	ands w1, wzr, #0xfc0003ff
92412c41	and x1, x2, #0x80000000000007ff
9259f05f	and sp, x2, #0xffffff8fffffffff
9257dbe1	and x1, xzr, #0xfffffe00ffffffff
b241d841	orr x1, x2, #0x803fffffffffffff
b259105f	orr sp, x2, #0x00000f8000000000
b21617e1	orr x1, xzr, #0x0000fc000000fc00
d21a0841	eor x1, x2, #0x000001c0000001c0
d273285f	eor sp, x2, #0x0000000000ffe000
d2637fe1	eor x1, xzr, #0x1fffffffe0000000
f25dec41	ands x1, x2, #0xfffffff87fffffff
f249bc5f	ands xzr, x2, #0xff80007fffffffff
f20c73e1	ands x1, xzr, #0xfff1fffffff1ffff
f279005f	tst x2, #0x0000000000000080
f201cbff	tst xzr, #0x8383838383838383
7203b05f	tst w2, #0xe3ffe3ff
720557ff	tst wzr, #0xf801ffff

";

    test_cases! {
        LOG_IMM_DB, untested_log_imm_db;
        test_and_32_0xffc0007f, and(W1, W2, 0xffc0007f).unwrap(), "and w1, w2, #0xffc0007f";
        test_and_wsp_32_0x0c0c0c0c, and(WSP, W2, 0x0c0c0c0c).unwrap(), "and wsp, w2, #0x0c0c0c0c";
        test_and_32_wzr_0x7fffc000, and(W1, WZR, 0x7fffc000).unwrap(), "and w1, wzr, #0x7fffc000";
        test_orr_32_0xc3ffc3ff, orr( W1, W2, 0xc3ffc3ff).unwrap(), "orr w1, w2, #0xc3ffc3ff";
        test_orr_wsp_32_0x007f007f, orr(WSP, W2, 0x007f007f).unwrap(), "orr wsp, w2, #0x007f007f";
        test_orr_32_wzr, orr( W1, WZR, 0x0000000e).unwrap(), "orr w1, wzr, #0x0000000e";
        test_eor_32_0x00ffe000, eor(W1, W2, 0x00ffe000).unwrap(), "eor w1, w2, #0x00ffe000";
        test_eor_wsp_32_0xff801fff, eor(WSP, W2, 0xff801fff).unwrap(), "eor wsp, w2, #0xff801fff";
        test_eor_32_wzr_0x00400040, eor(W1, WZR, 0x00400040).unwrap(), "eor w1, wzr, #0x00400040";
        test_ands_32_0x007ffc00, ands(W1, W2, 0x007ffc00).unwrap(), "ands w1, w2, #0x007ffc00";
        test_ands_wzr_32_0x0003fffe, ands(WZR, W2, 0x0003fffe).unwrap(), "ands wzr, w2, #0x0003fffe";
        test_ands_32_wzr_0xfc0003ff, ands(W1, WZR, 0xfc0003ff).unwrap(), "ands w1, wzr, #0xfc0003ff";
        test_and_64_0x80000000000007ff, and(X1, X2, 0x80000000000007ff).unwrap(), "and x1, x2, #0x80000000000007ff";
        test_and_sp_64_0xffffff8fffffffff, and(SP, X2, 0xffffff8fffffffff).unwrap(), "and sp, x2, #0xffffff8fffffffff";
        test_and_64_xzr_0xfffffe00ffffffff, and(X1, XZR, 0xfffffe00ffffffff).unwrap(), "and x1, xzr, #0xfffffe00ffffffff";
        test_orr_64_0x803fffffffffffff, orr(X1, X2, 0x803fffffffffffff).unwrap(), "orr x1, x2, #0x803fffffffffffff";
        test_orr_sp_64_0x00000f8000000000, orr(SP, X2, 0x00000f8000000000).unwrap(), "orr sp, x2, #0x00000f8000000000";
        test_orr_64_xzr_0x0000fc000000fc00, orr(X1, XZR, 0x0000fc000000fc00).unwrap(), "orr x1, xzr, #0x0000fc000000fc00";
        test_eor_64_0x000001c0000001c0, eor(X1, X2, 0x000001c0000001c0).unwrap(), "eor x1, x2, #0x000001c0000001c0";
        test_eor_sp_64_0x0000000000ffe000, eor(SP, X2, 0x0000000000ffe000).unwrap(), "eor sp, x2, #0x0000000000ffe000";
        test_eor_64_xzr_0x1fffffffe0000000, eor(X1, XZR, 0x1fffffffe0000000).unwrap(), "eor x1, xzr, #0x1fffffffe0000000";
        test_ands_64_0xfffffff87fffffff, ands(X1, X2, 0xfffffff87fffffff).unwrap(), "ands x1, x2, #0xfffffff87fffffff";
        test_ands_xzr_64_0xff80007fffffffff, ands(XZR, X2, 0xff80007fffffffff).unwrap(), "ands xzr, x2, #0xff80007fffffffff";
        test_ands_64_xzr_0xfff1fffffff1ffff, ands(X1, XZR, 0xfff1fffffff1ffff).unwrap(), "ands x1, xzr, #0xfff1fffffff1ffff";
        test_tst_x2_0x0000000000000080, tst(X2, 0x0000000000000080).unwrap(), "tst x2, #0x0000000000000080";
        test_tst_xzr_0x8383838383838383, tst(XZR, 0x8383838383838383).unwrap(), "tst xzr, #0x8383838383838383";
        test_tst_w2_0xe3ffe3ff, tst(W2, 0xe3ffe3ff).unwrap(), "tst w2, #0xe3ffe3ff";
        test_tst_wzr_0xf801ffff, tst(WZR, 0xf801ffff).unwrap(), "tst wzr, #0xf801ffff";
    }
}
