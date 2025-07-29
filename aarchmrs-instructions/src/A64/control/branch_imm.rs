/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod B_only_branch_imm {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111100000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00010100000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "B_only_branch_imm";
    #[inline]
    pub const fn B_only_branch_imm(
        imm26: ::aarchmrs_types::BitValue<26>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b000101u32 << 26u32 | imm26.into_inner() << 0u32,
        )
    }
}
pub mod BL_only_branch_imm {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111100000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10010100000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "BL_only_branch_imm";
    #[inline]
    pub const fn BL_only_branch_imm(
        imm26: ::aarchmrs_types::BitValue<26>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b100101u32 << 26u32 | imm26.into_inner() << 0u32,
        )
    }
}
