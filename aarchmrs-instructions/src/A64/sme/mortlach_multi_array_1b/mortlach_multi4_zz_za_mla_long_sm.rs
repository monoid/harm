/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod smlal_za_zzv_4x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001001110000011100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001011100000000100000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "smlal_za_zzv_4x1";
    #[inline]
    pub const fn smlal_za_zzv_4x1(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<5>,
        off2: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010111u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b010u32 << 10u32
                | Zn.into_inner() << 5u32
                | 0b000u32 << 2u32
                | off2.into_inner() << 0u32,
        )
    }
}
pub mod smlsl_za_zzv_4x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001001110000011100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001011100000000100000001000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "smlsl_za_zzv_4x1";
    #[inline]
    pub const fn smlsl_za_zzv_4x1(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<5>,
        off2: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010111u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b010u32 << 10u32
                | Zn.into_inner() << 5u32
                | 0b010u32 << 2u32
                | off2.into_inner() << 0u32,
        )
    }
}
pub mod umlal_za_zzv_4x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001001110000011100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001011100000000100000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "umlal_za_zzv_4x1";
    #[inline]
    pub const fn umlal_za_zzv_4x1(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<5>,
        off2: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010111u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b010u32 << 10u32
                | Zn.into_inner() << 5u32
                | 0b100u32 << 2u32
                | off2.into_inner() << 0u32,
        )
    }
}
pub mod umlsl_za_zzv_4x1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001001110000011100u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001011100000000100000011000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "umlsl_za_zzv_4x1";
    #[inline]
    pub const fn umlsl_za_zzv_4x1(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<5>,
        off2: ::aarchmrs_types::BitValue<2>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010111u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b010u32 << 10u32
                | Zn.into_inner() << 5u32
                | 0b110u32 << 2u32
                | off2.into_inner() << 0u32,
        )
    }
}
