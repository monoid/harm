/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod inch_z_zs_ {
    #[inline]
    pub fn inch_z_zs_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b11u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b110000u32 << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
pub mod dech_z_zs_ {
    #[inline]
    pub fn dech_z_zs_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b11u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b110001u32 << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
pub mod incw_z_zs_ {
    #[inline]
    pub fn incw_z_zs_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b11u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b110000u32 << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
pub mod decw_z_zs_ {
    #[inline]
    pub fn decw_z_zs_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b11u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b110001u32 << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
pub mod incd_z_zs_ {
    #[inline]
    pub fn incd_z_zs_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b11u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b110000u32 << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
pub mod decd_z_zs_ {
    #[inline]
    pub fn decd_z_zs_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b11u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b110001u32 << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
