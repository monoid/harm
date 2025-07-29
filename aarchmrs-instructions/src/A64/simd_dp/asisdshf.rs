/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod SSHR_asisdshf_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000001100110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01011111000000000000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SSHR_asisdshf_R";
    #[inline]
    pub const fn SSHR_asisdshf_R(
        immh: ::aarchmrs_types::BitValue<4>,
        immb: ::aarchmrs_types::BitValue<3>,
        o1: ::aarchmrs_types::BitValue<1>,
        o0: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010111110u32 << 23u32
                | immh.into_inner() << 19u32
                | immb.into_inner() << 16u32
                | 0b00u32 << 14u32
                | o1.into_inner() << 13u32
                | o0.into_inner() << 12u32
                | 0b01u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SSRA_asisdshf_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000001100110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01011111000000000000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SSRA_asisdshf_R";
    #[inline]
    pub const fn SSRA_asisdshf_R(
        immh: ::aarchmrs_types::BitValue<4>,
        immb: ::aarchmrs_types::BitValue<3>,
        o1: ::aarchmrs_types::BitValue<1>,
        o0: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010111110u32 << 23u32
                | immh.into_inner() << 19u32
                | immb.into_inner() << 16u32
                | 0b00u32 << 14u32
                | o1.into_inner() << 13u32
                | o0.into_inner() << 12u32
                | 0b01u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SRSHR_asisdshf_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000001100110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01011111000000000000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SRSHR_asisdshf_R";
    #[inline]
    pub const fn SRSHR_asisdshf_R(
        immh: ::aarchmrs_types::BitValue<4>,
        immb: ::aarchmrs_types::BitValue<3>,
        o1: ::aarchmrs_types::BitValue<1>,
        o0: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010111110u32 << 23u32
                | immh.into_inner() << 19u32
                | immb.into_inner() << 16u32
                | 0b00u32 << 14u32
                | o1.into_inner() << 13u32
                | o0.into_inner() << 12u32
                | 0b01u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SRSRA_asisdshf_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000001100110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01011111000000000000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SRSRA_asisdshf_R";
    #[inline]
    pub const fn SRSRA_asisdshf_R(
        immh: ::aarchmrs_types::BitValue<4>,
        immb: ::aarchmrs_types::BitValue<3>,
        o1: ::aarchmrs_types::BitValue<1>,
        o0: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010111110u32 << 23u32
                | immh.into_inner() << 19u32
                | immb.into_inner() << 16u32
                | 0b00u32 << 14u32
                | o1.into_inner() << 13u32
                | o0.into_inner() << 12u32
                | 0b01u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SHL_asisdshf_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01011111000000000101010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SHL_asisdshf_R";
    #[inline]
    pub const fn SHL_asisdshf_R(
        immh: ::aarchmrs_types::BitValue<4>,
        immb: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010111110u32 << 23u32
                | immh.into_inner() << 19u32
                | immb.into_inner() << 16u32
                | 0b010101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SQSHL_asisdshf_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000001110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01011111000000000110010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SQSHL_asisdshf_R";
    #[inline]
    pub const fn SQSHL_asisdshf_R(
        immh: ::aarchmrs_types::BitValue<4>,
        immb: ::aarchmrs_types::BitValue<3>,
        op: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010111110u32 << 23u32
                | immh.into_inner() << 19u32
                | immb.into_inner() << 16u32
                | 0b011u32 << 13u32
                | op.into_inner() << 12u32
                | 0b01u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SQSHRN_asisdshf_N {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01011111000000001001010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SQSHRN_asisdshf_N";
    #[inline]
    pub const fn SQSHRN_asisdshf_N(
        immh: ::aarchmrs_types::BitValue<4>,
        immb: ::aarchmrs_types::BitValue<3>,
        op: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010111110u32 << 23u32
                | immh.into_inner() << 19u32
                | immb.into_inner() << 16u32
                | 0b1001u32 << 12u32
                | op.into_inner() << 11u32
                | 0b1u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SQRSHRN_asisdshf_N {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01011111000000001001010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SQRSHRN_asisdshf_N";
    #[inline]
    pub const fn SQRSHRN_asisdshf_N(
        immh: ::aarchmrs_types::BitValue<4>,
        immb: ::aarchmrs_types::BitValue<3>,
        op: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010111110u32 << 23u32
                | immh.into_inner() << 19u32
                | immb.into_inner() << 16u32
                | 0b1001u32 << 12u32
                | op.into_inner() << 11u32
                | 0b1u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SCVTF_asisdshf_C {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01011111000000001110010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SCVTF_asisdshf_C";
    #[inline]
    pub const fn SCVTF_asisdshf_C(
        immh: ::aarchmrs_types::BitValue<4>,
        immb: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010111110u32 << 23u32
                | immh.into_inner() << 19u32
                | immb.into_inner() << 16u32
                | 0b111001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTZS_asisdshf_C {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01011111000000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTZS_asisdshf_C";
    #[inline]
    pub const fn FCVTZS_asisdshf_C(
        immh: ::aarchmrs_types::BitValue<4>,
        immb: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010111110u32 << 23u32
                | immh.into_inner() << 19u32
                | immb.into_inner() << 16u32
                | 0b111111u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod USHR_asisdshf_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000001100110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01111111000000000000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "USHR_asisdshf_R";
    #[inline]
    pub const fn USHR_asisdshf_R(
        immh: ::aarchmrs_types::BitValue<4>,
        immb: ::aarchmrs_types::BitValue<3>,
        o1: ::aarchmrs_types::BitValue<1>,
        o0: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011111110u32 << 23u32
                | immh.into_inner() << 19u32
                | immb.into_inner() << 16u32
                | 0b00u32 << 14u32
                | o1.into_inner() << 13u32
                | o0.into_inner() << 12u32
                | 0b01u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod USRA_asisdshf_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000001100110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01111111000000000000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "USRA_asisdshf_R";
    #[inline]
    pub const fn USRA_asisdshf_R(
        immh: ::aarchmrs_types::BitValue<4>,
        immb: ::aarchmrs_types::BitValue<3>,
        o1: ::aarchmrs_types::BitValue<1>,
        o0: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011111110u32 << 23u32
                | immh.into_inner() << 19u32
                | immb.into_inner() << 16u32
                | 0b00u32 << 14u32
                | o1.into_inner() << 13u32
                | o0.into_inner() << 12u32
                | 0b01u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod URSHR_asisdshf_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000001100110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01111111000000000000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "URSHR_asisdshf_R";
    #[inline]
    pub const fn URSHR_asisdshf_R(
        immh: ::aarchmrs_types::BitValue<4>,
        immb: ::aarchmrs_types::BitValue<3>,
        o1: ::aarchmrs_types::BitValue<1>,
        o0: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011111110u32 << 23u32
                | immh.into_inner() << 19u32
                | immb.into_inner() << 16u32
                | 0b00u32 << 14u32
                | o1.into_inner() << 13u32
                | o0.into_inner() << 12u32
                | 0b01u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod URSRA_asisdshf_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000001100110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01111111000000000000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "URSRA_asisdshf_R";
    #[inline]
    pub const fn URSRA_asisdshf_R(
        immh: ::aarchmrs_types::BitValue<4>,
        immb: ::aarchmrs_types::BitValue<3>,
        o1: ::aarchmrs_types::BitValue<1>,
        o0: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011111110u32 << 23u32
                | immh.into_inner() << 19u32
                | immb.into_inner() << 16u32
                | 0b00u32 << 14u32
                | o1.into_inner() << 13u32
                | o0.into_inner() << 12u32
                | 0b01u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SRI_asisdshf_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01111111000000000100010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SRI_asisdshf_R";
    #[inline]
    pub const fn SRI_asisdshf_R(
        immh: ::aarchmrs_types::BitValue<4>,
        immb: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011111110u32 << 23u32
                | immh.into_inner() << 19u32
                | immb.into_inner() << 16u32
                | 0b010001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SLI_asisdshf_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01111111000000000101010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SLI_asisdshf_R";
    #[inline]
    pub const fn SLI_asisdshf_R(
        immh: ::aarchmrs_types::BitValue<4>,
        immb: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011111110u32 << 23u32
                | immh.into_inner() << 19u32
                | immb.into_inner() << 16u32
                | 0b010101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SQSHLU_asisdshf_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000001110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01111111000000000110010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SQSHLU_asisdshf_R";
    #[inline]
    pub const fn SQSHLU_asisdshf_R(
        immh: ::aarchmrs_types::BitValue<4>,
        immb: ::aarchmrs_types::BitValue<3>,
        op: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011111110u32 << 23u32
                | immh.into_inner() << 19u32
                | immb.into_inner() << 16u32
                | 0b011u32 << 13u32
                | op.into_inner() << 12u32
                | 0b01u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod UQSHL_asisdshf_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000001110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01111111000000000110010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UQSHL_asisdshf_R";
    #[inline]
    pub const fn UQSHL_asisdshf_R(
        immh: ::aarchmrs_types::BitValue<4>,
        immb: ::aarchmrs_types::BitValue<3>,
        op: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011111110u32 << 23u32
                | immh.into_inner() << 19u32
                | immb.into_inner() << 16u32
                | 0b011u32 << 13u32
                | op.into_inner() << 12u32
                | 0b01u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SQSHRUN_asisdshf_N {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01111111000000001000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SQSHRUN_asisdshf_N";
    #[inline]
    pub const fn SQSHRUN_asisdshf_N(
        immh: ::aarchmrs_types::BitValue<4>,
        immb: ::aarchmrs_types::BitValue<3>,
        op: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011111110u32 << 23u32
                | immh.into_inner() << 19u32
                | immb.into_inner() << 16u32
                | 0b1000u32 << 12u32
                | op.into_inner() << 11u32
                | 0b1u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SQRSHRUN_asisdshf_N {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01111111000000001000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SQRSHRUN_asisdshf_N";
    #[inline]
    pub const fn SQRSHRUN_asisdshf_N(
        immh: ::aarchmrs_types::BitValue<4>,
        immb: ::aarchmrs_types::BitValue<3>,
        op: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011111110u32 << 23u32
                | immh.into_inner() << 19u32
                | immb.into_inner() << 16u32
                | 0b1000u32 << 12u32
                | op.into_inner() << 11u32
                | 0b1u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod UQSHRN_asisdshf_N {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01111111000000001001010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UQSHRN_asisdshf_N";
    #[inline]
    pub const fn UQSHRN_asisdshf_N(
        immh: ::aarchmrs_types::BitValue<4>,
        immb: ::aarchmrs_types::BitValue<3>,
        op: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011111110u32 << 23u32
                | immh.into_inner() << 19u32
                | immb.into_inner() << 16u32
                | 0b1001u32 << 12u32
                | op.into_inner() << 11u32
                | 0b1u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod UQRSHRN_asisdshf_N {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01111111000000001001010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UQRSHRN_asisdshf_N";
    #[inline]
    pub const fn UQRSHRN_asisdshf_N(
        immh: ::aarchmrs_types::BitValue<4>,
        immb: ::aarchmrs_types::BitValue<3>,
        op: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011111110u32 << 23u32
                | immh.into_inner() << 19u32
                | immb.into_inner() << 16u32
                | 0b1001u32 << 12u32
                | op.into_inner() << 11u32
                | 0b1u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod UCVTF_asisdshf_C {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01111111000000001110010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UCVTF_asisdshf_C";
    #[inline]
    pub const fn UCVTF_asisdshf_C(
        immh: ::aarchmrs_types::BitValue<4>,
        immb: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011111110u32 << 23u32
                | immh.into_inner() << 19u32
                | immb.into_inner() << 16u32
                | 0b111001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTZU_asisdshf_C {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111100000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01111111000000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTZU_asisdshf_C";
    #[inline]
    pub const fn FCVTZU_asisdshf_C(
        immh: ::aarchmrs_types::BitValue<4>,
        immb: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b011111110u32 << 23u32
                | immh.into_inner() << 19u32
                | immb.into_inner() << 16u32
                | 0b111111u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
