/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod RETAASPPC_only_miscbranch {
    #[inline]
    pub fn RETAASPPC_only_miscbranch(
        imm16: impl Into<::aarchmrs_types::BitValue<16>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01010101000u32 << 21u32 | u32::from(imm16.into()) << 5u32 | 0b11111u32 << 0u32,
        )
    }
}
pub mod RETABSPPC_only_miscbranch {
    #[inline]
    pub fn RETABSPPC_only_miscbranch(
        imm16: impl Into<::aarchmrs_types::BitValue<16>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01010101001u32 << 21u32 | u32::from(imm16.into()) << 5u32 | 0b11111u32 << 0u32,
        )
    }
}
