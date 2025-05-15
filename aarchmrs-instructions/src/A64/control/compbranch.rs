/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod CBZ_32_compbranch {
    #[inline]
    pub fn CBZ_32_compbranch(
        imm19: impl Into<::aarchmrs_types::BitValue<19>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00110100u32 << 24u32 | u32::from(imm19.into()) << 5u32 | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod CBNZ_32_compbranch {
    #[inline]
    pub fn CBNZ_32_compbranch(
        imm19: impl Into<::aarchmrs_types::BitValue<19>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00110101u32 << 24u32 | u32::from(imm19.into()) << 5u32 | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod CBZ_64_compbranch {
    #[inline]
    pub fn CBZ_64_compbranch(
        imm19: impl Into<::aarchmrs_types::BitValue<19>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10110100u32 << 24u32 | u32::from(imm19.into()) << 5u32 | u32::from(Rt.into()) << 0u32,
        )
    }
}
pub mod CBNZ_64_compbranch {
    #[inline]
    pub fn CBNZ_64_compbranch(
        imm19: impl Into<::aarchmrs_types::BitValue<19>>,
        Rt: impl Into<::aarchmrs_types::BitValue<5>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10110101u32 << 24u32 | u32::from(imm19.into()) << 5u32 | u32::from(Rt.into()) << 0u32,
        )
    }
}
