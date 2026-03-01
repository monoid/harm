/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod movaz_z_rza_b {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111110001111000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000000000000100000001000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "movaz_z_rza_b";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Zd_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Zd_WIDTH: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_off4_OFFSET: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_off4_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rs_OFFSET: u32 = 13u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rs_WIDTH: u32 = 2u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_V_OFFSET: u32 = 15u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_V_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn movaz_z_rza_b(
        V: ::aarchmrs_types::BitValue<1>,
        Rs: ::aarchmrs_types::BitValue<2>,
        off4: ::aarchmrs_types::BitValue<4>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000000000010u32 << 16u32
                | V.into_inner() << 15u32
                | Rs.into_inner() << 13u32
                | 0b0001u32 << 9u32
                | off4.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod movaz_z_rza_h {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111110001111000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000000010000100000001000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "movaz_z_rza_h";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Zd_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Zd_WIDTH: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_off3_OFFSET: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_off3_WIDTH: u32 = 3u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_ZAn_OFFSET: u32 = 8u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_ZAn_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rs_OFFSET: u32 = 13u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rs_WIDTH: u32 = 2u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_V_OFFSET: u32 = 15u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_V_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn movaz_z_rza_h(
        V: ::aarchmrs_types::BitValue<1>,
        Rs: ::aarchmrs_types::BitValue<2>,
        ZAn: ::aarchmrs_types::BitValue<1>,
        off3: ::aarchmrs_types::BitValue<3>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000001000010u32 << 16u32
                | V.into_inner() << 15u32
                | Rs.into_inner() << 13u32
                | 0b0001u32 << 9u32
                | ZAn.into_inner() << 8u32
                | off3.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod movaz_z_rza_w {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111110001111000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000000100000100000001000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "movaz_z_rza_w";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Zd_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Zd_WIDTH: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_off2_OFFSET: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_off2_WIDTH: u32 = 2u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_ZAn_OFFSET: u32 = 7u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_ZAn_WIDTH: u32 = 2u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rs_OFFSET: u32 = 13u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rs_WIDTH: u32 = 2u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_V_OFFSET: u32 = 15u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_V_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn movaz_z_rza_w(
        V: ::aarchmrs_types::BitValue<1>,
        Rs: ::aarchmrs_types::BitValue<2>,
        ZAn: ::aarchmrs_types::BitValue<2>,
        off2: ::aarchmrs_types::BitValue<2>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000010000010u32 << 16u32
                | V.into_inner() << 15u32
                | Rs.into_inner() << 13u32
                | 0b0001u32 << 9u32
                | ZAn.into_inner() << 7u32
                | off2.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod movaz_z_rza_d {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111110001111000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000000110000100000001000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "movaz_z_rza_d";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Zd_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Zd_WIDTH: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_o1_OFFSET: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_o1_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_ZAn_OFFSET: u32 = 6u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_ZAn_WIDTH: u32 = 3u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rs_OFFSET: u32 = 13u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rs_WIDTH: u32 = 2u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_V_OFFSET: u32 = 15u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_V_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn movaz_z_rza_d(
        V: ::aarchmrs_types::BitValue<1>,
        Rs: ::aarchmrs_types::BitValue<2>,
        ZAn: ::aarchmrs_types::BitValue<3>,
        o1: ::aarchmrs_types::BitValue<1>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000011000010u32 << 16u32
                | V.into_inner() << 15u32
                | Rs.into_inner() << 13u32
                | 0b0001u32 << 9u32
                | ZAn.into_inner() << 6u32
                | o1.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod movaz_z_rza_q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111110001111000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000000110000110000001000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "movaz_z_rza_q";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Zd_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Zd_WIDTH: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_ZAn_OFFSET: u32 = 5u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_ZAn_WIDTH: u32 = 4u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rs_OFFSET: u32 = 13u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rs_WIDTH: u32 = 2u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_V_OFFSET: u32 = 15u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_V_WIDTH: u32 = 1u32;
    #[inline]
    pub const fn movaz_z_rza_q(
        V: ::aarchmrs_types::BitValue<1>,
        Rs: ::aarchmrs_types::BitValue<2>,
        ZAn: ::aarchmrs_types::BitValue<4>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000011000011u32 << 16u32
                | V.into_inner() << 15u32
                | Rs.into_inner() << 13u32
                | 0b0001u32 << 9u32
                | ZAn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
