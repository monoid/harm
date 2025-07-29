/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod TBB_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101000110100001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "TBB_T1";
    #[inline]
    pub const fn TBB_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010001101u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b111100000000u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod TBH_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111111111110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11101000110100001111000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111111100000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "TBH_T1";
    #[inline]
    pub const fn TBH_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111010001101u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b111100000001u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
