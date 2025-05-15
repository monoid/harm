/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod STXP_SP32_ldstexclp {
    #[inline]
    pub fn STXP_SP32_ldstexclp(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10001000001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b0u32 << 15u32
                | u32::from(Rt2.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STLXP_SP32_ldstexclp {
    #[inline]
    pub fn STLXP_SP32_ldstexclp(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10001000001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(Rt2.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDXP_LP32_ldstexclp {
    #[inline]
    pub fn LDXP_LP32_ldstexclp(
        Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10001000011111110u32 << 15u32
                | u32::from(Rt2.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDAXP_LP32_ldstexclp {
    #[inline]
    pub fn LDAXP_LP32_ldstexclp(
        Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10001000011111111u32 << 15u32
                | u32::from(Rt2.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STXP_SP64_ldstexclp {
    #[inline]
    pub fn STXP_SP64_ldstexclp(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11001000001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b0u32 << 15u32
                | u32::from(Rt2.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod STLXP_SP64_ldstexclp {
    #[inline]
    pub fn STLXP_SP64_ldstexclp(
        Rs: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11001000001u32 << 21u32
                | u32::from(Rs.into()) << 16u32
                | 0b1u32 << 15u32
                | u32::from(Rt2.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDXP_LP64_ldstexclp {
    #[inline]
    pub fn LDXP_LP64_ldstexclp(
        Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11001000011111110u32 << 15u32
                | u32::from(Rt2.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDAXP_LP64_ldstexclp {
    #[inline]
    pub fn LDAXP_LP64_ldstexclp(
        Rt2: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11001000011111111u32 << 15u32
                | u32::from(Rt2.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
