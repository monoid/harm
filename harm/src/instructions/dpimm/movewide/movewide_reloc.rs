/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use harm_types::A64::register::{RegOrZero32, RegOrZero64};

use crate::{
    instructions::{RawInstruction as _, RelocatableInstruction},
    outcome::Unfallible,
    register::IntoReg,
    reloc::{LabelRef, Rel64},
    sealed::Sealed,
};

use super::{MakeMovArgs, MovZ, MoveImm16, Shift32, Shift64, movz};

#[doc(hidden)]
#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct AbsG0(LabelRef);

/// Lowest nibble (bits `[15:0]`) of the absolute address of the label, checked for unsigned overflow.
#[inline]
pub fn abs_g0(label: LabelRef) -> AbsG0 {
    AbsG0(label)
}

#[doc(hidden)]
#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct AbsG0Nc(LabelRef);

/// Lowest nibble (bits `[15:0]`) of the absolute address of the label, non-checked.
#[inline]
pub fn abs_g0_nc(label: LabelRef) -> AbsG0Nc {
    AbsG0Nc(label)
}

#[doc(hidden)]
#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct AbsG0S(LabelRef);

/// Lowest nibble (bits `[15:0]`) of the absolute address of the label, checked for signed overflow.
#[inline]
pub fn abs_g0_s(label: LabelRef) -> AbsG0S {
    AbsG0S(label)
}

#[doc(hidden)]
#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct AbsG1(LabelRef);

/// First nibble (bits `[31:16]`) of the absolute address of the label, checked for unsinged overflow.
#[inline]
pub fn abs_g1(label: LabelRef) -> AbsG1 {
    AbsG1(label)
}

#[doc(hidden)]
#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct AbsG1Nc(LabelRef);

/// First nibble (bits `[31:16]`) of the absolute address of the label, non-checked.
#[inline]
pub fn abs_g1_nc(label: LabelRef) -> AbsG1Nc {
    AbsG1Nc(label)
}

#[doc(hidden)]
#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct AbsG1S(LabelRef);

/// First nibble (bits `[31:16]`) of the absolute address of the label, checked for singed overflow.
#[inline]
pub fn abs_g1_s(label: LabelRef) -> AbsG1S {
    AbsG1S(label)
}

#[doc(hidden)]
#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct AbsG2(LabelRef);

/// Second nibble (bits `[47:32]`) of the absolute address of the label, checked for unsinged overflow.
#[inline]
pub fn abs_g2(label: LabelRef) -> AbsG2 {
    AbsG2(label)
}

#[doc(hidden)]
#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct AbsG2Nc(LabelRef);

/// Second nibble (bits `[47:32]`) of the absolute address of the label, non-checked.
#[inline]
pub fn abs_g2_nc(label: LabelRef) -> AbsG2Nc {
    AbsG2Nc(label)
}

#[doc(hidden)]
#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct AbsG2S(LabelRef);

/// Second nibble (bits `[47:32]`) of the absolute address of the label, checked for singed overflow.
#[inline]
pub fn abs_g2_s(label: LabelRef) -> AbsG2S {
    AbsG2S(label)
}

#[doc(hidden)]
#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct AbsG3(LabelRef);

/// Third nibble (bits `[63:48]`) of the absolute address of the label, cannot overflow.
#[inline]
pub fn abs_g3(label: LabelRef) -> AbsG3 {
    AbsG3(label)
}

#[derive(Debug, Copy, Clone)]
pub struct MovRelArgs<Rd, Rel> {
    pub rd: Rd,
    pub rel: Rel,
}

impl Sealed for MovRelArgs<RegOrZero64, AbsG0> {}

impl<RIn: IntoReg<RegOrZero64>> MakeMovArgs<RIn, AbsG0> for MovRelArgs<RegOrZero64, AbsG0> {
    type Outcome = Unfallible<Self>;

    fn new(rd: RIn, rel: AbsG0) -> Self::Outcome {
        Unfallible(Self {
            rd: rd.into_reg(),
            rel,
        })
    }
}

impl RelocatableInstruction for MovZ<MovRelArgs<RegOrZero64, AbsG0>> {
    fn to_code_with_reloc(&self) -> (aarchmrs_types::InstructionCode, Option<crate::reloc::Rel64>) {
        let code = movz(self.0.rd, (MoveImm16::default(), Shift64::new(0).unwrap())).to_code();
        let rel = Rel64::AbsG0(self.0.rel.0);
        (code, Some(rel))
    }
}

impl RelocatableInstruction for MovZ<MovRelArgs<RegOrZero32, AbsG0>> {
    fn to_code_with_reloc(&self) -> (aarchmrs_types::InstructionCode, Option<crate::reloc::Rel64>) {
        let code = movz(self.0.rd, (MoveImm16::default(), Shift32::new(0).unwrap())).to_code();
        let rel = Rel64::AbsG0(self.0.rel.0);
        (code, Some(rel))
    }
}

#[cfg(test)]
mod tests {
    use crate::register::Reg64::X1;
    use crate::reloc::LabelId;

    use super::*;

    #[test]
    fn test_rel() {
        let label = LabelRef {
            id: LabelId(1),
            addend: 42,
        };
        let inst = movz(X1, abs_g0(label)).to_code_with_reloc();
        assert_eq!(inst.0.0, movz(X1, (0, 0)).unwrap().to_code());
        assert_eq!(inst.1, Some(Rel64::AbsG0(label)));
    }
}
