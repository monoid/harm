/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod mova_za2_z_b1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111110001110000111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000000000001000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "mova_za2_z_b1";
    #[inline]
    pub const fn mova_za2_z_b1(
        V: ::aarchmrs_types::BitValue<1>,
        Rs: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<4>,
        off3: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000000000100u32 << 16u32
                | V.into_inner() << 15u32
                | Rs.into_inner() << 13u32
                | 0b000u32 << 10u32
                | Zn.into_inner() << 6u32
                | 0b000u32 << 3u32
                | off3.into_inner() << 0u32,
        )
    }
}
pub mod mova_za2_z_h1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111110001110000111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000000010001000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "mova_za2_z_h1";
    #[inline]
    pub const fn mova_za2_z_h1(
        V: ::aarchmrs_types::BitValue<1>,
        Rs: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<4>,
        ZAd: ::aarchmrs_types::BitValue<1>,
        off2: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000001000100u32 << 16u32
                | V.into_inner() << 15u32
                | Rs.into_inner() << 13u32
                | 0b000u32 << 10u32
                | Zn.into_inner() << 6u32
                | 0b000u32 << 3u32
                | ZAd.into_inner() << 2u32
                | off2.into_inner() << 0u32,
        )
    }
}
pub mod mova_za2_z_w1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111110001110000111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000000100001000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "mova_za2_z_w1";
    #[inline]
    pub const fn mova_za2_z_w1(
        V: ::aarchmrs_types::BitValue<1>,
        Rs: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<4>,
        ZAd: ::aarchmrs_types::BitValue<2>,
        o1: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000010000100u32 << 16u32
                | V.into_inner() << 15u32
                | Rs.into_inner() << 13u32
                | 0b000u32 << 10u32
                | Zn.into_inner() << 6u32
                | 0b000u32 << 3u32
                | ZAd.into_inner() << 1u32
                | o1.into_inner() << 0u32,
        )
    }
}
pub mod mova_za2_z_d1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111110001110000111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000000110001000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "mova_za2_z_d1";
    #[inline]
    pub const fn mova_za2_z_d1(
        V: ::aarchmrs_types::BitValue<1>,
        Rs: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<4>,
        ZAd: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000011000100u32 << 16u32
                | V.into_inner() << 15u32
                | Rs.into_inner() << 13u32
                | 0b000u32 << 10u32
                | Zn.into_inner() << 6u32
                | 0b000u32 << 3u32
                | ZAd.into_inner() << 0u32,
        )
    }
}
