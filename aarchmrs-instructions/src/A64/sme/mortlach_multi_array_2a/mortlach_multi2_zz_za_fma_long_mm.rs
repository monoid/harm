/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod bfmlal_za_zzw_2x2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000011001110000111100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001101000000000100000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "bfmlal_za_zzw_2x2";
    #[inline]
    pub const fn bfmlal_za_zzw_2x2(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<4>,
        off2: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001101u32 << 21u32
                | Zm.into_inner() << 17u32
                | 0b00u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b010u32 << 10u32
                | Zn.into_inner() << 6u32
                | 0b0100u32 << 2u32
                | off2.into_inner() << 0u32,
        )
    }
}
pub mod fmlal_za_zzw_2x2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000011001110000111100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001101000000000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fmlal_za_zzw_2x2";
    #[inline]
    pub const fn fmlal_za_zzw_2x2(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<4>,
        off2: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001101u32 << 21u32
                | Zm.into_inner() << 17u32
                | 0b00u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b010u32 << 10u32
                | Zn.into_inner() << 6u32
                | 0b0000u32 << 2u32
                | off2.into_inner() << 0u32,
        )
    }
}
pub mod bfmlsl_za_zzw_2x2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000011001110000111100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001101000000000100000011000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "bfmlsl_za_zzw_2x2";
    #[inline]
    pub const fn bfmlsl_za_zzw_2x2(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<4>,
        off2: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001101u32 << 21u32
                | Zm.into_inner() << 17u32
                | 0b00u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b010u32 << 10u32
                | Zn.into_inner() << 6u32
                | 0b0110u32 << 2u32
                | off2.into_inner() << 0u32,
        )
    }
}
pub mod fmlsl_za_zzw_2x2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000011001110000111100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001101000000000100000001000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fmlsl_za_zzw_2x2";
    #[inline]
    pub const fn fmlsl_za_zzw_2x2(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<4>,
        off2: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001101u32 << 21u32
                | Zm.into_inner() << 17u32
                | 0b00u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b010u32 << 10u32
                | Zn.into_inner() << 6u32
                | 0b0010u32 << 2u32
                | off2.into_inner() << 0u32,
        )
    }
}
