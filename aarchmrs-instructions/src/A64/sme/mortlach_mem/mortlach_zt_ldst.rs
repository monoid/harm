/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ldr_zt_br_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000011111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11100001000111111000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ldr_zt_br_";
    #[inline]
    pub const fn ldr_zt_br_(
        Rn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1110000100011111100000u32 << 10u32 | Rn.into_inner() << 5u32 | 0b00000u32 << 0u32,
        )
    }
}
pub mod str_zt_br_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000011111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11100001001111111000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "str_zt_br_";
    #[inline]
    pub const fn str_zt_br_(
        Rn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1110000100111111100000u32 << 10u32 | Rn.into_inner() << 5u32 | 0b00000u32 << 0u32,
        )
    }
}
