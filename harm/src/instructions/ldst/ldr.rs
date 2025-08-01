/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */
use aarchmrs_instructions::A64::ldst::{
    ldst_pos::{LDR_32_ldst_pos::LDR_32_ldst_pos, LDR_64_ldst_pos::LDR_64_ldst_pos},
    ldst_regoff::{LDR_32_ldst_regoff::LDR_32_ldst_regoff, LDR_64_ldst_regoff::LDR_64_ldst_regoff},
};

use super::shift_extend::*;
use super::{Load, MakeLoad, ScaledOffset32, ScaledOffset64};
use crate::{
    bits::BitError,
    instructions::Instruction,
    register::{IntoCode, Reg32, Reg64, RegOrSp64, RegOrZero32, RegOrZero64},
};

impl<Base, Ext> MakeLoad<Reg64, (Base, Ext)> for Load<Reg64, (RegOrSp64, Extended<Reg64, Reg64>)>
where
    Base: Into<RegOrSp64>,
    Ext: Into<Extended<Reg64, Reg64>>,
{
    type Output = Self;

    #[inline]
    fn new(dst: Reg64, (base, offset): (Base, Ext)) -> Self {
        Self {
            dst,
            addr: (base.into(), offset.into()),
        }
    }
}

impl Instruction for Load<Reg64, (RegOrSp64, Extended<Reg64, Reg64>)> {
    #[inline]
    fn represent(self) -> impl Iterator<Item = aarchmrs_types::InstructionCode> + 'static {
        let (base, offset) = self.addr;
        let code = LDR_64_ldst_regoff(
            offset.shifted.offset.code(),
            (offset.extend as u8).into(),
            offset.shifted.shifted.into(),
            base.code(),
            self.dst.code(),
        );
        std::iter::once(code)
    }
}

impl<Base, Ext> MakeLoad<Reg64, (Base, Ext)> for Load<Reg64, (RegOrSp64, Extended<Reg64, Reg32>)>
where
    Base: Into<RegOrSp64>,
    Ext: Into<Extended<Reg64, Reg32>>,
{
    type Output = Self;

    #[inline]
    fn new(dst: Reg64, (base, offset): (Base, Ext)) -> Self {
        Self {
            dst,
            addr: (base.into(), offset.into()),
        }
    }
}

impl Instruction for Load<Reg64, (RegOrSp64, Extended<Reg64, Reg32>)> {
    #[inline]
    fn represent(self) -> impl Iterator<Item = aarchmrs_types::InstructionCode> + 'static {
        let (base, offset) = self.addr;
        let code = LDR_64_ldst_regoff(
            offset.shifted.offset.code(),
            (offset.extend as u8).into(),
            offset.shifted.shifted.into(),
            base.code(),
            self.dst.code(),
        );
        std::iter::once(code)
    }
}

impl<Base, Ext> MakeLoad<Reg32, (Base, Ext)> for Load<Reg32, (RegOrSp64, Extended<Reg32, Reg64>)>
where
    Base: Into<RegOrSp64>,
    Ext: Into<Extended<Reg32, Reg64>>,
{
    type Output = Self;

    #[inline]
    fn new(dst: Reg32, (base, offset): (Base, Ext)) -> Self {
        Self {
            dst,
            addr: (base.into(), offset.into()),
        }
    }
}

impl Instruction for Load<Reg32, (RegOrSp64, Extended<Reg32, Reg64>)> {
    #[inline]
    fn represent(self) -> impl Iterator<Item = aarchmrs_types::InstructionCode> + 'static {
        let (base, offset) = self.addr;
        let code = LDR_64_ldst_regoff(
            offset.shifted.offset.code(),
            (offset.extend as u8).into(),
            offset.shifted.shifted.into(),
            base.code(),
            self.dst.code(),
        );
        std::iter::once(code)
    }
}

impl<Base, Ext> MakeLoad<Reg32, (Base, Ext)> for Load<Reg32, (RegOrSp64, Extended<Reg32, Reg32>)>
where
    Base: Into<RegOrSp64>,
    Ext: Into<Extended<Reg32, Reg32>>,
{
    type Output = Self;

    #[inline]
    fn new(dst: Reg32, (base, offset): (Base, Ext)) -> Self {
        Self {
            dst,
            addr: (base.into(), offset.into()),
        }
    }
}

impl<Dest, Base, Ext, Err> MakeLoad<Dest, (Base, Result<Ext, Err>)> for Load<Dest, (Base, Ext)>
where
    Load<Dest, (Base, Ext)>: MakeLoad<Dest, (Base, Ext)>,
{
    type Output = Result<<Self as MakeLoad<Dest, (Base, Ext)>>::Output, Err>;

    #[inline]
    fn new(dst: Dest, (base, offset): (Base, Result<Ext, Err>)) -> Self::Output {
        offset.map(|offset| Load::new(dst, (base, offset)))
    }
}

impl Instruction for Load<Reg32, (RegOrSp64, Extended<Reg32, Reg32>)> {
    #[inline]
    fn represent(self) -> impl Iterator<Item = aarchmrs_types::InstructionCode> + 'static {
        let (base, offset) = self.addr;
        let code = LDR_64_ldst_regoff(
            offset.shifted.offset.code(),
            (offset.extend as u8).into(),
            offset.shifted.shifted.into(),
            base.code(),
            self.dst.code(),
        );
        std::iter::once(code)
    }
}

impl<Base> MakeLoad<Reg64, Base> for Load<Reg64, (RegOrSp64, RegOrZero64)>
where
    Base: Into<RegOrSp64>,
{
    type Output = Self;

    #[inline]
    fn new(dst: Reg64, base: Base) -> Self {
        Self {
            dst,
            // TODO does the spec says something specific?
            addr: (base.into(), RegOrZero64::XZR),
        }
    }
}

impl<Base> MakeLoad<Reg64, (Base,)> for Load<Reg64, (RegOrSp64, RegOrZero64)>
where
    Base: Into<RegOrSp64>,
{
    type Output = Self;

    #[inline]
    fn new(dst: Reg64, (base,): (Base,)) -> Self {
        Self {
            dst,
            addr: (base.into(), RegOrZero64::XZR),
        }
    }
}

impl<Base, OffsetReg> MakeLoad<Reg64, (Base, OffsetReg)> for Load<Reg64, (RegOrSp64, RegOrZero64)>
where
    Base: Into<RegOrSp64>,
    OffsetReg: Into<RegOrZero64>,
{
    type Output = Self;

    #[inline]
    fn new(dst: Reg64, (base, offset): (Base, OffsetReg)) -> Self {
        Self {
            dst,
            addr: (base.into(), offset.into()),
        }
    }
}

impl<Base> MakeLoad<Reg32, Base> for Load<Reg32, (RegOrSp64, RegOrZero64)>
where
    Base: Into<RegOrSp64>,
{
    type Output = Self;

    #[inline]
    fn new(dst: Reg32, base: Base) -> Self {
        Self {
            dst,
            addr: (base.into(), RegOrZero64::XZR),
        }
    }
}

impl<Base> MakeLoad<Reg32, (Base,)> for Load<Reg32, (RegOrSp64, RegOrZero64)>
where
    Base: Into<RegOrSp64>,
{
    type Output = Self;

    #[inline]
    fn new(dst: Reg32, (base,): (Base,)) -> Self {
        Self {
            dst,
            addr: (base.into(), RegOrZero64::XZR),
        }
    }
}

impl Instruction for Load<Reg64, (RegOrSp64, RegOrZero64)> {
    #[inline]
    fn represent(self) -> impl Iterator<Item = aarchmrs_types::InstructionCode> + 'static {
        let (base, offset) = self.addr;
        let code = LDR_64_ldst_regoff(
            offset.code(),
            (LdrExtendOption64::default() as u8).into(),
            0b0.into(),
            base.code(),
            self.dst.code(),
        );
        std::iter::once(code)
    }
}

impl<B, O> MakeLoad<Reg32, (B, O)> for Load<Reg32, (RegOrSp64, RegOrZero64)>
where
    B: Into<RegOrSp64>,
    O: Into<RegOrZero64>,
{
    type Output = Self;

    #[inline]
    fn new(dst: Reg32, (base, offset): (B, O)) -> Self {
        Self {
            dst,
            addr: (base.into(), offset.into()),
        }
    }
}

impl Instruction for Load<Reg32, (RegOrSp64, RegOrZero64)> {
    #[inline]
    fn represent(self) -> impl Iterator<Item = aarchmrs_types::InstructionCode> + 'static {
        let (base, offset) = self.addr;
        let code = LDR_32_ldst_regoff(
            offset.code(),
            (LdrExtendOption64::default() as u8).into(),
            0b0.into(),
            base.code(),
            self.dst.code(),
        );
        std::iter::once(code)
    }
}

impl<B> MakeLoad<Reg32, (B, ScaledOffset32)> for Load<Reg32, (RegOrSp64, ScaledOffset32)>
where
    B: Into<RegOrSp64>,
{
    type Output = Self;

    #[inline]
    fn new(dst: Reg32, (base, offset): (B, ScaledOffset32)) -> Self {
        Self {
            dst,
            addr: (base.into(), offset),
        }
    }
}

impl Instruction for Load<Reg32, (RegOrSp64, ScaledOffset32)> {
    #[inline]
    fn represent(self) -> impl Iterator<Item = aarchmrs_types::InstructionCode> + 'static {
        let (base, offset) = self.addr;
        let code = LDR_32_ldst_pos(offset.into(), base.code(), self.dst.code());
        std::iter::once(code)
    }
}

impl<B> MakeLoad<Reg64, (B, ScaledOffset64)> for Load<Reg64, (RegOrSp64, ScaledOffset64)>
where
    B: Into<RegOrSp64>,
{
    type Output = Self;

    #[inline]
    fn new(dst: Reg64, (base, offset): (B, ScaledOffset64)) -> Self {
        Self {
            dst,
            addr: (base.into(), offset),
        }
    }
}

impl Instruction for Load<Reg64, (RegOrSp64, ScaledOffset64)> {
    #[inline]
    fn represent(self) -> impl Iterator<Item = aarchmrs_types::InstructionCode> + 'static {
        let (base, offset) = self.addr;
        let code = LDR_64_ldst_pos(offset.into(), base.code(), self.dst.code());
        std::iter::once(code)
    }
}

impl<B> MakeLoad<Reg32, (B, u32)> for Load<Reg32, (RegOrSp64, ScaledOffset32)>
where
    B: Into<RegOrSp64>,
{
    type Output = Result<Self, BitError>;

    #[inline]
    fn new(dst: Reg32, (base, offset): (B, u32)) -> Self::Output {
        let offset = ScaledOffset32::new(offset)?;
        Ok(Self {
            dst,
            addr: (base.into(), offset),
        })
    }
}

impl<B> MakeLoad<Reg64, (B, u32)> for Load<Reg64, (RegOrSp64, ScaledOffset64)>
where
    B: Into<RegOrSp64>,
{
    type Output = Result<Self, BitError>;

    #[inline]
    fn new(dst: Reg64, (base, offset): (B, u32)) -> Self::Output {
        let offset = ScaledOffset64::new(offset)?;
        Ok(Self {
            dst,
            addr: (base.into(), offset),
        })
    }
}

pub fn ldr<Dst, Addr, AddrReal>(
    dst: Dst,
    addr: Addr,
) -> <Load<Dst, AddrReal> as MakeLoad<Dst, Addr>>::Output
where
    Load<Dst, AddrReal>: MakeLoad<Dst, Addr>,
{
    Load::new(dst, addr)
}

#[cfg(test)]
mod tests {
    use crate::bits::UBitValue;

    use super::*;
    use Reg32::*;
    use Reg64::*;
    use RegOrSp64::SP;
    use RegOrZero64::XZR;
    use harm_test_utils::db;

    // 'ldr (w2|x2), [(x8|sp), (x3|xzr)]'
    const SIMPLE_LDR_REG_DB: &str = "
b8636902	ldr w2, [x8, x3]
b87f6902	ldr w2, [x8, xzr]
b8636be2	ldr w2, [sp, x3]
b87f6be2	ldr w2, [sp, xzr]
f8636902	ldr x2, [x8, x3]
f87f6902	ldr x2, [x8, xzr]
f8636be2	ldr x2, [sp, x3]
f87f6be2	ldr x2, [sp, xzr]
";

    const SIMPLE_LDR_REG_EXT_DB: &str = "
f8634902	ldr x2, [x8, w3, uxtw]
f8634902	ldr x2, [x8, w3, uxtw #0]
f8635902	ldr x2, [x8, w3, uxtw #3]
f863c902	ldr x2, [x8, w3, sxtw]
f863c902	ldr x2, [x8, w3, sxtw #0]
f863d902	ldr x2, [x8, w3, sxtw #3]
f863e902	ldr x2, [x8, x3, sxtx]
f863e902	ldr x2, [x8, x3, sxtx #0]
f863f902	ldr x2, [x8, x3, sxtx #3]
f8636902	ldr x2, [x8, x3]
f8636902	ldr x2, [x8, x3, lsl #0]
f8637902	ldr x2, [x8, x3, lsl #3]
b8634902	ldr w2, [x8, w3, uxtw]
b8634902	ldr w2, [x8, w3, uxtw #0]
b8635902	ldr w2, [x8, w3, uxtw #2]
b863c902	ldr w2, [x8, w3, sxtw]
b863c902	ldr w2, [x8, w3, sxtw #0]
b863d902	ldr w2, [x8, w3, sxtw #2]
b863e902	ldr w2, [x8, x3, sxtx]
b863e902	ldr w2, [x8, x3, sxtx #0]
b863f902	ldr w2, [x8, x3, sxtx #2]
b8636902	ldr w2, [x8, x3]
b8636902	ldr w2, [x8, x3, lsl #0]
b8637902	ldr w2, [x8, x3, lsl #2]";

    // 'ldr (w2|x2), [(x8|sp), #0x190]'
    const SIMPLE_LDR_SCALED_IMM: &str = "
b9419102	ldr w2, [x8, #0x190]
b94193e2	ldr w2, [sp, #0x190]
f940c902	ldr x2, [x8, #0x190]
f940cbe2	ldr x2, [sp, #0x190]
";

    #[test]
    fn test_ldr_r64_r64_r32_sxtw() {
        use LdrExtendOption32::*;
        let inst: Vec<_> = ldr(X2, (X8, extended(W3, SXTW))).represent().collect();
        assert_eq!(
            inst,
            vec![db(SIMPLE_LDR_REG_EXT_DB, "ldr x2, [x8, w3, sxtw]")]
        );
    }

    #[test]
    fn test_ldr_r64_r64_r32_uxtw() {
        use LdrExtendOption32::*;
        let inst: Vec<_> = ldr(X2, (X8, extended(W3, UXTW))).represent().collect();
        assert_eq!(
            inst,
            vec![db(SIMPLE_LDR_REG_EXT_DB, "ldr x2, [x8, w3, uxtw]")]
        );
    }

    #[test]
    fn test_ldr_r32_r64_r32_sxtw() {
        use LdrExtendOption32::*;
        let inst: Vec<_> = ldr(W2, (X8, extended(W3, SXTW))).represent().collect();
        assert_eq!(
            inst,
            vec![db(SIMPLE_LDR_REG_EXT_DB, "ldr x2, [x8, w3, sxtw]")]
        );
    }

    #[test]
    fn test_ldr_r32_r64_r32_uxtw() {
        use LdrExtendOption32::*;
        let inst: Vec<_> = ldr(W2, (X8, extended(W3, UXTW))).represent().collect();
        assert_eq!(
            inst,
            vec![db(SIMPLE_LDR_REG_EXT_DB, "ldr x2, [x8, w3, uxtw]")]
        );
    }

    #[test]
    fn test_ldr_r32_r64_r64() {
        let inst: Vec<_> = ldr(W2, (X8, X3)).represent().collect();
        assert_eq!(inst, vec![db(SIMPLE_LDR_REG_DB, "ldr w2, [x8, x3]")]);
    }

    #[test]
    fn test_ldr_r32_rsp_r64() {
        let inst: Vec<_> = ldr(W2, (SP, X3)).represent().collect();
        assert_eq!(inst, vec![db(SIMPLE_LDR_REG_DB, "ldr w2, [sp, x3]")]);
    }

    #[test]
    fn test_ldr_r64_r64_r64() {
        let inst: Vec<_> = ldr(X2, (X8, X3)).represent().collect();
        assert_eq!(inst, vec![db(SIMPLE_LDR_REG_DB, "ldr x2, [x8, x3]")]);
    }

    #[test]
    fn test_ldr_r64_rsp_r64() {
        let inst: Vec<_> = ldr(X2, (SP, X3)).represent().collect();
        assert_eq!(inst, vec![db(SIMPLE_LDR_REG_DB, "ldr x2, [sp, x3]")]);
    }

    #[test]
    fn test_ldr_r32_r64_xzr() {
        let inst: Vec<_> = ldr(W2, (X8, XZR)).represent().collect();
        assert_eq!(inst, vec![db(SIMPLE_LDR_REG_DB, "ldr w2, [x8, xzr]")]);
    }

    #[test]
    fn test_ldr_r32_rsp_xzr() {
        let inst: Vec<_> = ldr(W2, (SP, XZR)).represent().collect();
        assert_eq!(inst, vec![db(SIMPLE_LDR_REG_DB, "ldr w2, [sp, xzr]")]);
    }

    #[test]
    fn test_ldr_r64_r64_xzr() {
        let inst: Vec<_> = ldr(X2, (X8, XZR)).represent().collect();
        assert_eq!(inst, vec![db(SIMPLE_LDR_REG_DB, "ldr x2, [x8, xzr]")]);
    }

    #[test]
    fn test_ldr_r64_rsp_xzr() {
        let inst: Vec<_> = ldr(X2, (SP, XZR)).represent().collect();
        assert_eq!(inst, vec![db(SIMPLE_LDR_REG_DB, "ldr x2, [sp, xzr]")]);
    }

    #[test]
    fn test_ldr_r32_r64_scaled_imm() {
        let offset = UBitValue::<12, 2>::new(0x190).unwrap();
        let inst: Vec<_> = ldr(W2, (X8, offset)).represent().collect();
        assert_eq!(
            inst,
            vec![db(SIMPLE_LDR_SCALED_IMM, "ldr w2, [x8, #0x190]")]
        );
    }

    #[test]
    fn test_ldr_r32_sp_scaled_imm() {
        let offset = UBitValue::<12, 2>::new(0x190).unwrap();
        let inst: Vec<_> = ldr(W2, (SP, offset)).represent().collect();
        assert_eq!(
            inst,
            vec![db(SIMPLE_LDR_SCALED_IMM, "ldr w2, [sp, #0x190]")]
        );
    }

    #[test]
    fn test_ldr_r64_r64_scaled_imm() {
        let offset = UBitValue::<12, 3>::new(0x190).unwrap();
        let inst: Vec<_> = ldr(X2, (X8, offset)).represent().collect();
        assert_eq!(
            inst,
            vec![db(SIMPLE_LDR_SCALED_IMM, "ldr x2, [x8, #0x190]")]
        );
    }

    #[test]
    fn test_ldr_r64_sp_scaled_imm() {
        let offset = UBitValue::<12, 3>::new(0x190).unwrap();
        let inst: Vec<_> = ldr(X2, (SP, offset)).represent().collect();
        assert_eq!(
            inst,
            vec![db(SIMPLE_LDR_SCALED_IMM, "ldr x2, [sp, #0x190]")]
        );
    }
}
