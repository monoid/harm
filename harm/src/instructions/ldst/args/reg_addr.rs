use crate::{
    register::{IntoReg, RegOrSp64, RegOrZero32, RegOrZero64},
    sealed::Sealed,
};

#[derive(Debug, Clone, Copy)]
pub struct RegAddr<Reg> {
    pub reg: Reg,
    pub addr: RegOrSp64,
}

impl Sealed for RegAddr<RegOrZero32> {}
impl Sealed for RegAddr<RegOrZero64> {}

pub trait MakeRegAddr<RegInp, AddrRegInp>: Sealed {
    fn new(reg: RegInp, addr: AddrRegInp) -> Self;
}

impl<Reg: IntoReg<RegOrZero32>, Addr: IntoReg<RegOrSp64>> MakeRegAddr<Reg, Addr>
    for RegAddr<RegOrZero32>
{
    fn new(reg: Reg, addr: Addr) -> Self {
        Self {
            reg: reg.into_reg(),
            addr: addr.into_reg(),
        }
    }
}

impl<Reg: IntoReg<RegOrZero64>, Addr: IntoReg<RegOrSp64>> MakeRegAddr<Reg, Addr>
    for RegAddr<RegOrZero64>
{
    fn new(reg: Reg, addr: Addr) -> Self {
        Self {
            reg: reg.into_reg(),
            addr: addr.into_reg(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Reg32Addr {
    pub reg: RegOrZero32,
    pub addr: RegOrSp64,
}

impl Sealed for Reg32Addr {}

pub trait MakeReg32Addr<Reg32Inp, AddrRegInp>: Sealed {
    fn new(reg: Reg32Inp, addr: AddrRegInp) -> Self;
}

impl<Reg: IntoReg<RegOrZero32>, Addr: IntoReg<RegOrSp64>> MakeReg32Addr<Reg, Addr> for Reg32Addr {
    fn new(reg: Reg, addr: Addr) -> Self {
        Self {
            reg: reg.into_reg(),
            addr: addr.into_reg(),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct Reg64Addr {
    pub reg: RegOrZero64,
    pub addr: RegOrSp64,
}

impl Sealed for Reg64Addr {}

#[allow(dead_code)]
pub trait MakeReg64Addr<Reg64Inp, AddrRegInp>: Sealed {
    fn new(reg: Reg64Inp, addr: AddrRegInp) -> Self;
}

impl<Reg: IntoReg<RegOrZero64>, Addr: IntoReg<RegOrSp64>> MakeReg64Addr<Reg, Addr> for Reg64Addr {
    fn new(reg: Reg, addr: Addr) -> Self {
        Self {
            reg: reg.into_reg(),
            addr: addr.into_reg(),
        }
    }
}
