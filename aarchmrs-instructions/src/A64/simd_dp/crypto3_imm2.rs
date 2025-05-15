/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod SM3TT1A_VVV4_crypto3_imm2 {
    #[inline]
    pub fn SM3TT1A_VVV4_crypto3_imm2(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        imm2: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11001110010u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b10u32 << 14u32
                | u32::from(imm2.into()) << 12u32
                | 0b00u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SM3TT1B_VVV4_crypto3_imm2 {
    #[inline]
    pub fn SM3TT1B_VVV4_crypto3_imm2(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        imm2: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11001110010u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b10u32 << 14u32
                | u32::from(imm2.into()) << 12u32
                | 0b01u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SM3TT2A_VVV4_crypto3_imm2 {
    #[inline]
    pub fn SM3TT2A_VVV4_crypto3_imm2(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        imm2: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11001110010u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b10u32 << 14u32
                | u32::from(imm2.into()) << 12u32
                | 0b10u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod SM3TT2B_VVV_crypto3_imm2 {
    #[inline]
    pub fn SM3TT2B_VVV_crypto3_imm2(
        Rm: impl Into<::aarchmrs_types::BitValue<5>>,
        imm2: impl Into<::aarchmrs_types::BitValue<2>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11001110010u32 << 21u32
                | u32::from(Rm.into()) << 16u32
                | 0b10u32 << 14u32
                | u32::from(imm2.into()) << 12u32
                | 0b11u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
