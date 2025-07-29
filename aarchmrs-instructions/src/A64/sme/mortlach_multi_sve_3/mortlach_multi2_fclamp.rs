/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fclamp_mz_zz_2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111001000001111110000000001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fclamp_mz_zz_2";
    #[inline]
    pub const fn fclamp_mz_zz_2(
        size: ::aarchmrs_types::BitValue<2>,
        Zm: ::aarchmrs_types::BitValue<5>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001u32 << 24u32
                | size.into_inner() << 22u32
                | 0b1u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b110000u32 << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 1u32
                | 0b0u32 << 0u32,
        )
    }
}
pub mod bfclamp_mz_zz_2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111000001111110000000001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001000001100000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "bfclamp_mz_zz_2";
    #[inline]
    pub const fn bfclamp_mz_zz_2(
        Zm: ::aarchmrs_types::BitValue<5>,
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b11000001001u32 << 21u32
                | Zm.into_inner() << 16u32
                | 0b110000u32 << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 1u32
                | 0b0u32 << 0u32,
        )
    }
}
