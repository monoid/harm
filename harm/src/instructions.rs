/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use aarchmrs_types::InstructionCode;

use crate::reloc::Reloc;

pub mod arith;
pub mod branches;
pub mod ldst;

pub trait RawInstruction {
    fn to_code(&self) -> InstructionCode;
}

pub trait RelocatableInstruction {
    fn to_code_with_reloc(&self) -> (InstructionCode, Option<Reloc>);
}

impl<I> RelocatableInstruction for I
where
    I: RawInstruction,
{
    fn to_code_with_reloc(&self) -> (InstructionCode, Option<Reloc>) {
        (self.to_code(), None)
    }
}

/// Sequence of instructions with relocations.
///
/// Some virtual instruction may be assembled to multiple real instructions. For example, `mov xN, imm64` can be
/// expanded into up to four `movk` instructions. Another example is macros-like constructions: a user may define it by
/// implementing the `InstructionSeq` trait.
// It should be &self, but rustc bug makes the impl infected by its lifetime:
// https://github.com/rust-lang/rust/issues/42940
// When the bug is fixed, it can be changed to &self.
// // OTOH, currently all implementations are `Copy`.
pub trait InstructionSeq: Sized {
    fn encode(self) -> impl Iterator<Item = (InstructionCode, Option<Reloc>)> + 'static;

    #[inline]
    fn instructions(self) -> impl Iterator<Item = InstructionCode> + 'static {
        self.encode().map(|(inst, _rel)| inst)
    }

    // Compatibility with dynasmrt.  A feature gate?
    #[inline]
    fn bytes(self) -> impl Iterator<Item = u8> + 'static {
        self.encode().flat_map(|(inst, _rel)| inst.0)
    }
}

impl<I> InstructionSeq for I
where
    I: RelocatableInstruction,
{
    fn encode(self) -> impl Iterator<Item = (InstructionCode, Option<Reloc>)> + 'static {
        let code = self.to_code_with_reloc();
        core::iter::once(code)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BranchCond {
    EQ = 0b0000, // equal
    NE = 0b0001, // not equal
    CS = 0b0010, // carry set
    CC = 0b0011, // carry clear
    MI = 0b0100, // minus
    PL = 0b0101, // plus
    VS = 0b0110, // no overflow
    VC = 0b0111, // overflow
    HI = 0b1000, // unsigned higher
    LS = 0b1001, // unsigned lower or same
    GE = 0b1010, // signed greater or equal
    LT = 0b1011, // signed less than
    GT = 0b1100, // signed greater than
    LE = 0b1101, // signed less than or equal
    AL = 0b1110, // always
    NV = 0b1111, // always
}
