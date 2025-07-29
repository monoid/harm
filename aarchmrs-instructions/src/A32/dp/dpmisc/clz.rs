/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod CLZ_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00001111111111110000111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000001011011110000111100010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000011110000111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "CLZ_A1";
    #[inline]
    pub const fn CLZ_A1(
        cond: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            cond.into_inner() << 28u32
                | 0b000101101111u32 << 16u32
                | Rd.into_inner() << 12u32
                | 0b11110001u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
