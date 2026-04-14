/* Copyright (c) 2010-2026 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fmop4a_za16_z8z8_b1x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100011111111000111110u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000000001000000000000000001000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fmop4a_za16_z8z8_b1x1";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_ZAda_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_ZAda_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Zn_OFFSET: u32 = 6u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Zn_WIDTH: u32 = 3u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Zm_OFFSET: u32 = 17u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Zm_WIDTH: u32 = 3u32;
    #[inline]
    pub const fn fmop4a_za16_z8z8_b1x1(
        Zm: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b100000000010u32 << 20u32
                | Zm.into_inner() << 17u32
                | 0b00000000u32 << 9u32
                | Zn.into_inner() << 6u32
                | 0b00100u32 << 1u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod fmop4a_za16_z8z8_b1x2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100011111111000111110u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000000001100000000000000001000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fmop4a_za16_z8z8_b1x2";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_ZAda_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_ZAda_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Zn_OFFSET: u32 = 6u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Zn_WIDTH: u32 = 3u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Zm_OFFSET: u32 = 17u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Zm_WIDTH: u32 = 3u32;
    #[inline]
    pub const fn fmop4a_za16_z8z8_b1x2(
        Zm: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b100000000011u32 << 20u32
                | Zm.into_inner() << 17u32
                | 0b00000000u32 << 9u32
                | Zn.into_inner() << 6u32
                | 0b00100u32 << 1u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod fmop4a_za16_z8z8_b2x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100011111111000111110u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000000001000000000001000001000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fmop4a_za16_z8z8_b2x1";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_ZAda_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_ZAda_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Zn_OFFSET: u32 = 6u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Zn_WIDTH: u32 = 3u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Zm_OFFSET: u32 = 17u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Zm_WIDTH: u32 = 3u32;
    #[inline]
    pub const fn fmop4a_za16_z8z8_b2x1(
        Zm: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b100000000010u32 << 20u32
                | Zm.into_inner() << 17u32
                | 0b00000001u32 << 9u32
                | Zn.into_inner() << 6u32
                | 0b00100u32 << 1u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod fmop4a_za16_z8z8_b2x2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100011111111000111110u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b10000000001100000000001000001000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fmop4a_za16_z8z8_b2x2";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_ZAda_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_ZAda_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Zn_OFFSET: u32 = 6u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Zn_WIDTH: u32 = 3u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Zm_OFFSET: u32 = 17u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Zm_WIDTH: u32 = 3u32;
    #[inline]
    pub const fn fmop4a_za16_z8z8_b2x2(
        Zm: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<3>,
        ZAda: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b100000000011u32 << 20u32
                | Zm.into_inner() << 17u32
                | 0b00000001u32 << 9u32
                | Zn.into_inner() << 6u32
                | 0b00100u32 << 1u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
