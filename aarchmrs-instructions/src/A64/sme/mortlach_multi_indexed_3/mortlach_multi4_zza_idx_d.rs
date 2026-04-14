/* Copyright (c) 2010-2026 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fmla_za_zzi_d4xi {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001001100001111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001110100001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fmla_za_zzi_d4xi";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_off3_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_off3_WIDTH: u32 = 3u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Zn_OFFSET: u32 = 7u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Zn_WIDTH: u32 = 3u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_i1_OFFSET: u32 = 10u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_i1_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rv_OFFSET: u32 = 13u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rv_WIDTH: u32 = 2u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Zm_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Zm_WIDTH: u32 = 4u32;
    #[inline]
    pub const fn fmla_za_zzi_d4xi(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        i1: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<3>,
        off3: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000011101u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b00u32 << 11u32
                | i1.into_inner() << 10u32
                | Zn.into_inner() << 7u32
                | 0b0000u32 << 3u32
                | off3.into_inner() << 0u32,
        )
    }
}
pub mod sdot_za_zzi_d4xi {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001001100001111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001110100001000000000001000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sdot_za_zzi_d4xi";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_off3_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_off3_WIDTH: u32 = 3u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Zn_OFFSET: u32 = 7u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Zn_WIDTH: u32 = 3u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_i1_OFFSET: u32 = 10u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_i1_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rv_OFFSET: u32 = 13u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rv_WIDTH: u32 = 2u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Zm_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Zm_WIDTH: u32 = 4u32;
    #[inline]
    pub const fn sdot_za_zzi_d4xi(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        i1: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<3>,
        off3: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000011101u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b00u32 << 11u32
                | i1.into_inner() << 10u32
                | Zn.into_inner() << 7u32
                | 0b0001u32 << 3u32
                | off3.into_inner() << 0u32,
        )
    }
}
pub mod svdot_za_zzi_d4xi {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001001100001111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001110100001000100000001000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "svdot_za_zzi_d4xi";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_off3_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_off3_WIDTH: u32 = 3u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Zn_OFFSET: u32 = 7u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Zn_WIDTH: u32 = 3u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_i1_OFFSET: u32 = 10u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_i1_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rv_OFFSET: u32 = 13u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rv_WIDTH: u32 = 2u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Zm_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Zm_WIDTH: u32 = 4u32;
    #[inline]
    pub const fn svdot_za_zzi_d4xi(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        i1: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<3>,
        off3: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000011101u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b01u32 << 11u32
                | i1.into_inner() << 10u32
                | Zn.into_inner() << 7u32
                | 0b0001u32 << 3u32
                | off3.into_inner() << 0u32,
        )
    }
}
pub mod fmls_za_zzi_d4xi {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001001100001111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001110100001000000000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fmls_za_zzi_d4xi";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_off3_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_off3_WIDTH: u32 = 3u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Zn_OFFSET: u32 = 7u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Zn_WIDTH: u32 = 3u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_i1_OFFSET: u32 = 10u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_i1_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rv_OFFSET: u32 = 13u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rv_WIDTH: u32 = 2u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Zm_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Zm_WIDTH: u32 = 4u32;
    #[inline]
    pub const fn fmls_za_zzi_d4xi(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        i1: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<3>,
        off3: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000011101u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b00u32 << 11u32
                | i1.into_inner() << 10u32
                | Zn.into_inner() << 7u32
                | 0b0010u32 << 3u32
                | off3.into_inner() << 0u32,
        )
    }
}
pub mod udot_za_zzi_d4xi {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001001100001111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001110100001000000000011000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "udot_za_zzi_d4xi";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_off3_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_off3_WIDTH: u32 = 3u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Zn_OFFSET: u32 = 7u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Zn_WIDTH: u32 = 3u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_i1_OFFSET: u32 = 10u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_i1_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rv_OFFSET: u32 = 13u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rv_WIDTH: u32 = 2u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Zm_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Zm_WIDTH: u32 = 4u32;
    #[inline]
    pub const fn udot_za_zzi_d4xi(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        i1: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<3>,
        off3: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000011101u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b00u32 << 11u32
                | i1.into_inner() << 10u32
                | Zn.into_inner() << 7u32
                | 0b0011u32 << 3u32
                | off3.into_inner() << 0u32,
        )
    }
}
pub mod uvdot_za_zzi_d4xi {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001001100001111000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001110100001000100000011000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "uvdot_za_zzi_d4xi";
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_off3_OFFSET: u32 = 0u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_off3_WIDTH: u32 = 3u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Zn_OFFSET: u32 = 7u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Zn_WIDTH: u32 = 3u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_i1_OFFSET: u32 = 10u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_i1_WIDTH: u32 = 1u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rv_OFFSET: u32 = 13u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Rv_WIDTH: u32 = 2u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Zm_OFFSET: u32 = 16u32;
    #[cfg(feature = "meta_field")]
    #[allow(nonstandard_style)]
    pub const FIELD_Zm_WIDTH: u32 = 4u32;
    #[inline]
    pub const fn uvdot_za_zzi_d4xi(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        i1: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<3>,
        off3: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000011101u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b1u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b01u32 << 11u32
                | i1.into_inner() << 10u32
                | Zn.into_inner() << 7u32
                | 0b0011u32 << 3u32
                | off3.into_inner() << 0u32,
        )
    }
}
