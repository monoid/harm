/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod punpklo_p_p_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111101111111000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000101001100000100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "punpklo_p_p_";
    #[inline]
    pub const fn punpklo_p_p_(
        H: ::aarchmrs_types::BitValue<1>,
        Pn: ::aarchmrs_types::BitValue<4>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b000001010011000u32 << 17u32
                | H.into_inner() << 16u32
                | 0b0100000u32 << 9u32
                | Pn.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
pub mod punpkhi_p_p_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111101111111000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00000101001100000100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "punpkhi_p_p_";
    #[inline]
    pub const fn punpkhi_p_p_(
        H: ::aarchmrs_types::BitValue<1>,
        Pn: ::aarchmrs_types::BitValue<4>,
        Pd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b000001010011000u32 << 17u32
                | H.into_inner() << 16u32
                | 0b0100000u32 << 9u32
                | Pn.into_inner() << 5u32
                | 0b0u32 << 4u32
                | Pd.into_inner() << 0u32,
        )
    }
}
