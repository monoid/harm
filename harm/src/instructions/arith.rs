/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use crate::register::{Reg32, Reg64, RegOrZero32, RegOrZero64};

macro_rules! define_arith_shift {
    ($name:ident, $bits:expr, $cmd:ident, $reg:ty, $ztype:ty) => {
        paste! {
            impl $name<$reg, $reg> {
                #[inline]
                pub fn shift(self, mode: ShiftMode, amount: u8) -> $name<$ztype, ShiftedReg<$ztype>> {
                    $name::new(
                        $ztype::Reg(self.dst),
                        $ztype::Reg(self.src1),
                        ShiftedReg::new($ztype::Reg(self.src2)),
                    )
                        .expect("internal error: cannot happen")
                        .shift(mode, amount)
                }
            }

            impl Instruction for $name<$reg, $reg> {
                #[inline]
                fn represent(self) -> impl Iterator<Item = InstructionCode> {
                    $name {
                        dst: $ztype::Reg(self.dst),
                        src1: $ztype::Reg(self.src1),
                        src2: ShiftedReg::new($ztype::Reg(self.src2)),
                    }
                    .represent()
                }
            }

            impl [<Make $name>]<$ztype, $ztype> for $name<$ztype, $ztype> {
                #[inline]
                fn new(dst: $ztype, src1: $ztype, src2: $ztype) -> Result<Self, &'static str> {
                    Ok(Self { dst, src1, src2 })
                }
            }

            impl $name<$ztype, $ztype> {
                #[inline]
                pub fn shift(self, mode: ShiftMode, amount: u8) -> $name<$ztype, ShiftedReg<$ztype>> {
                    $name::new(self.dst, self.src1, ShiftedReg::new(self.src2))
                        .expect("internal error: cannot happen")
                        .shift(mode, amount)
                }
            }

            impl [<Make $name>]<$ztype, ShiftedReg<$ztype>> for $name<$ztype, ShiftedReg<$ztype>> {
                #[inline]
                fn new(
                    dst: $ztype,
                    src1: $ztype,
                    src2: ShiftedReg<$ztype>,
                ) -> Result<Self, &'static str> {
                    Ok(Self { dst, src1, src2 })
                }
            }

            impl $name<$ztype, ShiftedReg<$ztype>> {
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

                    [<$name:upper _ $bits _ $cmd _shift>]::new(
                        shift.into(),
                        rm.into(),
                        shift_amount_imm6.into(),
                        rn.into(),
                        rd.into(),
                    )
                        .build()
                }
            }

            impl Instruction for $name<$ztype, ShiftedReg<$ztype>> {
                #[inline]
                fn represent(self) -> impl Iterator<Item = InstructionCode> {
                    let opcode = self.add_opcode();

                    std::iter::once(opcode)
                }
            }
        }
    }
}
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

macro_rules! define_arith_extend {
    ($name:ident, $bits:expr, $cmd:ident, $reg:ty, $stype:ty, $ztype:ty) => {
        paste! {
            impl $name<$reg, $reg> {
                #[inline]
                pub fn extend(
                    self,
                    mode: ExtendMode,
                    amount: u8,
                ) -> $name<$stype, ExtendedReg<$ztype>> {
                    $name::new(
                        <$stype>::Reg(self.dst),
                        <$stype>::Reg(self.src1),
                        ExtendedReg::new(<$ztype>::Reg(self.src2)),
                    )
                    .expect("internal error: cannot happen")
                    .extend(mode, amount)
                }
            }

            impl [<Make $name>]<$stype, $ztype> for $name<$stype, $ztype> {
                #[inline]
                fn new(dst: $stype, src1: $stype, src2: $ztype) -> Result<Self, &'static str> {
                    Ok(Self { dst, src1, src2 })
                }
            }

            impl [<Make $name>]<$stype, $reg> for $name<$stype, $reg> {
                #[inline]
                fn new(dst: $stype, src1: $stype, src2: $reg) -> Result<Self, &'static str> {
                    Ok(Self { dst, src1, src2 })
                }
            }

            impl $name<$stype, $reg> {
                #[inline]
                pub fn extend(
                    self,
                    mode: ExtendMode,
                    amount: u8,
                ) -> $name<$stype, ExtendedReg<$ztype>> {
                    $name::new(self.dst, self.src1, ExtendedReg::new(self.src2.into()))
                        .expect("internal error: cannot happen")
                        .extend(mode, amount)
                }
            }

            impl [<Make $name>]<$stype, ExtendedReg<$ztype>>
                for $name<$stype, ExtendedReg<$ztype>>
            {
                #[inline]
                fn new(
                    dst: $stype,
                    src1: $stype,
                    src2: ExtendedReg<$ztype>,
                ) -> Result<Self, &'static str> {
                    Ok(Self { dst, src1, src2 })
                }
            }

            impl $name<$stype, ExtendedReg<$ztype>> {
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

                    [<$name:upper _ $bits _ $cmd _ext>]::new(
                        rm.into(), option.into(), imm3.into(), rn.into(), rd.into()
                    ).build()
                }
            }

            impl $name<$stype, $ztype> {
                #[inline]
                pub fn extend(self, mode: ExtendMode, amount: u8) -> $name<$stype, ExtendedReg<$ztype>> {
                    $name::new(self.dst, self.src1, ExtendedReg::new(self.src2))
                        .expect("internal error: cannot happen")
                        .extend(mode, amount)
                }
            }

            impl Instruction for $name<$stype, ExtendedReg<$ztype>> {
                #[inline]
                fn represent(self) -> impl Iterator<Item = InstructionCode> {
                    let opcode = self.add_opcode();

                    std::iter::once(opcode)
                }
            }
        }
    };
}

pub mod add;
pub mod sub;

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
