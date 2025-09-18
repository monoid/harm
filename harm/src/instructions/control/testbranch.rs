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
    register::{RegOrZero32, RegOrZero64, Register as _},
    sealed::Sealed,
};

pub struct TestBranch<Reg, Bit> {
    op: bool,
    reg: Reg,
    bit: Bit,
    offset: SBitValue<14, 2>,
}

impl<Reg, Bit> Sealed for TestBranch<Reg, Bit> {}

pub trait MakeTestBranch<Reg, Bit>: Sealed {
    fn new(op: bool, reg: Reg, bit: Bit, offset: SBitValue<14, 2>) -> Self;
}

impl<R: Into<RegOrZero64>> MakeTestBranch<R, UBitValue<6>>
    for TestBranch<RegOrZero64, UBitValue<6>>
{
    fn new(op: bool, reg: R, bit: UBitValue<6>, offset: SBitValue<14, 2>) -> Self {
        Self {
            op,
            reg: reg.into(),
            bit,
            offset,
        }
    }
}

impl<R: Into<RegOrZero32>> MakeTestBranch<R, UBitValue<5>>
    for TestBranch<RegOrZero32, UBitValue<5>>
{
    fn new(op: bool, reg: R, bit: UBitValue<5>, offset: SBitValue<14, 2>) -> Self {
        Self {
            op,
            reg: reg.into(),
            bit,
            offset,
        }
    }
}

impl RawInstruction for TestBranch<RegOrZero64, UBitValue<6>> {
    #[inline]
    fn to_code(&self) -> InstructionCode {
        let bit = self.bit.bits();
        let b5 = bit >> 5;
        let b40 = bit & 0b11111;

        if self.op {
            TBNZ_only_testbranch(b5.into(), b40.into(), self.offset.into(), self.reg.code())
        } else {
            TBZ_only_testbranch(
                (bit >> 5).into(),
                (bit & 0b11111).into(),
                self.offset.into(),
                self.reg.code(),
            )
        }
    }
}

impl RawInstruction for TestBranch<RegOrZero32, UBitValue<5>> {
    #[inline]
    fn to_code(&self) -> InstructionCode {
        let bit = self.bit.bits();
        let b5 = 0;
        let b40 = bit;

        if self.op {
            TBNZ_only_testbranch(b5.into(), b40.into(), self.offset.into(), self.reg.code())
        } else {
            TBZ_only_testbranch(b5.into(), b40.into(), self.offset.into(), self.reg.code())
        }
    }
}

pub fn tbnz<Reg, InpReg, Bit>(
    reg: InpReg,
    bit: Bit,
    offset: SBitValue<14, 2>,
) -> TestBranch<Reg, Bit>
where
    TestBranch<Reg, Bit>: MakeTestBranch<InpReg, Bit>,
{
    TestBranch::new(true, reg, bit, offset)
}

pub fn tbz<R, Reg, Bit>(reg: Reg, bit: Bit, offset: SBitValue<14, 2>) -> TestBranch<R, Bit>
where
    TestBranch<R, Bit>: MakeTestBranch<Reg, Bit>,
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
        assert_eq!(words, inst!([0x62, 0x02, 0x50, 0xb6])); // 0xb6500262
    }

    #[test]
    fn test_tbz_64_small_pos() {
        let offset = SBitValue::new(76).unwrap();
        let bit = UBitValue::new(29).unwrap();
        let it = tbz(X2, bit, offset);
        let words: Vec<_> = it.encode().collect();

        assert_eq!(words, inst!([0x62, 0x02, 0xe8, 0x36])); // 0x36e80262
    }

    #[test]
    fn test_tbz_xzr_big_pos() {
        let offset = SBitValue::new(76).unwrap();
        let bit = UBitValue::new(42).unwrap();
        let it = tbz(XZR, bit, offset);
        let words: Vec<_> = it.encode().collect();
        assert_eq!(words, inst!([0x7f, 0x02, 0x50, 0xb6])); // 0xb650027f
    }

    #[test]
    fn test_tbz_xzr_small_pos() {
        let offset = SBitValue::new(76).unwrap();
        let bit = UBitValue::new(29).unwrap();
        let it = tbz(XZR, bit, offset);
        let words: Vec<_> = it.encode().collect();
        assert_eq!(words, inst!([0x7f, 0x02, 0xe8, 0x36])); // 0x36e8027f
    }

    #[test]
    fn test_tbz_32_pos() {
        let offset = SBitValue::new(76).unwrap();
        let bit = UBitValue::new(29).unwrap();
        let it = tbz(W2, bit, offset);
        let words: Vec<_> = it.encode().collect();
        assert_eq!(words, inst!([0x62, 0x02, 0xe8, 0x36])); // 0x36e80262
    }

    #[test]
    fn test_tbz_wzr_pos() {
        let offset = SBitValue::new(76).unwrap();
        let bit = UBitValue::new(29).unwrap();
        let it = tbz(WZR, bit, offset);
        let words: Vec<_> = it.encode().collect();
        assert_eq!(words, inst!([0x7f, 0x02, 0xe8, 0x36])); // 0x36e8027f
    }
}
