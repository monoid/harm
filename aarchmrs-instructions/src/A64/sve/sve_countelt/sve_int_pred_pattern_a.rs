/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod incb_r_rs_ {
    #[inline]
    pub fn incb_r_rs_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b11u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b111000u32 << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Rdn.into()) << 0u32,
        )
    }
}
pub mod decb_r_rs_ {
    #[inline]
    pub fn decb_r_rs_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b11u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b111001u32 << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Rdn.into()) << 0u32,
        )
    }
}
pub mod inch_r_rs_ {
    #[inline]
    pub fn inch_r_rs_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b11u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b111000u32 << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Rdn.into()) << 0u32,
        )
    }
}
pub mod dech_r_rs_ {
    #[inline]
    pub fn dech_r_rs_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b11u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b111001u32 << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Rdn.into()) << 0u32,
        )
    }
}
pub mod incw_r_rs_ {
    #[inline]
    pub fn incw_r_rs_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b11u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b111000u32 << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Rdn.into()) << 0u32,
        )
    }
}
pub mod decw_r_rs_ {
    #[inline]
    pub fn decw_r_rs_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b11u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b111001u32 << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Rdn.into()) << 0u32,
        )
    }
}
pub mod incd_r_rs_ {
    #[inline]
    pub fn incd_r_rs_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b11u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b111000u32 << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Rdn.into()) << 0u32,
        )
    }
}
pub mod decd_r_rs_ {
    #[inline]
    pub fn decd_r_rs_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Rdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b11u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b111001u32 << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Rdn.into()) << 0u32,
        )
    }
}
