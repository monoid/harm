/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use aarchmrs_instructions::A64::dpreg::addsub_shift::{
    ADD_32_addsub_shift::ADD_32_addsub_shift, ADD_64_addsub_shift::ADD_64_addsub_shift,
};
use aarchmrs_types::InstructionCode;

use super::Instruction;
use crate::register::{
    GeneralRegister32, GeneralRegister64, IntoCode, RegistersAndZero32, RegistersAndZero64,
};

pub fn add<T, U>(dst: T, src1: T, src2: U) -> Add<T, U> {
    Add { dst, src1, src2 }
}

pub struct Add<T, U> {
    pub dst: T,
    pub src1: T,
    pub src2: U,
}

impl Add<RegistersAndZero64, ShiftedReg<RegistersAndZero64>> {
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

        ADD_64_addsub_shift::new(shift, rm, shift_amount_imm6, rn, rd).build()
    }
}

impl Instruction for Add<RegistersAndZero64, ShiftedReg<RegistersAndZero64>> {
    #[inline]
    fn reprsent(&self) -> impl Iterator<Item = InstructionCode> {
        let opcode = self.add_opcode();

        std::iter::once(opcode)
    }
}

impl Add<RegistersAndZero32, ShiftedReg<RegistersAndZero32>> {
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

        ADD_32_addsub_shift::new(shift, rm, shift_amount_imm6, rn, rd).build()
    }
}

impl Instruction for Add<RegistersAndZero32, ShiftedReg<RegistersAndZero32>> {
    #[inline]
    fn reprsent(&self) -> impl Iterator<Item = InstructionCode> {
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

impl From<GeneralRegister64> for ShiftedReg<RegistersAndZero64> {
    fn from(value: GeneralRegister64) -> Self {
        Self::new(value.into())
    }
}

impl From<GeneralRegister32> for ShiftedReg<RegistersAndZero32> {
    fn from(value: GeneralRegister32) -> Self {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_64() {
        use GeneralRegister64::*;

        let codes: Vec<_> = add(X1.into(), X2.into(), ShiftedReg::from(X12))
            .shift(ShiftMode::LSR, 4)
            .reprsent()
            .collect();

        assert_eq!(codes.len(), 1);
        assert_eq!(codes[0].0, [0x41, 0x10, 0x4c, 0x8b]); // 0x8b4c1041
    }

    #[test]
    fn test_add_64_zero() {
        use GeneralRegister64::*;
        use RegistersAndZero64::*;

        let codes: Vec<_> = add(
            X1.into(),
            XZR,
            ShiftedReg::from(X12).shift(ShiftMode::LSR, 4),
        )
        .reprsent()
        .collect();

        assert_eq!(codes.len(), 1);
        assert_eq!(codes[0].0, [0xe1, 0x13, 0x4c, 0x8b]); // 0x8b4c13e1
    }

    #[test]
    fn test_add_32() {
        use GeneralRegister32::*;

        let codes: Vec<_> = add(W1.into(), W2.into(), ShiftedReg::from(W12))
            .shift(ShiftMode::LSR, 4)
            .reprsent()
            .collect();

        assert_eq!(codes.len(), 1);
        assert_eq!(codes[0].0, [0x41, 0x10, 0x4c, 0x0b]); // 0x0b4c1041
    }

    #[test]
    fn test_add_32_zero() {
        use GeneralRegister32::*;
        use RegistersAndZero32::*;

        let codes: Vec<_> = add(
            W1.into(),
            WZR,
            ShiftedReg::from(W12).shift(ShiftMode::LSR, 4),
        )
        .reprsent()
        .collect();

        assert_eq!(codes.len(), 1);
        assert_eq!(codes[0].0, [0xe1, 0x13, 0x4c, 0x0b]); // 0x0b4c13e1
    }
}
