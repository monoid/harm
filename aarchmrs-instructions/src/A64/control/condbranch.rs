/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod B_only_condbranch {
    #[inline]
    pub fn B_only_condbranch(
        imm19: impl Into<::aarchmrs_types::BitValue<19>>,
        cond: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01010100u32 << 24u32
                | u32::from(imm19.into()) << 5u32
                | 0b0u32 << 4u32
                | u32::from(cond.into()) << 0u32,
        )
    }
}
pub mod BC_only_condbranch {
    #[inline]
    pub fn BC_only_condbranch(
        imm19: impl Into<::aarchmrs_types::BitValue<19>>,
        cond: impl Into<::aarchmrs_types::BitValue<4>>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01010100u32 << 24u32
                | u32::from(imm19.into()) << 5u32
                | 0b1u32 << 4u32
                | u32::from(cond.into()) << 0u32,
        )
    }
}
