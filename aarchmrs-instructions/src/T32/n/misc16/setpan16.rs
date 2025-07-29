/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod SETPAN_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b00000000000000001111111111110111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000000000000001011011000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000010111u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SETPAN_T1";
    #[inline]
    pub const fn SETPAN_T1(
        imm1: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b101101100001u32 << 4u32 | imm1.into_inner() << 3u32 | 0b000u32 << 0u32,
        )
    }
}
