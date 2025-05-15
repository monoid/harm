/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod asr_z_zi_ {
    #[inline]
    pub fn asr_z_zi_(
        tszh: impl Into<::aarchmrs_types::BitValue<2>>,
        tszl: impl Into<::aarchmrs_types::BitValue<2>>,
        imm3: impl Into<::aarchmrs_types::BitValue<3>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(tszh.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(tszl.into()) << 19u32
                | u32::from(imm3.into()) << 16u32
                | 0b10010u32 << 11u32
                | u32::from(U.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod lsl_z_zi_ {
    #[inline]
    pub fn lsl_z_zi_(
        tszh: impl Into<::aarchmrs_types::BitValue<2>>,
        tszl: impl Into<::aarchmrs_types::BitValue<2>>,
        imm3: impl Into<::aarchmrs_types::BitValue<3>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(tszh.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(tszl.into()) << 19u32
                | u32::from(imm3.into()) << 16u32
                | 0b100111u32 << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
pub mod lsr_z_zi_ {
    #[inline]
    pub fn lsr_z_zi_(
        tszh: impl Into<::aarchmrs_types::BitValue<2>>,
        tszl: impl Into<::aarchmrs_types::BitValue<2>>,
        imm3: impl Into<::aarchmrs_types::BitValue<3>>,
        U: impl Into<::aarchmrs_types::BitValue<1>>,
        Zn: impl Into<::aarchmrs_types::BitValue<5>>,
        Zd: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000100u32 << 24u32
                | u32::from(tszh.into()) << 22u32
                | 0b1u32 << 21u32
                | u32::from(tszl.into()) << 19u32
                | u32::from(imm3.into()) << 16u32
                | 0b10010u32 << 11u32
                | u32::from(U.into()) << 10u32
                | u32::from(Zn.into()) << 5u32
                | u32::from(Zd.into()) << 0u32,
        )
    }
}
