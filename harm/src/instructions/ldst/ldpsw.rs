/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use aarchmrs_instructions::A64::ldst::{
    ldstpair_off::LDPSW_64_ldstpair_off::LDPSW_64_ldstpair_off,
    ldstpair_post::LDPSW_64_ldstpair_post::LDPSW_64_ldstpair_post,
    ldstpair_pre::LDPSW_64_ldstpair_pre::LDPSW_64_ldstpair_pre,
};

use super::{Inc, LdpStpOffset32};
use crate::bits::BitError;
use crate::register::{RegOrSp64, RegOrZero64, Register as _};
use crate::sealed::Sealed;

/// A `ldpsw` instruction with a destination and an address.
pub struct Ldpsw<Rt, Addr> {
    rt: (Rt, Rt),
    addr: Addr,
}

impl<Rt, Addr> Ldpsw<Rt, Addr> {
    pub fn rt(&self) -> &(Rt, Rt) {
        &self.rt
    }

    pub fn addr(&self) -> &Addr {
        &self.addr
    }
}

impl<Rt, Addr> Sealed for Ldpsw<Rt, Addr> {}

/// Defines possible was to construct a `ldpsw` instruction.
pub trait MakeLdpsw<Rt1, Rt2, Addr>: Sealed {
    /// Allows defining both faillible and infallible constructors.
    type Output;

    fn new(rt: (Rt1, Rt2), addr: Addr) -> Self::Output;
}

pub fn ldpsw<DestInp1, DestInp2, TargetOut, AddrInp, AddrOut>(
    d1: DestInp1,
    d2: DestInp2,
    addr: AddrInp,
) -> <Ldpsw<TargetOut, AddrOut> as MakeLdpsw<DestInp1, DestInp2, AddrInp>>::Output
where
    Ldpsw<TargetOut, AddrOut>: MakeLdpsw<DestInp1, DestInp2, AddrInp>,
{
    Ldpsw::new((d1, d2), addr)
}

define_pair_imm_offset_rules!(Ldpsw, MakeLdpsw, LDPSW, RegOrZero64, "64", LdpStpOffset32);
define_pair_fallible_rules!(LDPSW, Ldpsw, MakeLdpsw);

#[cfg(test)]
mod tests {
    use harm_test_utils::test_cases;

    use super::*;
    use crate::instructions::InstructionSeq;
    use crate::instructions::ldst::{inc, postinc, preinc};
    use crate::register::Reg64::*;
    use RegOrSp64::SP;
    use RegOrZero64::XZR;

    const LDPSW_DB: &str = "
697f0c41	ldpsw x1, x3, [x2, -8]
69410c41	ldpsw x1, x3, [x2, 8]
695f8c41	ldpsw x1, x3, [x2, 252]
69600c41	ldpsw x1, x3, [x2, -256]
69400c41	ldpsw x1, x3, [x2, 0]
69400c41	ldpsw x1, x3, [x2]
697f0fe1	ldpsw x1, x3, [sp, -8]
69410fe1	ldpsw x1, x3, [sp, 8]
695f8fe1	ldpsw x1, x3, [sp, 252]
69600fe1	ldpsw x1, x3, [sp, -256]
69400fe1	ldpsw x1, x3, [sp, 0]
697f0c5f	ldpsw xzr, x3, [x2, -8]
69410c5f	ldpsw xzr, x3, [x2, 8]
695f8c5f	ldpsw xzr, x3, [x2, 252]
69600c5f	ldpsw xzr, x3, [x2, -256]
69400c5f	ldpsw xzr, x3, [x2, 0]
697f0fff	ldpsw xzr, x3, [sp, -8]
69410fff	ldpsw xzr, x3, [sp, 8]
695f8fff	ldpsw xzr, x3, [sp, 252]
69600fff	ldpsw xzr, x3, [sp, -256]
69400fff	ldpsw xzr, x3, [sp, 0]
";

    const LDPSW_PRE_POST_INC_DB: &str = "
68df8041	ldpsw x1, x0, [x2], #0xfc
68df805f	ldpsw xzr, x0, [x2], #0xfc
68df83e1	ldpsw x1, x0, [sp], #0xfc
69df8041	ldpsw x1, x0, [x2, #0xfc]!
69df805f	ldpsw xzr, x0, [x2, #0xfc]!
69df83e1	ldpsw x1, x0, [sp, #0xfc]!
68e00041	ldpsw x1, x0, [x2], #-0x100
68e003e1	ldpsw x1, x0, [sp], #-0x100
69e00041	ldpsw x1, x0, [x2, #-0x100]!
69e003e1	ldpsw x1, x0, [sp, #-0x100]!
";

    test_cases! {
        LDPSW_DB, untested_ldpsw_cases;
        test_ldpsw_x1_x2_m8, ldpsw(X1, X3, (X2, -8i32)).unwrap(), "ldpsw x1, x3, [x2, -8]";
        test_ldpsw_x1_x2_8, ldpsw(X1, X3, (X2, 8i32)).unwrap(), "ldpsw x1, x3, [x2, 8]";
        test_ldpsw_x1_x2_252, ldpsw(X1, X3, (X2, 252i32)).unwrap(), "ldpsw x1, x3, [x2, 252]";
        test_ldpsw_x1_x2_m256, ldpsw(X1, X3, (X2, -256i32)).unwrap(), "ldpsw x1, x3, [x2, -256]";
        test_ldpsw_x1_x2_0, ldpsw(X1, X3, (X2, 0i32)).unwrap(), "ldpsw x1, x3, [x2, 0]";
        test_ldpsw_x1_x2_simple, ldpsw(X1, X3, (X2,)), "ldpsw x1, x3, [x2]";
        test_ldpsw_x1_sp_m8, ldpsw(X1, X3, (SP, -8i32)).unwrap(), "ldpsw x1, x3, [sp, -8]";
        test_ldpsw_x1_sp_8, ldpsw(X1, X3, (SP, 8i32)).unwrap(), "ldpsw x1, x3, [sp, 8]";
        test_ldpsw_x1_sp_252, ldpsw(X1, X3, (SP, 252i32)).unwrap(), "ldpsw x1, x3, [sp, 252]";
        test_ldpsw_x1_sp_m256, ldpsw(X1, X3, (SP, -256i32)).unwrap(), "ldpsw x1, x3, [sp, -256]";
        test_ldpsw_x1_sp_0, ldpsw(X1, X3, (SP, 0i32)).unwrap(), "ldpsw x1, x3, [sp, 0]";
        test_ldpsw_xzr_x2_m8, ldpsw(XZR, X3, (X2, -8i32)).unwrap(), "ldpsw xzr, x3, [x2, -8]";
        test_ldpsw_xzr_x2_8, ldpsw(XZR, X3, (X2, 8i32)).unwrap(), "ldpsw xzr, x3, [x2, 8]";
        test_ldpsw_xzr_x2_252, ldpsw(XZR, X3, (X2, 252i32)).unwrap(), "ldpsw xzr, x3, [x2, 252]";
        test_ldpsw_xzr_x2_m256, ldpsw(XZR, X3, (X2, -256i32)).unwrap(), "ldpsw xzr, x3, [x2, -256]";
        test_ldpsw_xzr_x2_0, ldpsw(XZR, X3, (X2, 0i32)).unwrap(), "ldpsw xzr, x3, [x2, 0]";
        test_ldpsw_xzr_sp_m8, ldpsw(XZR, X3, (SP, -8i32)).unwrap(), "ldpsw xzr, x3, [sp, -8]";
        test_ldpsw_xzr_sp_8, ldpsw(XZR, X3, (SP, 8i32)).unwrap(), "ldpsw xzr, x3, [sp, 8]";
        test_ldpsw_xzr_sp_252, ldpsw(XZR, X3, (SP, 252i32)).unwrap(), "ldpsw xzr, x3, [sp, 252]";
        test_ldpsw_xzr_sp_m256, ldpsw(XZR, X3, (SP, -256i32)).unwrap(), "ldpsw xzr, x3, [sp, -256]";
        test_ldpsw_xzr_sp_0, ldpsw(XZR, X3, (SP, 0i32)).unwrap(), "ldpsw xzr, x3, [sp, 0]";
    }

    test_cases! {
        LDPSW_PRE_POST_INC_DB, untested_ldpsw_pre_post_inc;
        test_ldpsw_r64_r64_post_inc, ldpsw(X1, X0, (X2, inc(0xfc))).unwrap(), "ldpsw x1, x0, [x2], #0xfc";
        test_ldpsw_r64_r64_post_inc_neg, ldpsw(X1, X0, (X2, inc(-0x100))).unwrap(), "ldpsw x1, x0, [x2], #-0x100";
        test_ldpsw_r64_r64_postinc, ldpsw(X1, X0, postinc(X2, 0xfc)).unwrap(), "ldpsw x1, x0, [x2], #0xfc";
        test_ldpsw_r64_r64_postinc_neg, ldpsw(X1, X0, postinc(X2, -0x100)).unwrap(), "ldpsw x1, x0, [x2], #-0x100";
        test_ldpsw_r64_r64_pre_inc, ldpsw(X1, X0, (inc(0xfc), X2)).unwrap(), "ldpsw x1, x0, [x2, #0xfc]!";
        test_ldpsw_r64_r64_pre_inc_neg, ldpsw(X1, X0, (inc(-0x100), X2)).unwrap(), "ldpsw x1, x0, [x2, #-0x100]!";
        test_ldpsw_r64_r64_pre_inc_neg2, ldpsw(X1, X0, (inc(LdpStpOffset32::new(-0x100).unwrap()), X2)), "ldpsw x1, x0, [x2, #-0x100]!";
        test_ldpsw_r64_r64_preinc, ldpsw(X1, X0, preinc(X2, 0xfc)).unwrap(), "ldpsw x1, x0, [x2, #0xfc]!";
        test_ldpsw_r64_r64_preinc_neg, ldpsw(X1, X0, preinc(X2, -0x100)).unwrap(), "ldpsw x1, x0, [x2, #-0x100]!";
        test_ldpsw_r64_r64_preinc_neg2, ldpsw(X1, X0, preinc(X2, LdpStpOffset32::new(-0x100).unwrap())), "ldpsw x1, x0, [x2, #-0x100]!";
        test_ldpsw_r64_sp_post_inc, ldpsw(X1, X0, (SP, inc(0xfc))).unwrap(), "ldpsw x1, x0, [sp], #0xfc";
        test_ldpsw_r64_sp_post_inc_neg, ldpsw(X1, X0, (SP, inc(-0x100))).unwrap(), "ldpsw x1, x0, [sp], #-0x100";
        test_ldpsw_r64_sp_postinc, ldpsw(X1, X0, postinc(SP, 0xfc)).unwrap(), "ldpsw x1, x0, [sp], #0xfc";
        test_ldpsw_r64_sp_postinc_neg, ldpsw(X1, X0, postinc(SP, -0x100)).unwrap(), "ldpsw x1, x0, [sp], #-0x100";
        test_ldpsw_r64_sp_pre_inc, ldpsw(X1, X0, (inc(0xfc), SP)).unwrap(), "ldpsw x1, x0, [sp, #0xfc]!";
        test_ldpsw_r64_sp_pre_inc_neg, ldpsw(X1, X0, (inc(-0x100), SP)).unwrap(), "ldpsw x1, x0, [sp, #-0x100]!";
        test_ldpsw_r64_sp_preinc, ldpsw(X1, X0, preinc(SP, 0xfc)).unwrap(), "ldpsw x1, x0, [sp, #0xfc]!";
        test_ldpsw_r64_sp_preinc_neg, ldpsw(X1, X0, preinc(SP, -0x100)).unwrap(), "ldpsw x1, x0, [sp, #-0x100]!";
        test_ldpsw_xzr_r64_post_inc, ldpsw(XZR, X0, (X2, inc(0xfc))).unwrap(), "ldpsw xzr, x0, [x2], #0xfc";
        test_ldpsw_xzr_r64_pre_inc, ldpsw(XZR, X0, (inc(0xfc), X2)).unwrap(), "ldpsw xzr, x0, [x2, #0xfc]!";
    }

    #[test]
    fn test_ldpsw_r64_offset_underflow() {
        assert!(ldpsw(X1, X2, (X3, -0xfei32)).is_err());
        assert!(ldpsw(X1, X2, (X3, -0x100i32)).is_ok());
        assert!(ldpsw(X1, X2, (X3, -0x102i32)).is_err());
        assert!(ldpsw(X1, X2, (X3, -0x104i32)).is_err());
    }

    #[test]
    fn test_ldpsw_r64_offset_overflow() {
        assert!(ldpsw(X1, X2, (X3, 0xfai32)).is_err());
        assert!(ldpsw(X1, X2, (X3, 0xfci32)).is_ok());
        assert!(ldpsw(X1, X2, (X3, 0xfei32)).is_err());
        assert!(ldpsw(X1, X2, (X3, 0x100i32)).is_err());
    }
}
