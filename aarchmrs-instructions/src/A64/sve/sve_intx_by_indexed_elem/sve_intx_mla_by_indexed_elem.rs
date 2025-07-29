/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod mla_z_zzzi_h {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000001111100000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000100001000000000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "mla_z_zzzi_h";
    #[inline]
    pub const fn mla_z_zzzi_h(
        i3h: ::aarchmrs_types::BitValue<1>,
        i3l: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<3>,
        S: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zda: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010001000u32 << 23u32
                | i3h.into_inner() << 22u32
                | 0b1u32 << 21u32
                | i3l.into_inner() << 19u32
                | Zm.into_inner() << 16u32
                | 0b00001u32 << 11u32
                | S.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zda.into_inner() << 0u32,
        )
    }
}
pub mod mla_z_zzzi_s {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111100000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000100101000000000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "mla_z_zzzi_s";
    #[inline]
    pub const fn mla_z_zzzi_s(
        i2: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<3>,
        S: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zda: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100101u32 << 21u32
                | i2.into_inner() << 19u32
                | Zm.into_inner() << 16u32
                | 0b00001u32 << 11u32
                | S.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zda.into_inner() << 0u32,
        )
    }
}
pub mod mla_z_zzzi_d {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111100000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000100111000000000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "mla_z_zzzi_d";
    #[inline]
    pub const fn mla_z_zzzi_d(
        i1: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<4>,
        S: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zda: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100111u32 << 21u32
                | i1.into_inner() << 20u32
                | Zm.into_inner() << 16u32
                | 0b00001u32 << 11u32
                | S.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zda.into_inner() << 0u32,
        )
    }
}
pub mod mls_z_zzzi_h {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111101000001111100000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000100001000000000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "mls_z_zzzi_h";
    #[inline]
    pub const fn mls_z_zzzi_h(
        i3h: ::aarchmrs_types::BitValue<1>,
        i3l: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<3>,
        S: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zda: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b010001000u32 << 23u32
                | i3h.into_inner() << 22u32
                | 0b1u32 << 21u32
                | i3l.into_inner() << 19u32
                | Zm.into_inner() << 16u32
                | 0b00001u32 << 11u32
                | S.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zda.into_inner() << 0u32,
        )
    }
}
pub mod mls_z_zzzi_s {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111100000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000100101000000000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "mls_z_zzzi_s";
    #[inline]
    pub const fn mls_z_zzzi_s(
        i2: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<3>,
        S: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zda: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100101u32 << 21u32
                | i2.into_inner() << 19u32
                | Zm.into_inner() << 16u32
                | 0b00001u32 << 11u32
                | S.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zda.into_inner() << 0u32,
        )
    }
}
pub mod mls_z_zzzi_d {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111100000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b01000100111000000000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "mls_z_zzzi_d";
    #[inline]
    pub const fn mls_z_zzzi_d(
        i1: ::aarchmrs_types::BitValue<1>,
        Zm: ::aarchmrs_types::BitValue<4>,
        S: ::aarchmrs_types::BitValue<1>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zda: ::aarchmrs_types::BitValue<5>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b01000100111u32 << 21u32
                | i1.into_inner() << 20u32
                | Zm.into_inner() << 16u32
                | 0b00001u32 << 11u32
                | S.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | Zda.into_inner() << 0u32,
        )
    }
}
