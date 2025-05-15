/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod SSHR_asimdshf_R {
    #[inline]
    pub fn SSHR_asimdshf_R(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        immh: impl Into<::aarchmrs_types::BitValue<4>>,
        immb: impl Into<::aarchmrs_types::BitValue<3>>,
        o1: impl Into<::aarchmrs_types::BitValue<1>>,
        o0: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b0011110u32 << 23u32
                | u32::from(immh.into()) << 19u32
                | u32::from(immb.into()) << 16u32
                | 0b00u32 << 14u32
                | u32::from(o1.into()) << 13u32
                | u32::from(o0.into()) << 12u32
                | 0b01u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SSRA_asimdshf_R {
    #[inline]
    pub fn SSRA_asimdshf_R(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        immh: impl Into<::aarchmrs_types::BitValue<4>>,
        immb: impl Into<::aarchmrs_types::BitValue<3>>,
        o1: impl Into<::aarchmrs_types::BitValue<1>>,
        o0: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b0011110u32 << 23u32
                | u32::from(immh.into()) << 19u32
                | u32::from(immb.into()) << 16u32
                | 0b00u32 << 14u32
                | u32::from(o1.into()) << 13u32
                | u32::from(o0.into()) << 12u32
                | 0b01u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SRSHR_asimdshf_R {
    #[inline]
    pub fn SRSHR_asimdshf_R(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        immh: impl Into<::aarchmrs_types::BitValue<4>>,
        immb: impl Into<::aarchmrs_types::BitValue<3>>,
        o1: impl Into<::aarchmrs_types::BitValue<1>>,
        o0: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b0011110u32 << 23u32
                | u32::from(immh.into()) << 19u32
                | u32::from(immb.into()) << 16u32
                | 0b00u32 << 14u32
                | u32::from(o1.into()) << 13u32
                | u32::from(o0.into()) << 12u32
                | 0b01u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SRSRA_asimdshf_R {
    #[inline]
    pub fn SRSRA_asimdshf_R(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        immh: impl Into<::aarchmrs_types::BitValue<4>>,
        immb: impl Into<::aarchmrs_types::BitValue<3>>,
        o1: impl Into<::aarchmrs_types::BitValue<1>>,
        o0: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b0011110u32 << 23u32
                | u32::from(immh.into()) << 19u32
                | u32::from(immb.into()) << 16u32
                | 0b00u32 << 14u32
                | u32::from(o1.into()) << 13u32
                | u32::from(o0.into()) << 12u32
                | 0b01u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SHL_asimdshf_R {
    #[inline]
    pub fn SHL_asimdshf_R(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        immh: impl Into<::aarchmrs_types::BitValue<4>>,
        immb: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b0011110u32 << 23u32
                | u32::from(immh.into()) << 19u32
                | u32::from(immb.into()) << 16u32
                | 0b010101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SQSHL_asimdshf_R {
    #[inline]
    pub fn SQSHL_asimdshf_R(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        immh: impl Into<::aarchmrs_types::BitValue<4>>,
        immb: impl Into<::aarchmrs_types::BitValue<3>>,
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b0011110u32 << 23u32
                | u32::from(immh.into()) << 19u32
                | u32::from(immb.into()) << 16u32
                | 0b011u32 << 13u32
                | u32::from(op.into()) << 12u32
                | 0b01u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SHRN_asimdshf_N {
    #[inline]
    pub fn SHRN_asimdshf_N(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        immh: impl Into<::aarchmrs_types::BitValue<4>>,
        immb: impl Into<::aarchmrs_types::BitValue<3>>,
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b0011110u32 << 23u32
                | u32::from(immh.into()) << 19u32
                | u32::from(immb.into()) << 16u32
                | 0b1000u32 << 12u32
                | u32::from(op.into()) << 11u32
                | 0b1u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod RSHRN_asimdshf_N {
    #[inline]
    pub fn RSHRN_asimdshf_N(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        immh: impl Into<::aarchmrs_types::BitValue<4>>,
        immb: impl Into<::aarchmrs_types::BitValue<3>>,
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b0011110u32 << 23u32
                | u32::from(immh.into()) << 19u32
                | u32::from(immb.into()) << 16u32
                | 0b1000u32 << 12u32
                | u32::from(op.into()) << 11u32
                | 0b1u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SQSHRN_asimdshf_N {
    #[inline]
    pub fn SQSHRN_asimdshf_N(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        immh: impl Into<::aarchmrs_types::BitValue<4>>,
        immb: impl Into<::aarchmrs_types::BitValue<3>>,
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b0011110u32 << 23u32
                | u32::from(immh.into()) << 19u32
                | u32::from(immb.into()) << 16u32
                | 0b1001u32 << 12u32
                | u32::from(op.into()) << 11u32
                | 0b1u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SQRSHRN_asimdshf_N {
    #[inline]
    pub fn SQRSHRN_asimdshf_N(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        immh: impl Into<::aarchmrs_types::BitValue<4>>,
        immb: impl Into<::aarchmrs_types::BitValue<3>>,
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b0011110u32 << 23u32
                | u32::from(immh.into()) << 19u32
                | u32::from(immb.into()) << 16u32
                | 0b1001u32 << 12u32
                | u32::from(op.into()) << 11u32
                | 0b1u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SSHLL_asimdshf_L {
    #[inline]
    pub fn SSHLL_asimdshf_L(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        immh: impl Into<::aarchmrs_types::BitValue<4>>,
        immb: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b0011110u32 << 23u32
                | u32::from(immh.into()) << 19u32
                | u32::from(immb.into()) << 16u32
                | 0b101001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SCVTF_asimdshf_C {
    #[inline]
    pub fn SCVTF_asimdshf_C(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        immh: impl Into<::aarchmrs_types::BitValue<4>>,
        immb: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b0011110u32 << 23u32
                | u32::from(immh.into()) << 19u32
                | u32::from(immb.into()) << 16u32
                | 0b111001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FCVTZS_asimdshf_C {
    #[inline]
    pub fn FCVTZS_asimdshf_C(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        immh: impl Into<::aarchmrs_types::BitValue<4>>,
        immb: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b0011110u32 << 23u32
                | u32::from(immh.into()) << 19u32
                | u32::from(immb.into()) << 16u32
                | 0b111111u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod USHR_asimdshf_R {
    #[inline]
    pub fn USHR_asimdshf_R(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        immh: impl Into<::aarchmrs_types::BitValue<4>>,
        immb: impl Into<::aarchmrs_types::BitValue<3>>,
        o1: impl Into<::aarchmrs_types::BitValue<1>>,
        o0: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b1011110u32 << 23u32
                | u32::from(immh.into()) << 19u32
                | u32::from(immb.into()) << 16u32
                | 0b00u32 << 14u32
                | u32::from(o1.into()) << 13u32
                | u32::from(o0.into()) << 12u32
                | 0b01u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod USRA_asimdshf_R {
    #[inline]
    pub fn USRA_asimdshf_R(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        immh: impl Into<::aarchmrs_types::BitValue<4>>,
        immb: impl Into<::aarchmrs_types::BitValue<3>>,
        o1: impl Into<::aarchmrs_types::BitValue<1>>,
        o0: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b1011110u32 << 23u32
                | u32::from(immh.into()) << 19u32
                | u32::from(immb.into()) << 16u32
                | 0b00u32 << 14u32
                | u32::from(o1.into()) << 13u32
                | u32::from(o0.into()) << 12u32
                | 0b01u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod URSHR_asimdshf_R {
    #[inline]
    pub fn URSHR_asimdshf_R(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        immh: impl Into<::aarchmrs_types::BitValue<4>>,
        immb: impl Into<::aarchmrs_types::BitValue<3>>,
        o1: impl Into<::aarchmrs_types::BitValue<1>>,
        o0: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b1011110u32 << 23u32
                | u32::from(immh.into()) << 19u32
                | u32::from(immb.into()) << 16u32
                | 0b00u32 << 14u32
                | u32::from(o1.into()) << 13u32
                | u32::from(o0.into()) << 12u32
                | 0b01u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod URSRA_asimdshf_R {
    #[inline]
    pub fn URSRA_asimdshf_R(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        immh: impl Into<::aarchmrs_types::BitValue<4>>,
        immb: impl Into<::aarchmrs_types::BitValue<3>>,
        o1: impl Into<::aarchmrs_types::BitValue<1>>,
        o0: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b1011110u32 << 23u32
                | u32::from(immh.into()) << 19u32
                | u32::from(immb.into()) << 16u32
                | 0b00u32 << 14u32
                | u32::from(o1.into()) << 13u32
                | u32::from(o0.into()) << 12u32
                | 0b01u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SRI_asimdshf_R {
    #[inline]
    pub fn SRI_asimdshf_R(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        immh: impl Into<::aarchmrs_types::BitValue<4>>,
        immb: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b1011110u32 << 23u32
                | u32::from(immh.into()) << 19u32
                | u32::from(immb.into()) << 16u32
                | 0b010001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SLI_asimdshf_R {
    #[inline]
    pub fn SLI_asimdshf_R(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        immh: impl Into<::aarchmrs_types::BitValue<4>>,
        immb: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b1011110u32 << 23u32
                | u32::from(immh.into()) << 19u32
                | u32::from(immb.into()) << 16u32
                | 0b010101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SQSHLU_asimdshf_R {
    #[inline]
    pub fn SQSHLU_asimdshf_R(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        immh: impl Into<::aarchmrs_types::BitValue<4>>,
        immb: impl Into<::aarchmrs_types::BitValue<3>>,
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b1011110u32 << 23u32
                | u32::from(immh.into()) << 19u32
                | u32::from(immb.into()) << 16u32
                | 0b011u32 << 13u32
                | u32::from(op.into()) << 12u32
                | 0b01u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod UQSHL_asimdshf_R {
    #[inline]
    pub fn UQSHL_asimdshf_R(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        immh: impl Into<::aarchmrs_types::BitValue<4>>,
        immb: impl Into<::aarchmrs_types::BitValue<3>>,
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b1011110u32 << 23u32
                | u32::from(immh.into()) << 19u32
                | u32::from(immb.into()) << 16u32
                | 0b011u32 << 13u32
                | u32::from(op.into()) << 12u32
                | 0b01u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SQSHRUN_asimdshf_N {
    #[inline]
    pub fn SQSHRUN_asimdshf_N(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        immh: impl Into<::aarchmrs_types::BitValue<4>>,
        immb: impl Into<::aarchmrs_types::BitValue<3>>,
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b1011110u32 << 23u32
                | u32::from(immh.into()) << 19u32
                | u32::from(immb.into()) << 16u32
                | 0b1000u32 << 12u32
                | u32::from(op.into()) << 11u32
                | 0b1u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SQRSHRUN_asimdshf_N {
    #[inline]
    pub fn SQRSHRUN_asimdshf_N(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        immh: impl Into<::aarchmrs_types::BitValue<4>>,
        immb: impl Into<::aarchmrs_types::BitValue<3>>,
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b1011110u32 << 23u32
                | u32::from(immh.into()) << 19u32
                | u32::from(immb.into()) << 16u32
                | 0b1000u32 << 12u32
                | u32::from(op.into()) << 11u32
                | 0b1u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod UQSHRN_asimdshf_N {
    #[inline]
    pub fn UQSHRN_asimdshf_N(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        immh: impl Into<::aarchmrs_types::BitValue<4>>,
        immb: impl Into<::aarchmrs_types::BitValue<3>>,
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b1011110u32 << 23u32
                | u32::from(immh.into()) << 19u32
                | u32::from(immb.into()) << 16u32
                | 0b1001u32 << 12u32
                | u32::from(op.into()) << 11u32
                | 0b1u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod UQRSHRN_asimdshf_N {
    #[inline]
    pub fn UQRSHRN_asimdshf_N(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        immh: impl Into<::aarchmrs_types::BitValue<4>>,
        immb: impl Into<::aarchmrs_types::BitValue<3>>,
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b1011110u32 << 23u32
                | u32::from(immh.into()) << 19u32
                | u32::from(immb.into()) << 16u32
                | 0b1001u32 << 12u32
                | u32::from(op.into()) << 11u32
                | 0b1u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod USHLL_asimdshf_L {
    #[inline]
    pub fn USHLL_asimdshf_L(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        immh: impl Into<::aarchmrs_types::BitValue<4>>,
        immb: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b1011110u32 << 23u32
                | u32::from(immh.into()) << 19u32
                | u32::from(immb.into()) << 16u32
                | 0b101001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod UCVTF_asimdshf_C {
    #[inline]
    pub fn UCVTF_asimdshf_C(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        immh: impl Into<::aarchmrs_types::BitValue<4>>,
        immb: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b1011110u32 << 23u32
                | u32::from(immh.into()) << 19u32
                | u32::from(immb.into()) << 16u32
                | 0b111001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FCVTZU_asimdshf_C {
    #[inline]
    pub fn FCVTZU_asimdshf_C(
        Q: impl Into<::aarchmrs_types::BitValue<1>>,
        immh: impl Into<::aarchmrs_types::BitValue<4>>,
        immb: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | u32::from(Q.into()) << 30u32
                | 0b1011110u32 << 23u32
                | u32::from(immh.into()) << 19u32
                | u32::from(immb.into()) << 16u32
                | 0b111111u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
