/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod orr_z_zi_ {
    #[inline]
    pub fn orr_z_zi_(
        imm13: impl Into<::aarchmrs_types::BitValue<13>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000101000000u32 << 18u32
                | u32::from(imm13.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
pub mod eor_z_zi_ {
    #[inline]
    pub fn eor_z_zi_(
        imm13: impl Into<::aarchmrs_types::BitValue<13>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000101010000u32 << 18u32
                | u32::from(imm13.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
pub mod and_z_zi_ {
    #[inline]
    pub fn and_z_zi_(
        imm13: impl Into<::aarchmrs_types::BitValue<13>>,
        Zdn: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00000101100000u32 << 18u32
                | u32::from(imm13.into()) << 5u32
                | u32::from(Zdn.into()) << 0u32,
        )
    }
}
