use aarchmrs_instructions::A64::dpreg::addsub_shift::ADD_64_addsub_shift::ADD_64_addsub_shift;
use aarchmrs_types::InstructionCode;

use super::Instruction;
use crate::register::{GeneralRegister64, IntoCode, RegistersAndZero64};

pub fn add<T1, T, U1, U>(dst: T, src1: T, src2: U) -> Add<T1, U1>
where
    T: Into<T1>,
    U: Into<U1>,
{
    Add {
        dst: dst.into(),
        src1: src1.into(),
        src2: src2.into(),
    }
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

        ADD_64_addsub_shift(shift, rm, shift_amount_imm6, rn, rd)
    }
}

impl Instruction for Add<RegistersAndZero64, ShiftedReg<RegistersAndZero64>> {
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

impl Into<ShiftedReg<RegistersAndZero64>> for GeneralRegister64 {
    fn into(self) -> ShiftedReg<RegistersAndZero64> {
        ShiftedReg {
            reg: self.into(),
            shift: <_>::default(),
        }
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
    use crate::register::GeneralRegister64;

    use super::*;

    #[test]
    fn test_add() {
        use GeneralRegister64::*;

        let codes: Vec<_> = add(X1, X2, X12)
            .shift(ShiftMode::LSR, 4)
            .reprsent()
            .collect();

        assert_eq!(codes.len(), 1);
        assert_eq!(codes[0].0, [0x41, 0x10, 0x4c, 0x8b]); // 0x8b4c1041
    }
}
