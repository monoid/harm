/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod LDRAA_64_ldst_pac {
    #[inline]
    pub fn LDRAA_64_ldst_pac(
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110000u32 << 23u32
                | u32::from(S.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b01u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDRAA_64W_ldst_pac {
    #[inline]
    pub fn LDRAA_64W_ldst_pac(
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110000u32 << 23u32
                | u32::from(S.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b11u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDRAB_64_ldst_pac {
    #[inline]
    pub fn LDRAB_64_ldst_pac(
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110001u32 << 23u32
                | u32::from(S.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b01u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod LDRAB_64W_ldst_pac {
    #[inline]
    pub fn LDRAB_64W_ldst_pac(
        S: impl Into<::aarchmrs_types::BitValue<1>>,
        imm9: impl Into<::aarchmrs_types::BitValue<9>>,
        Rn: impl Into<::aarchmrs_types::BitValue<5>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110001u32 << 23u32
                | u32::from(S.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(imm9.into()) << 12u32
                | 0b11u32 << 10u32
                | u32::from(Rn.into()) << 5u32
                | u32::from(Rt.into()) << 0u32,
        )
    }
}
