/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod B_only_condbranch {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111000000000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01010100000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "B_only_condbranch";
    #[inline]
    pub const fn B_only_condbranch(
        imm19: ::aarchmrs_types::BitValue<19>,
        cond: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01010100u32 << 24u32
                | imm19.into_inner() << 5u32
                | 0b0u32 << 4u32
                | cond.into_inner() << 0u32,
        )
    }
}
pub mod BC_only_condbranch {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111000000000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01010100000000000000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "BC_only_condbranch";
    #[inline]
    pub const fn BC_only_condbranch(
        imm19: ::aarchmrs_types::BitValue<19>,
        cond: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01010100u32 << 24u32
                | imm19.into_inner() << 5u32
                | 0b1u32 << 4u32
                | cond.into_inner() << 0u32,
        )
    }
}
