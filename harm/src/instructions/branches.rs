use crate::register::{GeneralRegister64, IntoCode as _, RegistersAndZero64};
use aarchmrs_types::InstructionCode;

use super::{BranchCond, Instruction};

#[inline]
pub fn b(offset: PcOffset) -> Branch<PcDst> {
    Branch::new(PcDst(offset))
}

#[inline]
pub fn b_cond(cond: BranchCond, offset: PcOffset) -> Branch<PcDst> {
    Branch::new(PcDst(offset)).when(cond)
}

#[derive(Debug, Copy, Clone)]
pub struct Branch<Dst> {
    pub dst: Dst,
    pub condition: Option<BranchCond>,
}

pub type PcOffset = i32;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct PcDst(pub PcOffset);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct RegDst(pub GeneralRegister64);

impl<T> Branch<T> {
    #[inline]
    pub fn new(dst: T) -> Self {
        Self {
            dst,
            condition: None,
        }
    }
}
impl Branch<PcDst> {
    #[inline]
    pub fn when(mut self, cond: BranchCond) -> Self {
        self.condition = Some(cond);
        self
    }
}

impl Instruction for Branch<PcDst> {
    fn reprsent(&self) -> impl Iterator<Item = InstructionCode> {
        // TODO first offset is 19 bits, the second is 29 bits!
        // what to do with the possible overflow?
        let code = match self.condition {
            Some(cond) => branch_cond(self.dst.0, cond),
            None => branch_nocond(self.dst.0),
        };
        std::iter::once(code)
    }
}

fn branch_cond(offset: PcOffset, cond: BranchCond) -> InstructionCode {
    const COND_BRANCH_PREFIX: u32 = 0b01010100 << 24;
    // TODO validate alignment and size
    let imm19 = (offset as u32 / 4) & ((1 << 20) - 1);
    let cond = cond as u32;

    InstructionCode::from_u32(COND_BRANCH_PREFIX | (imm19 << 5) | (cond << 0))
}

fn branch_nocond(offset: PcOffset) -> InstructionCode {
    const SIMPLE_BRANCH_PREFIX: u32 = 0b000101 << 26;
    // TODO validate alignment and size
    let imm26 = (offset as u32 / 4) & ((1 << 27) - 1);
    InstructionCode::from_u32(SIMPLE_BRANCH_PREFIX | (imm26 << 0))
}

// TODO return from a register
#[inline]
pub fn ret() -> Ret {
    Ret
}

#[derive(Debug, Copy, Clone)]
pub struct Ret;

impl Instruction for Ret {
    fn reprsent(&self) -> impl Iterator<Item = InstructionCode> {
        let reg = RegistersAndZero64::General(GeneralRegister64::LR);
        let reg_code = reg.code() as u32;
        let code = InstructionCode::from_u32((0b110101100101111100000 << 9) | (reg_code << 5));
        std::iter::once(code)
    }
}
