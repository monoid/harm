/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod BXJ_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011110000001000111100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000010111111111111u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "BXJ_T1";
    #[inline]
    pub const fn BXJ_T1(Rm: ::aarchmrs_types::BitValue<4>) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100111100u32 << 20u32 | Rm.into_inner() << 16u32 | 0b1000111100000000u32 << 0u32,
        )
    }
}
