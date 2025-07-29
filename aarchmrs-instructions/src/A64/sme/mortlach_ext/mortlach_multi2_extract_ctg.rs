/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod mova_mz2_za_b1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111110001111100000001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000000000001100000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "mova_mz2_za_b1";
    #[inline]
    pub const fn mova_mz2_za_b1(
        V: ::aarchmrs_types::BitValue<1>,
        Rs: ::aarchmrs_types::BitValue<2>,
        off3: ::aarchmrs_types::BitValue<3>,
        Zd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000000000110u32 << 16u32
                | V.into_inner() << 15u32
                | Rs.into_inner() << 13u32
                | 0b00000u32 << 8u32
                | off3.into_inner() << 5u32
                | Zd.into_inner() << 1u32
                | 0b0u32 << 0u32,
        )
    }
}
pub mod mova_mz2_za_h1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111110001111100000001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000000010001100000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "mova_mz2_za_h1";
    #[inline]
    pub const fn mova_mz2_za_h1(
        V: ::aarchmrs_types::BitValue<1>,
        Rs: ::aarchmrs_types::BitValue<2>,
        ZAn: ::aarchmrs_types::BitValue<1>,
        off2: ::aarchmrs_types::BitValue<2>,
        Zd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000001000110u32 << 16u32
                | V.into_inner() << 15u32
                | Rs.into_inner() << 13u32
                | 0b00000u32 << 8u32
                | ZAn.into_inner() << 7u32
                | off2.into_inner() << 5u32
                | Zd.into_inner() << 1u32
                | 0b0u32 << 0u32,
        )
    }
}
pub mod mova_mz2_za_w1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111110001111100000001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000000100001100000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "mova_mz2_za_w1";
    #[inline]
    pub const fn mova_mz2_za_w1(
        V: ::aarchmrs_types::BitValue<1>,
        Rs: ::aarchmrs_types::BitValue<2>,
        ZAn: ::aarchmrs_types::BitValue<2>,
        o1: ::aarchmrs_types::BitValue<1>,
        Zd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000010000110u32 << 16u32
                | V.into_inner() << 15u32
                | Rs.into_inner() << 13u32
                | 0b00000u32 << 8u32
                | ZAn.into_inner() << 6u32
                | o1.into_inner() << 5u32
                | Zd.into_inner() << 1u32
                | 0b0u32 << 0u32,
        )
    }
}
pub mod mova_mz2_za_d1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111110001111100000001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000000110001100000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "mova_mz2_za_d1";
    #[inline]
    pub const fn mova_mz2_za_d1(
        V: ::aarchmrs_types::BitValue<1>,
        Rs: ::aarchmrs_types::BitValue<2>,
        ZAn: ::aarchmrs_types::BitValue<3>,
        Zd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000011000110u32 << 16u32
                | V.into_inner() << 15u32
                | Rs.into_inner() << 13u32
                | 0b00000u32 << 8u32
                | ZAn.into_inner() << 5u32
                | Zd.into_inner() << 1u32
                | 0b0u32 << 0u32,
        )
    }
}
