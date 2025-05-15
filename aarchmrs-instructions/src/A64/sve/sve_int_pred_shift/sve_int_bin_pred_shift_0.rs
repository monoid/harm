/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod asr_z_p_zi_ {
    #[inline]
    pub fn asr_z_p_zi_(
        tszh: impl Into<::aarchmrs_types::BitValue<2>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        tszl: impl Into<::aarchmrs_types::BitValue<2>>,
        imm3: impl Into<::aarchmrs_types::BitValue<3>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(tszh.into()) << 22u32
                | 0b00000u32 << 17u32
                | u32::from(U.into()) << 16u32
                | 0b100u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(tszl.into()) << 8u32
                | u32::from(imm3.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
pub mod lsl_z_p_zi_ {
    #[inline]
    pub fn lsl_z_p_zi_(
        tszh: impl Into<::aarchmrs_types::BitValue<2>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        tszl: impl Into<::aarchmrs_types::BitValue<2>>,
        imm3: impl Into<::aarchmrs_types::BitValue<3>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(tszh.into()) << 22u32
                | 0b000011100u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(tszl.into()) << 8u32
                | u32::from(imm3.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
pub mod asrd_z_p_zi_ {
    #[inline]
    pub fn asrd_z_p_zi_(
        tszh: impl Into<::aarchmrs_types::BitValue<2>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        tszl: impl Into<::aarchmrs_types::BitValue<2>>,
        imm3: impl Into<::aarchmrs_types::BitValue<3>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(tszh.into()) << 22u32
                | 0b000100100u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(tszl.into()) << 8u32
                | u32::from(imm3.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
pub mod sqshl_z_p_zi_ {
    #[inline]
    pub fn sqshl_z_p_zi_(
        tszh: impl Into<::aarchmrs_types::BitValue<2>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        tszl: impl Into<::aarchmrs_types::BitValue<2>>,
        imm3: impl Into<::aarchmrs_types::BitValue<3>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(tszh.into()) << 22u32
                | 0b00011u32 << 17u32
                | u32::from(U.into()) << 16u32
                | 0b100u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(tszl.into()) << 8u32
                | u32::from(imm3.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
pub mod srshr_z_p_zi_ {
    #[inline]
    pub fn srshr_z_p_zi_(
        tszh: impl Into<::aarchmrs_types::BitValue<2>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        tszl: impl Into<::aarchmrs_types::BitValue<2>>,
        imm3: impl Into<::aarchmrs_types::BitValue<3>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(tszh.into()) << 22u32
                | 0b00110u32 << 17u32
                | u32::from(U.into()) << 16u32
                | 0b100u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(tszl.into()) << 8u32
                | u32::from(imm3.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
pub mod sqshlu_z_p_zi_ {
    #[inline]
    pub fn sqshlu_z_p_zi_(
        tszh: impl Into<::aarchmrs_types::BitValue<2>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        tszl: impl Into<::aarchmrs_types::BitValue<2>>,
        imm3: impl Into<::aarchmrs_types::BitValue<3>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(tszh.into()) << 22u32
                | 0b001111100u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(tszl.into()) << 8u32
                | u32::from(imm3.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
pub mod lsr_z_p_zi_ {
    #[inline]
    pub fn lsr_z_p_zi_(
        tszh: impl Into<::aarchmrs_types::BitValue<2>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        tszl: impl Into<::aarchmrs_types::BitValue<2>>,
        imm3: impl Into<::aarchmrs_types::BitValue<3>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(tszh.into()) << 22u32
                | 0b00000u32 << 17u32
                | u32::from(U.into()) << 16u32
                | 0b100u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(tszl.into()) << 8u32
                | u32::from(imm3.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
pub mod uqshl_z_p_zi_ {
    #[inline]
    pub fn uqshl_z_p_zi_(
        tszh: impl Into<::aarchmrs_types::BitValue<2>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        tszl: impl Into<::aarchmrs_types::BitValue<2>>,
        imm3: impl Into<::aarchmrs_types::BitValue<3>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(tszh.into()) << 22u32
                | 0b00011u32 << 17u32
                | u32::from(U.into()) << 16u32
                | 0b100u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(tszl.into()) << 8u32
                | u32::from(imm3.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
pub mod urshr_z_p_zi_ {
    #[inline]
    pub fn urshr_z_p_zi_(
        tszh: impl Into<::aarchmrs_types::BitValue<2>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        Pg: impl Into<::aarchmrs_types::BitValue<3>>,
        tszl: impl Into<::aarchmrs_types::BitValue<2>>,
        imm3: impl Into<::aarchmrs_types::BitValue<3>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(tszh.into()) << 22u32
                | 0b00110u32 << 17u32
                | u32::from(U.into()) << 16u32
                | 0b100u32 << 13u32
                | u32::from(Pg.into()) << 10u32
                | u32::from(tszl.into()) << 8u32
                | u32::from(imm3.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
