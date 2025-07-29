/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod sqincp_z_p_z_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111101111111000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001010001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqincp_z_p_z_";
    #[inline]
    pub const fn sqincp_z_p_z_(
        size: ::aarchmrs_types::BitValue<2>,
        U: ::aarchmrs_types::BitValue<1>,
        Pm: ::aarchmrs_types::BitValue<4>,
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b10100u32 << 17u32
                | U.into_inner() << 16u32
                | 0b1000000u32 << 9u32
                | Pm.into_inner() << 5u32
                | Zdn.into_inner() << 0u32,
        )
    }
}
pub mod sqdecp_z_p_z_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111101111111000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001010101000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqdecp_z_p_z_";
    #[inline]
    pub const fn sqdecp_z_p_z_(
        size: ::aarchmrs_types::BitValue<2>,
        U: ::aarchmrs_types::BitValue<1>,
        Pm: ::aarchmrs_types::BitValue<4>,
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b10101u32 << 17u32
                | U.into_inner() << 16u32
                | 0b1000000u32 << 9u32
                | Pm.into_inner() << 5u32
                | Zdn.into_inner() << 0u32,
        )
    }
}
pub mod uqincp_z_p_z_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111101111111000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001010001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "uqincp_z_p_z_";
    #[inline]
    pub const fn uqincp_z_p_z_(
        size: ::aarchmrs_types::BitValue<2>,
        U: ::aarchmrs_types::BitValue<1>,
        Pm: ::aarchmrs_types::BitValue<4>,
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b10100u32 << 17u32
                | U.into_inner() << 16u32
                | 0b1000000u32 << 9u32
                | Pm.into_inner() << 5u32
                | Zdn.into_inner() << 0u32,
        )
    }
}
pub mod uqdecp_z_p_z_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111101111111000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b00100101001010101000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "uqdecp_z_p_z_";
    #[inline]
    pub const fn uqdecp_z_p_z_(
        size: ::aarchmrs_types::BitValue<2>,
        U: ::aarchmrs_types::BitValue<1>,
        Pm: ::aarchmrs_types::BitValue<4>,
        Zdn: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b00100101u32 << 24u32
                | size.into_inner() << 22u32
                | 0b10101u32 << 17u32
                | U.into_inner() << 16u32
                | 0b1000000u32 << 9u32
                | Pm.into_inner() << 5u32
                | Zdn.into_inner() << 0u32,
        )
    }
}
