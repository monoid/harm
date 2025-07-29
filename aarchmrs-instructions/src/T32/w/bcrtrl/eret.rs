/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod SUBS_PC_T5_AS {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111111100000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011110100001000111100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000010111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SUBS_PC_T5_AS";
    #[inline]
    pub const fn SUBS_PC_T5_AS(
        Rn: ::aarchmrs_types::BitValue<4>,
        imm8: ::aarchmrs_types::BitValue<8>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111100111101u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b10001111u32 << 8u32
                | imm8.into_inner() << 0u32,
        )
    }
}
pub mod ERET_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011110111101000111100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000010111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ERET_T1";
    #[inline]
    pub const fn ERET_T1() -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(0b11110011110111101000111100000000u32 << 0u32)
    }
}
