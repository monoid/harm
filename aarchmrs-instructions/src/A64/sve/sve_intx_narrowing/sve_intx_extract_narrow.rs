/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod sqxtnb_z_zz_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101001111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101001000000100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqxtnb_z_zz_";
    #[inline]
    pub const fn sqxtnb_z_zz_(
        tszh: ::aarchmrs_types::BitValue<1>,
        tszl: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010001010u32 << 23u32
                | tszh.into_inner() << 22u32
                | 0b1u32 << 21u32
                | tszl.into_inner() << 19u32
                | 0b000010000u32 << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod sqxtunb_z_zz_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101001111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101001000000101000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqxtunb_z_zz_";
    #[inline]
    pub const fn sqxtunb_z_zz_(
        tszh: ::aarchmrs_types::BitValue<1>,
        tszl: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010001010u32 << 23u32
                | tszh.into_inner() << 22u32
                | 0b1u32 << 21u32
                | tszl.into_inner() << 19u32
                | 0b000010100u32 << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod sqxtnt_z_zz_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101001111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101001000000100010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqxtnt_z_zz_";
    #[inline]
    pub const fn sqxtnt_z_zz_(
        tszh: ::aarchmrs_types::BitValue<1>,
        tszl: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010001010u32 << 23u32
                | tszh.into_inner() << 22u32
                | 0b1u32 << 21u32
                | tszl.into_inner() << 19u32
                | 0b000010001u32 << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod sqxtunt_z_zz_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101001111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101001000000101010000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sqxtunt_z_zz_";
    #[inline]
    pub const fn sqxtunt_z_zz_(
        tszh: ::aarchmrs_types::BitValue<1>,
        tszl: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010001010u32 << 23u32
                | tszh.into_inner() << 22u32
                | 0b1u32 << 21u32
                | tszl.into_inner() << 19u32
                | 0b000010101u32 << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod uqxtnb_z_zz_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101001111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101001000000100100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "uqxtnb_z_zz_";
    #[inline]
    pub const fn uqxtnb_z_zz_(
        tszh: ::aarchmrs_types::BitValue<1>,
        tszl: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010001010u32 << 23u32
                | tszh.into_inner() << 22u32
                | 0b1u32 << 21u32
                | tszl.into_inner() << 19u32
                | 0b000010010u32 << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
pub mod uqxtnt_z_zz_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101001111111110000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000101001000000100110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "uqxtnt_z_zz_";
    #[inline]
    pub const fn uqxtnt_z_zz_(
        tszh: ::aarchmrs_types::BitValue<1>,
        tszl: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010001010u32 << 23u32
                | tszh.into_inner() << 22u32
                | 0b1u32 << 21u32
                | tszl.into_inner() << 19u32
                | 0b000010011u32 << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 0u32,
        )
    }
}
