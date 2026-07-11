/* Copyright (C) 2026 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use aarchmrs_instructions::A64::ldst::ldstnapair_offs::{
    STNP_32_ldstnapair_offs::STNP_32_ldstnapair_offs,
    STNP_64_ldstnapair_offs::STNP_64_ldstnapair_offs,
};

use crate::bits::BitError;
use crate::register::{IntoReg, RegOrSp64, RegOrZero32, RegOrZero64, Register};
use crate::sealed::Sealed;

use super::{LdpStpOffset32, LdpStpOffset64};

/// A `stnp` instruction with a destination and an address.
pub struct Stnp<Rt, Addr> {
    rt: (Rt, Rt),
    addr: Addr,
}

impl<Rt, Addr> Stnp<Rt, Addr> {
    pub fn rt(&self) -> &(Rt, Rt) {
        &self.rt
    }

    pub fn addr(&self) -> &Addr {
        &self.addr
    }
}

impl<Rt, Addr> Sealed for Stnp<Rt, Addr> {}

/// Defines possible was to construct a `stnp` instruction.
pub trait MakeStnp<Rt1, Rt2, Addr>: Sealed {
    /// Allows defining both faillible and infallible constructors.
    type Output;

    fn new(rt: (Rt1, Rt2), addr: Addr) -> Self::Output;
}

pub fn stnp<DestInp1, DestInp2, TargetOut, AddrInp, AddrOut>(
    d1: DestInp1,
    d2: DestInp2,
    addr: AddrInp,
) -> <Stnp<TargetOut, AddrOut> as MakeStnp<DestInp1, DestInp2, AddrInp>>::Output
where
    Stnp<TargetOut, AddrOut>: MakeStnp<DestInp1, DestInp2, AddrInp>,
{
    Stnp::new((d1, d2), addr)
}

define_simple_pair_imm_offset_rules!(
    Stnp,
    MakeStnp,
    STNP,
    RegOrZero32,
    "32",
    LdpStpOffset32,
    "ldstnapair_offs"
);
define_simple_pair_imm_offset_rules!(
    Stnp,
    MakeStnp,
    STNP,
    RegOrZero64,
    "64",
    LdpStpOffset64,
    "ldstnapair_offs"
);
define_pair_fallible_rules!(STNP, Stnp, MakeStnp);

#[cfg(test)]
mod tests {
    use harm_test_utils::test_cases;

    use super::*;
    use crate::instructions::InstructionSeq;
    use crate::register::Reg32::*;
    use crate::register::Reg64::*;
    use RegOrSp64::SP;
    use RegOrZero32::WZR;
    use RegOrZero64::XZR;

    const STNP_DB: &str = "
a83f8c41	stnp x1, x3, [x2, -8]
a8008c41	stnp x1, x3, [x2, 8]
a81f8c41	stnp x1, x3, [x2, 504]
a8300c41	stnp x1, x3, [x2, -256]
a8000c41	stnp x1, x3, [x2, 0]
a8000c41	stnp x1, x3, [x2]
a83f8fe1	stnp x1, x3, [sp, -8]
a8008fe1	stnp x1, x3, [sp, 8]
a81f8fe1	stnp x1, x3, [sp, 504]
a8300fe1	stnp x1, x3, [sp, -256]
a8000fe1	stnp x1, x3, [sp, 0]
283f8c41	stnp w1, w3, [x2, -4]
28008c41	stnp w1, w3, [x2, 4]
281f8c41	stnp w1, w3, [x2, 252]
28200c41	stnp w1, w3, [x2, -256]
28000c41	stnp w1, w3, [x2, 0]
283f8fe1	stnp w1, w3, [sp, -4]
28008fe1	stnp w1, w3, [sp, 4]
281f8fe1	stnp w1, w3, [sp, 252]
28200fe1	stnp w1, w3, [sp, -256]
28000fe1	stnp w1, w3, [sp, 0]
a83f8c5f	stnp xzr, x3, [x2, -8]
a8008c5f	stnp xzr, x3, [x2, 8]
a81f8c5f	stnp xzr, x3, [x2, 504]
a8300c5f	stnp xzr, x3, [x2, -256]
a8000c5f	stnp xzr, x3, [x2, 0]
a83f8fff	stnp xzr, x3, [sp, -8]
a8008fff	stnp xzr, x3, [sp, 8]
a81f8fff	stnp xzr, x3, [sp, 504]
a8300fff	stnp xzr, x3, [sp, -256]
a8000fff	stnp xzr, x3, [sp, 0]
283f8c5f	stnp wzr, w3, [x2, -4]
28008c5f	stnp wzr, w3, [x2, 4]
281f8c5f	stnp wzr, w3, [x2, 252]
28200c5f	stnp wzr, w3, [x2, -256]
28000c5f	stnp wzr, w3, [x2, 0]
283f8fff	stnp wzr, w3, [sp, -4]
28008fff	stnp wzr, w3, [sp, 4]
281f8fff	stnp wzr, w3, [sp, 252]
28200fff	stnp wzr, w3, [sp, -256]
28000fff	stnp wzr, w3, [sp, 0]
";

    test_cases! {
        STNP_DB, untested_stnp_cases;
        test_stnp_x1_x2_m8, stnp(X1, X3, (X2, -8i32)).unwrap(), "stnp x1, x3, [x2, -8]";
        test_stnp_x1_x2_8, stnp(X1, X3, (X2, 8i32)).unwrap(), "stnp x1, x3, [x2, 8]";
        test_stnp_x1_x2_504, stnp(X1, X3, (X2, 504i32)).unwrap(), "stnp x1, x3, [x2, 504]";
        test_stnp_x1_x2_m256, stnp(X1, X3, (X2, -256i32)).unwrap(), "stnp x1, x3, [x2, -256]";
        test_stnp_x1_x2_0, stnp(X1, X3, (X2, 0i32)).unwrap(), "stnp x1, x3, [x2, 0]";
        test_stnp_x1_x2_simple, stnp(X1, X3, (X2,)), "stnp x1, x3, [x2]";
        test_stnp_x1_sp_m8, stnp(X1, X3, (SP, -8i32)).unwrap(), "stnp x1, x3, [sp, -8]";
        test_stnp_x1_sp_8, stnp(X1, X3, (SP, 8i32)).unwrap(), "stnp x1, x3, [sp, 8]";
        test_stnp_x1_sp_504, stnp(X1, X3, (SP, 504i32)).unwrap(), "stnp x1, x3, [sp, 504]";
        test_stnp_x1_sp_m256, stnp(X1, X3, (SP, -256i32)).unwrap(), "stnp x1, x3, [sp, -256]";
        test_stnp_x1_sp_0, stnp(X1, X3, (SP, 0i32)).unwrap(), "stnp x1, x3, [sp, 0]";
        test_stnp_w1_x2_m4, stnp(W1, W3, (X2, -4i32)).unwrap(), "stnp w1, w3, [x2, -4]";
        test_stnp_w1_x2_4, stnp(W1, W3, (X2, 4i32)).unwrap(), "stnp w1, w3, [x2, 4]";
        test_stnp_w1_x2_252, stnp(W1, W3, (X2, 252i32)).unwrap(), "stnp w1, w3, [x2, 252]";
        test_stnp_w1_x2_m256, stnp(W1, W3, (X2, -256i32)).unwrap(), "stnp w1, w3, [x2, -256]";
        test_stnp_w1_x2_0, stnp(W1, W3, (X2, 0i32)).unwrap(), "stnp w1, w3, [x2, 0]";
        test_stnp_w1_sp_m4, stnp(W1, W3, (SP, -4i32)).unwrap(), "stnp w1, w3, [sp, -4]";
        test_stnp_w1_sp_4, stnp(W1, W3, (SP, 4i32)).unwrap(), "stnp w1, w3, [sp, 4]";
        test_stnp_w1_sp_252, stnp(W1, W3, (SP, 252i32)).unwrap(), "stnp w1, w3, [sp, 252]";
        test_stnp_w1_sp_m256, stnp(W1, W3, (SP, -256i32)).unwrap(), "stnp w1, w3, [sp, -256]";
        test_stnp_w1_sp_0, stnp(W1, W3, (SP, 0i32)).unwrap(), "stnp w1, w3, [sp, 0]";
        test_stnp_xzr_x2_m8, stnp(XZR, X3, (X2, -8i32)).unwrap(), "stnp xzr, x3, [x2, -8]";
        test_stnp_xzr_x2_8, stnp(XZR, X3, (X2, 8i32)).unwrap(), "stnp xzr, x3, [x2, 8]";
        test_stnp_xzr_x2_504, stnp(XZR, X3, (X2, 504i32)).unwrap(), "stnp xzr, x3, [x2, 504]";
        test_stnp_xzr_x2_m256, stnp(XZR, X3, (X2, -256i32)).unwrap(), "stnp xzr, x3, [x2, -256]";
        test_stnp_xzr_x2_0, stnp(XZR, X3, (X2, 0i32)).unwrap(), "stnp xzr, x3, [x2, 0]";
        test_stnp_xzr_sp_m8, stnp(XZR, X3, (SP, -8i32)).unwrap(), "stnp xzr, x3, [sp, -8]";
        test_stnp_xzr_sp_8, stnp(XZR, X3, (SP, 8i32)).unwrap(), "stnp xzr, x3, [sp, 8]";
        test_stnp_xzr_sp_504, stnp(XZR, X3, (SP, 504i32)).unwrap(), "stnp xzr, x3, [sp, 504]";
        test_stnp_xzr_sp_m256, stnp(XZR, X3, (SP, -256i32)).unwrap(), "stnp xzr, x3, [sp, -256]";
        test_stnp_xzr_sp_0, stnp(XZR, X3, (SP, 0i32)).unwrap(), "stnp xzr, x3, [sp, 0]";
        test_stnp_wzr_x2_m4, stnp(WZR, W3, (X2, -4i32)).unwrap(), "stnp wzr, w3, [x2, -4]";
        test_stnp_wzr_x2_4, stnp(WZR, W3, (X2, 4i32)).unwrap(), "stnp wzr, w3, [x2, 4]";
        test_stnp_wzr_x2_252, stnp(WZR, W3, (X2, 252i32)).unwrap(), "stnp wzr, w3, [x2, 252]";
        test_stnp_wzr_x2_m256, stnp(WZR, W3, (X2, -256i32)).unwrap(), "stnp wzr, w3, [x2, -256]";
        test_stnp_wzr_x2_0, stnp(WZR, W3, (X2, 0i32)).unwrap(), "stnp wzr, w3, [x2, 0]";
        test_stnp_wzr_sp_m4, stnp(WZR, W3, (SP, -4i32)).unwrap(), "stnp wzr, w3, [sp, -4]";
        test_stnp_wzr_sp_4, stnp(WZR, W3, (SP, 4i32)).unwrap(), "stnp wzr, w3, [sp, 4]";
        test_stnp_wzr_sp_252, stnp(WZR, W3, (SP, 252i32)).unwrap(), "stnp wzr, w3, [sp, 252]";
        test_stnp_wzr_sp_m256, stnp(WZR, W3, (SP, -256i32)).unwrap(), "stnp wzr, w3, [sp, -256]";
        test_stnp_wzr_sp_0, stnp(WZR, W3, (SP, 0i32)).unwrap(), "stnp wzr, w3, [sp, 0]";
    }

    #[test]
    fn test_stnp_r64_offset_underflow() {
        assert!(stnp(X1, X2, (X3, -0x1fci32)).is_err());
        assert!(stnp(X1, X2, (X3, -0x200i32)).is_ok());
        assert!(stnp(X1, X2, (X3, -0x204i32)).is_err());
        assert!(stnp(X1, X2, (X3, -0x208i32)).is_err());
    }

    #[test]
    fn test_stnp_r32_offset_underflow() {
        assert!(stnp(W1, W2, (X3, -0xfei32)).is_err());
        assert!(stnp(W1, W2, (X3, -0x100i32)).is_ok());
        assert!(stnp(W1, W2, (X3, -0x102i32)).is_err());
        assert!(stnp(W1, W2, (X3, -0x104i32)).is_err());
    }

    #[test]
    fn test_stnp_r64_offset_overflow() {
        assert!(stnp(X1, X2, (X3, 0x1f4i32)).is_err());
        assert!(stnp(X1, X2, (X3, 0x1f8i32)).is_ok());
        assert!(stnp(X1, X2, (X3, 0x1fci32)).is_err());
        assert!(stnp(X1, X2, (X3, 0x200i32)).is_err());
    }

    #[test]
    fn test_stnp_r32_offset_overflow() {
        assert!(stnp(W1, W2, (X3, 0xfai32)).is_err());
        assert!(stnp(W1, W2, (X3, 0xfci32)).is_ok());
        assert!(stnp(W1, W2, (X3, 0xfei32)).is_err());
        assert!(stnp(W1, W2, (X3, 0x100i32)).is_err());
    }
}
