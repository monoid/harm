/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod SETPAN_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110001000100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000011111111110100001111u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SETPAN_A1";
    #[inline]
    pub const fn SETPAN_A1(
        imm1: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1111000100010000000000u32 << 10u32
                | imm1.into_inner() << 9u32
                | 0b000000000u32 << 0u32,
        )
    }
}
