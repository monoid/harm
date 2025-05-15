/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod sqinch_z_zs_ {
    #[inline]
    pub fn sqinch_z_zs_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b11000u32 << 11u32
                | u32::from(U.into()) << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
pub mod sqdech_z_zs_ {
    #[inline]
    pub fn sqdech_z_zs_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b11001u32 << 11u32
                | u32::from(U.into()) << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
pub mod uqinch_z_zs_ {
    #[inline]
    pub fn uqinch_z_zs_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b11000u32 << 11u32
                | u32::from(U.into()) << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
pub mod uqdech_z_zs_ {
    #[inline]
    pub fn uqdech_z_zs_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b11001u32 << 11u32
                | u32::from(U.into()) << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
pub mod sqincw_z_zs_ {
    #[inline]
    pub fn sqincw_z_zs_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b11000u32 << 11u32
                | u32::from(U.into()) << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
pub mod sqdecw_z_zs_ {
    #[inline]
    pub fn sqdecw_z_zs_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b11001u32 << 11u32
                | u32::from(U.into()) << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
pub mod uqincw_z_zs_ {
    #[inline]
    pub fn uqincw_z_zs_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b11000u32 << 11u32
                | u32::from(U.into()) << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
pub mod uqdecw_z_zs_ {
    #[inline]
    pub fn uqdecw_z_zs_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b11001u32 << 11u32
                | u32::from(U.into()) << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
pub mod sqincd_z_zs_ {
    #[inline]
    pub fn sqincd_z_zs_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b11000u32 << 11u32
                | u32::from(U.into()) << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
pub mod sqdecd_z_zs_ {
    #[inline]
    pub fn sqdecd_z_zs_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b11001u32 << 11u32
                | u32::from(U.into()) << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
pub mod uqincd_z_zs_ {
    #[inline]
    pub fn uqincd_z_zs_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b11000u32 << 11u32
                | u32::from(U.into()) << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
pub mod uqdecd_z_zs_ {
    #[inline]
    pub fn uqdecd_z_zs_(
        size: impl Into<::aarchmrs_types::BitValue<2>>,
        imm4: impl Into<::aarchmrs_types::BitValue<4>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        pattern: impl Into<::aarchmrs_types::BitValue<5>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(size.into()) << 22u32
                | 0b10u32 << 20u32
                | u32::from(imm4.into()) << 16u32
                | 0b11001u32 << 11u32
                | u32::from(U.into()) << 10u32
                | u32::from(pattern.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
