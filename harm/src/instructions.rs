/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use aarchmrs_types::InstructionCode;

pub mod arith;
pub mod branches;
pub mod load;

// It should be &self, but rustc bug makes the impl infected by its lifetime:
// https://github.com/rust-lang/rust/issues/42940
// When the bug is fixed, it can be changed to &self.
// // OTOH, currently all implementations are `Copy`.
pub trait Instruction: Sized {
    fn represent(self) -> impl Iterator<Item = InstructionCode> + 'static;

    #[inline]
    fn bytes(self) -> impl Iterator<Item = u8> + 'static {
        self.represent().flat_map(|i| i.0)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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
