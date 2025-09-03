/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

use aarchmrs_instructions::A64::ldst::{
    ldstpair_off::{
        STP_32_ldstpair_off::STP_32_ldstpair_off, STP_64_ldstpair_off::STP_64_ldstpair_off,
    },
    ldstpair_post::{
        STP_32_ldstpair_post::STP_32_ldstpair_post, STP_64_ldstpair_post::STP_64_ldstpair_post,
    },
    ldstpair_pre::{
        STP_32_ldstpair_pre::STP_32_ldstpair_pre, STP_64_ldstpair_pre::STP_64_ldstpair_pre,
    },
};

use super::{Inc, LdpStpOffset32, LdpStpOffset64};
use crate::bits::BitError;
use crate::register::{IntoCode, RegOrSp64, RegOrZero32, RegOrZero64};

/// A `STP` instruction with a destination and an address.
pub struct Stp<Rt, Addr> {
    rt: (Rt, Rt),
    addr: Addr,
}

impl<Rt, Addr> Stp<Rt, Addr> {
    pub fn rt(&self) -> &(Rt, Rt) {
        &self.rt
    }

    pub fn addr(&self) -> &Addr {
        &self.addr
    }
}

/// Defines possible was to construct a `STP` instruction.
// TODO sealed trait?
pub trait MakeStp<Rt1, Rt2, Addr> {
    /// Allows defining both faillible and infallible constructors.
    type Output;

    fn new(rt: (Rt1, Rt2), addr: Addr) -> Self::Output;
}

pub fn stp<DestInp1, DestInp2, TargetOut, AddrInp, AddrOut>(
    d1: DestInp1,
    d2: DestInp2,
    addr: AddrInp,
) -> <Stp<TargetOut, AddrOut> as MakeStp<DestInp1, DestInp2, AddrInp>>::Output
where
    Stp<TargetOut, AddrOut>: MakeStp<DestInp1, DestInp2, AddrInp>,
{
    Stp::new((d1, d2), addr)
}

define_pair_imm_offset_rules!(Stp, MakeStp, STP, RegOrZero32, "32", LdpStpOffset32);
define_pair_imm_offset_rules!(Stp, MakeStp, STP, RegOrZero64, "64", LdpStpOffset64);
define_pair_fallible_rules!(STP, Stp, MakeStp);

#[cfg(test)]
mod tests {
    use harm_test_utils::test_cases;

    use super::*;
    use crate::instructions::InstructionSeq;
    use crate::instructions::ldst::{inc, postinc, preinc};
    use crate::register::Reg32::*;
    use crate::register::Reg64::*;
    use RegOrSp64::SP;
    use RegOrZero32::WZR;
    use RegOrZero64::XZR;

    const STP_DB: &str = "
a93f8c41	stp x1, x3, [x2, -8]
a9008c41	stp x1, x3, [x2, 8]
a91f8c41	stp x1, x3, [x2, 504]
a9300c41	stp x1, x3, [x2, -256]
a9000c41	stp x1, x3, [x2, 0]
a93f8fe1	stp x1, x3, [sp, -8]
a9008fe1	stp x1, x3, [sp, 8]
a91f8fe1	stp x1, x3, [sp, 504]
a9300fe1	stp x1, x3, [sp, -256]
a9000fe1	stp x1, x3, [sp, 0]
293f8c41	stp w1, w3, [x2, -4]
29008c41	stp w1, w3, [x2, 4]
291f8c41	stp w1, w3, [x2, 252]
29200c41	stp w1, w3, [x2, -256]
29000c41	stp w1, w3, [x2, 0]
293f8fe1	stp w1, w3, [sp, -4]
29008fe1	stp w1, w3, [sp, 4]
291f8fe1	stp w1, w3, [sp, 252]
29200fe1	stp w1, w3, [sp, -256]
29000fe1	stp w1, w3, [sp, 0]
a93f8c5f	stp xzr, x3, [x2, -8]
a9008c5f	stp xzr, x3, [x2, 8]
a91f8c5f	stp xzr, x3, [x2, 504]
a9300c5f	stp xzr, x3, [x2, -256]
a9000c5f	stp xzr, x3, [x2, 0]
a93f8fff	stp xzr, x3, [sp, -8]
a9008fff	stp xzr, x3, [sp, 8]
a91f8fff	stp xzr, x3, [sp, 504]
a9300fff	stp xzr, x3, [sp, -256]
a9000fff	stp xzr, x3, [sp, 0]
293f8c5f	stp wzr, w3, [x2, -4]
29008c5f	stp wzr, w3, [x2, 4]
291f8c5f	stp wzr, w3, [x2, 252]
29200c5f	stp wzr, w3, [x2, -256]
29000c5f	stp wzr, w3, [x2, 0]
293f8fff	stp wzr, w3, [sp, -4]
29008fff	stp wzr, w3, [sp, 4]
291f8fff	stp wzr, w3, [sp, 252]
29200fff	stp wzr, w3, [sp, -256]
29000fff	stp wzr, w3, [sp, 0]
a9000c41	stp x1, x3, [x2]
";

    const STP_PRE_POST_INC_DB: &str = "
289f8041	stp w1, w0, [x2], #0xfc
289f805f	stp wzr, w0, [x2], #0xfc
289f83e1	stp w1, w0, [sp], #0xfc
299f8041	stp w1, w0, [x2, #0xfc]!
299f805f	stp wzr, w0, [x2, #0xfc]!
299f83e1	stp w1, w0, [sp, #0xfc]!
28a00041	stp w1, w0, [x2], #-0x100
28a003e1	stp w1, w0, [sp], #-0x100
29a00041	stp w1, w0, [x2, #-0x100]!
29a003e1	stp w1, w0, [sp, #-0x100]!
a89f8041	stp x1, x0, [x2], #0x1f8
a89f805f	stp xzr, x0, [x2], #0x1f8
a89f83e1	stp x1, x0, [sp], #0x1f8
a99f8041	stp x1, x0, [x2, #0x1f8]!
a99f805f	stp xzr, x0, [x2, #0x1f8]!
a99f83e1	stp x1, x0, [sp, #0x1f8]!
a8a00041	stp x1, x0, [x2], #-0x200
a8a003e1	stp x1, x0, [sp], #-0x200
a9a00041	stp x1, x0, [x2, #-0x200]!
a9a003e1	stp x1, x0, [sp, #-0x200]!
";

    test_cases! {
        STP_DB, untested_stp_cases;
        test_stp_x1_x2_m8, stp(X1, X3, (X2, -8i32)).unwrap(), "stp x1, x3, [x2, -8]";
        test_stp_x1_x2_8, stp(X1, X3, (X2, 8i32)).unwrap(), "stp x1, x3, [x2, 8]";
        test_stp_x1_x2_504, stp(X1, X3, (X2, 504i32)).unwrap(), "stp x1, x3, [x2, 504]";
        test_stp_x1_x2_m256, stp(X1, X3, (X2, -256i32)).unwrap(), "stp x1, x3, [x2, -256]";
        test_stp_x1_x2_0, stp(X1, X3, (X2, 0i32)).unwrap(), "stp x1, x3, [x2, 0]";
        test_stp_x1_x2_simple, stp(X1, X3, (X2,)), "stp x1, x3, [x2]";
        test_stp_x1_sp_m8, stp(X1, X3, (SP, -8i32)).unwrap(), "stp x1, x3, [sp, -8]";
        test_stp_x1_sp_8, stp(X1, X3, (SP, 8i32)).unwrap(), "stp x1, x3, [sp, 8]";
        test_stp_x1_sp_504, stp(X1, X3, (SP, 504i32)).unwrap(), "stp x1, x3, [sp, 504]";
        test_stp_x1_sp_m256, stp(X1, X3, (SP, -256i32)).unwrap(), "stp x1, x3, [sp, -256]";
        test_stp_x1_sp_0, stp(X1, X3, (SP, 0i32)).unwrap(), "stp x1, x3, [sp, 0]";
        test_stp_w1_x2_m4, stp(W1, W3, (X2, -4i32)).unwrap(), "stp w1, w3, [x2, -4]";
        test_stp_w1_x2_4, stp(W1, W3, (X2, 4i32)).unwrap(), "stp w1, w3, [x2, 4]";
        test_stp_w1_x2_252, stp(W1, W3, (X2, 252i32)).unwrap(), "stp w1, w3, [x2, 252]";
        test_stp_w1_x2_m256, stp(W1, W3, (X2, -256i32)).unwrap(), "stp w1, w3, [x2, -256]";
        test_stp_w1_x2_0, stp(W1, W3, (X2, 0i32)).unwrap(), "stp w1, w3, [x2, 0]";
        test_stp_w1_sp_m4, stp(W1, W3, (SP, -4i32)).unwrap(), "stp w1, w3, [sp, -4]";
        test_stp_w1_sp_4, stp(W1, W3, (SP, 4i32)).unwrap(), "stp w1, w3, [sp, 4]";
        test_stp_w1_sp_252, stp(W1, W3, (SP, 252i32)).unwrap(), "stp w1, w3, [sp, 252]";
        test_stp_w1_sp_m256, stp(W1, W3, (SP, -256i32)).unwrap(), "stp w1, w3, [sp, -256]";
        test_stp_w1_sp_0, stp(W1, W3, (SP, 0i32)).unwrap(), "stp w1, w3, [sp, 0]";
        test_stp_xzr_x2_m8, stp(XZR, X3, (X2, -8i32)).unwrap(), "stp xzr, x3, [x2, -8]";
        test_stp_xzr_x2_8, stp(XZR, X3, (X2, 8i32)).unwrap(), "stp xzr, x3, [x2, 8]";
        test_stp_xzr_x2_504, stp(XZR, X3, (X2, 504i32)).unwrap(), "stp xzr, x3, [x2, 504]";
        test_stp_xzr_x2_m256, stp(XZR, X3, (X2, -256i32)).unwrap(), "stp xzr, x3, [x2, -256]";
        test_stp_xzr_x2_0, stp(XZR, X3, (X2, 0i32)).unwrap(), "stp xzr, x3, [x2, 0]";
        test_stp_xzr_sp_m8, stp(XZR, X3, (SP, -8i32)).unwrap(), "stp xzr, x3, [sp, -8]";
        test_stp_xzr_sp_8, stp(XZR, X3, (SP, 8i32)).unwrap(), "stp xzr, x3, [sp, 8]";
        test_stp_xzr_sp_504, stp(XZR, X3, (SP, 504i32)).unwrap(), "stp xzr, x3, [sp, 504]";
        test_stp_xzr_sp_m256, stp(XZR, X3, (SP, -256i32)).unwrap(), "stp xzr, x3, [sp, -256]";
        test_stp_xzr_sp_0, stp(XZR, X3, (SP, 0i32)).unwrap(), "stp xzr, x3, [sp, 0]";
        test_stp_wzr_x2_m4, stp(WZR, W3, (X2, -4i32)).unwrap(), "stp wzr, w3, [x2, -4]";
        test_stp_wzr_x2_4, stp(WZR, W3, (X2, 4i32)).unwrap(), "stp wzr, w3, [x2, 4]";
        test_stp_wzr_x2_252, stp(WZR, W3, (X2, 252i32)).unwrap(), "stp wzr, w3, [x2, 252]";
        test_stp_wzr_x2_m256, stp(WZR, W3, (X2, -256i32)).unwrap(), "stp wzr, w3, [x2, -256]";
        test_stp_wzr_x2_0, stp(WZR, W3, (X2, 0i32)).unwrap(), "stp wzr, w3, [x2, 0]";
        test_stp_wzr_sp_m4, stp(WZR, W3, (SP, -4i32)).unwrap(), "stp wzr, w3, [sp, -4]";
        test_stp_wzr_sp_4, stp(WZR, W3, (SP, 4i32)).unwrap(), "stp wzr, w3, [sp, 4]";
        test_stp_wzr_sp_252, stp(WZR, W3, (SP, 252i32)).unwrap(), "stp wzr, w3, [sp, 252]";
        test_stp_wzr_sp_m256, stp(WZR, W3, (SP, -256i32)).unwrap(), "stp wzr, w3, [sp, -256]";
        test_stp_wzr_sp_0, stp(WZR, W3, (SP, 0i32)).unwrap(), "stp wzr, w3, [sp, 0]";
    }

    test_cases! {
        STP_PRE_POST_INC_DB, untested_stp_pre_post_inc;
        test_stp_r32_r64_post_inc, stp(W1, W0, (X2, inc(0xfc))).unwrap(), "stp w1, w0, [x2], #0xfc";
        test_stp_r32_r64_post_inc_neg, stp(W1, W0, (X2, inc(-0x100))).unwrap(), "stp w1, w0, [x2], #-0x100";
        test_stp_r32_r64_postinc, stp(W1, W0, postinc(X2, 0xfc)).unwrap(), "stp w1, w0, [x2], #0xfc";
        test_stp_r32_r64_postinc_neg, stp(W1, W0, postinc(X2, -0x100)).unwrap(), "stp w1, w0, [x2], #-0x100";
        test_stp_r32_r64_pre_inc, stp(W1, W0, (inc(0xfc), X2)).unwrap(), "stp w1, w0, [x2, #0xfc]!";
        test_stp_r32_r64_pre_inc_neg, stp(W1, W0, (inc(-0x100), X2)).unwrap(), "stp w1, w0, [x2, #-0x100]!";
        test_stp_r32_r64_preinc, stp(W1, W0, preinc(X2, 0xfc)).unwrap(), "stp w1, w0, [x2, #0xfc]!";
        test_stp_r32_r64_preinc_neg, stp(W1, W0, preinc(X2, -0x100)).unwrap(), "stp w1, w0, [x2, #-0x100]!";
        test_stp_r32_sp_post_inc, stp(W1, W0, (SP, inc(0xfc))).unwrap(), "stp w1, w0, [sp], #0xfc";
        test_stp_r32_sp_post_inc_neg, stp(W1, W0, (SP, inc(-0x100))).unwrap(), "stp w1, w0, [sp], #-0x100";
        test_stp_r32_sp_postinc, stp(W1, W0, postinc(SP, 0xfc)).unwrap(), "stp w1, w0, [sp], #0xfc";
        test_stp_r32_sp_postinc_neg, stp(W1, W0, postinc(SP, -0x100)).unwrap(), "stp w1, w0, [sp], #-0x100";
        test_stp_r32_sp_pre_inc, stp(W1, W0, (inc(0xfc), SP)).unwrap(), "stp w1, w0, [sp, #0xfc]!";
        test_stp_r32_sp_pre_inc2, stp(W1, W0, (inc(LdpStpOffset32::new(0xfc).unwrap()), SP)), "stp w1, w0, [sp, #0xfc]!";
        test_stp_r32_sp_pre_inc_neg, stp(W1, W0, (inc(-0x100), SP)).unwrap(), "stp w1, w0, [sp, #-0x100]!";
        test_stp_r32_sp_preinc, stp(W1, W0, preinc(SP, 0xfc)).unwrap(), "stp w1, w0, [sp, #0xfc]!";
        test_stp_r32_sp_preinc2, stp(W1, W0, preinc(SP, LdpStpOffset32::new(0xfc).unwrap())), "stp w1, w0, [sp, #0xfc]!";
        test_stp_r32_sp_preinc_neg, stp(W1, W0, preinc(SP, -0x100)).unwrap(), "stp w1, w0, [sp, #-0x100]!";
        test_stp_r64_r64_post_inc, stp(X1, X0, (X2, inc(0x1f8))).unwrap(), "stp x1, x0, [x2], #0x1f8";
        test_stp_r64_r64_post_inc_neg, stp(X1, X0, (X2, inc(-0x200))).unwrap(), "stp x1, x0, [x2], #-0x200";
        test_stp_r64_r64_postinc, stp(X1, X0, postinc(X2, 0x1f8)).unwrap(), "stp x1, x0, [x2], #0x1f8";
        test_stp_r64_r64_postinc_neg, stp(X1, X0, postinc(X2, -0x200)).unwrap(), "stp x1, x0, [x2], #-0x200";
        test_stp_r64_r64_pre_inc, stp(X1, X0, (inc(0x1f8), X2)).unwrap(), "stp x1, x0, [x2, #0x1f8]!";
        test_stp_r64_r64_pre_inc_neg, stp(X1, X0, (inc(-0x200), X2)).unwrap(), "stp x1, x0, [x2, #-0x200]!";
        test_stp_r64_r64_pre_inc_neg2, stp(X1, X0, (inc(LdpStpOffset64::new(-0x200).unwrap()), X2)), "stp x1, x0, [x2, #-0x200]!";
        test_stp_r64_r64_preinc, stp(X1, X0, preinc(X2, 0x1f8)).unwrap(), "stp x1, x0, [x2, #0x1f8]!";
        test_stp_r64_r64_preinc_neg, stp(X1, X0, preinc(X2, -0x200)).unwrap(), "stp x1, x0, [x2, #-0x200]!";
        test_stp_r64_r64_preinc_neg2, stp(X1, X0, preinc(X2, LdpStpOffset64::new(-0x200).unwrap())), "stp x1, x0, [x2, #-0x200]!";
        test_stp_r64_sp_post_inc, stp(X1, X0, (SP, inc(0x1f8))).unwrap(), "stp x1, x0, [sp], #0x1f8";
        test_stp_r64_sp_post_inc_neg, stp(X1, X0, (SP, inc(-0x200))).unwrap(), "stp x1, x0, [sp], #-0x200";
        test_stp_r64_sp_postinc, stp(X1, X0, postinc(SP, 0x1f8)).unwrap(), "stp x1, x0, [sp], #0x1f8";
        test_stp_r64_sp_postinc_neg, stp(X1, X0, postinc(SP, -0x200)).unwrap(), "stp x1, x0, [sp], #-0x200";
        test_stp_r64_sp_pre_inc, stp(X1, X0, (inc(0x1f8), SP)).unwrap(), "stp x1, x0, [sp, #0x1f8]!";
        test_stp_r64_sp_pre_inc_neg, stp(X1, X0, (inc(-0x200), SP)).unwrap(), "stp x1, x0, [sp, #-0x200]!";
        test_stp_r64_sp_preinc, stp(X1, X0, preinc(SP, 0x1f8)).unwrap(), "stp x1, x0, [sp, #0x1f8]!";
        test_stp_r64_sp_preinc_neg, stp(X1, X0, preinc(SP, -0x200)).unwrap(), "stp x1, x0, [sp, #-0x200]!";
        test_stp_wzr_r64_post_inc, stp(WZR, W0, (X2, inc(0xfc))).unwrap(), "stp wzr, w0, [x2], #0xfc";
        test_stp_wzr_r64_pre_inc, stp(WZR, W0, (inc(0xfc), X2)).unwrap(), "stp wzr, w0, [x2, #0xfc]!";
        test_stp_xzr_r64_post_inc, stp(XZR, X0, (X2, inc(0x1f8))).unwrap(), "stp xzr, x0, [x2], #0x1f8";
        test_stp_xzr_r64_pre_inc, stp(XZR, X0, (inc(0x1f8), X2)).unwrap(), "stp xzr, x0, [x2, #0x1f8]!";
    }

    #[test]
    fn test_stp_r64_offset_underflow() {
        assert!(stp(X1, X2, (X3, -0x1fci32)).is_err());
        assert!(stp(X1, X2, (X3, -0x200i32)).is_ok());
        assert!(stp(X1, X2, (X3, -0x204i32)).is_err());
        assert!(stp(X1, X2, (X3, -0x208i32)).is_err());
    }

    #[test]
    fn test_stp_r32_offset_underflow() {
        assert!(stp(W1, W2, (X3, -0xfei32)).is_err());
        assert!(stp(W1, W2, (X3, -0x100i32)).is_ok());
        assert!(stp(W1, W2, (X3, -0x102i32)).is_err());
        assert!(stp(W1, W2, (X3, -0x104i32)).is_err());
    }

    #[test]
    fn test_stp_r64_offset_overflow() {
        assert!(stp(X1, X2, (X3, 0x1f4i32)).is_err());
        assert!(stp(X1, X2, (X3, 0x1f8i32)).is_ok());
        assert!(stp(X1, X2, (X3, 0x1fci32)).is_err());
        assert!(stp(X1, X2, (X3, 0x200i32)).is_err());
    }

    #[test]
    fn test_stp_r32_offset_overflow() {
        assert!(stp(W1, W2, (X3, 0xfai32)).is_err());
        assert!(stp(W1, W2, (X3, 0xfci32)).is_ok());
        assert!(stp(W1, W2, (X3, 0xfei32)).is_err());
        assert!(stp(W1, W2, (X3, 0x100i32)).is_err());
    }
}
