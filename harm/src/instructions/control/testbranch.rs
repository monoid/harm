/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use aarchmrs_instructions::A64::control::testbranch::{
    TBNZ_only_testbranch::TBNZ_only_testbranch, TBZ_only_testbranch::TBZ_only_testbranch,
};
use aarchmrs_types::InstructionCode;

use crate::{
    bits::{SBitValue, UBitValue},
    instructions::{RawInstruction, RelocatableInstruction},
    register::{IntoReg, RegOrZero32, RegOrZero64, Register as _},
    reloc::{LabelRef, Rel64},
    sealed::Sealed,
};

pub type TestBranchBit64 = UBitValue<6>;
pub type TestBranchBit32 = UBitValue<5>;
pub type TestBranchOffset = SBitValue<14, 2>;

pub struct TestBranch<Reg, Bit, Offset> {
    op: bool,
    reg: Reg,
    bit: Bit,
    offset: Offset,
}

impl Sealed for TestBranch<RegOrZero64, TestBranchBit64, TestBranchOffset> {}
impl Sealed for TestBranch<RegOrZero32, TestBranchBit32, TestBranchOffset> {}
impl Sealed for TestBranch<RegOrZero64, TestBranchBit64, LabelRef> {}
impl Sealed for TestBranch<RegOrZero32, TestBranchBit32, LabelRef> {}

pub trait MakeTestBranch<Reg, Bit, Offset>: Sealed {
    fn new(op: bool, reg: Reg, bit: Bit, offset: Offset) -> Self;
}

impl<R: IntoReg<RegOrZero64>> MakeTestBranch<R, TestBranchBit64, TestBranchOffset>
    for TestBranch<RegOrZero64, TestBranchBit64, TestBranchOffset>
{
    fn new(op: bool, reg: R, bit: TestBranchBit64, offset: TestBranchOffset) -> Self {
        Self {
            op,
            reg: reg.into_reg(),
            bit,
            offset,
        }
    }
}

impl<R: IntoReg<RegOrZero32>> MakeTestBranch<R, TestBranchBit32, TestBranchOffset>
    for TestBranch<RegOrZero32, TestBranchBit32, TestBranchOffset>
{
    fn new(op: bool, reg: R, bit: TestBranchBit32, offset: TestBranchOffset) -> Self {
        Self {
            op,
            reg: reg.into_reg(),
            bit,
            offset,
        }
    }
}

impl<R: IntoReg<RegOrZero64>> MakeTestBranch<R, TestBranchBit64, LabelRef>
    for TestBranch<RegOrZero64, TestBranchBit64, LabelRef>
{
    fn new(op: bool, reg: R, bit: TestBranchBit64, offset: LabelRef) -> Self {
        Self {
            op,
            reg: reg.into_reg(),
            bit,
            offset,
        }
    }
}

impl<R: IntoReg<RegOrZero32>> MakeTestBranch<R, TestBranchBit32, LabelRef>
    for TestBranch<RegOrZero32, TestBranchBit32, LabelRef>
{
    fn new(op: bool, reg: R, bit: TestBranchBit32, offset: LabelRef) -> Self {
        Self {
            op,
            reg: reg.into_reg(),
            bit,
            offset,
        }
    }
}

impl RawInstruction for TestBranch<RegOrZero64, TestBranchBit64, TestBranchOffset> {
    #[inline]
    fn to_code(&self) -> InstructionCode {
        let bit = self.bit.bits();
        let b5 = bit >> 5;
        let b40 = bit & 0b11111;

        if self.op {
            TBNZ_only_testbranch(b5.into(), b40.into(), self.offset.into(), self.reg.index())
        } else {
            TBZ_only_testbranch(b5.into(), b40.into(), self.offset.into(), self.reg.index())
        }
    }
}

impl RawInstruction for TestBranch<RegOrZero32, TestBranchBit32, TestBranchOffset> {
    #[inline]
    fn to_code(&self) -> InstructionCode {
        let bit = self.bit.bits();
        let b5 = 0;
        let b40 = bit;

        if self.op {
            TBNZ_only_testbranch(b5.into(), b40.into(), self.offset.into(), self.reg.index())
        } else {
            TBZ_only_testbranch(b5.into(), b40.into(), self.offset.into(), self.reg.index())
        }
    }
}

impl RelocatableInstruction for TestBranch<RegOrZero64, TestBranchBit64, LabelRef> {
    #[inline]
    fn to_code_with_reloc(&self) -> (InstructionCode, Option<Rel64>) {
        let code = TestBranch {
            op: self.op,
            reg: self.reg,
            bit: self.bit,
            offset: TestBranchOffset::default(),
        }
        .to_code();
        let rel = Rel64::tst_br14(self.offset.clone());
        (code, Some(rel))
    }
}

impl RelocatableInstruction for TestBranch<RegOrZero32, TestBranchBit32, LabelRef> {
    #[inline]
    fn to_code_with_reloc(&self) -> (InstructionCode, Option<Rel64>) {
        let code = TestBranch {
            op: self.op,
            reg: self.reg,
            bit: self.bit,
            offset: TestBranchOffset::default(),
        }
        .to_code();
        let rel = Rel64::tst_br14(self.offset.clone());
        (code, Some(rel))
    }
}

pub fn tbnz<Reg, InpReg, Bit, Offset>(
    reg: InpReg,
    bit: Bit,
    offset: Offset,
) -> TestBranch<Reg, Bit, Offset>
where
    TestBranch<Reg, Bit, Offset>: MakeTestBranch<InpReg, Bit, Offset>,
{
    TestBranch::new(true, reg, bit, offset)
}

pub fn tbz<R, Reg, Bit, Offset>(reg: Reg, bit: Bit, offset: Offset) -> TestBranch<R, Bit, Offset>
where
    TestBranch<R, Bit, Offset>: MakeTestBranch<Reg, Bit, Offset>,
{
    TestBranch::new(false, reg, bit, offset)
}

#[cfg(test)]
mod tests {
    extern crate alloc;
    use harm_test_utils::inst;

    use super::*;
    use crate::instructions::InstructionSeq as _;
    use crate::register::Reg32::*;
    use crate::register::Reg64::*;
    use crate::register::RegOrZero32::WZR;
    use crate::register::RegOrZero64::XZR;
    use crate::reloc::LabelId;
    use alloc::vec::Vec;

    #[test]
    fn test_tbz_64_big_pos() {
        let offset = TestBranchOffset::new(76).unwrap();
        let bit = TestBranchBit64::new(42).unwrap();
        let it = tbz(X2, bit, offset);
        let words: Vec<_> = it.encode().collect();
        // tbz x2, 42, 76
        assert_eq!(words, inst!(0xb6500262));
    }

    #[test]
    fn test_tbz_64_small_pos() {
        let offset = TestBranchOffset::new(76).unwrap();
        let bit = TestBranchBit64::new(29).unwrap();
        let it = tbz(X2, bit, offset);
        let words: Vec<_> = it.encode().collect();

        assert_eq!(words, inst!(0x36e80262));
    }

    #[test]
    fn test_tbz_xzr_big_pos() {
        let offset = TestBranchOffset::new(76).unwrap();
        let bit = TestBranchBit64::new(42).unwrap();
        let it = tbz(XZR, bit, offset);
        let words: Vec<_> = it.encode().collect();
        assert_eq!(words, inst!(0xb650027f));
    }

    #[test]
    fn test_tbz_64_neg_pos() {
        let offset = TestBranchOffset::new(-76).unwrap();
        let bit = TestBranchBit64::new(29).unwrap();
        let it = tbz(X2, bit, offset);
        let words: Vec<_> = it.encode().collect();

        assert_eq!(words, inst!(0x36effda2));
    }

    #[test]
    fn test_tbz_xzr_small_pos() {
        let offset = TestBranchOffset::new(76).unwrap();
        let bit = TestBranchBit64::new(29).unwrap();
        let it = tbz(XZR, bit, offset);
        let words: Vec<_> = it.encode().collect();
        assert_eq!(words, inst!(0x36e8027f));
    }

    #[test]
    fn test_tbz_32_pos() {
        let offset = TestBranchOffset::new(76).unwrap();
        let bit = TestBranchBit32::new(29).unwrap();
        let it = tbz(W2, bit, offset);
        let words: Vec<_> = it.encode().collect();
        assert_eq!(words, inst!(0x36e80262));
    }

    #[test]
    fn test_tbz_wzr_pos() {
        let offset = TestBranchOffset::new(76).unwrap();
        let bit = TestBranchBit32::new(29).unwrap();
        let it = tbz(WZR, bit, offset);
        let words: Vec<_> = it.encode().collect();
        assert_eq!(words, inst!(0x36e8027f));
    }

    #[test]
    fn test_tbnz_64_big_pos() {
        let offset = TestBranchOffset::new(76).unwrap();
        let bit = TestBranchBit64::new(42).unwrap();
        let it = tbnz(X2, bit, offset);
        let words: Vec<_> = it.encode().collect();
        // tbnz x2, 42, 76
        assert_eq!(words, inst!(0xb7500262));
    }

    #[test]
    fn test_tbnz_64_small_pos() {
        let offset = TestBranchOffset::new(76).unwrap();
        let bit = TestBranchBit64::new(29).unwrap();
        let it = tbnz(X2, bit, offset);
        let words: Vec<_> = it.encode().collect();

        assert_eq!(words, inst!(0x37e80262));
    }

    #[test]
    fn test_tbnz_xzr_big_pos() {
        let offset = TestBranchOffset::new(76).unwrap();
        let bit = TestBranchBit64::new(42).unwrap();
        let it = tbnz(XZR, bit, offset);
        let words: Vec<_> = it.encode().collect();
        assert_eq!(words, inst!(0xb750027f));
    }

    #[test]
    fn test_tbnz_64_neg_pos() {
        let offset = TestBranchOffset::new(-76).unwrap();
        let bit = TestBranchBit64::new(29).unwrap();
        let it = tbnz(X2, bit, offset);
        let words: Vec<_> = it.encode().collect();

        assert_eq!(words, inst!(0x37effda2));
    }

    #[test]
    fn test_tbnz_xzr_small_pos() {
        let offset = TestBranchOffset::new(76).unwrap();
        let bit = TestBranchBit64::new(29).unwrap();
        let it = tbnz(XZR, bit, offset);
        let words: Vec<_> = it.encode().collect();
        assert_eq!(words, inst!(0x37e8027f));
    }

    #[test]
    fn test_tbnz_32_pos() {
        let offset = TestBranchOffset::new(76).unwrap();
        let bit = TestBranchBit32::new(29).unwrap();
        let it = tbnz(W2, bit, offset);
        let words: Vec<_> = it.encode().collect();
        assert_eq!(words, inst!(0x37e80262));
    }

    #[test]
    fn test_tbnz_wzr_pos() {
        let offset = TestBranchOffset::new(76).unwrap();
        let bit = TestBranchBit32::new(29).unwrap();
        let it = tbnz(WZR, bit, offset);
        let words: Vec<_> = it.encode().collect();
        assert_eq!(words, inst!(0x37e8027f));
    }

    #[test]
    fn test_tbz_64_reloc() {
        let bit = TestBranchBit64::new(29).unwrap();
        let label = LabelRef {
            id: LabelId(8),
            addend: 3,
        };
        let (opcode, rel) = tbz(X2, bit, label.clone()).to_code_with_reloc();
        assert_eq!(opcode, tbz(X2, bit, TestBranchOffset::default()).to_code());
        assert_eq!(rel, Some(Rel64::tst_br14(label)));
    }

    #[test]
    fn test_tbnz_64_reloc() {
        let bit = TestBranchBit64::new(29).unwrap();
        let label = LabelRef {
            id: LabelId(8),
            addend: 3,
        };
        let (opcode, rel) = tbnz(X2, bit, label.clone()).to_code_with_reloc();
        assert_eq!(opcode, tbnz(X2, bit, TestBranchOffset::default()).to_code());
        assert_eq!(rel, Some(Rel64::tst_br14(label)));
    }
}
