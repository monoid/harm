/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod cntb_r_s_ {
    #[inline]
    pub fn cntb_r_s_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b111000u32 << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod cnth_r_s_ {
    #[inline]
    pub fn cnth_r_s_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b111000u32 << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod cntw_r_s_ {
    #[inline]
    pub fn cntw_r_s_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b111000u32 << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
pub mod cntd_r_s_ {
    #[inline]
    pub fn cntd_r_s_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Rd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b111000u32 << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Rd.into()) << 0u32,
        )
    }
}
