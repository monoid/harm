/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod srshl_mz_zzw_4x4 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000111111111111100010u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001000001011101000100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "srshl_mz_zzw_4x4";
    #[inline]
    pub const fn srshl_mz_zzw_4x4(
        size: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<3>,
        Zdn: ::aarchmrs_types::BitValue<3>,
        U: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | size.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Zm.into_inner() << 18u32
                | 0b0010111010001u32 << 5u32
                | Zdn.into_inner() << 2u32
                | 0b0u32 << 1u32
                | U.into_inner() << 0u32,
        )
    }
}
pub mod urshl_mz_zzw_4x4 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000111111111111100010u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001000001011101000100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "urshl_mz_zzw_4x4";
    #[inline]
    pub const fn urshl_mz_zzw_4x4(
        size: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<3>,
        Zdn: ::aarchmrs_types::BitValue<3>,
        U: ::aarchmrs_types::BitValue<1>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | size.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Zm.into_inner() << 18u32
                | 0b0010111010001u32 << 5u32
                | Zdn.into_inner() << 2u32
                | 0b0u32 << 1u32
                | U.into_inner() << 0u32,
        )
    }
}
