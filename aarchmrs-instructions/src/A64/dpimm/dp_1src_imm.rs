/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod AUTIASPPC_only_dp_1src_imm {
    #[inline]
    pub fn AUTIASPPC_only_dp_1src_imm(
        imm16: impl Into<::aarchmrs_types::BitValue<16>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110011100u32 << 21u32 | u32::from(imm16.into()) << 5u32 | 0b11111u32 << 0u32,
        )
    }
}
pub mod AUTIBSPPC_only_dp_1src_imm {
    #[inline]
    pub fn AUTIBSPPC_only_dp_1src_imm(
        imm16: impl Into<::aarchmrs_types::BitValue<16>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110011101u32 << 21u32 | u32::from(imm16.into()) << 5u32 | 0b11111u32 << 0u32,
        )
    }
}
