/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod sunpk_mz_z_4 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111111110000100011u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001101011110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "sunpk_mz_z_4";
    #[inline]
    pub const fn sunpk_mz_z_4(
        size: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<4>,
        Zd: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | size.into_inner() << 22u32
                | 0b110101111000u32 << 10u32
                | Zn.into_inner() << 6u32
                | 0b0u32 << 5u32
                | Zd.into_inner() << 2u32
                | 0b00u32 << 0u32,
        )
    }
}
pub mod uunpk_mz_z_4 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001111111111110000100011u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001101011110000000000001u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "uunpk_mz_z_4";
    #[inline]
    pub const fn uunpk_mz_z_4(
        size: ::aarchmrs_types::BitValue<2>,
        Zn: ::aarchmrs_types::BitValue<4>,
        Zd: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | size.into_inner() << 22u32
                | 0b110101111000u32 << 10u32
                | Zn.into_inner() << 6u32
                | 0b0u32 << 5u32
                | Zd.into_inner() << 2u32
                | 0b01u32 << 0u32,
        )
    }
}
