/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod bfmlal_za_zzv_1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001001110000011000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001000000000110000010000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "bfmlal_za_zzv_1";
    #[inline]
    pub const fn bfmlal_za_zzv_1(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<5>,
        off3: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010010u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b011u32 << 10u32
                | Zn.into_inner() << 5u32
                | 0b10u32 << 3u32
                | off3.into_inner() << 0u32,
        )
    }
}
pub mod fmlal_za_zzv_1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001001110000011000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001000000000110000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fmlal_za_zzv_1";
    #[inline]
    pub const fn fmlal_za_zzv_1(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<5>,
        off3: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010010u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b011u32 << 10u32
                | Zn.into_inner() << 5u32
                | 0b00u32 << 3u32
                | off3.into_inner() << 0u32,
        )
    }
}
pub mod bfmlsl_za_zzv_1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001001110000011000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001000000000110000011000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "bfmlsl_za_zzv_1";
    #[inline]
    pub const fn bfmlsl_za_zzv_1(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<5>,
        off3: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010010u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b011u32 << 10u32
                | Zn.into_inner() << 5u32
                | 0b11u32 << 3u32
                | off3.into_inner() << 0u32,
        )
    }
}
pub mod fmlsl_za_zzv_1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001001110000011000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001000000000110000001000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fmlsl_za_zzv_1";
    #[inline]
    pub const fn fmlsl_za_zzv_1(
        Zm: ::aarchmrs_types::BitValue<4>,
        Rv: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<5>,
        off3: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b110000010010u32 << 20u32
                | Zm.into_inner() << 16u32
                | 0b0u32 << 15u32
                | Rv.into_inner() << 13u32
                | 0b011u32 << 10u32
                | Zn.into_inner() << 5u32
                | 0b01u32 << 3u32
                | off3.into_inner() << 0u32,
        )
    }
}
