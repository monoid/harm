/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod UDF_A1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11100111111100000000000011110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UDF_A1";
    #[inline]
    pub const fn UDF_A1(
        imm12: ::aarchmrs_types::BitValue<12>,
        imm4: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111001111111u32 << 20u32
                | imm12.into_inner() << 8u32
                | 0b1111u32 << 4u32
                | imm4.into_inner() << 0u32,
        )
    }
}
