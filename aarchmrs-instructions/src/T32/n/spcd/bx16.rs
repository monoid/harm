/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod BX_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00000000000000001111111110000111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000000100011100000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000111u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "BX_T1";
    #[inline]
    pub const fn BX_T1(Rm: ::aarchmrs_types::BitValue<4>) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010001110u32 << 7u32 | Rm.into_inner() << 3u32 | 0b000u32 << 0u32,
        )
    }
}
pub mod BLX_r_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00000000000000001111111110000111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000000100011110000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000111u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "BLX_r_T1";
    #[inline]
    pub const fn BLX_r_T1(Rm: ::aarchmrs_types::BitValue<4>) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010001111u32 << 7u32 | Rm.into_inner() << 3u32 | 0b000u32 << 0u32,
        )
    }
}
