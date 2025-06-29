/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use crate::register::{Reg32, Reg64, RegOrZero32, RegOrZero64};

macro_rules! define_arith_imm12 {
    ($name:ident, $bits:expr, $cmd:ident, $reg:ty, $etype:ty) => {
        paste! {
            impl [<Make $name>]<$reg, u32> for $name<$reg, u32> {
                #[inline]
                fn new(dst: $reg, src1: $reg, src2: u32) -> Result<Self, &'static str> {
                    let imm12 = $crate::instructions::arith::validate_imm12(src2)?;
                    Ok(Self {
                        dst,
                        src1,
                        src2: imm12,
                    })
                }
            }

            impl Instruction for $name<$reg, u32> {
                #[inline]
                fn represent(self) -> impl Iterator<Item = InstructionCode> {
                    let dst = $etype::Reg(self.dst);
                    let src1 = $etype::Reg(self.src1);
                    let src2 = self.src2;
                    $name::<$etype, u32> { dst, src1, src2 }.represent()
                }
            }

            impl [<Make $name>]<$etype, u32> for $name<$etype, u32> {
                #[inline]
                fn new(dst: $etype, src1: $etype, src2: u32) -> Result<Self, &'static str> {
                    let imm12 = $crate::instructions::arith::validate_imm12(src2)?;
                    Ok(Self {
                        dst,
                        src1,
                        src2: imm12,
                    })
                }
            }

            impl Instruction for $name<$etype, u32> {
                #[inline]
                fn represent(self) -> impl Iterator<Item = InstructionCode> {
                    let shift = self.src2 & ((1 << 12) - 1) == 0;
                    let imm12 = if shift { self.src2 >> 12 } else { self.src2 };
                    assert!(imm12 < (1 << 12));
                    let rn = self.src1.code();
                    let rd = self.dst.code();

                    let opcode =
                        [<$name:upper _ $bits _ $cmd _imm>]::new(shift.into(), imm12.into(), rn.into(), rd.into()).build();

                    std::iter::once(opcode)
                }
            }
        }
    }
}

pub mod add;

pub type Error = &'static str;

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

pub(crate) fn validate_imm12(imm12: u32) -> Result<u32, &'static str> {
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
