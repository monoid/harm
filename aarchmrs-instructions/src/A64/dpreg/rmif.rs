/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod RMIF_only_rmif {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000000111110000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10111010000000000000010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "RMIF_only_rmif";
    #[inline]
    pub const fn RMIF_only_rmif(
        imm6: ::aarchmrs_types::BitValue<6>,
        Rn: ::aarchmrs_types::BitValue<5>,
        mask: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b10111010000u32 << 21u32
                | imm6.into_inner() << 15u32
                | 0b00001u32 << 10u32
                | Rn.into_inner() << 5u32
                | 0b0u32 << 4u32
                | mask.into_inner() << 0u32,
        )
    }
}
