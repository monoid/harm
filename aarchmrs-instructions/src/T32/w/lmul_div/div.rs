/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod SDIV_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111011100100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SDIV_T1";
    #[inline]
    pub const fn SDIV_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110111001u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b1111u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
pub mod UDIV_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11111011101100001111000011110000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UDIV_T1";
    #[inline]
    pub const fn UDIV_T1(
        Rn: ::aarchmrs_types::BitValue<4>,
        Rd: ::aarchmrs_types::BitValue<4>,
        Rm: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111110111011u32 << 20u32
                | Rn.into_inner() << 16u32
                | 0b1111u32 << 12u32
                | Rd.into_inner() << 8u32
                | 0b1111u32 << 4u32
                | Rm.into_inner() << 0u32,
        )
    }
}
