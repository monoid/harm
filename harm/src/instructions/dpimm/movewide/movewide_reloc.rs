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

use super::{
    MakeMovArgs, MovK, MovN, MovWideTypeTag, MovZ, MoveImm16, Shift32, Shift64, movk, movn, movz,
};

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

/// First nibble (bits `[31:16]`) of the absolute address of the label, checked for unsigned overflow.
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

/// First nibble (bits `[31:16]`) of the absolute address of the label, checked for signed overflow.
#[inline]
pub fn abs_g1_s(label: LabelRef) -> AbsG1S {
    AbsG1S(label)
}

#[doc(hidden)]
#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct AbsG2(LabelRef);

/// Second nibble (bits `[47:32]`) of the absolute address of the label, checked for unsigned overflow.
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

/// Second nibble (bits `[47:32]`) of the absolute address of the label, checked for signed overflow.
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

impl Sealed for AbsG0 {}
impl Sealed for AbsG0Nc {}
impl Sealed for AbsG0S {}
impl Sealed for AbsG1 {}
impl Sealed for AbsG1Nc {}
impl Sealed for AbsG1S {}
impl Sealed for AbsG2 {}
impl Sealed for AbsG2Nc {}
impl Sealed for AbsG2S {}
impl Sealed for AbsG3 {}

#[derive(Debug, Copy, Clone)]
pub struct MovRelArgs<Rd, Rel> {
    pub rd: Rd,
    pub rel: Rel,
}

pub trait MoveWideReloc64: Sealed {
    fn get_shift64(&self) -> Shift64;
    fn get_relocation(&self) -> Rel64;
}

pub trait MoveWideReloc32: MoveWideReloc64 {
    fn get_shift32(&self) -> Shift32;
}

impl<Rel: MoveWideReloc64> Sealed for MovRelArgs<RegOrZero64, Rel> {}
impl<Rel: MoveWideReloc32> Sealed for MovRelArgs<RegOrZero32, Rel> {}

impl MoveWideReloc64 for AbsG0 {
    #[inline]
    fn get_shift64(&self) -> Shift64 {
        Shift64::new(0).unwrap()
    }

    #[inline]
    fn get_relocation(&self) -> Rel64 {
        Rel64::MovWAbsG0(self.0)
    }
}

impl MoveWideReloc64 for AbsG0Nc {
    #[inline]
    fn get_shift64(&self) -> Shift64 {
        Shift64::new(0).unwrap()
    }

    #[inline]
    fn get_relocation(&self) -> Rel64 {
        Rel64::MovWAbsG0Nc(self.0)
    }
}

impl MoveWideReloc64 for AbsG0S {
    #[inline]
    fn get_shift64(&self) -> Shift64 {
        Shift64::new(0).unwrap()
    }

    #[inline]
    fn get_relocation(&self) -> Rel64 {
        Rel64::MovWAbsG0S(self.0)
    }
}

impl MoveWideReloc64 for AbsG1 {
    #[inline]
    fn get_shift64(&self) -> Shift64 {
        Shift64::new(1 << 4).unwrap()
    }

    #[inline]
    fn get_relocation(&self) -> Rel64 {
        Rel64::MovWAbsG1(self.0)
    }
}

impl MoveWideReloc64 for AbsG1Nc {
    #[inline]
    fn get_shift64(&self) -> Shift64 {
        Shift64::new(1 << 4).unwrap()
    }

    #[inline]
    fn get_relocation(&self) -> Rel64 {
        Rel64::MovWAbsG1Nc(self.0)
    }
}

impl MoveWideReloc64 for AbsG1S {
    #[inline]
    fn get_shift64(&self) -> Shift64 {
        Shift64::new(1 << 4).unwrap()
    }

    #[inline]
    fn get_relocation(&self) -> Rel64 {
        Rel64::MovWAbsG1S(self.0)
    }
}

impl MoveWideReloc64 for AbsG2 {
    #[inline]
    fn get_shift64(&self) -> Shift64 {
        Shift64::new(2 << 4).unwrap()
    }

    #[inline]
    fn get_relocation(&self) -> Rel64 {
        Rel64::MovWAbsG2(self.0)
    }
}

impl MoveWideReloc64 for AbsG2Nc {
    #[inline]
    fn get_shift64(&self) -> Shift64 {
        Shift64::new(2 << 4).unwrap()
    }

    #[inline]
    fn get_relocation(&self) -> Rel64 {
        Rel64::MovWAbsG2Nc(self.0)
    }
}

impl MoveWideReloc64 for AbsG2S {
    #[inline]
    fn get_shift64(&self) -> Shift64 {
        Shift64::new(2 << 4).unwrap()
    }

    #[inline]
    fn get_relocation(&self) -> Rel64 {
        Rel64::MovWAbsG2S(self.0)
    }
}

impl MoveWideReloc64 for AbsG3 {
    #[inline]
    fn get_shift64(&self) -> Shift64 {
        Shift64::new(3 << 4).unwrap()
    }

    #[inline]
    fn get_relocation(&self) -> Rel64 {
        Rel64::MovWAbsG3(self.0)
    }
}

impl MoveWideReloc32 for AbsG0 {
    #[inline]
    fn get_shift32(&self) -> Shift32 {
        Shift32::new(0).unwrap()
    }
}

impl MoveWideReloc32 for AbsG0Nc {
    #[inline]
    fn get_shift32(&self) -> Shift32 {
        Shift32::new(0).unwrap()
    }
}

impl MoveWideReloc32 for AbsG0S {
    #[inline]
    fn get_shift32(&self) -> Shift32 {
        Shift32::new(0).unwrap()
    }
}

impl MoveWideReloc32 for AbsG1 {
    #[inline]
    fn get_shift32(&self) -> Shift32 {
        Shift32::new(1 << 4).unwrap()
    }
}

impl MoveWideReloc32 for AbsG1Nc {
    #[inline]
    fn get_shift32(&self) -> Shift32 {
        Shift32::new(1 << 4).unwrap()
    }
}

impl MoveWideReloc32 for AbsG1S {
    #[inline]
    fn get_shift32(&self) -> Shift32 {
        Shift32::new(1 << 4).unwrap()
    }
}

impl<RIn: IntoReg<RegOrZero64>, Rel: MoveWideReloc64> MakeMovArgs<RIn, Rel>
    for MovWideTypeTag<MovRelArgs<RegOrZero64, Rel>>
{
    type Outcome = Unfallible<MovRelArgs<RegOrZero64, Rel>>;

    fn new(rd: RIn, rel: Rel) -> Self::Outcome {
        Unfallible(MovRelArgs {
            rd: rd.into_reg(),
            rel,
        })
    }
}

impl<RIn: IntoReg<RegOrZero32>, Rel: MoveWideReloc32> MakeMovArgs<RIn, Rel>
    for MovWideTypeTag<MovRelArgs<RegOrZero32, Rel>>
{
    type Outcome = Unfallible<MovRelArgs<RegOrZero32, Rel>>;

    fn new(rd: RIn, rel: Rel) -> Self::Outcome {
        Unfallible(MovRelArgs {
            rd: rd.into_reg(),
            rel,
        })
    }
}

impl<Rel: MoveWideReloc64> RelocatableInstruction for MovZ<MovRelArgs<RegOrZero64, Rel>> {
    fn to_code_with_reloc(&self) -> (aarchmrs_types::InstructionCode, Option<crate::reloc::Rel64>) {
        let MovRelArgs { rd, ref rel } = self.0;
        let code = movz(rd, (MoveImm16::default(), rel.get_shift64())).to_code();
        let rel = rel.get_relocation();
        (code, Some(rel))
    }
}

impl<Rel: MoveWideReloc32> RelocatableInstruction for MovZ<MovRelArgs<RegOrZero32, Rel>> {
    fn to_code_with_reloc(&self) -> (aarchmrs_types::InstructionCode, Option<crate::reloc::Rel64>) {
        let MovRelArgs { rd, ref rel } = self.0;
        let code = movz(rd, (MoveImm16::default(), rel.get_shift32())).to_code();
        let rel = rel.get_relocation();
        (code, Some(rel))
    }
}

impl<Rel: MoveWideReloc64> RelocatableInstruction for MovK<MovRelArgs<RegOrZero64, Rel>> {
    fn to_code_with_reloc(&self) -> (aarchmrs_types::InstructionCode, Option<crate::reloc::Rel64>) {
        let MovRelArgs { rd, ref rel } = self.0;
        let code = movk(rd, (MoveImm16::default(), rel.get_shift64())).to_code();
        let rel = rel.get_relocation();
        (code, Some(rel))
    }
}

impl<Rel: MoveWideReloc32> RelocatableInstruction for MovK<MovRelArgs<RegOrZero32, Rel>> {
    fn to_code_with_reloc(&self) -> (aarchmrs_types::InstructionCode, Option<crate::reloc::Rel64>) {
        let MovRelArgs { rd, ref rel } = self.0;
        let code = movk(rd, (MoveImm16::default(), rel.get_shift32())).to_code();
        let rel = rel.get_relocation();
        (code, Some(rel))
    }
}

impl<Rel: MoveWideReloc64> RelocatableInstruction for MovN<MovRelArgs<RegOrZero64, Rel>> {
    fn to_code_with_reloc(&self) -> (aarchmrs_types::InstructionCode, Option<crate::reloc::Rel64>) {
        let MovRelArgs { rd, ref rel } = self.0;
        let code = movn(rd, (MoveImm16::default(), rel.get_shift64())).to_code();
        let rel = rel.get_relocation();
        (code, Some(rel))
    }
}

impl<Rel: MoveWideReloc32> RelocatableInstruction for MovN<MovRelArgs<RegOrZero32, Rel>> {
    fn to_code_with_reloc(&self) -> (aarchmrs_types::InstructionCode, Option<crate::reloc::Rel64>) {
        let MovRelArgs { rd, ref rel } = self.0;
        let code = movn(rd, (MoveImm16::default(), rel.get_shift32())).to_code();
        let rel = rel.get_relocation();
        (code, Some(rel))
    }
}

#[cfg(test)]
mod tests {
    use crate::register::Reg32::*;
    use crate::register::Reg64::*;
    use crate::reloc::LabelId;

    use super::*;

    #[test]
    fn test_64_abs_g0_movz() {
        let label = LabelRef {
            id: LabelId(1),
            addend: 42,
        };
        let inst = movz(X1, abs_g0(label)).to_code_with_reloc();
        assert_eq!(inst.0, movz(X1, (0, 0)).unwrap().to_code());
        assert_eq!(inst.1, Some(Rel64::MovWAbsG0(label)));
    }

    #[test]
    fn test_64_abs_g1_movk() {
        let label = LabelRef {
            id: LabelId(2),
            addend: 43,
        };
        let inst = movk(X2, abs_g1(label)).to_code_with_reloc();
        assert_eq!(inst.0, movk(X2, (0, 16)).unwrap().to_code());
        assert_eq!(inst.1, Some(Rel64::MovWAbsG1(label)));
    }

    #[test]
    fn test_64_abs_g1s_movn() {
        let label = LabelRef {
            id: LabelId(3),
            addend: 44,
        };
        // Makes little sense, but gas accepts it.
        let inst = movn(X3, abs_g1_s(label)).to_code_with_reloc();
        assert_eq!(inst.0, movn(X3, (0, 16)).unwrap().to_code());
        assert_eq!(inst.1, Some(Rel64::MovWAbsG1S(label)));
    }

    #[test]
    fn test_64_abs_g1nc_movk() {
        let label = LabelRef {
            id: LabelId(4),
            addend: 45,
        };
        let inst = movk(X4, abs_g1_nc(label)).to_code_with_reloc();
        assert_eq!(inst.0, movk(X4, (0, 16)).unwrap().to_code());
        assert_eq!(inst.1, Some(Rel64::MovWAbsG1Nc(label)));
    }

    #[test]
    fn test_64_abs_g2_movz() {
        let label = LabelRef {
            id: LabelId(5),
            addend: 46,
        };
        let inst = movz(X5, abs_g2(label)).to_code_with_reloc();
        assert_eq!(inst.0, movz(X5, (0, 32)).unwrap().to_code());
        assert_eq!(inst.1, Some(Rel64::MovWAbsG2(label)));
    }

    #[test]
    fn test_64_abs_g2s_movz() {
        let label = LabelRef {
            id: LabelId(6),
            addend: 47,
        };
        let inst = movz(X6, abs_g2_s(label)).to_code_with_reloc();
        assert_eq!(inst.0, movz(X6, (0, 32)).unwrap().to_code());
        assert_eq!(inst.1, Some(Rel64::MovWAbsG2S(label)));
    }

    #[test]
    fn test_64_abs_g2nc_movk() {
        let label = LabelRef {
            id: LabelId(7),
            addend: 48,
        };
        let inst = movk(X7, abs_g2_nc(label)).to_code_with_reloc();
        assert_eq!(inst.0, movk(X7, (0, 32)).unwrap().to_code());
        assert_eq!(inst.1, Some(Rel64::MovWAbsG2Nc(label)));
    }

    #[test]
    fn test_64_abs_g3_movk() {
        let label = LabelRef {
            id: LabelId(8),
            addend: 49,
        };
        let inst = movk(X8, abs_g3(label)).to_code_with_reloc();
        assert_eq!(inst.0, movk(X8, (0, 48)).unwrap().to_code());
        assert_eq!(inst.1, Some(Rel64::MovWAbsG3(label)));
    }

    #[test]
    fn test_32_abs_g0_movz() {
        let label = LabelRef {
            id: LabelId(1),
            addend: 42,
        };
        let inst = movz(W1, abs_g0(label)).to_code_with_reloc();
        assert_eq!(inst.0, movz(W1, (0, 0)).unwrap().to_code());
        assert_eq!(inst.1, Some(Rel64::MovWAbsG0(label)));
    }

    #[test]
    fn test_32_abs_g1_movk() {
        let label = LabelRef {
            id: LabelId(2),
            addend: 43,
        };
        let inst = movk(W2, abs_g1(label)).to_code_with_reloc();
        assert_eq!(inst.0, movk(W2, (0, 16)).unwrap().to_code());
        assert_eq!(inst.1, Some(Rel64::MovWAbsG1(label)));
    }

    #[test]
    fn test_32_abs_g1s_movn() {
        let label = LabelRef {
            id: LabelId(3),
            addend: 44,
        };
        // Makes little sense, but gas accepts it.
        let inst = movn(W3, abs_g1_s(label)).to_code_with_reloc();
        assert_eq!(inst.0, movn(W3, (0, 16)).unwrap().to_code());
        assert_eq!(inst.1, Some(Rel64::MovWAbsG1S(label)));
    }

    #[test]
    fn test_32_abs_g1nc_movk() {
        let label = LabelRef {
            id: LabelId(4),
            addend: 45,
        };
        let inst = movk(W4, abs_g1_nc(label)).to_code_with_reloc();
        assert_eq!(inst.0, movk(W4, (0, 16)).unwrap().to_code());
        assert_eq!(inst.1, Some(Rel64::MovWAbsG1Nc(label)));
    }
}
