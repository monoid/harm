/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use aarchmrs_instructions::A64::{
    dpimm::addsub_imm::ADD_64_addsub_imm::ADD_64_addsub_imm,
    dpreg::{
        addsub_ext::{ADD_32_addsub_ext::ADD_32_addsub_ext, ADD_64_addsub_ext::ADD_64_addsub_ext},
        addsub_shift::{
            ADD_32_addsub_shift::ADD_32_addsub_shift, ADD_64_addsub_shift::ADD_64_addsub_shift,
        },
    },
};
use aarchmrs_types::InstructionCode;

use super::Instruction;
use crate::register::{IntoCode as _, Reg32, Reg64, RegOrSp64, RegOrZero32, RegOrZero64};

pub type Error = &'static str;

pub fn add<T, U>(dst: T, src1: T, src2: U) -> Result<Add<T, U>, Error>
where
    Add<T, U>: MakeAdd<T, U>,
{
    Add::<T, U>::new(dst, src1, src2)
}

pub trait MakeAdd<T, U>: Sized {
    fn new(dst: T, src1: T, src2: U) -> Result<Self, Error>;
}

pub struct Add<T, U> {
    pub dst: T,
    pub src1: T,
    pub src2: U,
}

impl MakeAdd<Reg64, Reg64> for Add<Reg64, Reg64> {
    #[inline]
    fn new(dst: Reg64, src1: Reg64, src2: Reg64) -> Result<Self, &'static str> {
        Ok(Self { dst, src1, src2 })
    }
}

impl Add<Reg64, Reg64> {
    #[inline]
    pub fn shift(self, mode: ShiftMode, amount: u8) -> Add<RegOrZero64, ShiftedReg<RegOrZero64>> {
        add(
            RegOrZero64::Reg(self.dst),
            RegOrZero64::Reg(self.src1),
            ShiftedReg::new(RegOrZero64::Reg(self.src2)),
        )
        .expect("internal error: cannot happen")
        .shift(mode, amount)
    }

    #[inline]
    pub fn extend(
        self,
        mode: ExtendMode,
        amount: u8,
    ) -> Add<RegOrZero64, ExtendedReg<RegOrZero64>> {
        add(
            RegOrZero64::Reg(self.dst),
            RegOrZero64::Reg(self.src1),
            ExtendedReg::new(RegOrZero64::Reg(self.src2)),
        )
        .expect("internal error: cannot happen")
        .extend(mode, amount)
    }
}

impl Instruction for Add<Reg64, Reg64> {
    #[inline]
    fn represent(self) -> impl Iterator<Item = InstructionCode> {
        Add {
            dst: RegOrZero64::Reg(self.dst),
            src1: RegOrZero64::Reg(self.src1),
            src2: ShiftedReg::new(RegOrZero64::Reg(self.src2)),
        }
        .represent()
    }
}

impl MakeAdd<RegOrZero64, RegOrZero64> for Add<RegOrZero64, RegOrZero64> {
    #[inline]
    fn new(dst: RegOrZero64, src1: RegOrZero64, src2: RegOrZero64) -> Result<Self, &'static str> {
        Ok(Self { dst, src1, src2 })
    }
}

impl Add<RegOrZero64, RegOrZero64> {
    #[inline]
    pub fn shift(self, mode: ShiftMode, amount: u8) -> Add<RegOrZero64, ShiftedReg<RegOrZero64>> {
        add(self.dst, self.src1, ShiftedReg::new(self.src2))
            .expect("internal error: cannot happen")
            .shift(mode, amount)
    }

    #[inline]
    pub fn extend(
        self,
        mode: ExtendMode,
        amount: u8,
    ) -> Add<RegOrZero64, ExtendedReg<RegOrZero64>> {
        add(self.dst, self.src1, ExtendedReg::new(self.src2))
            .expect("internal error: cannot happen")
            .extend(mode, amount)
    }
}

impl MakeAdd<RegOrZero64, ShiftedReg<RegOrZero64>> for Add<RegOrZero64, ShiftedReg<RegOrZero64>> {
    #[inline]
    fn new(
        dst: RegOrZero64,
        src1: RegOrZero64,
        src2: ShiftedReg<RegOrZero64>,
    ) -> Result<Self, &'static str> {
        Ok(Self { dst, src1, src2 })
    }
}

impl Add<RegOrZero64, ShiftedReg<RegOrZero64>> {
    #[inline]
    pub fn shift(mut self, mode: ShiftMode, amount: u8) -> Self {
        self.src2.shift = Shift { mode, amount };
        self
    }

    #[inline]
    fn add_opcode(&self) -> InstructionCode {
        let shift = self.src2.shift.mode as u8;
        let rm = self.src2.reg.code();
        let shift_amount_imm6 = self.src2.shift.amount;
        let rn = self.src1.code();
        let rd = self.dst.code();

        ADD_64_addsub_shift::new(
            shift.into(),
            rm.into(),
            shift_amount_imm6.into(),
            rn.into(),
            rd.into(),
        )
        .build()
    }
}

impl Instruction for Add<RegOrZero64, ShiftedReg<RegOrZero64>> {
    #[inline]
    fn represent(self) -> impl Iterator<Item = InstructionCode> {
        let opcode = self.add_opcode();

        std::iter::once(opcode)
    }
}

impl MakeAdd<RegOrZero64, ExtendedReg<RegOrZero64>> for Add<RegOrZero64, ExtendedReg<RegOrZero64>> {
    #[inline]
    fn new(
        dst: RegOrZero64,
        src1: RegOrZero64,
        src2: ExtendedReg<RegOrZero64>,
    ) -> Result<Self, &'static str> {
        Ok(Self { dst, src1, src2 })
    }
}

impl Add<RegOrZero64, ExtendedReg<RegOrZero64>> {
    #[inline]
    pub fn extend(mut self, mode: ExtendMode, amount: u8) -> Self {
        self.src2.extend = Extend { mode, amount };
        self
    }

    #[inline]
    fn add_opcode(&self) -> InstructionCode {
        let option = self.src2.extend.mode as u8;
        let rm = self.src2.reg.code();
        let imm3 = self.src2.extend.amount;
        let rn = self.src1.code();
        let rd = self.dst.code();

        ADD_64_addsub_ext::new(rm.into(), option.into(), imm3.into(), rn.into(), rd.into()).build()
    }
}

impl Instruction for Add<RegOrZero64, ExtendedReg<RegOrZero64>> {
    #[inline]
    fn represent(self) -> impl Iterator<Item = InstructionCode> {
        let opcode = self.add_opcode();

        std::iter::once(opcode)
    }
}

impl MakeAdd<Reg64, u32> for Add<Reg64, u32> {
    #[inline]
    fn new(dst: Reg64, src1: Reg64, src2: u32) -> Result<Self, &'static str> {
        let imm12 = validate_imm12(src2)?;
        Ok(Self {
            dst,
            src1,
            src2: imm12,
        })
    }
}

impl Instruction for Add<Reg64, u32> {
    #[inline]
    fn represent(self) -> impl Iterator<Item = InstructionCode> {
        let dst = RegOrSp64::Reg(self.dst);
        let src1 = RegOrSp64::Reg(self.src1);
        let src2 = self.src2;
        Add::<RegOrSp64, u32> { dst, src1, src2 }.represent()
    }
}

impl MakeAdd<RegOrSp64, u32> for Add<RegOrSp64, u32> {
    #[inline]
    fn new(dst: RegOrSp64, src1: RegOrSp64, src2: u32) -> Result<Self, &'static str> {
        let imm12 = validate_imm12(src2)?;
        Ok(Self {
            dst,
            src1,
            src2: imm12,
        })
    }
}

impl Instruction for Add<RegOrSp64, u32> {
    #[inline]
    fn represent(self) -> impl Iterator<Item = InstructionCode> {
        let shift = self.src2 & ((1 << 12) - 1) == 0;
        let imm12 = if shift { self.src2 >> 12 } else { self.src2 };
        assert!(imm12 < (1 << 12));
        let rn = self.src1.code();
        let rd = self.dst.code();

        let opcode =
            ADD_64_addsub_imm::new(shift.into(), imm12.into(), rn.into(), rd.into()).build();

        std::iter::once(opcode)
    }
}

impl MakeAdd<Reg32, Reg32> for Add<Reg32, Reg32> {
    #[inline]
    fn new(dst: Reg32, src1: Reg32, src2: Reg32) -> Result<Self, Error> {
        Ok(Self { dst, src1, src2 })
    }
}

impl Add<Reg32, Reg32> {
    #[inline]
    pub fn shift(self, mode: ShiftMode, amount: u8) -> Add<RegOrZero32, ShiftedReg<RegOrZero32>> {
        let dst = RegOrZero32::Reg(self.dst);
        let src1 = RegOrZero32::Reg(self.src1);
        let src2 = ShiftedReg::new(RegOrZero32::Reg(self.src2));
        Add { dst, src1, src2 }.shift(mode, amount)
    }

    #[inline]
    pub fn extend(
        self,
        mode: ExtendMode,
        amount: u8,
    ) -> Add<RegOrZero32, ExtendedReg<RegOrZero32>> {
        add(
            RegOrZero32::Reg(self.dst),
            RegOrZero32::Reg(self.src1),
            ExtendedReg::new(RegOrZero32::Reg(self.src2)),
        )
        .expect("internal error: cannot happen")
        .extend(mode, amount)
    }
}

impl Instruction for Add<Reg32, Reg32> {
    #[inline]
    fn represent(self) -> impl Iterator<Item = InstructionCode> {
        let shift = 0;
        let rm = self.src2.code();
        let shift_amount_imm6 = 0;
        let rn = self.src1.code();
        let rd = self.dst.code();

        let opcode = ADD_32_addsub_shift::new(
            shift.into(),
            rm.into(),
            shift_amount_imm6.into(),
            rn.into(),
            rd.into(),
        )
        .build();

        std::iter::once(opcode)
    }
}

impl MakeAdd<RegOrZero32, RegOrZero32> for Add<RegOrZero32, RegOrZero32> {
    #[inline]
    fn new(dst: RegOrZero32, src1: RegOrZero32, src2: RegOrZero32) -> Result<Self, &'static str> {
        Ok(Self { dst, src1, src2 })
    }
}

impl Add<RegOrZero32, RegOrZero32> {
    #[inline]
    pub fn shift(self, mode: ShiftMode, amount: u8) -> Add<RegOrZero32, ShiftedReg<RegOrZero32>> {
        add(self.dst, self.src1, ShiftedReg::new(self.src2))
            .expect("internal error: cannot happen")
            .shift(mode, amount)
    }

    #[inline]
    pub fn extend(
        self,
        mode: ExtendMode,
        amount: u8,
    ) -> Add<RegOrZero32, ExtendedReg<RegOrZero32>> {
        add(self.dst, self.src1, ExtendedReg::new(self.src2))
            .expect("internal error: cannot happen")
            .extend(mode, amount)
    }
}

impl MakeAdd<RegOrZero32, ShiftedReg<RegOrZero32>> for Add<RegOrZero32, ShiftedReg<RegOrZero32>> {
    #[inline]
    fn new(
        dst: RegOrZero32,
        src1: RegOrZero32,
        src2: ShiftedReg<RegOrZero32>,
    ) -> Result<Self, Error> {
        Ok(Self { dst, src1, src2 })
    }
}

impl Add<RegOrZero32, ShiftedReg<RegOrZero32>> {
    #[inline]
    pub fn shift(mut self, mode: ShiftMode, amount: u8) -> Self {
        self.src2.shift = Shift { mode, amount };
        self
    }

    #[inline]
    fn add_opcode(&self) -> InstructionCode {
        let shift = self.src2.shift.mode as u8;
        let rm = self.src2.reg.code();
        let shift_amount_imm6 = self.src2.shift.amount;
        let rn = self.src1.code();
        let rd = self.dst.code();

        ADD_32_addsub_shift::new(
            shift.into(),
            rm.into(),
            shift_amount_imm6.into(),
            rn.into(),
            rd.into(),
        )
        .build()
    }
}

impl Instruction for Add<RegOrZero32, ShiftedReg<RegOrZero32>> {
    #[inline]
    fn represent(self) -> impl Iterator<Item = InstructionCode> {
        let opcode = self.add_opcode();

        std::iter::once(opcode)
    }
}

impl MakeAdd<RegOrZero32, ExtendedReg<RegOrZero32>> for Add<RegOrZero32, ExtendedReg<RegOrZero32>> {
    #[inline]
    fn new(
        dst: RegOrZero32,
        src1: RegOrZero32,
        src2: ExtendedReg<RegOrZero32>,
    ) -> Result<Self, &'static str> {
        Ok(Self { dst, src1, src2 })
    }
}

impl Add<RegOrZero32, ExtendedReg<RegOrZero32>> {
    #[inline]
    pub fn extend(mut self, mode: ExtendMode, amount: u8) -> Self {
        self.src2.extend = Extend { mode, amount };
        self
    }

    #[inline]
    fn add_opcode(&self) -> InstructionCode {
        let option = self.src2.extend.mode as u8;
        let rm = self.src2.reg.code();
        let imm3 = self.src2.extend.amount;
        let rn = self.src1.code();
        let rd = self.dst.code();

        ADD_32_addsub_ext::new(rm.into(), option.into(), imm3.into(), rn.into(), rd.into()).build()
    }
}

impl Instruction for Add<RegOrZero32, ExtendedReg<RegOrZero32>> {
    #[inline]
    fn represent(self) -> impl Iterator<Item = InstructionCode> {
        let opcode = self.add_opcode();

        std::iter::once(opcode)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ShiftedReg<T> {
    reg: T,
    shift: Shift,
}

impl<T> ShiftedReg<T> {
    pub fn new(reg: T) -> Self {
        Self {
            reg,
            shift: <_>::default(),
        }
    }

    pub fn shift(mut self, mode: ShiftMode, amount: u8) -> Self {
        self.shift = Shift { mode, amount };
        self
    }
}

impl From<Reg64> for ShiftedReg<RegOrZero64> {
    #[inline]
    fn from(value: Reg64) -> Self {
        Self::new(value.into())
    }
}

impl From<Reg32> for ShiftedReg<RegOrZero32> {
    #[inline]
    fn from(value: Reg32) -> Self {
        Self::new(value.into())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct Shift {
    mode: ShiftMode,
    amount: u8,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum ShiftMode {
    #[default]
    LSL = 0b00,
    LSR = 0b01,
    ASR = 0b10,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ExtendedReg<T> {
    reg: T,
    extend: Extend,
}

impl<T> ExtendedReg<T> {
    pub fn new(reg: T) -> Self {
        Self {
            reg,
            extend: <_>::default(),
        }
    }

    pub fn extend(mut self, mode: ExtendMode, amount: u8) -> Self {
        self.extend = Extend { mode, amount };
        self
    }
}

impl From<Reg64> for ExtendedReg<RegOrZero64> {
    fn from(value: Reg64) -> Self {
        Self::new(value.into())
    }
}

impl From<Reg32> for ExtendedReg<RegOrZero32> {
    fn from(value: Reg32) -> Self {
        Self::new(value.into())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum ExtendMode {
    UXTB = 0b000,
    UXTH = 0b001,
    UXTW = 0b010,
    #[default]
    UXTX = 0b011, // it is essentially a noop for any register, so it is the default
    SXTB = 0b100,
    SXTH = 0b101,
    SXTW = 0b110,
    SXTX = 0b111,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct Extend {
    mode: ExtendMode,
    amount: u8,
}

fn validate_imm12(imm12: u32) -> Result<u32, &'static str> {
    const BITS_12: u32 = (1 << 12) - 1;
    if imm12 <= BITS_12 {
        Ok(imm12)
    } else {
        let shift = imm12 & BITS_12 == 0;
        if shift {
            let imm12_shifted = imm12 >> 12;
            if imm12_shifted <= BITS_12 {
                return Ok(imm12);
            }
        }
        Err("Immediate value out of range for an arithmetic instruction")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_64() {
        use Reg64::*;

        let codes: Vec<_> = add(X1, X2, X12).unwrap().represent().collect();

        assert_eq!(codes.len(), 1);
        assert_eq!(codes[0].0, [0x41, 0x00, 0x0c, 0x8b]); // 0x8b0c0041
    }

    #[test]
    fn test_add_64_shift() {
        use Reg64::*;

        let codes: Vec<_> = add(X1, X2, X12)
            .unwrap()
            .shift(ShiftMode::LSR, 4)
            .represent()
            .collect();

        assert_eq!(codes.len(), 1);
        assert_eq!(codes[0].0, [0x41, 0x10, 0x4c, 0x8b]); // 0x8b4c1041
    }

    #[test]
    fn test_add_64_zero() {
        use Reg64::*;
        use RegOrZero64::*;

        let codes: Vec<_> = add(
            X1.into(),
            XZR,
            ShiftedReg::from(X12).shift(ShiftMode::LSR, 4),
        )
        .unwrap()
        .represent()
        .collect();

        assert_eq!(codes.len(), 1);
        assert_eq!(codes[0].0, [0xe1, 0x13, 0x4c, 0x8b]); // 0x8b4c13e1
    }

    #[test]
    fn test_add_64_extend_uxtx() {
        use Reg64::*;
        use RegOrZero64::*;

        let codes: Vec<_> = add(Reg(X1), Reg(X2), Reg(X12))
            .unwrap()
            .extend(ExtendMode::UXTX, 3)
            .represent()
            .collect();

        assert_eq!(codes.len(), 1);
        // add x1, x2, x12, uxtx #3
        assert_eq!(codes[0].0, [0x41, 0x6c, 0x2c, 0x8b]); // 0x8b2c6c41
    }

    #[test]
    fn test_add_64_extend_uxtw() {
        use Reg64::*;

        let codes: Vec<_> = add(X1, X2, X12)
            .unwrap()
            .extend(ExtendMode::UXTW, 3)
            .represent()
            .collect();

        assert_eq!(codes.len(), 1);
        // add x1, x2, w12, uxtw #3
        assert_eq!(codes[0].0, [0x41, 0x4c, 0x2c, 0x8b]); // 0x8b2c4c41
    }

    #[test]
    fn test_add_64_extend_uxth_xzr() {
        use Reg64::*;
        use RegOrZero64::*;

        let codes: Vec<_> = add(Reg(X1), Reg(X2), XZR)
            .unwrap()
            .extend(ExtendMode::UXTH, 3)
            .represent()
            .collect();

        assert_eq!(codes.len(), 1);
        // add x1, x2, wzr, uxtw #3
        assert_eq!(codes[0].0, [0x41, 0x2c, 0x3f, 0x8b]); // 0x8b3f2c41
    }

    #[test]
    fn test_add_64_const_1() {
        use Reg64::*;

        let codes: Vec<_> = add(X1, X2, 1).unwrap().represent().collect();

        assert_eq!(codes.len(), 1);
        assert_eq!(codes[0].0, [0x41, 0x04, 0x00, 0x91]); // 0x91000441
    }

    #[test]
    fn test_add_64_const_0x1000() {
        use Reg64::*;

        let codes: Vec<_> = add(X1, X2, 0x1000).unwrap().represent().collect();

        assert_eq!(codes.len(), 1);
        assert_eq!(codes[0].0, [0x41, 0x04, 0x40, 0x91]); // 0x91400441
    }

    #[test]
    fn test_add_64_const_0x1001() {
        use Reg64::*;

        let a = add(X1, X2, 0x1001);
        assert!(a.is_err());
    }

    #[test]
    fn test_add_32() {
        use Reg32::*;

        let codes: Vec<_> = add(W1, W2, W12).unwrap().represent().collect();

        assert_eq!(codes.len(), 1);
        assert_eq!(codes[0].0, [0x41, 0x00, 0x0c, 0x0b]); // 0x0b0c0041
    }

    #[test]
    fn test_add_32_shift() {
        use Reg32::*;

        let codes: Vec<_> = add(W1, W2, W12)
            .unwrap()
            .shift(ShiftMode::LSR, 4)
            .represent()
            .collect();

        assert_eq!(codes.len(), 1);
        assert_eq!(codes[0].0, [0x41, 0x10, 0x4c, 0x0b]); // 0x0b4c1041
    }

    #[test]
    fn test_add_32_zero() {
        use Reg32::*;
        use RegOrZero32::*;

        let codes: Vec<_> = add(
            W1.into(),
            WZR,
            ShiftedReg::from(W12).shift(ShiftMode::LSR, 4),
        )
        .unwrap()
        .represent()
        .collect();

        assert_eq!(codes.len(), 1);
        assert_eq!(codes[0].0, [0xe1, 0x13, 0x4c, 0x0b]); // 0x0b4c13e1
    }

    #[test]
    fn test_add_32_extend_uxtx() {
        use Reg32::*;

        let codes: Vec<_> = add(W1, W2, W12)
            .unwrap()
            .extend(ExtendMode::UXTX, 3)
            .represent()
            .collect();

        assert_eq!(codes.len(), 1);
        // add x1, x2, x12, uxtx #3
        assert_eq!(codes[0].0, [0x41, 0x6c, 0x2c, 0x0b]); // 0x0b2c6c41
    }

    #[test]
    fn test_add_32_extend_uxtw() {
        use Reg32::*;

        let codes: Vec<_> = add(W1, W2, W12)
            .unwrap()
            .extend(ExtendMode::UXTW, 3)
            .represent()
            .collect();

        assert_eq!(codes.len(), 1);
        // add w1, w2, w12, uxtw #3
        assert_eq!(codes[0].0, [0x41, 0x4c, 0x2c, 0x0b]); // 0x0b2c4c41
    }

    #[test]
    fn test_add_32_extend_uxtx_wzr() {
        use Reg32::*;
        use RegOrZero32::*;

        let codes: Vec<_> = add(Reg(W1), Reg(W2), WZR)
            .unwrap()
            .extend(ExtendMode::UXTX, 3)
            .represent()
            .collect();

        assert_eq!(codes.len(), 1);
        // add w1, w2, wzr, uxtx #3
        assert_eq!(codes[0].0, [0x41, 0x6c, 0x3f, 0x0b]); // 0x0b3f6c41
    }

    #[test]
    fn test_add_32_extend_uxtw_wzr() {
        use Reg32::*;
        use RegOrZero32::*;

        let codes: Vec<_> = add(Reg(W1), Reg(W2), WZR)
            .unwrap()
            .extend(ExtendMode::UXTW, 3)
            .represent()
            .collect();

        assert_eq!(codes.len(), 1);
        // add w1, w2, wzr, uxtw #3
        assert_eq!(codes[0].0, [0x41, 0x4c, 0x3f, 0x0b]); // 0x0b3f4c41
    }
}
