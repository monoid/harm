/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

pub mod ret;

use aarchmrs_instructions::A64::control::{
    branch_imm::B_only_branch_imm::B_only_branch_imm, // TODO BL: branch with link
    condbranch::B_only_condbranch::B_only_condbranch, // TODO BC: branch consistent conditionally
};
use aarchmrs_types::InstructionCode;

pub use self::ret::*;
use super::{BranchCond, Instruction};
use crate::{
    bits::SBitValue,
    register::{IntoCode as _, Reg32, Reg64},
};

#[inline]
pub fn b(offset: PcOffset) -> Branch<PcDst> {
    Branch::new(PcDst(offset))
}

#[inline]
pub fn b_cond(cond: BranchCond, offset: PcOffset) -> Branch<PcDst> {
    Branch::new(PcDst(offset)).when(cond)
}

#[derive(Debug, Copy, Clone)]
pub struct Branch<Dst> {
    pub dst: Dst,
    pub condition: Option<BranchCond>,
}

pub type PcOffset = i32;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct PcDst(pub PcOffset);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct RegDst(pub Reg64);

impl<T> Branch<T> {
    #[inline]
    pub fn new(dst: T) -> Self {
        Self {
            dst,
            condition: None,
        }
    }
}

impl Branch<PcDst> {
    #[inline]
    pub fn when(mut self, cond: BranchCond) -> Self {
        self.condition = Some(cond);
        self
    }
}

impl Instruction for Branch<PcDst> {
    fn represent(self) -> impl Iterator<Item = InstructionCode> + 'static {
        // TODO first offset is 19 bits, the second is 29 bits!
        // what to do with the possible overflow?
        let code = match self.condition {
            Some(cond) => branch_cond(self.dst.0, cond),
            None => branch_nocond(self.dst.0),
        };
        std::iter::once(code)
    }
}

fn branch_cond(offset: PcOffset, cond: BranchCond) -> InstructionCode {
    // TODO validate alignment and size
    let imm19 = (offset as u32 / 4) & ((1 << 20) - 1);
    let cond = cond as u32;

    B_only_condbranch::new(imm19.into(), cond.into()).build()
}

fn branch_nocond(offset: PcOffset) -> InstructionCode {
    // TODO validate alignment and size
    let imm26 = (offset as u32 / 4) & ((1 << 27) - 1);
    B_only_branch_imm::new(imm26.into()).build()
}

pub struct CompareBranch<Reg> {
    equal: bool,
    reg: Reg,
    offset: SBitValue<19, 2>,
}

pub trait MakeCompareBranch<Reg>: Sized {
    fn new(equal: bool, reg: Reg, offset: SBitValue<19, 2>) -> Self;
}

impl MakeCompareBranch<Reg64> for CompareBranch<Reg64> {
    fn new(equal: bool, reg: Reg64, offset: SBitValue<19, 2>) -> Self {
        Self { equal, reg, offset }
    }
}

impl MakeCompareBranch<Reg32> for CompareBranch<Reg32> {
    fn new(equal: bool, reg: Reg32, offset: SBitValue<19, 2>) -> Self {
        Self { equal, reg, offset }
    }
}

impl Instruction for CompareBranch<Reg64> {
    fn represent(self) -> impl Iterator<Item = InstructionCode> + 'static {
        use aarchmrs_instructions::A64::control::compbranch;
        let code = if self.equal {
            compbranch::CBZ_64_compbranch::CBZ_64_compbranch::new(
                self.offset.into(),
                self.reg.code().into(),
            )
            .build()
        } else {
            compbranch::CBNZ_64_compbranch::CBNZ_64_compbranch::new(
                self.offset.into(),
                self.reg.code().into(),
            )
            .build()
        };
        std::iter::once(code)
    }
}

impl Instruction for CompareBranch<Reg32> {
    fn represent(self) -> impl Iterator<Item = InstructionCode> + 'static {
        use aarchmrs_instructions::A64::control::compbranch;

        let code = if self.equal {
            compbranch::CBZ_32_compbranch::CBZ_32_compbranch::new(
                self.offset.into(),
                self.reg.code().into(),
            )
            .build()
        } else {
            compbranch::CBNZ_32_compbranch::CBNZ_32_compbranch::new(
                self.offset.into(),
                self.reg.code().into(),
            )
            .build()
        };
        std::iter::once(code)
    }
}

pub fn cbnz<Reg>(reg: Reg, offset: SBitValue<19, 2>) -> CompareBranch<Reg>
where
    CompareBranch<Reg>: MakeCompareBranch<Reg>,
{
    CompareBranch::new(false, reg, offset)
}

pub fn cbz<Reg>(reg: Reg, offset: SBitValue<19, 2>) -> CompareBranch<Reg>
where
    CompareBranch<Reg>: MakeCompareBranch<Reg>,
{
    CompareBranch::new(true, reg, offset)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::register::Reg32::*;
    use crate::register::Reg64::*;

    #[test]
    fn test_cbnz_64_pos() {
        let offset = SBitValue::new(8).unwrap();
        let it = cbnz(X2, offset);
        let words: Vec<_> = it.represent().collect();
        assert_eq!(words, vec![InstructionCode([0x42, 0x00, 0x00, 0xb5])]); // 0xb5000042
    }

    #[test]
    fn test_cbnz_64_neg() {
        let offset = SBitValue::new(-8).unwrap();
        let it = cbnz(X2, offset);
        let words: Vec<_> = it.represent().collect();
        assert_eq!(words, vec![InstructionCode([0xc2, 0xff, 0xff, 0xb5])]); // 0xb5ffffc2
    }

    #[test]
    fn test_cbz_64_pos() {
        let offset = SBitValue::new(8).unwrap();
        let it = cbz(X2, offset);
        let words: Vec<_> = it.represent().collect();
        assert_eq!(words, vec![InstructionCode([0x42, 0x00, 0x00, 0xb4])]); // 0xb4000042
    }

    #[test]
    fn test_cbz_64_neg() {
        let offset = SBitValue::new(-8).unwrap();
        let it = cbz(X2, offset);
        let words: Vec<_> = it.represent().collect();
        assert_eq!(words, vec![InstructionCode([0xc2, 0xff, 0xff, 0xb4])]); // 0xb4ffffc2
    }

    #[test]
    fn test_cbnz_32_pos() {
        let offset = SBitValue::new(8).unwrap();
        let it = cbnz(W2, offset);
        let words: Vec<_> = it.represent().collect();
        assert_eq!(words, vec![InstructionCode([0x42, 0x00, 0x00, 0x35])]); // 35000042
    }

    #[test]
    fn test_cbnz_32_neg() {
        let offset = SBitValue::new(-8).unwrap();
        let it = cbnz(W2, offset);
        let words: Vec<_> = it.represent().collect();
        assert_eq!(words, vec![InstructionCode([0xc2, 0xff, 0xff, 0x35])]); // 35ffffc2
    }

    #[test]
    fn test_cbz_32_pos() {
        let offset = SBitValue::new(8).unwrap();
        let it = cbz(W2, offset);
        let words: Vec<_> = it.represent().collect();
        assert_eq!(words, vec![InstructionCode([0x42, 0x00, 0x00, 0x34])]); // 0x34000042
    }

    #[test]
    fn test_cbz_32_neg() {
        let offset = SBitValue::new(-8).unwrap();
        let it = cbz(W2, offset);
        let words: Vec<_> = it.represent().collect();
        assert_eq!(words, vec![InstructionCode([0xc2, 0xff, 0xff, 0x34])]); // 0x34ffffc2
    }
}
