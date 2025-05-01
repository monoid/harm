use crate::register::{IntoCode, RegistersAndZero64};
use aarchmrs_types::InstructionCode;

use super::Instruction;

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

    fn add_opcode(&self, sf: bool) -> InstructionCode {
        let sf = (sf as u32) << 31;
        const REG_ADD_PREFIX: u32 = 0b0001011;
        let shift = self.src2.shift.mode as u32;
        let rm = self.src2.reg.code() as u32;
        let shift_amount_imm6 = self.src2.shift.amount as u32;
        let rn = self.src1.code() as u32;
        let rd = self.dst.code() as u32;

        InstructionCode::from_u32(
            sf | (REG_ADD_PREFIX << 24)
                | (shift << 22)
                | (rm << 16)
                | (shift_amount_imm6 << 10)
                | (rn << 5)
                | (rd << 0),
        )
    }
}

impl Instruction for Add<RegistersAndZero64, ShiftedReg<RegistersAndZero64>> {
    #[inline]
    fn reprsent(&self) -> impl Iterator<Item = InstructionCode> {
        let sf = true;
        let opcode = self.add_opcode(sf);

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
pub enum ShiftMode {
    #[default]
    LSL = 0b00,
    LSR = 0b01,
    ASR = 0b10,
}
