/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod scvtf_mz_z_2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000100001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001000101110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "scvtf_mz_z_2";
    #[inline]
    pub const fn scvtf_mz_z_2(
        Zn: ::aarchmrs_types::BitValue<4>,
        Zd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000100100010111000u32 << 10u32
                | Zn.into_inner() << 6u32
                | 0b0u32 << 5u32
                | Zd.into_inner() << 1u32
                | 0b0u32 << 0u32,
        )
    }
}
pub mod ucvtf_mz_z_2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110000100001u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001000101110000000100000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ucvtf_mz_z_2";
    #[inline]
    pub const fn ucvtf_mz_z_2(
        Zn: ::aarchmrs_types::BitValue<4>,
        Zd: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000100100010111000u32 << 10u32
                | Zn.into_inner() << 6u32
                | 0b1u32 << 5u32
                | Zd.into_inner() << 1u32
                | 0b0u32 << 0u32,
        )
    }
}
