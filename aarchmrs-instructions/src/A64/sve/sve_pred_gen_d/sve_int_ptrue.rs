/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod ptrue_p_s_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111101111110000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101000110001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ptrue_p_s_";
    #[inline]
    pub const fn ptrue_p_s_(
        size: ::aarchmrs_types::BitValue<2>,
        S: ::aarchmrs_types::BitValue<1>,
        pattern: ::aarchmrs_types::BitValue<5>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b01100u32 << 17u32
                | S.into_inner() << 16u32
                | 0b111000u32 << 10u32
                | pattern.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
pub mod ptrues_p_s_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111101111110000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101000110001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ptrues_p_s_";
    #[inline]
    pub const fn ptrues_p_s_(
        size: ::aarchmrs_types::BitValue<2>,
        S: ::aarchmrs_types::BitValue<1>,
        pattern: ::aarchmrs_types::BitValue<5>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b01100u32 << 17u32
                | S.into_inner() << 16u32
                | 0b111000u32 << 10u32
                | pattern.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
