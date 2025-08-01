/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod zip_mz_z_4q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110001100011u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001101111110000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "zip_mz_z_4q";
    #[inline]
    pub const fn zip_mz_z_4q(
        Zn: ::aarchmrs_types::BitValue<3>,
        Zd: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000100110111111000u32 << 10u32
                | Zn.into_inner() << 7u32
                | 0b00u32 << 5u32
                | Zd.into_inner() << 2u32
                | 0b00u32 << 0u32,
        )
    }
}
pub mod uzp_mz_z_4q {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111111111111110001100011u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11000001001101111110000000000010u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "uzp_mz_z_4q";
    #[inline]
    pub const fn uzp_mz_z_4q(
        Zn: ::aarchmrs_types::BitValue<3>,
        Zd: ::aarchmrs_types::BitValue<3>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b1100000100110111111000u32 << 10u32
                | Zn.into_inner() << 7u32
                | 0b00u32 << 5u32
                | Zd.into_inner() << 2u32
                | 0b10u32 << 0u32,
        )
    }
}
