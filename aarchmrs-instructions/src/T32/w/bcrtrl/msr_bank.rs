/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod MSR_br_T1_AS {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111000011101111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110011100000001000000000100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000010000011001111u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "MSR_br_T1_AS";
    #[inline]
    pub const fn MSR_br_T1_AS(
        R: ::aarchmrs_types::BitValue<1>,
        Rn: ::aarchmrs_types::BitValue<4>,
        M1: ::aarchmrs_types::BitValue<4>,
        M: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11110011100u32 << 21u32
                | R.into_inner() << 20u32
                | Rn.into_inner() << 16u32
                | 0b1000u32 << 12u32
                | M1.into_inner() << 8u32
                | 0b001u32 << 5u32
                | M.into_inner() << 4u32
                | 0b0000u32 << 0u32,
        )
    }
}
