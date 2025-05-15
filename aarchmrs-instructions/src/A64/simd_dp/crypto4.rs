/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod EOR3_VVV16_crypto4 {
    #[inline]
    pub fn EOR3_VVV16_crypto4(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Ra: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11001110000u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b0u32 << 15u32
                | u32::from(Ra.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod BCAX_VVV16_crypto4 {
    #[inline]
    pub fn BCAX_VVV16_crypto4(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Ra: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11001110001u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b0u32 << 15u32
                | u32::from(Ra.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SM3SS1_VVV4_crypto4 {
    #[inline]
    pub fn SM3SS1_VVV4_crypto4(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        Ra: impl Into<::aarchmrs_types::BitValue<5>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11001110010u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b0u32 << 15u32
                | u32::from(Ra.into()) << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
