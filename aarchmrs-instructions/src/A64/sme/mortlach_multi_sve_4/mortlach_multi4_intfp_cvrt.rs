/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod scvtf_mz_z_4 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110001000011u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001100101110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "scvtf_mz_z_4";
    #[inline]
    pub const fn scvtf_mz_z_4(
        Zn: ::aarchmrs_types::BitValue<3>,
        U: ::aarchmrs_types::BitValue<1>,
        Zd: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000100110010111000u32 << 10u32
                | Zn.into_inner() << 7u32
                | 0b0u32 << 6u32
                | U.into_inner() << 5u32
                | Zd.into_inner() << 2u32
                | 0b00u32 << 0u32,
        )
    }
}
pub mod ucvtf_mz_z_4 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110001000011u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001100101110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "ucvtf_mz_z_4";
    #[inline]
    pub const fn ucvtf_mz_z_4(
        Zn: ::aarchmrs_types::BitValue<3>,
        U: ::aarchmrs_types::BitValue<1>,
        Zd: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000100110010111000u32 << 10u32
                | Zn.into_inner() << 7u32
                | 0b0u32 << 6u32
                | U.into_inner() << 5u32
                | Zd.into_inner() << 2u32
                | 0b00u32 << 0u32,
        )
    }
}
