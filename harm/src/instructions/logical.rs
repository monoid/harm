/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

mod args;

pub use self::args::{LogicalArgs, MakeSpLogicalArgs, MakeTstLogicalArgs, MakeZeroLogicalArgs};
use crate::{bits::UBitValue, outcome::Outcome};

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum LogicalShift {
    LSL = 0b00,
    LSR = 0b01,
    ASR = 0b10,
    ROR = 0b11,
}

pub type LogicalShiftAmount32 = UBitValue<5>;
pub type LogicalShiftAmount64 = UBitValue<6>;

#[derive(Debug, Copy, Clone)]
pub struct And<Args>(pub Args);

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

#[derive(Debug, Copy, Clone)]
pub struct Ands<Args>(pub Args);

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

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Copy, Clone)]
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
