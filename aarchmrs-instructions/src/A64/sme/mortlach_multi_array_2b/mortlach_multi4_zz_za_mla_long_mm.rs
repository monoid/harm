/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod smlal_za_zzw_4x4 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000111001110001111100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001111000010000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "smlal_za_zzw_4x4";
    #[inline]
    pub const fn smlal_za_zzw_4x4(
        Zm: ::aarchmrs_types::BitValue<3>,
        Rv: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<3>,
        off2: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001111u32 << 21u32
                | Zm.into_inner() << 18u32
                | 0b010u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b010u32 << 10u32
                | Zn.into_inner() << 7u32
                | 0b00000u32 << 2u32
                | off2.into_inner() << 0u32,
        )
    }
}
pub mod smlsl_za_zzw_4x4 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000111001110001111100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001111000010000100000001000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "smlsl_za_zzw_4x4";
    #[inline]
    pub const fn smlsl_za_zzw_4x4(
        Zm: ::aarchmrs_types::BitValue<3>,
        Rv: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<3>,
        off2: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001111u32 << 21u32
                | Zm.into_inner() << 18u32
                | 0b010u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b010u32 << 10u32
                | Zn.into_inner() << 7u32
                | 0b00010u32 << 2u32
                | off2.into_inner() << 0u32,
        )
    }
}
pub mod umlal_za_zzw_4x4 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000111001110001111100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001111000010000100000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "umlal_za_zzw_4x4";
    #[inline]
    pub const fn umlal_za_zzw_4x4(
        Zm: ::aarchmrs_types::BitValue<3>,
        Rv: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<3>,
        off2: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001111u32 << 21u32
                | Zm.into_inner() << 18u32
                | 0b010u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b010u32 << 10u32
                | Zn.into_inner() << 7u32
                | 0b00100u32 << 2u32
                | off2.into_inner() << 0u32,
        )
    }
}
pub mod umlsl_za_zzw_4x4 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000111001110001111100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001111000010000100000011000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "umlsl_za_zzw_4x4";
    #[inline]
    pub const fn umlsl_za_zzw_4x4(
        Zm: ::aarchmrs_types::BitValue<3>,
        Rv: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<3>,
        off2: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001111u32 << 21u32
                | Zm.into_inner() << 18u32
                | 0b010u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b010u32 << 10u32
                | Zn.into_inner() << 7u32
                | 0b00110u32 << 2u32
                | off2.into_inner() << 0u32,
        )
    }
}
