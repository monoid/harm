/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod SSHR_asisdshf_R {
    #[inline]
    pub fn SSHR_asisdshf_R(
        immh: impl Into<::aarchmrs_types::BitValue<4>>,
        immb: impl Into<::aarchmrs_types::BitValue<3>>,
        o1: impl Into<::aarchmrs_types::BitValue<1>>,
        o0: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010111110u32 << 23u32
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
pub mod SSRA_asisdshf_R {
    #[inline]
    pub fn SSRA_asisdshf_R(
        immh: impl Into<::aarchmrs_types::BitValue<4>>,
        immb: impl Into<::aarchmrs_types::BitValue<3>>,
        o1: impl Into<::aarchmrs_types::BitValue<1>>,
        o0: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010111110u32 << 23u32
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
pub mod SRSHR_asisdshf_R {
    #[inline]
    pub fn SRSHR_asisdshf_R(
        immh: impl Into<::aarchmrs_types::BitValue<4>>,
        immb: impl Into<::aarchmrs_types::BitValue<3>>,
        o1: impl Into<::aarchmrs_types::BitValue<1>>,
        o0: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010111110u32 << 23u32
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
pub mod SRSRA_asisdshf_R {
    #[inline]
    pub fn SRSRA_asisdshf_R(
        immh: impl Into<::aarchmrs_types::BitValue<4>>,
        immb: impl Into<::aarchmrs_types::BitValue<3>>,
        o1: impl Into<::aarchmrs_types::BitValue<1>>,
        o0: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010111110u32 << 23u32
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
pub mod SHL_asisdshf_R {
    #[inline]
    pub fn SHL_asisdshf_R(
        immh: impl Into<::aarchmrs_types::BitValue<4>>,
        immb: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010111110u32 << 23u32
                | u32::from(immh.into()) << 19u32
                | u32::from(immb.into()) << 16u32
                | 0b010101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SQSHL_asisdshf_R {
    #[inline]
    pub fn SQSHL_asisdshf_R(
        immh: impl Into<::aarchmrs_types::BitValue<4>>,
        immb: impl Into<::aarchmrs_types::BitValue<3>>,
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010111110u32 << 23u32
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
pub mod SQSHRN_asisdshf_N {
    #[inline]
    pub fn SQSHRN_asisdshf_N(
        immh: impl Into<::aarchmrs_types::BitValue<4>>,
        immb: impl Into<::aarchmrs_types::BitValue<3>>,
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010111110u32 << 23u32
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
pub mod SQRSHRN_asisdshf_N {
    #[inline]
    pub fn SQRSHRN_asisdshf_N(
        immh: impl Into<::aarchmrs_types::BitValue<4>>,
        immb: impl Into<::aarchmrs_types::BitValue<3>>,
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010111110u32 << 23u32
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
pub mod SCVTF_asisdshf_C {
    #[inline]
    pub fn SCVTF_asisdshf_C(
        immh: impl Into<::aarchmrs_types::BitValue<4>>,
        immb: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010111110u32 << 23u32
                | u32::from(immh.into()) << 19u32
                | u32::from(immb.into()) << 16u32
                | 0b111001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FCVTZS_asisdshf_C {
    #[inline]
    pub fn FCVTZS_asisdshf_C(
        immh: impl Into<::aarchmrs_types::BitValue<4>>,
        immb: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010111110u32 << 23u32
                | u32::from(immh.into()) << 19u32
                | u32::from(immb.into()) << 16u32
                | 0b111111u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod USHR_asisdshf_R {
    #[inline]
    pub fn USHR_asisdshf_R(
        immh: impl Into<::aarchmrs_types::BitValue<4>>,
        immb: impl Into<::aarchmrs_types::BitValue<3>>,
        o1: impl Into<::aarchmrs_types::BitValue<1>>,
        o0: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011111110u32 << 23u32
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
pub mod USRA_asisdshf_R {
    #[inline]
    pub fn USRA_asisdshf_R(
        immh: impl Into<::aarchmrs_types::BitValue<4>>,
        immb: impl Into<::aarchmrs_types::BitValue<3>>,
        o1: impl Into<::aarchmrs_types::BitValue<1>>,
        o0: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011111110u32 << 23u32
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
pub mod URSHR_asisdshf_R {
    #[inline]
    pub fn URSHR_asisdshf_R(
        immh: impl Into<::aarchmrs_types::BitValue<4>>,
        immb: impl Into<::aarchmrs_types::BitValue<3>>,
        o1: impl Into<::aarchmrs_types::BitValue<1>>,
        o0: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011111110u32 << 23u32
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
pub mod URSRA_asisdshf_R {
    #[inline]
    pub fn URSRA_asisdshf_R(
        immh: impl Into<::aarchmrs_types::BitValue<4>>,
        immb: impl Into<::aarchmrs_types::BitValue<3>>,
        o1: impl Into<::aarchmrs_types::BitValue<1>>,
        o0: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011111110u32 << 23u32
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
pub mod SRI_asisdshf_R {
    #[inline]
    pub fn SRI_asisdshf_R(
        immh: impl Into<::aarchmrs_types::BitValue<4>>,
        immb: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011111110u32 << 23u32
                | u32::from(immh.into()) << 19u32
                | u32::from(immb.into()) << 16u32
                | 0b010001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SLI_asisdshf_R {
    #[inline]
    pub fn SLI_asisdshf_R(
        immh: impl Into<::aarchmrs_types::BitValue<4>>,
        immb: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011111110u32 << 23u32
                | u32::from(immh.into()) << 19u32
                | u32::from(immb.into()) << 16u32
                | 0b010101u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SQSHLU_asisdshf_R {
    #[inline]
    pub fn SQSHLU_asisdshf_R(
        immh: impl Into<::aarchmrs_types::BitValue<4>>,
        immb: impl Into<::aarchmrs_types::BitValue<3>>,
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011111110u32 << 23u32
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
pub mod UQSHL_asisdshf_R {
    #[inline]
    pub fn UQSHL_asisdshf_R(
        immh: impl Into<::aarchmrs_types::BitValue<4>>,
        immb: impl Into<::aarchmrs_types::BitValue<3>>,
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011111110u32 << 23u32
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
pub mod SQSHRUN_asisdshf_N {
    #[inline]
    pub fn SQSHRUN_asisdshf_N(
        immh: impl Into<::aarchmrs_types::BitValue<4>>,
        immb: impl Into<::aarchmrs_types::BitValue<3>>,
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011111110u32 << 23u32
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
pub mod SQRSHRUN_asisdshf_N {
    #[inline]
    pub fn SQRSHRUN_asisdshf_N(
        immh: impl Into<::aarchmrs_types::BitValue<4>>,
        immb: impl Into<::aarchmrs_types::BitValue<3>>,
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011111110u32 << 23u32
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
pub mod UQSHRN_asisdshf_N {
    #[inline]
    pub fn UQSHRN_asisdshf_N(
        immh: impl Into<::aarchmrs_types::BitValue<4>>,
        immb: impl Into<::aarchmrs_types::BitValue<3>>,
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011111110u32 << 23u32
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
pub mod UQRSHRN_asisdshf_N {
    #[inline]
    pub fn UQRSHRN_asisdshf_N(
        immh: impl Into<::aarchmrs_types::BitValue<4>>,
        immb: impl Into<::aarchmrs_types::BitValue<3>>,
        op: impl Into<::aarchmrs_types::BitValue<1>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011111110u32 << 23u32
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
pub mod UCVTF_asisdshf_C {
    #[inline]
    pub fn UCVTF_asisdshf_C(
        immh: impl Into<::aarchmrs_types::BitValue<4>>,
        immb: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011111110u32 << 23u32
                | u32::from(immh.into()) << 19u32
                | u32::from(immb.into()) << 16u32
                | 0b111001u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod FCVTZU_asisdshf_C {
    #[inline]
    pub fn FCVTZU_asisdshf_C(
        immh: impl Into<::aarchmrs_types::BitValue<4>>,
        immb: impl Into<::aarchmrs_types::BitValue<3>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011111110u32 << 23u32
                | u32::from(immh.into()) << 19u32
                | u32::from(immb.into()) << 16u32
                | 0b111111u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
