/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod incp_z_p_z_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111111111000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001011001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "incp_z_p_z_";
    #[inline]
    pub const fn incp_z_p_z_(
        size: ::aarchmrs_types::BitValue<2>,
        Pm: ::aarchmrs_types::BitValue<4>,
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b1011001000000u32 << 9u32
                | Pm.into_inner() << 5u32
                | Zdn.into_inner() << 0u32,
        )
    }
}
pub mod decp_z_p_z_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111111111000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001011011000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "decp_z_p_z_";
    #[inline]
    pub const fn decp_z_p_z_(
        size: ::aarchmrs_types::BitValue<2>,
        Pm: ::aarchmrs_types::BitValue<4>,
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b1011011000000u32 << 9u32
                | Pm.into_inner() << 5u32
                | Zdn.into_inner() << 0u32,
        )
    }
}
