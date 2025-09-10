/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

pub(crate) mod reg;
pub(crate) mod testbranch;

use aarchmrs_instructions::A64::control::{
    branch_imm::{B_only_branch_imm::B_only_branch_imm, BL_only_branch_imm::BL_only_branch_imm},
    condbranch::B_only_condbranch::B_only_condbranch, // TODO BC: branch consistent conditionally
};
use aarchmrs_types::InstructionCode;

use super::RawInstruction;
use crate::{
    bits::{BitError, SBitValue},
    register::{IntoCode as _, RegOrZero32, RegOrZero64},
};

pub use self::reg::*;
pub use self::testbranch::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
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

pub type BranchOffset = SBitValue<26, 2>;
pub type BranchCondOffset = SBitValue<19, 2>;

pub type CompareBranchOffset = SBitValue<19, 2>;

pub trait MakeBranch<Args> {
    type Output;

    fn make(args: Args) -> Self::Output;
}

pub trait MakeBranchCond<Args> {
    type Output;

    fn make(cond: BranchCond, args: Args) -> Self::Output;
}

#[inline]
pub fn b<InpArgs, RealArgs>(args: InpArgs) -> <Branch<RealArgs> as MakeBranch<InpArgs>>::Output
where
    Branch<RealArgs>: MakeBranch<InpArgs>,
{
    <Branch<RealArgs> as MakeBranch<InpArgs>>::make(args)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Branch<B>(B);

impl MakeBranch<BranchOffset> for Branch<BranchOffset> {
    type Output = Self;

    #[inline]
    fn make(offset: BranchOffset) -> Self::Output {
        Self(offset)
    }
}

impl MakeBranch<i32> for Branch<BranchOffset> {
    type Output = Result<Self, BitError>;

    #[inline]
    fn make(offset: i32) -> Self::Output {
        BranchOffset::try_from(offset).map(Self)
    }
}

#[cfg(feature = "rich_api")]
impl<S, D> MakeBranch<(BranchCond, S)> for Branch<(BranchCond, D)>
where
    Branch<(BranchCond, D)>: MakeBranchCond<S>,
{
    type Output = <Branch<(BranchCond, D)> as MakeBranchCond<S>>::Output;

    fn make((cond, addr): (BranchCond, S)) -> Self::Output {
        <Branch<(BranchCond, D)> as MakeBranchCond<S>>::make(cond, addr)
    }
}

impl RawInstruction for Branch<BranchOffset> {
    #[inline]
    fn to_code(&self) -> InstructionCode {
        let imm26 = self.0;
        B_only_branch_imm(imm26.into())
    }
}

pub fn b_cond<InpAddr, RealAddr>(
    cond: BranchCond,
    offset: InpAddr,
) -> <Branch<(BranchCond, RealAddr)> as MakeBranchCond<InpAddr>>::Output
where
    Branch<(BranchCond, RealAddr)>: MakeBranchCond<InpAddr>,
{
    <Branch<_> as MakeBranchCond<InpAddr>>::make(cond, offset)
}

impl MakeBranchCond<BranchCondOffset> for Branch<(BranchCond, BranchCondOffset)> {
    type Output = Self;

    #[inline]
    fn make(cond: BranchCond, addr: BranchCondOffset) -> Self::Output {
        Self((cond, addr))
    }
}

impl MakeBranchCond<i32> for Branch<(BranchCond, BranchCondOffset)> {
    type Output = Result<Self, BitError>;

    #[inline]
    fn make(cond: BranchCond, offset: i32) -> Self::Output {
        BranchCondOffset::try_from(offset).map(|offset| Self((cond, offset)))
    }
}

impl RawInstruction for Branch<(BranchCond, BranchCondOffset)> {
    #[inline]
    fn to_code(&self) -> InstructionCode {
        let (cond, imm19) = self.0;
        B_only_condbranch(imm19.into(), (cond as u8).into())
    }
}

pub trait MakeBranchLink<Args> {
    type Output;

    fn make(args: Args) -> Self::Output;
}

#[inline]
pub fn bl<InpArgs>(args: InpArgs) -> <BranchLink as MakeBranchLink<InpArgs>>::Output
where
    BranchLink: MakeBranchLink<InpArgs>,
{
    <BranchLink as MakeBranchLink<InpArgs>>::make(args)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct BranchLink(BranchOffset);

impl MakeBranchLink<BranchOffset> for BranchLink {
    type Output = Self;

    #[inline]
    fn make(offset: BranchOffset) -> Self::Output {
        Self(offset)
    }
}

impl MakeBranchLink<i32> for BranchLink {
    type Output = Result<Self, BitError>;

    #[inline]
    fn make(offset: i32) -> Self::Output {
        BranchOffset::try_from(offset).map(Self)
    }
}

impl RawInstruction for BranchLink {
    #[inline]
    fn to_code(&self) -> InstructionCode {
        let imm26 = self.0;
        BL_only_branch_imm(imm26.into())
    }
}

pub struct CompareBranch<Reg> {
    equal: bool,
    reg: Reg,
    offset: CompareBranchOffset,
}

pub trait MakeCompareBranch<Reg, Offset>: Sized {
    type Output;

    fn new(equal: bool, reg: Reg, offset: Offset) -> Self::Output;
}

impl<R64> MakeCompareBranch<R64, CompareBranchOffset> for CompareBranch<RegOrZero64>
where
    R64: Into<RegOrZero64>,
{
    type Output = Self;

    fn new(equal: bool, reg: R64, offset: CompareBranchOffset) -> Self {
        Self {
            equal,
            reg: reg.into(),
            offset,
        }
    }
}

// N.B. joining these two implementation abstracting over `RegDst` in `CompareBranch<RegDst>`
// doesn't work: Rust doesn't seems to be able to deduce the `RegDst` type
impl<RegSrc> MakeCompareBranch<RegSrc, i32> for CompareBranch<RegOrZero64>
where
    RegOrZero64: From<RegSrc>,
{
    type Output = Result<Self, BitError>;

    fn new(equal: bool, reg: RegSrc, offset: i32) -> Result<Self, BitError> {
        CompareBranchOffset::try_from(offset).map(|offset| Self {
            equal,
            reg: reg.into(),
            offset,
        })
    }
}

impl<RegSrc> MakeCompareBranch<RegSrc, i32> for CompareBranch<RegOrZero32>
where
    RegOrZero32: From<RegSrc>,
{
    type Output = Result<Self, BitError>;

    fn new(equal: bool, reg: RegSrc, offset: i32) -> Result<Self, BitError> {
        CompareBranchOffset::try_from(offset).map(|offset| Self {
            equal,
            reg: reg.into(),
            offset,
        })
    }
}

impl<R32> MakeCompareBranch<R32, CompareBranchOffset> for CompareBranch<RegOrZero32>
where
    R32: Into<RegOrZero32>,
{
    type Output = Self;

    fn new(equal: bool, reg: R32, offset: CompareBranchOffset) -> Self {
        Self {
            equal,
            reg: reg.into(),
            offset,
        }
    }
}

impl RawInstruction for CompareBranch<RegOrZero64> {
    #[inline]
    fn to_code(&self) -> InstructionCode {
        use aarchmrs_instructions::A64::control::compbranch;

        if self.equal {
            compbranch::CBZ_64_compbranch::CBZ_64_compbranch(self.offset.into(), self.reg.code())
        } else {
            compbranch::CBNZ_64_compbranch::CBNZ_64_compbranch(self.offset.into(), self.reg.code())
        }
    }
}

impl RawInstruction for CompareBranch<RegOrZero32> {
    #[inline]
    fn to_code(&self) -> InstructionCode {
        use aarchmrs_instructions::A64::control::compbranch;

        if self.equal {
            compbranch::CBZ_32_compbranch::CBZ_32_compbranch(self.offset.into(), self.reg.code())
        } else {
            compbranch::CBNZ_32_compbranch::CBNZ_32_compbranch(self.offset.into(), self.reg.code())
        }
    }
}

pub fn cbz<RegSrc, Offset, RegDst>(
    reg: RegSrc,
    offset: Offset,
) -> <CompareBranch<RegDst> as MakeCompareBranch<RegSrc, Offset>>::Output
where
    CompareBranch<RegDst>: MakeCompareBranch<RegSrc, Offset>,
{
    CompareBranch::new(true, reg, offset)
}

pub fn cbnz<RegSrc, Offset, RegDst>(
    reg: RegSrc,
    offset: Offset,
) -> <CompareBranch<RegDst> as MakeCompareBranch<RegSrc, Offset>>::Output
where
    CompareBranch<RegDst>: MakeCompareBranch<RegSrc, Offset>,
{
    CompareBranch::new(false, reg, offset)
}

#[cfg(test)]
mod tests {
    extern crate alloc;
    use super::*;
    use crate::instructions::InstructionSeq;
    use crate::register::Reg32::*;
    use crate::register::Reg64::*;
    use crate::register::RegOrZero32::WZR;
    use crate::register::RegOrZero64::XZR;
    use alloc::vec::Vec;
    use harm_test_utils::inst;

    #[test]
    fn test_b_i32() {
        let it = b(0x48);
        let codes: Vec<_> = it.unwrap().encode().collect();
        // TODO check
        assert_eq!(codes, inst!(0x14000012));
    }

    #[test]
    fn test_b_i32_neg() {
        let it = b(-0x48);
        let codes: Vec<_> = it.unwrap().encode().collect();
        // TODO check
        assert_eq!(codes, inst!(0x17ffffee));
    }

    #[test]
    fn test_b_sbits() {
        let offset = BranchOffset::new(48).unwrap();
        let it = b(offset);
        let codes: Vec<_> = it.encode().collect();
        // TODO check
        assert_eq!(codes, inst!(0x1400000c));
    }

    #[test]
    fn test_bl_sbits() {
        let offset = BranchOffset::new(48).unwrap();
        let it = bl(offset);
        let codes: Vec<_> = it.encode().collect();
        // TODO check
        assert_eq!(codes, inst!(0x9400000c));
    }

    #[test]
    fn test_bl_i32() {
        let it = bl(0x48);
        let codes: Vec<_> = it.unwrap().encode().collect();
        // TODO check
        assert_eq!(codes, inst!(0x94000012));
    }

    #[test]
    fn test_bl_i32_neg() {
        let it = bl(-0x48);
        let codes: Vec<_> = it.unwrap().encode().collect();
        // TODO check
        assert_eq!(codes, inst!(0x97ffffee));
    }

    #[test]
    fn test_b_cond_i32() {
        use BranchCond::*;
        let it = b_cond(EQ, 0x48);
        let codes: Vec<_> = it.unwrap().encode().collect();
        // TODO check
        assert_eq!(codes, inst!(0x54000240));
    }

    #[test]
    fn test_b_cond_i32_neg() {
        use BranchCond::*;
        let it = b_cond(EQ, -0x48);
        let codes: Vec<_> = it.unwrap().encode().collect();
        // TODO check
        assert_eq!(codes, inst!(0x54fffdc0));
    }

    #[test]
    fn test_b_cond_sbits() {
        use BranchCond::*;
        let offset = BranchCondOffset::new(0x48).unwrap();
        let it = b_cond(EQ, offset);
        let codes: Vec<_> = it.encode().collect();
        assert_eq!(codes, inst!(0x54000240));
    }

    #[test]
    fn test_b_overflow() {
        // 1 bit of sign + 25 bits + 2 bits of offset
        assert!(b(1 << 27).is_err());
        assert!(b((1 << 27) - 1).is_err());
        assert!(b((1 << 27) - 4).is_ok());
        assert!(b(-((1 << 27) + 4)).is_err());
        assert!(b(-(1 << 27)).is_ok());
        assert!(b((1 << 27) - 4).is_ok());
    }

    #[test]
    fn test_b_cond_overflow() {
        use BranchCond::*;
        // 1 bit of sign + 18 bits + 2 bits of offset
        assert!(b_cond(EQ, 1 << 20).is_err());
        assert!(b_cond(NE, (1 << 20) - 1).is_err());
        assert!(b_cond(GT, (1 << 20) - 4).is_ok());
        assert!(b_cond(LE, -((1 << 20) + 4)).is_err());
        assert!(b_cond(PL, -(1 << 20)).is_ok());
        assert!(b_cond(VC, (1 << 20) - 4).is_ok());
    }

    #[test]
    #[cfg(feature = "rich_api")]
    fn test_b_vs_bcond() {
        use BranchCond::*;

        assert_eq!(b((EQ, 42)), b_cond(EQ, 42));
        assert_eq!(b((GT, 0x23456789)), b_cond(GT, 0x23456789));
    }

    #[test]
    fn test_cbnz_64_pos() {
        let offset = SBitValue::new(8).unwrap();
        let it = cbnz(X2, offset);

        let codes: Vec<_> = it.encode().collect();
        assert_eq!(codes, inst!([0x42, 0x00, 0x00, 0xb5])); // 0xb5000042
    }

    #[test]
    fn test_cbnz_64_neg() {
        let offset = SBitValue::new(-8).unwrap();
        let it = cbnz(X2, offset);

        let codes: Vec<_> = it.encode().collect();
        assert_eq!(codes, inst!([0xc2, 0xff, 0xff, 0xb5])); // 0xb5ffffc2
    }

    #[test]
    fn test_cbnz_xzr_pos() {
        let offset = SBitValue::new(8).unwrap();
        let it = cbnz(XZR, offset);

        let codes: Vec<_> = it.encode().collect();
        assert_eq!(codes, inst!(0xb500005f));
    }

    #[test]
    fn test_cbnz_64_i32() {
        let it = cbnz(X2, 8).unwrap();

        let codes: Vec<_> = it.encode().collect();
        assert_eq!(codes, inst!([0x42, 0x00, 0x00, 0xb5])); // 0xb5000042
    }

    #[test]
    fn test_cbz_64_pos() {
        let offset = SBitValue::new(8).unwrap();
        let it = cbz(X2, offset);

        let codes: Vec<_> = it.encode().collect();
        assert_eq!(codes, inst!([0x42, 0x00, 0x00, 0xb4])); // 0xb4000042
    }

    #[test]
    fn test_cbz_64_i32() {
        let it = cbz(X2, 8).unwrap();

        let codes: Vec<_> = it.encode().collect();
        assert_eq!(codes, inst!([0x42, 0x00, 0x00, 0xb4])); // 0xb4000042
    }

    #[test]
    fn test_cbz_64_neg() {
        let offset = SBitValue::new(-8).unwrap();
        let it = cbz(X2, offset);

        let codes: Vec<_> = it.encode().collect();
        assert_eq!(codes, inst!([0xc2, 0xff, 0xff, 0xb4])); // 0xb4ffffc2
    }

    #[test]
    fn test_cbz_xzr_pos() {
        let offset = SBitValue::new(8).unwrap();
        let it = cbz(XZR, offset);
        let codes: Vec<_> = it.encode().collect();
        assert_eq!(codes, inst!(0xb400005f))
    }

    #[test]
    fn test_cbnz_32_pos() {
        let offset = SBitValue::new(8).unwrap();
        let it = cbnz(W2, offset);

        let codes: Vec<_> = it.encode().collect();
        assert_eq!(codes, inst!([0x42, 0x00, 0x00, 0x35])); // 35000042
    }

    #[test]
    fn test_cbnz_32_neg() {
        let offset = SBitValue::new(-8).unwrap();
        let it = cbnz(W2, offset);

        let codes: Vec<_> = it.encode().collect();
        assert_eq!(codes, inst!([0xc2, 0xff, 0xff, 0x35])); // 35ffffc2
    }

    #[test]
    fn test_cbnz_32_i32() {
        let it = cbnz(W2, 8).unwrap();

        let codes: Vec<_> = it.encode().collect();
        assert_eq!(codes, inst!(0x35000042));
    }

    #[test]
    fn test_cbz_32_pos() {
        let offset = SBitValue::new(8).unwrap();
        let it = cbz(W2, offset);

        let codes: Vec<_> = it.encode().collect();
        assert_eq!(codes, inst!([0x42, 0x00, 0x00, 0x34])); // 0x34000042
    }

    #[test]
    fn test_cbz_32_neg() {
        let offset = SBitValue::new(-8).unwrap();
        let it = cbz(W2, offset);

        let codes: Vec<_> = it.encode().collect();
        assert_eq!(codes, inst!([0xc2, 0xff, 0xff, 0x34])); // 0x34ffffc2
    }

    #[test]
    fn test_cbz_wzr_pos() {
        let offset = SBitValue::new(8).unwrap();
        let it = cbz(WZR, offset);
        let codes: Vec<_> = it.encode().collect();
        assert_eq!(codes, inst!(0x3400005f))
    }

    #[test]
    fn test_cbz_32_i32() {
        let it = cbz(W2, 8).unwrap();

        let codes: Vec<_> = it.encode().collect();
        assert_eq!(codes, inst!(0x34000042));
    }
}
