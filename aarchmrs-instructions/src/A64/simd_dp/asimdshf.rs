/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod SSHR_asimdshf_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111100000001100110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001111000000000000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SSHR_asimdshf_R";
    #[inline]
    pub const fn SSHR_asimdshf_R(
        Q: ::aarchmrs_types::BitValue<1>,
        immh: ::aarchmrs_types::BitValue<4>,
        immb: ::aarchmrs_types::BitValue<3>,
        o1: ::aarchmrs_types::BitValue<1>,
        o0: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b0011110u32 << 23u32
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
pub mod SSRA_asimdshf_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111100000001100110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001111000000000000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SSRA_asimdshf_R";
    #[inline]
    pub const fn SSRA_asimdshf_R(
        Q: ::aarchmrs_types::BitValue<1>,
        immh: ::aarchmrs_types::BitValue<4>,
        immb: ::aarchmrs_types::BitValue<3>,
        o1: ::aarchmrs_types::BitValue<1>,
        o0: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b0011110u32 << 23u32
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
pub mod SRSHR_asimdshf_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111100000001100110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001111000000000000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SRSHR_asimdshf_R";
    #[inline]
    pub const fn SRSHR_asimdshf_R(
        Q: ::aarchmrs_types::BitValue<1>,
        immh: ::aarchmrs_types::BitValue<4>,
        immb: ::aarchmrs_types::BitValue<3>,
        o1: ::aarchmrs_types::BitValue<1>,
        o0: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b0011110u32 << 23u32
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
pub mod SRSRA_asimdshf_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111100000001100110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001111000000000000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SRSRA_asimdshf_R";
    #[inline]
    pub const fn SRSRA_asimdshf_R(
        Q: ::aarchmrs_types::BitValue<1>,
        immh: ::aarchmrs_types::BitValue<4>,
        immb: ::aarchmrs_types::BitValue<3>,
        o1: ::aarchmrs_types::BitValue<1>,
        o0: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b0011110u32 << 23u32
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
pub mod SHL_asimdshf_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111100000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001111000000000101010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SHL_asimdshf_R";
    #[inline]
    pub const fn SHL_asimdshf_R(
        Q: ::aarchmrs_types::BitValue<1>,
        immh: ::aarchmrs_types::BitValue<4>,
        immb: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b0011110u32 << 23u32
                | immh.into_inner() << 19u32
                | immb.into_inner() << 16u32
                | 0b010101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SQSHL_asimdshf_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111100000001110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001111000000000110010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SQSHL_asimdshf_R";
    #[inline]
    pub const fn SQSHL_asimdshf_R(
        Q: ::aarchmrs_types::BitValue<1>,
        immh: ::aarchmrs_types::BitValue<4>,
        immb: ::aarchmrs_types::BitValue<3>,
        op: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b0011110u32 << 23u32
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
pub mod SHRN_asimdshf_N {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111100000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001111000000001000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SHRN_asimdshf_N";
    #[inline]
    pub const fn SHRN_asimdshf_N(
        Q: ::aarchmrs_types::BitValue<1>,
        immh: ::aarchmrs_types::BitValue<4>,
        immb: ::aarchmrs_types::BitValue<3>,
        op: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b0011110u32 << 23u32
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
pub mod RSHRN_asimdshf_N {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111100000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001111000000001000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "RSHRN_asimdshf_N";
    #[inline]
    pub const fn RSHRN_asimdshf_N(
        Q: ::aarchmrs_types::BitValue<1>,
        immh: ::aarchmrs_types::BitValue<4>,
        immb: ::aarchmrs_types::BitValue<3>,
        op: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b0011110u32 << 23u32
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
pub mod SQSHRN_asimdshf_N {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111100000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001111000000001001010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SQSHRN_asimdshf_N";
    #[inline]
    pub const fn SQSHRN_asimdshf_N(
        Q: ::aarchmrs_types::BitValue<1>,
        immh: ::aarchmrs_types::BitValue<4>,
        immb: ::aarchmrs_types::BitValue<3>,
        op: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b0011110u32 << 23u32
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
pub mod SQRSHRN_asimdshf_N {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111100000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001111000000001001010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SQRSHRN_asimdshf_N";
    #[inline]
    pub const fn SQRSHRN_asimdshf_N(
        Q: ::aarchmrs_types::BitValue<1>,
        immh: ::aarchmrs_types::BitValue<4>,
        immb: ::aarchmrs_types::BitValue<3>,
        op: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b0011110u32 << 23u32
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
pub mod SSHLL_asimdshf_L {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111100000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001111000000001010010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SSHLL_asimdshf_L";
    #[inline]
    pub const fn SSHLL_asimdshf_L(
        Q: ::aarchmrs_types::BitValue<1>,
        immh: ::aarchmrs_types::BitValue<4>,
        immb: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b0011110u32 << 23u32
                | immh.into_inner() << 19u32
                | immb.into_inner() << 16u32
                | 0b101001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SCVTF_asimdshf_C {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111100000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001111000000001110010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SCVTF_asimdshf_C";
    #[inline]
    pub const fn SCVTF_asimdshf_C(
        Q: ::aarchmrs_types::BitValue<1>,
        immh: ::aarchmrs_types::BitValue<4>,
        immb: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b0011110u32 << 23u32
                | immh.into_inner() << 19u32
                | immb.into_inner() << 16u32
                | 0b111001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTZS_asimdshf_C {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111100000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00001111000000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTZS_asimdshf_C";
    #[inline]
    pub const fn FCVTZS_asimdshf_C(
        Q: ::aarchmrs_types::BitValue<1>,
        immh: ::aarchmrs_types::BitValue<4>,
        immb: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b0011110u32 << 23u32
                | immh.into_inner() << 19u32
                | immb.into_inner() << 16u32
                | 0b111111u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod USHR_asimdshf_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111100000001100110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101111000000000000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "USHR_asimdshf_R";
    #[inline]
    pub const fn USHR_asimdshf_R(
        Q: ::aarchmrs_types::BitValue<1>,
        immh: ::aarchmrs_types::BitValue<4>,
        immb: ::aarchmrs_types::BitValue<3>,
        o1: ::aarchmrs_types::BitValue<1>,
        o0: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b1011110u32 << 23u32
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
pub mod USRA_asimdshf_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111100000001100110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101111000000000000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "USRA_asimdshf_R";
    #[inline]
    pub const fn USRA_asimdshf_R(
        Q: ::aarchmrs_types::BitValue<1>,
        immh: ::aarchmrs_types::BitValue<4>,
        immb: ::aarchmrs_types::BitValue<3>,
        o1: ::aarchmrs_types::BitValue<1>,
        o0: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b1011110u32 << 23u32
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
pub mod URSHR_asimdshf_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111100000001100110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101111000000000000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "URSHR_asimdshf_R";
    #[inline]
    pub const fn URSHR_asimdshf_R(
        Q: ::aarchmrs_types::BitValue<1>,
        immh: ::aarchmrs_types::BitValue<4>,
        immb: ::aarchmrs_types::BitValue<3>,
        o1: ::aarchmrs_types::BitValue<1>,
        o0: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b1011110u32 << 23u32
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
pub mod URSRA_asimdshf_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111100000001100110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101111000000000000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "URSRA_asimdshf_R";
    #[inline]
    pub const fn URSRA_asimdshf_R(
        Q: ::aarchmrs_types::BitValue<1>,
        immh: ::aarchmrs_types::BitValue<4>,
        immb: ::aarchmrs_types::BitValue<3>,
        o1: ::aarchmrs_types::BitValue<1>,
        o0: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b1011110u32 << 23u32
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
pub mod SRI_asimdshf_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111100000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101111000000000100010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SRI_asimdshf_R";
    #[inline]
    pub const fn SRI_asimdshf_R(
        Q: ::aarchmrs_types::BitValue<1>,
        immh: ::aarchmrs_types::BitValue<4>,
        immb: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b1011110u32 << 23u32
                | immh.into_inner() << 19u32
                | immb.into_inner() << 16u32
                | 0b010001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SLI_asimdshf_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111100000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101111000000000101010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SLI_asimdshf_R";
    #[inline]
    pub const fn SLI_asimdshf_R(
        Q: ::aarchmrs_types::BitValue<1>,
        immh: ::aarchmrs_types::BitValue<4>,
        immb: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b1011110u32 << 23u32
                | immh.into_inner() << 19u32
                | immb.into_inner() << 16u32
                | 0b010101u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod SQSHLU_asimdshf_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111100000001110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101111000000000110010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SQSHLU_asimdshf_R";
    #[inline]
    pub const fn SQSHLU_asimdshf_R(
        Q: ::aarchmrs_types::BitValue<1>,
        immh: ::aarchmrs_types::BitValue<4>,
        immb: ::aarchmrs_types::BitValue<3>,
        op: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b1011110u32 << 23u32
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
pub mod UQSHL_asimdshf_R {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111100000001110110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101111000000000110010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UQSHL_asimdshf_R";
    #[inline]
    pub const fn UQSHL_asimdshf_R(
        Q: ::aarchmrs_types::BitValue<1>,
        immh: ::aarchmrs_types::BitValue<4>,
        immb: ::aarchmrs_types::BitValue<3>,
        op: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b1011110u32 << 23u32
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
pub mod SQSHRUN_asimdshf_N {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111100000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101111000000001000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SQSHRUN_asimdshf_N";
    #[inline]
    pub const fn SQSHRUN_asimdshf_N(
        Q: ::aarchmrs_types::BitValue<1>,
        immh: ::aarchmrs_types::BitValue<4>,
        immb: ::aarchmrs_types::BitValue<3>,
        op: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b1011110u32 << 23u32
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
pub mod SQRSHRUN_asimdshf_N {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111100000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101111000000001000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SQRSHRUN_asimdshf_N";
    #[inline]
    pub const fn SQRSHRUN_asimdshf_N(
        Q: ::aarchmrs_types::BitValue<1>,
        immh: ::aarchmrs_types::BitValue<4>,
        immb: ::aarchmrs_types::BitValue<3>,
        op: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b1011110u32 << 23u32
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
pub mod UQSHRN_asimdshf_N {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111100000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101111000000001001010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UQSHRN_asimdshf_N";
    #[inline]
    pub const fn UQSHRN_asimdshf_N(
        Q: ::aarchmrs_types::BitValue<1>,
        immh: ::aarchmrs_types::BitValue<4>,
        immb: ::aarchmrs_types::BitValue<3>,
        op: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b1011110u32 << 23u32
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
pub mod UQRSHRN_asimdshf_N {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111100000001111010000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101111000000001001010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UQRSHRN_asimdshf_N";
    #[inline]
    pub const fn UQRSHRN_asimdshf_N(
        Q: ::aarchmrs_types::BitValue<1>,
        immh: ::aarchmrs_types::BitValue<4>,
        immb: ::aarchmrs_types::BitValue<3>,
        op: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b1011110u32 << 23u32
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
pub mod USHLL_asimdshf_L {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111100000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101111000000001010010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "USHLL_asimdshf_L";
    #[inline]
    pub const fn USHLL_asimdshf_L(
        Q: ::aarchmrs_types::BitValue<1>,
        immh: ::aarchmrs_types::BitValue<4>,
        immb: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b1011110u32 << 23u32
                | immh.into_inner() << 19u32
                | immb.into_inner() << 16u32
                | 0b101001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod UCVTF_asimdshf_C {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111100000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101111000000001110010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UCVTF_asimdshf_C";
    #[inline]
    pub const fn UCVTF_asimdshf_C(
        Q: ::aarchmrs_types::BitValue<1>,
        immh: ::aarchmrs_types::BitValue<4>,
        immb: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b1011110u32 << 23u32
                | immh.into_inner() << 19u32
                | immb.into_inner() << 16u32
                | 0b111001u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
pub mod FCVTZU_asimdshf_C {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b10111111100000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00101111000000001111110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "FCVTZU_asimdshf_C";
    #[inline]
    pub const fn FCVTZU_asimdshf_C(
        Q: ::aarchmrs_types::BitValue<1>,
        immh: ::aarchmrs_types::BitValue<4>,
        immb: ::aarchmrs_types::BitValue<3>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b0u32 << 31u32
                | Q.into_inner() << 30u32
                | 0b1011110u32 << 23u32
                | immh.into_inner() << 19u32
                | immb.into_inner() << 16u32
                | 0b111111u32 << 10u32
                | Rn.into_inner() << 5u32
                | Rd.into_inner() << 0u32,
        )
    }
}
