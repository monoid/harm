/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use aarchmrs_instructions::A64::ldst::ldst_unscaled::LDURSW_64_ldst_unscaled::LDURSW_64_ldst_unscaled;

use crate::bits::BitError;
use crate::instructions::RawInstruction;
use crate::register::{RegOrSp64, RegOrZero64, Register};
use crate::sealed::Sealed;

use super::UnscaledOffset;

/// A `LDURSW` instruction with a destination and an address.
pub struct Ldursw<Rt, Addr> {
    rt: Rt,
    addr: Addr,
}

impl<Rt, Addr> Ldursw<Rt, Addr> {
    pub fn rt(&self) -> &Rt {
        &self.rt
    }

    pub fn addr(&self) -> &Addr {
        &self.addr
    }
}

impl<Rt, Addr> Sealed for Ldursw<Rt, Addr> {}

/// Defines possible was to construct a `ldursw` instruction.
pub trait MakeLdursw<Rt, Addr>: Sealed {
    /// Allows defining both faillible and infallible constructors.
    type Output;

    fn new(rt: Rt, addr: Addr) -> Self::Output;
}

define_unscaled_imm_offset_rules!(Ldursw, MakeLdursw, LDURSW, RegOrZero64, 64);

pub fn ldursw<TargetInp, TargetOut, AddrInp, AddrOut>(
    dst: TargetInp,
    addr: AddrInp,
) -> <Ldursw<TargetOut, AddrOut> as MakeLdursw<TargetInp, AddrInp>>::Output
where
    Ldursw<TargetOut, AddrOut>: MakeLdursw<TargetInp, AddrInp>,
{
    Ldursw::new(dst, addr)
}

#[cfg(test)]
mod tests {
    use harm_test_utils::test_cases;

    use super::*;
    use crate::instructions::InstructionSeq;
    use crate::register::Reg64::*;
    use RegOrSp64::SP;
    use RegOrZero64::XZR;

    // 'ldursw (x1|w1|xzr|wzr), [(x2|sp), (-1|1|255|-256|0)]
    const LDURSW_DB: &str = "
b89ff041	ldursw x1, [x2, -1]
b8801041	ldursw x1, [x2, 1]
b88ff041	ldursw x1, [x2, 255]
b8900041	ldursw x1, [x2, -256]
b8800041	ldursw x1, [x2, 0]
b8800041	ldursw x1, [x2]
b89ff3e1	ldursw x1, [sp, -1]
b88013e1	ldursw x1, [sp, 1]
b88ff3e1	ldursw x1, [sp, 255]
b89003e1	ldursw x1, [sp, -256]
b88003e1	ldursw x1, [sp, 0]
b89ff05f	ldursw xzr, [x2, -1]
b880105f	ldursw xzr, [x2, 1]
b88ff05f	ldursw xzr, [x2, 255]
b890005f	ldursw xzr, [x2, -256]
b880005f	ldursw xzr, [x2, 0]
b89ff3ff	ldursw xzr, [sp, -1]
b88013ff	ldursw xzr, [sp, 1]
b88ff3ff	ldursw xzr, [sp, 255]
b89003ff	ldursw xzr, [sp, -256]
b88003ff	ldursw xzr, [sp, 0]
";

    test_cases! {
        LDURSW_DB, untested_ldursw_cases;
        test_ldursw_x1_x2_m1, ldursw(X1, (X2, -1)).unwrap(), "ldursw x1, [x2, -1]";
        test_ldursw_x1_x2_1, ldursw(X1, (X2, 1)).unwrap(), "ldursw x1, [x2, 1]";
        test_ldursw_x1_x2_255, ldursw(X1, (X2, 255)).unwrap(), "ldursw x1, [x2, 255]";
        test_ldursw_x1_x2_m256, ldursw(X1, (X2, -256)).unwrap(), "ldursw x1, [x2, -256]";
        test_ldursw_x1_x2_0, ldursw(X1, (X2, 0)).unwrap(), "ldursw x1, [x2, 0]";
        test_ldursw_x1_x2_simple, ldursw(X1, (X2,)), "ldursw x1, [x2]";
        test_ldursw_x1_sp_m1, ldursw(X1, (SP, -1)).unwrap(), "ldursw x1, [sp, -1]";
        test_ldursw_x1_sp_1, ldursw(X1, (SP, 1)).unwrap(), "ldursw x1, [sp, 1]";
        test_ldursw_x1_sp_255, ldursw(X1, (SP, 255)).unwrap(), "ldursw x1, [sp, 255]";
        test_ldursw_x1_sp_m256, ldursw(X1, (SP, -256)).unwrap(), "ldursw x1, [sp, -256]";
        test_ldursw_x1_sp_0, ldursw(X1, (SP, 0)).unwrap(), "ldursw x1, [sp, 0]";
        test_ldursw_xzr_x2_m1, ldursw(XZR, (X2, -1)).unwrap(), "ldursw xzr, [x2, -1]";
        test_ldursw_xzr_x2_1, ldursw(XZR, (X2, 1)).unwrap(), "ldursw xzr, [x2, 1]";
        test_ldursw_xzr_x2_255, ldursw(XZR, (X2, 255)).unwrap(), "ldursw xzr, [x2, 255]";
        test_ldursw_xzr_x2_m256, ldursw(XZR, (X2, -256)).unwrap(), "ldursw xzr, [x2, -256]";
        test_ldursw_xzr_x2_0, ldursw(XZR, (X2, 0)).unwrap(), "ldursw xzr, [x2, 0]";
        test_ldursw_xzr_sp_m1, ldursw(XZR, (SP, -1)).unwrap(), "ldursw xzr, [sp, -1]";
        test_ldursw_xzr_sp_1, ldursw(XZR, (SP, 1)).unwrap(), "ldursw xzr, [sp, 1]";
        test_ldursw_xzr_sp_255, ldursw(XZR, (SP, 255)).unwrap(), "ldursw xzr, [sp, 255]";
        test_ldursw_xzr_sp_m256, ldursw(XZR, (SP, -256)).unwrap(), "ldursw xzr, [sp, -256]";
        test_ldursw_xzr_sp_0, ldursw(XZR, (SP, 0)).unwrap(), "ldursw xzr, [sp, 0]";
    }
}
