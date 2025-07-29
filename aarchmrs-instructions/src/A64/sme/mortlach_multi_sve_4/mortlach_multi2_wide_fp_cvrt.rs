/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod fcvt_mz2_z_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001101000001110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fcvt_mz2_z_";
    #[inline]
    pub const fn fcvt_mz2_z_(
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000110100000111000u32 << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 1u32
                | 0b0u32 << 0u32,
        )
    }
}
pub mod fcvtl_mz2_z_ {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000000001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001101000001110000000000001u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "fcvtl_mz2_z_";
    #[inline]
    pub const fn fcvtl_mz2_z_(
        Zn: ::aarchmrs_types::BitValue<5>,
        Zd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000110100000111000u32 << 10u32
                | Zn.into_inner() << 5u32
                | Zd.into_inner() << 1u32
                | 0b1u32 << 0u32,
        )
    }
}
