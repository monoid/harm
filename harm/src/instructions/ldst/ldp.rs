/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use aarchmrs_instructions::A64::ldst::{
    ldstpair_off::{
        LDP_32_ldstpair_off::LDP_32_ldstpair_off, LDP_64_ldstpair_off::LDP_64_ldstpair_off,
    },
    ldstpair_post::{
        LDP_32_ldstpair_post::LDP_32_ldstpair_post, LDP_64_ldstpair_post::LDP_64_ldstpair_post,
    },
    ldstpair_pre::{
        LDP_32_ldstpair_pre::LDP_32_ldstpair_pre, LDP_64_ldstpair_pre::LDP_64_ldstpair_pre,
    },
};

use super::{Inc, LdpStpOffset32, LdpStpOffset64};
use crate::bits::BitError;
use crate::register::{IntoCode, RegOrSp64, RegOrZero32, RegOrZero64};

/// A `LDP` instruction with a destination and an address.
pub struct Ldp<Rt, Addr> {
    rt: (Rt, Rt),
    addr: Addr,
}

impl<Rt, Addr> Ldp<Rt, Addr> {
    pub fn rt(&self) -> &(Rt, Rt) {
        &self.rt
    }

    pub fn addr(&self) -> &Addr {
        &self.addr
    }
}

/// Defines possible was to construct a `Load` instruction.
// TODO sealed trait?
pub trait MakeLdp<Rt1, Rt2, Addr> {
    /// Allows defining both faillible and infallible constructors.
    type Output;

    fn new(rt: (Rt1, Rt2), addr: Addr) -> Self::Output;
}

pub fn ldp<TargetInp1, TargetInp2, TargetOut, AddrInp, AddrOut>(
    dst: (TargetInp1, TargetInp2),
    addr: AddrInp,
) -> <Ldp<TargetOut, AddrOut> as MakeLdp<TargetInp1, TargetInp2, AddrInp>>::Output
where
    Ldp<TargetOut, AddrOut>: MakeLdp<TargetInp1, TargetInp2, AddrInp>,
{
    Ldp::new(dst, addr)
}

define_pair_imm_offset_rules!(Ldp, MakeLdp, LDP, RegOrZero32, "32", LdpStpOffset32);
define_pair_imm_offset_rules!(Ldp, MakeLdp, LDP, RegOrZero64, "64", LdpStpOffset64);
define_pair_fallible_rules!(LDP, Ldp, MakeLdp);

#[cfg(test)]
mod tests {
    use harm_test_utils::test_cases;

    use super::*;
    use crate::instructions::InstructionSeq;
    use crate::instructions::ldst::inc;
    use crate::instructions::ldst::postinc;
    use crate::instructions::ldst::preinc;
    use crate::register::Reg32::*;
    use crate::register::Reg64::*;
    use RegOrSp64::SP;
    use RegOrZero32::WZR;
    use RegOrZero64::XZR;

    const LDP_DB: &str = "
a97f8c41	ldp x1, x3, [x2, -8]
a9408c41	ldp x1, x3, [x2, 8]
a95f8c41	ldp x1, x3, [x2, 504]
a9700c41	ldp x1, x3, [x2, -256]
a9400c41	ldp x1, x3, [x2, 0]
a97f8fe1	ldp x1, x3, [sp, -8]
a9408fe1	ldp x1, x3, [sp, 8]
a95f8fe1	ldp x1, x3, [sp, 504]
a9700fe1	ldp x1, x3, [sp, -256]
a9400fe1	ldp x1, x3, [sp, 0]
297f8c41	ldp w1, w3, [x2, -4]
29408c41	ldp w1, w3, [x2, 4]
295f8c41	ldp w1, w3, [x2, 252]
29600c41	ldp w1, w3, [x2, -256]
29400c41	ldp w1, w3, [x2, 0]
297f8fe1	ldp w1, w3, [sp, -4]
29408fe1	ldp w1, w3, [sp, 4]
295f8fe1	ldp w1, w3, [sp, 252]
29600fe1	ldp w1, w3, [sp, -256]
29400fe1	ldp w1, w3, [sp, 0]
a97f8c5f	ldp xzr, x3, [x2, -8]
a9408c5f	ldp xzr, x3, [x2, 8]
a95f8c5f	ldp xzr, x3, [x2, 504]
a9700c5f	ldp xzr, x3, [x2, -256]
a9400c5f	ldp xzr, x3, [x2, 0]
a97f8fff	ldp xzr, x3, [sp, -8]
a9408fff	ldp xzr, x3, [sp, 8]
a95f8fff	ldp xzr, x3, [sp, 504]
a9700fff	ldp xzr, x3, [sp, -256]
a9400fff	ldp xzr, x3, [sp, 0]
297f8c5f	ldp wzr, w3, [x2, -4]
29408c5f	ldp wzr, w3, [x2, 4]
295f8c5f	ldp wzr, w3, [x2, 252]
29600c5f	ldp wzr, w3, [x2, -256]
29400c5f	ldp wzr, w3, [x2, 0]
297f8fff	ldp wzr, w3, [sp, -4]
29408fff	ldp wzr, w3, [sp, 4]
295f8fff	ldp wzr, w3, [sp, 252]
29600fff	ldp wzr, w3, [sp, -256]
29400fff	ldp wzr, w3, [sp, 0]
";

    const LDP_PRE_POST_INC_DB: &str = "
28df8041	ldp w1, w0, [x2], #0xfc
28df805f	ldp wzr, w0, [x2], #0xfc
28df83e1	ldp w1, w0, [sp], #0xfc
29df8041	ldp w1, w0, [x2, #0xfc]!
29df805f	ldp wzr, w0, [x2, #0xfc]!
29df83e1	ldp w1, w0, [sp, #0xfc]!
28e00041	ldp w1, w0, [x2], #-0x100
28e003e1	ldp w1, w0, [sp], #-0x100
29e00041	ldp w1, w0, [x2, #-0x100]!
29e003e1	ldp w1, w0, [sp, #-0x100]!
a8df8041	ldp x1, x0, [x2], #0x1f8
a8df805f	ldp xzr, x0, [x2], #0x1f8
a8df83e1	ldp x1, x0, [sp], #0x1f8
a9df8041	ldp x1, x0, [x2, #0x1f8]!
a9df805f	ldp xzr, x0, [x2, #0x1f8]!
a9df83e1	ldp x1, x0, [sp, #0x1f8]!
a8e00041	ldp x1, x0, [x2], #-0x200
a8e003e1	ldp x1, x0, [sp], #-0x200
a9e00041	ldp x1, x0, [x2, #-0x200]!
a9e003e1	ldp x1, x0, [sp, #-0x200]!
";

    test_cases! {
        LDP_DB, untested_ldp_cases;
        test_ldp_x1_x2_m8, ldp((X1, X3), (X2, -8i32)).unwrap(), "ldp x1, x3, [x2, -8]";
        test_ldp_x1_x2_8, ldp((X1, X3), (X2, 8i32)).unwrap(), "ldp x1, x3, [x2, 8]";
        test_ldp_x1_x2_504, ldp((X1, X3), (X2, 504i32)).unwrap(), "ldp x1, x3, [x2, 504]";
        test_ldp_x1_x2_m256, ldp((X1, X3), (X2, -256i32)).unwrap(), "ldp x1, x3, [x2, -256]";
        test_ldp_x1_x2_0, ldp((X1, X3), (X2, 0i32)).unwrap(), "ldp x1, x3, [x2, 0]";
        test_ldp_x1_sp_m8, ldp((X1, X3), (SP, -8i32)).unwrap(), "ldp x1, x3, [sp, -8]";
        test_ldp_x1_sp_8, ldp((X1, X3), (SP, 8i32)).unwrap(), "ldp x1, x3, [sp, 8]";
        test_ldp_x1_sp_504, ldp((X1, X3), (SP, 504i32)).unwrap(), "ldp x1, x3, [sp, 504]";
        test_ldp_x1_sp_m256, ldp((X1, X3), (SP, -256i32)).unwrap(), "ldp x1, x3, [sp, -256]";
        test_ldp_x1_sp_0, ldp((X1, X3), (SP, 0i32)).unwrap(), "ldp x1, x3, [sp, 0]";
        test_ldp_w1_x2_m4, ldp((W1, W3), (X2, -4i32)).unwrap(), "ldp w1, w3, [x2, -4]";
        test_ldp_w1_x2_4, ldp((W1, W3), (X2, 4i32)).unwrap(), "ldp w1, w3, [x2, 4]";
        test_ldp_w1_x2_252, ldp((W1, W3), (X2, 252i32)).unwrap(), "ldp w1, w3, [x2, 252]";
        test_ldp_w1_x2_m256, ldp((W1, W3), (X2, -256i32)).unwrap(), "ldp w1, w3, [x2, -256]";
        test_ldp_w1_x2_0, ldp((W1, W3), (X2, 0i32)).unwrap(), "ldp w1, w3, [x2, 0]";
        test_ldp_w1_sp_m4, ldp((W1, W3), (SP, -4i32)).unwrap(), "ldp w1, w3, [sp, -4]";
        test_ldp_w1_sp_4, ldp((W1, W3), (SP, 4i32)).unwrap(), "ldp w1, w3, [sp, 4]";
        test_ldp_w1_sp_252, ldp((W1, W3), (SP, 252i32)).unwrap(), "ldp w1, w3, [sp, 252]";
        test_ldp_w1_sp_m256, ldp((W1, W3), (SP, -256i32)).unwrap(), "ldp w1, w3, [sp, -256]";
        test_ldp_w1_sp_0, ldp((W1, W3), (SP, 0i32)).unwrap(), "ldp w1, w3, [sp, 0]";
        test_ldp_xzr_x2_m8, ldp((XZR, X3), (X2, -8i32)).unwrap(), "ldp xzr, x3, [x2, -8]";
        test_ldp_xzr_x2_8, ldp((XZR, X3), (X2, 8i32)).unwrap(), "ldp xzr, x3, [x2, 8]";
        test_ldp_xzr_x2_504, ldp((XZR, X3), (X2, 504i32)).unwrap(), "ldp xzr, x3, [x2, 504]";
        test_ldp_xzr_x2_m256, ldp((XZR, X3), (X2, -256i32)).unwrap(), "ldp xzr, x3, [x2, -256]";
        test_ldp_xzr_x2_0, ldp((XZR, X3), (X2, 0i32)).unwrap(), "ldp xzr, x3, [x2, 0]";
        test_ldp_xzr_sp_m8, ldp((XZR, X3), (SP, -8i32)).unwrap(), "ldp xzr, x3, [sp, -8]";
        test_ldp_xzr_sp_8, ldp((XZR, X3), (SP, 8i32)).unwrap(), "ldp xzr, x3, [sp, 8]";
        test_ldp_xzr_sp_504, ldp((XZR, X3), (SP, 504i32)).unwrap(), "ldp xzr, x3, [sp, 504]";
        test_ldp_xzr_sp_m256, ldp((XZR, X3), (SP, -256i32)).unwrap(), "ldp xzr, x3, [sp, -256]";
        test_ldp_xzr_sp_0, ldp((XZR, X3), (SP, 0i32)).unwrap(), "ldp xzr, x3, [sp, 0]";
        test_ldp_wzr_x2_m4, ldp((WZR, W3), (X2, -4i32)).unwrap(), "ldp wzr, w3, [x2, -4]";
        test_ldp_wzr_x2_4, ldp((WZR, W3), (X2, 4i32)).unwrap(), "ldp wzr, w3, [x2, 4]";
        test_ldp_wzr_x2_252, ldp((WZR, W3), (X2, 252i32)).unwrap(), "ldp wzr, w3, [x2, 252]";
        test_ldp_wzr_x2_m256, ldp((WZR, W3), (X2, -256i32)).unwrap(), "ldp wzr, w3, [x2, -256]";
        test_ldp_wzr_x2_0, ldp((WZR, W3), (X2, 0i32)).unwrap(), "ldp wzr, w3, [x2, 0]";
        test_ldp_wzr_sp_m4, ldp((WZR, W3), (SP, -4i32)).unwrap(), "ldp wzr, w3, [sp, -4]";
        test_ldp_wzr_sp_4, ldp((WZR, W3), (SP, 4i32)).unwrap(), "ldp wzr, w3, [sp, 4]";
        test_ldp_wzr_sp_252, ldp((WZR, W3), (SP, 252i32)).unwrap(), "ldp wzr, w3, [sp, 252]";
        test_ldp_wzr_sp_m256, ldp((WZR, W3), (SP, -256i32)).unwrap(), "ldp wzr, w3, [sp, -256]";
        test_ldp_wzr_sp_0, ldp((WZR, W3), (SP, 0i32)).unwrap(), "ldp wzr, w3, [sp, 0]";
    }

    test_cases! {
        LDP_PRE_POST_INC_DB, untested_ldp_pre_post_inc;
        test_ldp_r32_r64_post_inc, ldp((W1, W0), (X2, inc(0xfc))).unwrap(), "ldp w1, w0, [x2], #0xfc";
        test_ldp_r32_r64_post_inc_neg, ldp((W1, W0), (X2, inc(-0x100))).unwrap(), "ldp w1, w0, [x2], #-0x100";
        test_ldp_r32_r64_postinc, ldp((W1, W0), postinc(X2, 0xfc)).unwrap(), "ldp w1, w0, [x2], #0xfc";
        test_ldp_r32_r64_postinc_neg, ldp((W1, W0), postinc(X2, -0x100)).unwrap(), "ldp w1, w0, [x2], #-0x100";
        test_ldp_r32_r64_pre_inc, ldp((W1, W0), (inc(0xfc), X2)).unwrap(), "ldp w1, w0, [x2, #0xfc]!";
        test_ldp_r32_r64_pre_inc_neg, ldp((W1, W0), (inc(-0x100), X2)).unwrap(), "ldp w1, w0, [x2, #-0x100]!";
        test_ldp_r32_r64_preinc, ldp((W1, W0), preinc(X2, 0xfc)).unwrap(), "ldp w1, w0, [x2, #0xfc]!";
        test_ldp_r32_r64_preinc_neg, ldp((W1, W0), preinc(X2, -0x100)).unwrap(), "ldp w1, w0, [x2, #-0x100]!";
        test_ldp_r32_sp_post_inc, ldp((W1, W0), (SP, inc(0xfc))).unwrap(), "ldp w1, w0, [sp], #0xfc";
        test_ldp_r32_sp_post_inc_neg, ldp((W1, W0), (SP, inc(-0x100))).unwrap(), "ldp w1, w0, [sp], #-0x100";
        test_ldp_r32_sp_postinc, ldp((W1, W0), postinc(SP, 0xfc)).unwrap(), "ldp w1, w0, [sp], #0xfc";
        test_ldp_r32_sp_postinc_neg, ldp((W1, W0), postinc(SP, -0x100)).unwrap(), "ldp w1, w0, [sp], #-0x100";
        test_ldp_r32_sp_pre_inc, ldp((W1, W0), (inc(0xfc), SP)).unwrap(), "ldp w1, w0, [sp, #0xfc]!";
        test_ldp_r32_sp_pre_inc2, ldp((W1, W0), (inc(LdpStpOffset32::new(0xfc).unwrap()), SP)), "ldp w1, w0, [sp, #0xfc]!";
        test_ldp_r32_sp_pre_inc_neg, ldp((W1, W0), (inc(-0x100), SP)).unwrap(), "ldp w1, w0, [sp, #-0x100]!";
        test_ldp_r32_sp_preinc, ldp((W1, W0), preinc(SP, 0xfc)).unwrap(), "ldp w1, w0, [sp, #0xfc]!";
        test_ldp_r32_sp_preinc2, ldp((W1, W0), preinc(SP, LdpStpOffset32::new(0xfc).unwrap())), "ldp w1, w0, [sp, #0xfc]!";
        test_ldp_r32_sp_preinc_neg, ldp((W1, W0), preinc(SP, -0x100)).unwrap(), "ldp w1, w0, [sp, #-0x100]!";
        test_ldp_r64_r64_post_inc, ldp((X1, X0), (X2, inc(0x1f8))).unwrap(), "ldp x1, x0, [x2], #0x1f8";
        test_ldp_r64_r64_post_inc_neg, ldp((X1, X0), (X2, inc(-0x200))).unwrap(), "ldp x1, x0, [x2], #-0x200";
        test_ldp_r64_r64_postinc, ldp((X1, X0), postinc(X2, 0x1f8)).unwrap(), "ldp x1, x0, [x2], #0x1f8";
        test_ldp_r64_r64_postinc_neg, ldp((X1, X0), postinc(X2, -0x200)).unwrap(), "ldp x1, x0, [x2], #-0x200";
        test_ldp_r64_r64_pre_inc, ldp((X1, X0), (inc(0x1f8), X2)).unwrap(), "ldp x1, x0, [x2, #0x1f8]!";
        test_ldp_r64_r64_pre_inc_neg, ldp((X1, X0), (inc(-0x200), X2)).unwrap(), "ldp x1, x0, [x2, #-0x200]!";
        test_ldp_r64_r64_pre_inc_neg2, ldp((X1, X0), (inc(LdpStpOffset64::new(-0x200).unwrap()), X2)), "ldp x1, x0, [x2, #-0x200]!";
        test_ldp_r64_r64_preinc, ldp((X1, X0), preinc(X2, 0x1f8)).unwrap(), "ldp x1, x0, [x2, #0x1f8]!";
        test_ldp_r64_r64_preinc_neg, ldp((X1, X0), preinc(X2, -0x200)).unwrap(), "ldp x1, x0, [x2, #-0x200]!";
        test_ldp_r64_r64_preinc_neg2, ldp((X1, X0), preinc(X2, LdpStpOffset64::new(-0x200).unwrap())), "ldp x1, x0, [x2, #-0x200]!";
        test_ldp_r64_sp_post_inc, ldp((X1, X0), (SP, inc(0x1f8))).unwrap(), "ldp x1, x0, [sp], #0x1f8";
        test_ldp_r64_sp_post_inc_neg, ldp((X1, X0), (SP, inc(-0x200))).unwrap(), "ldp x1, x0, [sp], #-0x200";
        test_ldp_r64_sp_postinc, ldp((X1, X0), postinc(SP, 0x1f8)).unwrap(), "ldp x1, x0, [sp], #0x1f8";
        test_ldp_r64_sp_postinc_neg, ldp((X1, X0), postinc(SP, -0x200)).unwrap(), "ldp x1, x0, [sp], #-0x200";
        test_ldp_r64_sp_pre_inc, ldp((X1, X0), (inc(0x1f8), SP)).unwrap(), "ldp x1, x0, [sp, #0x1f8]!";
        test_ldp_r64_sp_pre_inc_neg, ldp((X1, X0), (inc(-0x200), SP)).unwrap(), "ldp x1, x0, [sp, #-0x200]!";
        test_ldp_r64_sp_preinc, ldp((X1, X0), preinc(SP, 0x1f8)).unwrap(), "ldp x1, x0, [sp, #0x1f8]!";
        test_ldp_r64_sp_preinc_neg, ldp((X1, X0), preinc(SP, -0x200)).unwrap(), "ldp x1, x0, [sp, #-0x200]!";
        test_ldp_wzr_r64_post_inc, ldp((WZR, W0), (X2, inc(0xfc))).unwrap(), "ldp wzr, w0, [x2], #0xfc";
        test_ldp_wzr_r64_pre_inc, ldp((WZR, W0), (inc(0xfc), X2)).unwrap(), "ldp wzr, w0, [x2, #0xfc]!";
        test_ldp_xzr_r64_post_inc, ldp((XZR, X0), (X2, inc(0x1f8))).unwrap(), "ldp xzr, x0, [x2], #0x1f8";
        test_ldp_xzr_r64_pre_inc, ldp((XZR, X0), (inc(0x1f8), X2)).unwrap(), "ldp xzr, x0, [x2, #0x1f8]!";
    }

    #[test]
    fn test_ldp_r64_offset_underflow() {
        assert!(ldp((X1, X2), (X3, -0x1fci32)).is_err());
        assert!(ldp((X1, X2), (X3, -0x200i32)).is_ok());
        assert!(ldp((X1, X2), (X3, -0x204i32)).is_err());
        assert!(ldp((X1, X2), (X3, -0x208i32)).is_err());
    }

    #[test]
    fn test_ldp_r32_offset_underflow() {
        assert!(ldp((W1, W2), (X3, -0xfei32)).is_err());
        assert!(ldp((W1, W2), (X3, -0x100i32)).is_ok());
        assert!(ldp((W1, W2), (X3, -0x102i32)).is_err());
        assert!(ldp((W1, W2), (X3, -0x104i32)).is_err());
    }

    #[test]
    fn test_ldp_r64_offset_overflow() {
        assert!(ldp((X1, X2), (X3, 0x1f4i32)).is_err());
        assert!(ldp((X1, X2), (X3, 0x1f8i32)).is_ok());
        assert!(ldp((X1, X2), (X3, 0x1fci32)).is_err());
        assert!(ldp((X1, X2), (X3, 0x200i32)).is_err());
    }

    #[test]
    fn test_ldp_r32_offset_overflow() {
        assert!(ldp((W1, W2), (X3, 0xfai32)).is_err());
        assert!(ldp((W1, W2), (X3, 0xfci32)).is_ok());
        assert!(ldp((W1, W2), (X3, 0xfei32)).is_err());
        assert!(ldp((W1, W2), (X3, 0x100i32)).is_err());
    }
}
