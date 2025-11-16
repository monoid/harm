/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

/*! `ADR` and `ADRP` instructions. */

use aarchmrs_instructions::A64::dpimm::pcreladdr::{
    ADR_only_pcreladdr::ADR_only_pcreladdr, ADRP_only_pcreladdr::ADRP_only_pcreladdr,
};

use crate::{
    bits::SBitValue, instructions::RawInstruction, register::{IntoReg, RegOrZero64, Register}, InstructionCode
};

pub type AdrOffset = SBitValue<19>;
pub type AdrpOffset = SBitValue<19, 12>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Adr<Offset> {
    pub reg: RegOrZero64,
    pub offset: Offset,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Adrp<Offset> {
    pub reg: RegOrZero64,
    pub offset: Offset,
}

pub trait MakeAdr<OffsetIn> {
    type Outcome;

    fn new(rd: RegOrZero64, offset: OffsetIn) -> Self::Outcome;
}

impl MakeAdr<AdrOffset> for Adr<AdrOffset> {
    type Outcome = Self;

    fn new(rd: RegOrZero64, offset: AdrOffset) -> Self::Outcome {
        Self { reg: rd, offset }
    }
}

pub trait MakeAdrp<OffsetIn> {
    type Outcome;

    fn new(rd: RegOrZero64, offset: OffsetIn) -> Self::Outcome;
}

impl MakeAdrp<AdrpOffset> for Adrp<AdrpOffset> {
    type Outcome = Self;

    fn new(rd: RegOrZero64, offset: AdrpOffset) -> Self::Outcome {
        Self { reg: rd, offset }
    }
}

pub fn adr<OffsetIn>(
    rd: impl IntoReg<RegOrZero64>,
    offset: OffsetIn,
) -> <Adr<AdrOffset> as MakeAdr<OffsetIn>>::Outcome
where
    Adr<AdrOffset>: MakeAdr<OffsetIn>,
{
    <Adr<AdrOffset> as MakeAdr<OffsetIn>>::new(rd.into_reg(), offset)
}

pub fn adrp<OffsetIn>(
    rd: impl IntoReg<RegOrZero64>,
    offset: OffsetIn,
) -> <Adrp<AdrpOffset> as MakeAdrp<OffsetIn>>::Outcome
where
    Adrp<AdrpOffset>: MakeAdrp<OffsetIn>,
{
    <Adrp<AdrpOffset> as MakeAdrp<OffsetIn>>::new(rd.into_reg(), offset)
}

const LO_BITS: u32 = 2;
const LO_BITS_MASK: u32 = (1 << LO_BITS) - 1;

impl RawInstruction for Adr<AdrOffset> {
    fn to_code(&self) -> InstructionCode {
        let immlo = self.offset.bits() & LO_BITS_MASK;
        let immhi = self.offset.bits() >> LO_BITS;
        ADR_only_pcreladdr(immlo.into(), immhi.into(), self.reg.index())
    }
}

impl RawInstruction for Adrp<AdrpOffset> {
    fn to_code(&self) -> InstructionCode {
        let immlo = self.offset.bits() & LO_BITS_MASK;
        let immhi = self.offset.bits() >> LO_BITS;
        ADRP_only_pcreladdr(immlo.into(), immhi.into(), self.reg.index())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::register::Reg64::*;
    
    #[test]
    fn test_adr_m4() {
        let adr_offset: AdrOffset = SBitValue::new(-4).unwrap();
        let adr = adr(X29, adr_offset);
        let adr_code = adr.to_code();
        assert_eq!(adr_code.unpack(), 0x103ffffd, "0x{:x}", adr_code.unpack());
    }

    #[test]
    fn test_adr_0x12345() {
        let adr_offset: AdrOffset = SBitValue::new(0x12345).unwrap();
        let adr = adr(X28, adr_offset);
        let adr_code = adr.to_code();
        // TODO check the value is correct
        assert_eq!(adr_code.unpack(), 0x30091a3c, "0x{:x}", adr_code.unpack());
    }

    #[test]
    fn test_adrp_m4() {
        let adr_offset: AdrpOffset = SBitValue::new(-4 << 12).unwrap();
        let adrp = adrp(X27, adr_offset);
        let adrp_code = adrp.to_code();
        // TODO check the value is correct
        assert_eq!(adrp_code.unpack(), 0x903ffffb, "0x{:x}", adrp_code.unpack());
    }

    #[test]
    fn test_adrp_0x12345() {
        let adr_offset: AdrpOffset = SBitValue::new(0x12345 << 12).unwrap();
        let adrp = adrp(X9, adr_offset);
        let adrp_code = adrp.to_code();
        // TODO check the value is correct
        assert_eq!(adrp_code.unpack(), 0xb0091a29, "0x{:x}", adrp_code.unpack());
    }
}
