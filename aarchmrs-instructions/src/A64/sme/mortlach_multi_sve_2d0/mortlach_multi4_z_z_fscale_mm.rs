/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fscale_mz_zzw_4x4 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000111111111111100011u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001000001011100110000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fscale_mz_zzw_4x4";
    #[inline]
    pub const fn fscale_mz_zzw_4x4(
        size: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<3>,
        Zdn: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | size.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Zm.into_inner() << 18u32
                | 0b0010111001100u32 << 5u32
                | Zdn.into_inner() << 2u32
                | 0b00u32 << 0u32,
        )
    }
}
pub mod bfscale_mz_zzw_4x4 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000111111111111100011u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001000001011100110000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "bfscale_mz_zzw_4x4";
    #[inline]
    pub const fn bfscale_mz_zzw_4x4(
        Zm: ::aarchmrs_types::BitValue<3>,
        Zdn: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001001u32 << 21u32
                | Zm.into_inner() << 18u32
                | 0b0010111001100u32 << 5u32
                | Zdn.into_inner() << 2u32
                | 0b00u32 << 0u32,
        )
    }
}
