/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod STXP_SP32_ldstexclp {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10001000001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STXP_SP32_ldstexclp";
    #[inline]
    pub const fn STXP_SP32_ldstexclp(
        Rs: ::aarchmrs_types::BitValue<5>,
        Rt2: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10001000001u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b0u32 << 15u32
                | Rt2.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod STLXP_SP32_ldstexclp {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10001000001000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STLXP_SP32_ldstexclp";
    #[inline]
    pub const fn STLXP_SP32_ldstexclp(
        Rs: ::aarchmrs_types::BitValue<5>,
        Rt2: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10001000001u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b1u32 << 15u32
                | Rt2.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LDXP_LP32_ldstexclp {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10001000011111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDXP_LP32_ldstexclp";
    #[inline]
    pub const fn LDXP_LP32_ldstexclp(
        Rt2: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10001000011111110u32 << 15u32
                | Rt2.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LDAXP_LP32_ldstexclp {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10001000011111111000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDAXP_LP32_ldstexclp";
    #[inline]
    pub const fn LDAXP_LP32_ldstexclp(
        Rt2: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10001000011111111u32 << 15u32
                | Rt2.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod STXP_SP64_ldstexclp {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11001000001000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STXP_SP64_ldstexclp";
    #[inline]
    pub const fn STXP_SP64_ldstexclp(
        Rs: ::aarchmrs_types::BitValue<5>,
        Rt2: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11001000001u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b0u32 << 15u32
                | Rt2.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod STLXP_SP64_ldstexclp {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11001000001000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "STLXP_SP64_ldstexclp";
    #[inline]
    pub const fn STLXP_SP64_ldstexclp(
        Rs: ::aarchmrs_types::BitValue<5>,
        Rt2: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11001000001u32 << 21u32
                | Rs.into_inner() << 16u32
                | 0b1u32 << 15u32
                | Rt2.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LDXP_LP64_ldstexclp {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11001000011111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDXP_LP64_ldstexclp";
    #[inline]
    pub const fn LDXP_LP64_ldstexclp(
        Rt2: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11001000011111110u32 << 15u32
                | Rt2.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
pub mod LDAXP_LP64_ldstexclp {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11001000011111111000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000111110000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "LDAXP_LP64_ldstexclp";
    #[inline]
    pub const fn LDAXP_LP64_ldstexclp(
        Rt2: ::aarchmrs_types::BitValue<5>,
        Rn: ::aarchmrs_types::BitValue<5>,
        Rt: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11001000011111111u32 << 15u32
                | Rt2.into_inner() << 10u32
                | Rn.into_inner() << 5u32
                | Rt.into_inner() << 0u32,
        )
    }
}
