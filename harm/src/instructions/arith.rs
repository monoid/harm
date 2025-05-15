use aarchmrs_instructions::A64::dpreg::addsub_shift::ADD_64_addsub_shift::ADD_64_addsub_shift;
use aarchmrs_types::InstructionCode;

use super::Instruction;
use crate::register::{IntoCode, RegistersAndZero64};

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
