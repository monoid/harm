/* Copyright (C) 2026 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use aarchmrs_instructions::A64::ldst::ldstnapair_offs::{
    LDNP_32_ldstnapair_offs::LDNP_32_ldstnapair_offs,
    LDNP_64_ldstnapair_offs::LDNP_64_ldstnapair_offs,
};

use crate::bits::BitError;
use crate::register::{IntoReg, RegOrSp64, RegOrZero32, RegOrZero64, Register};
use crate::sealed::Sealed;

use super::{LdpStpOffset32, LdpStpOffset64};

/// A `ldnp` instruction with a destination and an address.
pub struct Ldnp<Rt, Addr> {
    rt: (Rt, Rt),
    addr: Addr,
}

impl<Rt, Addr> Ldnp<Rt, Addr> {
    pub fn rt(&self) -> &(Rt, Rt) {
        &self.rt
    }

    pub fn addr(&self) -> &Addr {
        &self.addr
    }
}

impl<Rt, Addr> Sealed for Ldnp<Rt, Addr> {}

/// Defines possible was to construct a `ldnp` instruction.
pub trait MakeLdnp<Rt1, Rt2, Addr>: Sealed {
    /// Allows defining both faillible and infallible constructors.
    type Output;

    fn new(rt: (Rt1, Rt2), addr: Addr) -> Self::Output;
}

pub fn ldnp<DestInp1, DestInp2, TargetOut, AddrInp, AddrOut>(
    d1: DestInp1,
    d2: DestInp2,
    addr: AddrInp,
) -> <Ldnp<TargetOut, AddrOut> as MakeLdnp<DestInp1, DestInp2, AddrInp>>::Output
where
    Ldnp<TargetOut, AddrOut>: MakeLdnp<DestInp1, DestInp2, AddrInp>,
{
    Ldnp::new((d1, d2), addr)
}

define_simple_pair_imm_offset_rules!(
    Ldnp,
    MakeLdnp,
    LDNP,
    RegOrZero32,
    "32",
    LdpStpOffset32,
    "ldstnapair_offs"
);
define_simple_pair_imm_offset_rules!(
    Ldnp,
    MakeLdnp,
    LDNP,
    RegOrZero64,
    "64",
    LdpStpOffset64,
    "ldstnapair_offs"
);
define_pair_fallible_rules!(LDNP, Ldnp, MakeLdnp);

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

    const LDNP_DB: &str = "
a87f8c41	ldnp x1, x3, [x2, -8]
a8408c41	ldnp x1, x3, [x2, 8]
a85f8c41	ldnp x1, x3, [x2, 504]
a8700c41	ldnp x1, x3, [x2, -256]
a8400c41	ldnp x1, x3, [x2, 0]
a8400c41	ldnp x1, x3, [x2]
a87f8fe1	ldnp x1, x3, [sp, -8]
a8408fe1	ldnp x1, x3, [sp, 8]
a85f8fe1	ldnp x1, x3, [sp, 504]
a8700fe1	ldnp x1, x3, [sp, -256]
a8400fe1	ldnp x1, x3, [sp, 0]
287f8c41	ldnp w1, w3, [x2, -4]
28408c41	ldnp w1, w3, [x2, 4]
285f8c41	ldnp w1, w3, [x2, 252]
28600c41	ldnp w1, w3, [x2, -256]
28400c41	ldnp w1, w3, [x2, 0]
287f8fe1	ldnp w1, w3, [sp, -4]
28408fe1	ldnp w1, w3, [sp, 4]
285f8fe1	ldnp w1, w3, [sp, 252]
28600fe1	ldnp w1, w3, [sp, -256]
28400fe1	ldnp w1, w3, [sp, 0]
a87f8c5f	ldnp xzr, x3, [x2, -8]
a8408c5f	ldnp xzr, x3, [x2, 8]
a85f8c5f	ldnp xzr, x3, [x2, 504]
a8700c5f	ldnp xzr, x3, [x2, -256]
a8400c5f	ldnp xzr, x3, [x2, 0]
a87f8fff	ldnp xzr, x3, [sp, -8]
a8408fff	ldnp xzr, x3, [sp, 8]
a85f8fff	ldnp xzr, x3, [sp, 504]
a8700fff	ldnp xzr, x3, [sp, -256]
a8400fff	ldnp xzr, x3, [sp, 0]
287f8c5f	ldnp wzr, w3, [x2, -4]
28408c5f	ldnp wzr, w3, [x2, 4]
285f8c5f	ldnp wzr, w3, [x2, 252]
28600c5f	ldnp wzr, w3, [x2, -256]
28400c5f	ldnp wzr, w3, [x2, 0]
287f8fff	ldnp wzr, w3, [sp, -4]
28408fff	ldnp wzr, w3, [sp, 4]
285f8fff	ldnp wzr, w3, [sp, 252]
28600fff	ldnp wzr, w3, [sp, -256]
28400fff	ldnp wzr, w3, [sp, 0]
";

    test_cases! {
        LDNP_DB, untested_ldnp_cases;
        test_ldnp_x1_x2_m8, ldnp(X1, X3, (X2, -8i32)).unwrap(), "ldnp x1, x3, [x2, -8]";
        test_ldnp_x1_x2_8, ldnp(X1, X3, (X2, 8i32)).unwrap(), "ldnp x1, x3, [x2, 8]";
        test_ldnp_x1_x2_504, ldnp(X1, X3, (X2, 504i32)).unwrap(), "ldnp x1, x3, [x2, 504]";
        test_ldnp_x1_x2_m256, ldnp(X1, X3, (X2, -256i32)).unwrap(), "ldnp x1, x3, [x2, -256]";
        test_ldnp_x1_x2_0, ldnp(X1, X3, (X2, 0i32)).unwrap(), "ldnp x1, x3, [x2, 0]";
        test_ldnp_x1_x2_simple, ldnp(X1, X3, (X2,)), "ldnp x1, x3, [x2]";
        test_ldnp_x1_sp_m8, ldnp(X1, X3, (SP, -8i32)).unwrap(), "ldnp x1, x3, [sp, -8]";
        test_ldnp_x1_sp_8, ldnp(X1, X3, (SP, 8i32)).unwrap(), "ldnp x1, x3, [sp, 8]";
        test_ldnp_x1_sp_504, ldnp(X1, X3, (SP, 504i32)).unwrap(), "ldnp x1, x3, [sp, 504]";
        test_ldnp_x1_sp_m256, ldnp(X1, X3, (SP, -256i32)).unwrap(), "ldnp x1, x3, [sp, -256]";
        test_ldnp_x1_sp_0, ldnp(X1, X3, (SP, 0i32)).unwrap(), "ldnp x1, x3, [sp, 0]";
        test_ldnp_w1_x2_m4, ldnp(W1, W3, (X2, -4i32)).unwrap(), "ldnp w1, w3, [x2, -4]";
        test_ldnp_w1_x2_4, ldnp(W1, W3, (X2, 4i32)).unwrap(), "ldnp w1, w3, [x2, 4]";
        test_ldnp_w1_x2_252, ldnp(W1, W3, (X2, 252i32)).unwrap(), "ldnp w1, w3, [x2, 252]";
        test_ldnp_w1_x2_m256, ldnp(W1, W3, (X2, -256i32)).unwrap(), "ldnp w1, w3, [x2, -256]";
        test_ldnp_w1_x2_0, ldnp(W1, W3, (X2, 0i32)).unwrap(), "ldnp w1, w3, [x2, 0]";
        test_ldnp_w1_sp_m4, ldnp(W1, W3, (SP, -4i32)).unwrap(), "ldnp w1, w3, [sp, -4]";
        test_ldnp_w1_sp_4, ldnp(W1, W3, (SP, 4i32)).unwrap(), "ldnp w1, w3, [sp, 4]";
        test_ldnp_w1_sp_252, ldnp(W1, W3, (SP, 252i32)).unwrap(), "ldnp w1, w3, [sp, 252]";
        test_ldnp_w1_sp_m256, ldnp(W1, W3, (SP, -256i32)).unwrap(), "ldnp w1, w3, [sp, -256]";
        test_ldnp_w1_sp_0, ldnp(W1, W3, (SP, 0i32)).unwrap(), "ldnp w1, w3, [sp, 0]";
        test_ldnp_xzr_x2_m8, ldnp(XZR, X3, (X2, -8i32)).unwrap(), "ldnp xzr, x3, [x2, -8]";
        test_ldnp_xzr_x2_8, ldnp(XZR, X3, (X2, 8i32)).unwrap(), "ldnp xzr, x3, [x2, 8]";
        test_ldnp_xzr_x2_504, ldnp(XZR, X3, (X2, 504i32)).unwrap(), "ldnp xzr, x3, [x2, 504]";
        test_ldnp_xzr_x2_m256, ldnp(XZR, X3, (X2, -256i32)).unwrap(), "ldnp xzr, x3, [x2, -256]";
        test_ldnp_xzr_x2_0, ldnp(XZR, X3, (X2, 0i32)).unwrap(), "ldnp xzr, x3, [x2, 0]";
        test_ldnp_xzr_sp_m8, ldnp(XZR, X3, (SP, -8i32)).unwrap(), "ldnp xzr, x3, [sp, -8]";
        test_ldnp_xzr_sp_8, ldnp(XZR, X3, (SP, 8i32)).unwrap(), "ldnp xzr, x3, [sp, 8]";
        test_ldnp_xzr_sp_504, ldnp(XZR, X3, (SP, 504i32)).unwrap(), "ldnp xzr, x3, [sp, 504]";
        test_ldnp_xzr_sp_m256, ldnp(XZR, X3, (SP, -256i32)).unwrap(), "ldnp xzr, x3, [sp, -256]";
        test_ldnp_xzr_sp_0, ldnp(XZR, X3, (SP, 0i32)).unwrap(), "ldnp xzr, x3, [sp, 0]";
        test_ldnp_wzr_x2_m4, ldnp(WZR, W3, (X2, -4i32)).unwrap(), "ldnp wzr, w3, [x2, -4]";
        test_ldnp_wzr_x2_4, ldnp(WZR, W3, (X2, 4i32)).unwrap(), "ldnp wzr, w3, [x2, 4]";
        test_ldnp_wzr_x2_252, ldnp(WZR, W3, (X2, 252i32)).unwrap(), "ldnp wzr, w3, [x2, 252]";
        test_ldnp_wzr_x2_m256, ldnp(WZR, W3, (X2, -256i32)).unwrap(), "ldnp wzr, w3, [x2, -256]";
        test_ldnp_wzr_x2_0, ldnp(WZR, W3, (X2, 0i32)).unwrap(), "ldnp wzr, w3, [x2, 0]";
        test_ldnp_wzr_sp_m4, ldnp(WZR, W3, (SP, -4i32)).unwrap(), "ldnp wzr, w3, [sp, -4]";
        test_ldnp_wzr_sp_4, ldnp(WZR, W3, (SP, 4i32)).unwrap(), "ldnp wzr, w3, [sp, 4]";
        test_ldnp_wzr_sp_252, ldnp(WZR, W3, (SP, 252i32)).unwrap(), "ldnp wzr, w3, [sp, 252]";
        test_ldnp_wzr_sp_m256, ldnp(WZR, W3, (SP, -256i32)).unwrap(), "ldnp wzr, w3, [sp, -256]";
        test_ldnp_wzr_sp_0, ldnp(WZR, W3, (SP, 0i32)).unwrap(), "ldnp wzr, w3, [sp, 0]";
    }

    #[test]
    fn test_ldnp_r64_offset_underflow() {
        assert!(ldnp(X1, X2, (X3, -0x1fci32)).is_err());
        assert!(ldnp(X1, X2, (X3, -0x200i32)).is_ok());
        assert!(ldnp(X1, X2, (X3, -0x204i32)).is_err());
        assert!(ldnp(X1, X2, (X3, -0x208i32)).is_err());
    }

    #[test]
    fn test_ldnp_r32_offset_underflow() {
        assert!(ldnp(W1, W2, (X3, -0xfei32)).is_err());
        assert!(ldnp(W1, W2, (X3, -0x100i32)).is_ok());
        assert!(ldnp(W1, W2, (X3, -0x102i32)).is_err());
        assert!(ldnp(W1, W2, (X3, -0x104i32)).is_err());
    }

    #[test]
    fn test_ldnp_r64_offset_overflow() {
        assert!(ldnp(X1, X2, (X3, 0x1f4i32)).is_err());
        assert!(ldnp(X1, X2, (X3, 0x1f8i32)).is_ok());
        assert!(ldnp(X1, X2, (X3, 0x1fci32)).is_err());
        assert!(ldnp(X1, X2, (X3, 0x200i32)).is_err());
    }

    #[test]
    fn test_ldnp_r32_offset_overflow() {
        assert!(ldnp(W1, W2, (X3, 0xfai32)).is_err());
        assert!(ldnp(W1, W2, (X3, 0xfci32)).is_ok());
        assert!(ldnp(W1, W2, (X3, 0xfei32)).is_err());
        assert!(ldnp(W1, W2, (X3, 0x100i32)).is_err());
    }
}
