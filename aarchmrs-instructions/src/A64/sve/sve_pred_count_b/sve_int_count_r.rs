/* Copyright (c) 2010-2026 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod incp_r_p_r_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111111111000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001011001000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "incp_r_p_r_";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rdn_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rdn_WIDTH: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Pm_OFFSET: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Pm_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_size_OFFSET: u32 = 22u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_size_WIDTH: u32 = 2u32;
    #[inline]
    pub const fn incp_r_p_r_(
        size: ::aarchmrs_types::BitValue<2>,
        Pm: ::aarchmrs_types::BitValue<4>,
        Rdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b1011001000100u32 << 9u32
                | Pm.into_inner() << 5u32
                | Rdn.into_inner() << 0u32,
        )
    }
}
pub mod decp_r_p_r_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111111111000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001011011000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "decp_r_p_r_";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rdn_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rdn_WIDTH: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Pm_OFFSET: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Pm_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_size_OFFSET: u32 = 22u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_size_WIDTH: u32 = 2u32;
    #[inline]
    pub const fn decp_r_p_r_(
        size: ::aarchmrs_types::BitValue<2>,
        Pm: ::aarchmrs_types::BitValue<4>,
        Rdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b1011011000100u32 << 9u32
                | Pm.into_inner() << 5u32
                | Rdn.into_inner() << 0u32,
        )
    }
}
