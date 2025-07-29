/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod addha_za_pp_z_32 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111110000000000011100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000000100100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "addha_za_pp_z_32";
    #[inline]
    pub const fn addha_za_pp_z_32(
        Pm: ::aarchmrs_types::BitValue<3>,
        Pn: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        ZAda: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000010010000u32 << 16u32
                | Pm.into_inner() << 13u32
                | Pn.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | 0b000u32 << 2u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod addha_za_pp_z_64 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111110000000000011000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000000110100000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "addha_za_pp_z_64";
    #[inline]
    pub const fn addha_za_pp_z_64(
        Pm: ::aarchmrs_types::BitValue<3>,
        Pn: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        ZAda: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000011010000u32 << 16u32
                | Pm.into_inner() << 13u32
                | Pn.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | 0b00u32 << 3u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod addva_za_pp_z_32 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111110000000000011100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000000100100010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "addva_za_pp_z_32";
    #[inline]
    pub const fn addva_za_pp_z_32(
        Pm: ::aarchmrs_types::BitValue<3>,
        Pn: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        ZAda: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000010010001u32 << 16u32
                | Pm.into_inner() << 13u32
                | Pn.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | 0b000u32 << 2u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
pub mod addva_za_pp_z_64 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111110000000000011000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000000110100010000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "addva_za_pp_z_64";
    #[inline]
    pub const fn addva_za_pp_z_64(
        Pm: ::aarchmrs_types::BitValue<3>,
        Pn: ::aarchmrs_types::BitValue<3>,
        Zn: ::aarchmrs_types::BitValue<5>,
        ZAda: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000011010001u32 << 16u32
                | Pm.into_inner() << 13u32
                | Pn.into_inner() << 10u32
                | Zn.into_inner() << 5u32
                | 0b00u32 << 3u32
                | ZAda.into_inner() << 0u32,
        )
    }
}
