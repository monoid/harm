use crate::{
    register::{IntoReg, RegOrSp64, RegOrZero32, RegOrZero64},
    sealed::Sealed,
};

pub struct ExclusiveStoreArgs<Reg> {
    pub status: RegOrZero32,
    pub reg: Reg,
    pub addr: RegOrSp64,
}

impl Sealed for ExclusiveStoreArgs<RegOrZero32> {}
impl Sealed for ExclusiveStoreArgs<RegOrZero64> {}

pub trait MakeExclusiveStoreArgs<StatusInp, RegInp, AddrRegInp>: Sealed {
    fn new(status: StatusInp, reg: RegInp, addr: AddrRegInp) -> Self;
}

impl<StatusInp, RegInp, AddrRegInp> MakeExclusiveStoreArgs<StatusInp, RegInp, AddrRegInp>
    for ExclusiveStoreArgs<RegOrZero32>
where
    StatusInp: IntoReg<RegOrZero32>,
    RegInp: IntoReg<RegOrZero32>,
    AddrRegInp: IntoReg<RegOrSp64>,
{
    fn new(status: StatusInp, reg: RegInp, addr: AddrRegInp) -> Self {
        Self {
            status: status.into_reg(),
            reg: reg.into_reg(),
            addr: addr.into_reg(),
        }
    }
}

impl<StatusInp, RegInp, AddrRegInp> MakeExclusiveStoreArgs<StatusInp, RegInp, AddrRegInp>
    for ExclusiveStoreArgs<RegOrZero64>
where
    StatusInp: IntoReg<RegOrZero32>,
    RegInp: IntoReg<RegOrZero64>,
    AddrRegInp: IntoReg<RegOrSp64>,
{
    fn new(status: StatusInp, reg: RegInp, addr: AddrRegInp) -> Self {
        Self {
            status: status.into_reg(),
            reg: reg.into_reg(),
            addr: addr.into_reg(),
        }
    }
}
pub struct ExclusiveStore32Args {
    pub status: RegOrZero32,
    pub reg: RegOrZero32,
    pub addr: RegOrSp64,
}

impl Sealed for ExclusiveStore32Args {}

pub trait MakeExclusiveStore32Args<StatusInp, RegInp, AddrRegInp>: Sealed {
    fn new(status: StatusInp, reg: RegInp, addr: AddrRegInp) -> Self;
}

impl<StatusInp, RegInp, AddrRegInp> MakeExclusiveStore32Args<StatusInp, RegInp, AddrRegInp>
    for ExclusiveStore32Args
where
    StatusInp: IntoReg<RegOrZero32>,
    RegInp: IntoReg<RegOrZero32>,
    AddrRegInp: IntoReg<RegOrSp64>,
{
    fn new(status: StatusInp, reg: RegInp, addr: AddrRegInp) -> Self {
        Self {
            status: status.into_reg(),
            reg: reg.into_reg(),
            addr: addr.into_reg(),
        }
    }
}

pub struct ExclusiveStore64Args {
    pub status: RegOrZero32,
    pub reg: RegOrZero64,
    pub addr: RegOrSp64,
}

impl Sealed for ExclusiveStore64Args {}

pub trait MakeExclusiveStore64Args<StatusInp, RegInp, AddrRegInp>: Sealed {
    fn new(status: StatusInp, reg: RegInp, addr: AddrRegInp) -> Self;
}

impl<StatusInp, RegInp, AddrRegInp> MakeExclusiveStore64Args<StatusInp, RegInp, AddrRegInp>
    for ExclusiveStore64Args
where
    StatusInp: IntoReg<RegOrZero32>,
    RegInp: IntoReg<RegOrZero64>,
    AddrRegInp: IntoReg<RegOrSp64>,
{
    fn new(status: StatusInp, reg: RegInp, addr: AddrRegInp) -> Self {
        Self {
            status: status.into_reg(),
            reg: reg.into_reg(),
            addr: addr.into_reg(),
        }
    }
}

pub struct ExclusivePairLoadArgs<Reg> {
    pub reg1: Reg,
    pub reg2: Reg,
    pub addr: RegOrSp64,
}

impl Sealed for ExclusivePairLoadArgs<RegOrZero32> {}
impl Sealed for ExclusivePairLoadArgs<RegOrZero64> {}

pub trait MakeExclusivePairLoadArgs<Reg1Inp, Reg2Inp, AddrRegInp>: Sealed {
    fn new(reg1: Reg1Inp, reg2: Reg2Inp, addr: AddrRegInp) -> Self;
}

impl<Reg1Inp, Reg2Inp, AddrRegInp> MakeExclusivePairLoadArgs<Reg1Inp, Reg2Inp, AddrRegInp>
    for ExclusivePairLoadArgs<RegOrZero32>
where
    Reg1Inp: IntoReg<RegOrZero32>,
    Reg2Inp: IntoReg<RegOrZero32>,
    AddrRegInp: IntoReg<RegOrSp64>,
{
    fn new(reg1: Reg1Inp, reg2: Reg2Inp, addr: AddrRegInp) -> Self {
        Self {
            reg1: reg1.into_reg(),
            reg2: reg2.into_reg(),
            addr: addr.into_reg(),
        }
    }
}

impl<Reg1Inp, Reg2Inp, AddrRegInp> MakeExclusivePairLoadArgs<Reg1Inp, Reg2Inp, AddrRegInp>
    for ExclusivePairLoadArgs<RegOrZero64>
where
    Reg1Inp: IntoReg<RegOrZero64>,
    Reg2Inp: IntoReg<RegOrZero64>,
    AddrRegInp: IntoReg<RegOrSp64>,
{
    fn new(reg1: Reg1Inp, reg2: Reg2Inp, addr: AddrRegInp) -> Self {
        Self {
            reg1: reg1.into_reg(),
            reg2: reg2.into_reg(),
            addr: addr.into_reg(),
        }
    }
}

pub struct ExclusivePairStoreArgs<Reg> {
    pub status: RegOrZero32,
    pub reg1: Reg,
    pub reg2: Reg,
    pub addr: RegOrSp64,
}

impl Sealed for ExclusivePairStoreArgs<RegOrZero32> {}
impl Sealed for ExclusivePairStoreArgs<RegOrZero64> {}

pub trait MakeExclusivePairStoreArgs<StatusInp, Reg1Inp, Reg2Inp, AddrRegInp>: Sealed {
    fn new(status: StatusInp, reg1: Reg1Inp, reg2: Reg2Inp, addr: AddrRegInp) -> Self;
}

impl<StatusInp, Reg1Inp, Reg2Inp, AddrRegInp>
    MakeExclusivePairStoreArgs<StatusInp, Reg1Inp, Reg2Inp, AddrRegInp>
    for ExclusivePairStoreArgs<RegOrZero32>
where
    StatusInp: IntoReg<RegOrZero32>,
    Reg1Inp: IntoReg<RegOrZero32>,
    Reg2Inp: IntoReg<RegOrZero32>,
    AddrRegInp: IntoReg<RegOrSp64>,
{
    fn new(status: StatusInp, reg1: Reg1Inp, reg2: Reg2Inp, addr: AddrRegInp) -> Self {
        Self {
            status: status.into_reg(),
            reg1: reg1.into_reg(),
            reg2: reg2.into_reg(),
            addr: addr.into_reg(),
        }
    }
}

impl<StatusInp, Reg1Inp, Reg2Inp, AddrRegInp>
    MakeExclusivePairStoreArgs<StatusInp, Reg1Inp, Reg2Inp, AddrRegInp>
    for ExclusivePairStoreArgs<RegOrZero64>
where
    StatusInp: IntoReg<RegOrZero32>,
    Reg1Inp: IntoReg<RegOrZero64>,
    Reg2Inp: IntoReg<RegOrZero64>,
    AddrRegInp: IntoReg<RegOrSp64>,
{
    fn new(status: StatusInp, reg1: Reg1Inp, reg2: Reg2Inp, addr: AddrRegInp) -> Self {
        Self {
            status: status.into_reg(),
            reg1: reg1.into_reg(),
            reg2: reg2.into_reg(),
            addr: addr.into_reg(),
        }
    }
}
