/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod add_za_zw_2x2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101111111001110000111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001101000000001110000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "add_za_zw_2x2";
    #[inline]
    pub const fn add_za_zw_2x2(
        sz: ::aarchmrs_types::BitValue<1>,
        Rv: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<4>,
        off3: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000011u32 << 23u32
                | sz.into_inner() << 22u32
                | 0b1000000u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b111u32 << 10u32
                | Zm.into_inner() << 6u32
                | 0b010u32 << 3u32
                | off3.into_inner() << 0u32,
        )
    }
}
pub mod sub_za_zw_2x2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101111111001110000111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001101000000001110000011000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sub_za_zw_2x2";
    #[inline]
    pub const fn sub_za_zw_2x2(
        sz: ::aarchmrs_types::BitValue<1>,
        Rv: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<4>,
        off3: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000011u32 << 23u32
                | sz.into_inner() << 22u32
                | 0b1000000u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b111u32 << 10u32
                | Zm.into_inner() << 6u32
                | 0b011u32 << 3u32
                | off3.into_inner() << 0u32,
        )
    }
}
