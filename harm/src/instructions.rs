/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use aarchmrs_types::InstructionCode;

use crate::reloc::Reloc;

pub mod arith;
pub mod control;
pub mod dpimm;
pub mod dpreg;
pub mod ldst;
pub mod logical;
pub mod movewide;

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
