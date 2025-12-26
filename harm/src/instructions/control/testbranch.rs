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
    instructions::RawInstruction,
    register::{IntoReg, RegOrZero32, RegOrZero64, Register as _},
    sealed::Sealed,
};

pub type TestBit64 = UBitValue<6>;
pub type TestBit32 = UBitValue<5>;
pub type TestOffset = SBitValue<14, 2>;

pub struct TestBranch<Reg, Bit, Offset> {
    op: bool,
    reg: Reg,
    bit: Bit,
    offset: Offset,
}

impl Sealed for TestBranch<RegOrZero64, TestBit64, TestOffset> {}
impl Sealed for TestBranch<RegOrZero32, TestBit32, TestOffset> {}

pub trait MakeTestBranch<Reg, Bit, Offset>: Sealed {
    fn new(op: bool, reg: Reg, bit: Bit, offset: Offset) -> Self;
}

impl<R: IntoReg<RegOrZero64>> MakeTestBranch<R, TestBit64, TestOffset>
    for TestBranch<RegOrZero64, UBitValue<6>, TestOffset>
{
    fn new(op: bool, reg: R, bit: UBitValue<6>, offset: TestOffset) -> Self {
        Self {
            op,
            reg: reg.into_reg(),
            bit,
            offset,
        }
    }
}

impl<R: Into<RegOrZero32>> MakeTestBranch<R, TestBit32, TestOffset>
    for TestBranch<RegOrZero32, TestBit32, TestOffset>
{
    fn new(op: bool, reg: R, bit: TestBit32, offset: TestOffset) -> Self {
        Self {
            op,
            reg: reg.into(),
            bit,
            offset,
        }
    }
}

impl RawInstruction for TestBranch<RegOrZero64, TestBit64, TestOffset> {
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

impl RawInstruction for TestBranch<RegOrZero32, TestBit32, TestOffset> {
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
    use alloc::vec::Vec;

    #[test]
    fn test_tbz_64_big_pos() {
        let offset = SBitValue::new(76).unwrap();
        let bit = UBitValue::new(42).unwrap();
        let it = tbz(X2, bit, offset);
        let words: Vec<_> = it.encode().collect();
        // tbz x2, 42, 76
        assert_eq!(words, inst!(0xb6500262));
    }

    #[test]
    fn test_tbz_64_small_pos() {
        let offset = SBitValue::new(76).unwrap();
        let bit = UBitValue::new(29).unwrap();
        let it = tbz(X2, bit, offset);
        let words: Vec<_> = it.encode().collect();

        assert_eq!(words, inst!(0x36e80262));
    }

    #[test]
    fn test_tbz_xzr_big_pos() {
        let offset = SBitValue::new(76).unwrap();
        let bit = UBitValue::new(42).unwrap();
        let it = tbz(XZR, bit, offset);
        let words: Vec<_> = it.encode().collect();
        assert_eq!(words, inst!(0xb650027f));
    }

    #[test]
    fn test_tbz_xzr_small_pos() {
        let offset = SBitValue::new(76).unwrap();
        let bit = UBitValue::new(29).unwrap();
        let it = tbz(XZR, bit, offset);
        let words: Vec<_> = it.encode().collect();
        assert_eq!(words, inst!(0x36e8027f));
    }

    #[test]
    fn test_tbz_32_pos() {
        let offset = SBitValue::new(76).unwrap();
        let bit = UBitValue::new(29).unwrap();
        let it = tbz(W2, bit, offset);
        let words: Vec<_> = it.encode().collect();
        assert_eq!(words, inst!(0x36e80262));
    }

    #[test]
    fn test_tbz_wzr_pos() {
        let offset = SBitValue::new(76).unwrap();
        let bit = UBitValue::new(29).unwrap();
        let it = tbz(WZR, bit, offset);
        let words: Vec<_> = it.encode().collect();
        assert_eq!(words, inst!(0x36e8027f));
    }
}
