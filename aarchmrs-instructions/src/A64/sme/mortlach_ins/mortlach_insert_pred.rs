/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod mova_za_p_rz_b {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111110000000000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "mova_za_p_rz_b";
    #[inline]
    pub const fn mova_za_p_rz_b(
        V: ::aarchmrs_types::BitValue<1>,
        Rs: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        off4: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000000000000u32 << 16u32
                | V.into_inner() << 15u32
                | Rs.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | 0b0u32 << 4u32
                | off4.into_inner() << 0u32,
        )
    }
}
pub mod mova_za_p_rz_h {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111110000000000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000000010000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "mova_za_p_rz_h";
    #[inline]
    pub const fn mova_za_p_rz_h(
        V: ::aarchmrs_types::BitValue<1>,
        Rs: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        ZAd: ::aarchmrs_types::BitValue<1>,
        off3: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000001000000u32 << 16u32
                | V.into_inner() << 15u32
                | Rs.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | 0b0u32 << 4u32
                | ZAd.into_inner() << 3u32
                | off3.into_inner() << 0u32,
        )
    }
}
pub mod mova_za_p_rz_w {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111110000000000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000000100000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "mova_za_p_rz_w";
    #[inline]
    pub const fn mova_za_p_rz_w(
        V: ::aarchmrs_types::BitValue<1>,
        Rs: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        ZAd: ::aarchmrs_types::BitValue<2>,
        off2: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000010000000u32 << 16u32
                | V.into_inner() << 15u32
                | Rs.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | 0b0u32 << 4u32
                | ZAd.into_inner() << 2u32
                | off2.into_inner() << 0u32,
        )
    }
}
pub mod mova_za_p_rz_d {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111110000000000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000000110000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "mova_za_p_rz_d";
    #[inline]
    pub const fn mova_za_p_rz_d(
        V: ::aarchmrs_types::BitValue<1>,
        Rs: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        ZAd: ::aarchmrs_types::BitValue<3>,
        o1: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000011000000u32 << 16u32
                | V.into_inner() << 15u32
                | Rs.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | 0b0u32 << 4u32
                | ZAd.into_inner() << 1u32
                | o1.into_inner() << 0u32,
        )
    }
}
pub mod mova_za_p_rz_q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111110000000000010000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000000110000010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "mova_za_p_rz_q";
    #[inline]
    pub const fn mova_za_p_rz_q(
        V: ::aarchmrs_types::BitValue<1>,
        Rs: ::aarchmrs_types::BitValue<2>,
        Pg: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        ZAd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000011000001u32 << 16u32
                | V.into_inner() << 15u32
                | Rs.into_inner() << 13u32
                | Pg.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | 0b0u32 << 4u32
                | ZAd.into_inner() << 0u32,
        )
    }
}
