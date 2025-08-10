use aarchmrs_instructions::A64::{
    dpimm::addsub_imm::{
        SUB_32_addsub_imm::SUB_32_addsub_imm, SUB_64_addsub_imm::SUB_64_addsub_imm,
    },
    dpreg::addsub_ext::{
        SUB_32_addsub_ext::SUB_32_addsub_ext, SUB_64_addsub_ext::SUB_64_addsub_ext,
    },
    dpreg::addsub_shift::{
        SUB_32_addsub_shift::SUB_32_addsub_shift, SUB_64_addsub_shift::SUB_64_addsub_shift,
    },
};
use aarchmrs_types::InstructionCode;

use super::Error;
use crate::{
    instructions::{
        Instruction,
        arith::{Extend, ExtendMode, ExtendedReg, Shift, ShiftMode, ShiftedReg},
    },
    register::{IntoCode as _, Reg32, Reg64, RegOrSp32, RegOrSp64, RegOrZero32, RegOrZero64},
};

pub fn sub<T, U>(dst: T, src1: T, src2: U) -> Result<Sub<T, U>, Error>
where
    Sub<T, U>: MakeSub<T, U>,
{
    Sub::<T, U>::new(dst, src1, src2)
}

pub trait MakeSub<T, U>: Sized {
    fn new(dst: T, src1: T, src2: U) -> Result<Self, Error>;
}

pub struct Sub<T, U> {
    pub dst: T,
    pub src1: T,
    pub src2: U,
}

impl MakeSub<Reg64, Reg64> for Sub<Reg64, Reg64> {
    #[inline]
    fn new(dst: Reg64, src1: Reg64, src2: Reg64) -> Result<Self, Error> {
        Ok(Self { dst, src1, src2 })
    }
}

impl MakeSub<Reg32, Reg32> for Sub<Reg32, Reg32> {
    #[inline]
    fn new(dst: Reg32, src1: Reg32, src2: Reg32) -> Result<Self, Error> {
        Ok(Self { dst, src1, src2 })
    }
}

define_arith_shift!(Sub, 32, addsub, Reg32, RegOrZero32);
define_arith_shift!(Sub, 64, addsub, Reg64, RegOrZero64);

define_arith_extend!(Sub, 32, addsub, Reg32, RegOrSp32, RegOrZero32);
define_arith_extend!(Sub, 64, addsub, Reg64, RegOrSp64, RegOrZero64);

define_arith_imm12!(Sub, 32, addsub, Reg32, RegOrSp32);
define_arith_imm12!(Sub, 64, addsub, Reg64, RegOrSp64);

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;
    use alloc::vec::Vec;

    #[test]
    fn test_sub_sp_64_const_0x823() {
        use RegOrSp64::SP;

        let codes: Vec<_> = sub(SP, SP, 0x823).unwrap().represent().collect();

        assert_eq!(codes.len(), 1);
        // sub sp, sp, 0x123
        assert_eq!(codes[0].0, [0xff, 0x8f, 0x20, 0xd1]); // 0xd1208fff
    }

    #[test]
    fn test_sub_32_const_0x823() {
        use Reg32::*;

        let codes: Vec<_> = sub(W1, W2, 0x823).unwrap().represent().collect();

        assert_eq!(codes.len(), 1);
        // sub w1, w2, 0x123
        assert_eq!(codes[0].0, [0x41, 0x8c, 0x20, 0x51]); // 0x51208c41
    }

    #[test]
    fn test_sub_sp_32_const_0x823() {
        use RegOrSp32::WSP;

        let codes: Vec<_> = sub(WSP, WSP, 0x823).unwrap().represent().collect();

        assert_eq!(codes.len(), 1);
        // sub wsp, wsp, 0x123
        assert_eq!(codes[0].0, [0xff, 0x8f, 0x20, 0x51]); // 0x51208fff
    }

    #[test]
    fn test_sub_32_const_0x823000() {
        use Reg32::*;

        let codes: Vec<_> = sub(W1, W2, 0x823000).unwrap().represent().collect();

        assert_eq!(codes.len(), 1);
        // sub w1, w2, #0x123, lsl #12 ; =0x123000
        assert_eq!(codes[0].0, [0x41, 0x8c, 0x60, 0x51]); // 0x51608c41
    }

    #[test]
    fn test_sub_32_const_0x1001() {
        use Reg32::*;

        let a = sub(W1, W2, 0x1001);
        assert!(a.is_err());
    }
}
